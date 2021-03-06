use crate::{
    comp::{
        projectile, Damage, DamageSource, Energy, EnergySource, Group, HealthChange, HealthSource,
        Loadout, Ori, PhysicsState, Pos, Projectile, Vel,
    },
    event::{EventBus, LocalEvent, ServerEvent},
    metrics::SysMetrics,
    span,
    state::DeltaTime,
    sync::UidAllocator,
    util::Dir,
};
use specs::{
    saveload::MarkerAllocator, Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage,
};
use std::time::Duration;
use vek::*;

/// This system is responsible for handling projectile effect triggers
pub struct Sys;
impl<'a> System<'a> for Sys {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        Read<'a, DeltaTime>,
        Read<'a, UidAllocator>,
        Read<'a, EventBus<LocalEvent>>,
        Read<'a, EventBus<ServerEvent>>,
        ReadExpect<'a, SysMetrics>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, PhysicsState>,
        ReadStorage<'a, Vel>,
        WriteStorage<'a, Ori>,
        WriteStorage<'a, Projectile>,
        WriteStorage<'a, Energy>,
        ReadStorage<'a, Loadout>,
        ReadStorage<'a, Group>,
    );

    fn run(
        &mut self,
        (
            entities,
            dt,
            uid_allocator,
            local_bus,
            server_bus,
            sys_metrics,
            positions,
            physics_states,
            velocities,
            mut orientations,
            mut projectiles,
            mut energies,
            loadouts,
            groups,
        ): Self::SystemData,
    ) {
        let start_time = std::time::Instant::now();
        span!(_guard, "run", "projectile::Sys::run");
        let mut local_emitter = local_bus.emitter();
        let mut server_emitter = server_bus.emitter();

        // Attacks
        for (entity, pos, physics, ori, projectile) in (
            &entities,
            &positions,
            &physics_states,
            &mut orientations,
            &mut projectiles,
        )
            .join()
        {
            // Hit entity
            for other in physics.touch_entities.iter().copied() {
                if projectile.ignore_group
                    // Skip if in the same group
                    && projectile
                        .owner
                        // Note: somewhat inefficient since we do the lookup for every touching
                        // entity, but if we pull this out of the loop we would want to do it only
                        // if there is at least one touching entity
                        .and_then(|uid| uid_allocator.retrieve_entity_internal(uid.into()))
                        .and_then(|e| groups.get(e))
                        .map_or(false, |owner_group|
                            Some(owner_group) == uid_allocator
                            .retrieve_entity_internal(other.into())
                            .and_then(|e| groups.get(e))
                        )
                {
                    continue;
                }

                if projectile.owner == Some(other) {
                    continue;
                }

                for effect in projectile.hit_entity.drain(..) {
                    match effect {
                        projectile::Effect::Damage(healthchange) => {
                            let owner_uid = projectile.owner.unwrap();
                            let mut damage = Damage {
                                healthchange: healthchange as f32,
                                source: DamageSource::Projectile,
                            };

                            let other_entity = uid_allocator.retrieve_entity_internal(other.into());
                            if let Some(loadout) = other_entity.and_then(|e| loadouts.get(e)) {
                                damage.modify_damage(false, loadout);
                            }

                            if other != owner_uid {
                                if damage.healthchange < 0.0 {
                                    server_emitter.emit(ServerEvent::Damage {
                                        uid: other,
                                        change: HealthChange {
                                            amount: damage.healthchange as i32,
                                            cause: HealthSource::Projectile {
                                                owner: Some(owner_uid),
                                            },
                                        },
                                    });
                                } else if damage.healthchange > 0.0 {
                                    server_emitter.emit(ServerEvent::Damage {
                                        uid: other,
                                        change: HealthChange {
                                            amount: damage.healthchange as i32,
                                            cause: HealthSource::Healing {
                                                by: Some(owner_uid),
                                            },
                                        },
                                    });
                                }
                            }
                        },
                        projectile::Effect::Knockback(knockback) => {
                            if let Some(entity) =
                                uid_allocator.retrieve_entity_internal(other.into())
                            {
                                local_emitter.emit(LocalEvent::ApplyImpulse {
                                    entity,
                                    impulse: knockback
                                        * *Dir::slerp(ori.0, Dir::new(Vec3::unit_z()), 0.5),
                                });
                            }
                        },
                        projectile::Effect::RewardEnergy(energy) => {
                            if let Some(energy_mut) = projectile
                                .owner
                                .and_then(|o| uid_allocator.retrieve_entity_internal(o.into()))
                                .and_then(|o| energies.get_mut(o))
                            {
                                energy_mut.change_by(energy as i32, EnergySource::HitEnemy);
                            }
                        },
                        projectile::Effect::Explode(e) => {
                            server_emitter.emit(ServerEvent::Explosion {
                                pos: pos.0,
                                explosion: e,
                                owner: projectile.owner,
                                friendly_damage: false,
                                reagent: None,
                            })
                        },
                        projectile::Effect::Vanish => server_emitter.emit(ServerEvent::Destroy {
                            entity,
                            cause: HealthSource::World,
                        }),
                        projectile::Effect::Possess => {
                            if other != projectile.owner.unwrap() {
                                if let Some(owner) = projectile.owner {
                                    server_emitter.emit(ServerEvent::Possess(owner, other));
                                }
                            }
                        },
                        _ => {},
                    }
                }
            }

            // Hit something solid
            if physics.on_wall.is_some() || physics.on_ground || physics.on_ceiling {
                for effect in projectile.hit_solid.drain(..) {
                    match effect {
                        projectile::Effect::Explode(e) => {
                            server_emitter.emit(ServerEvent::Explosion {
                                pos: pos.0,
                                explosion: e,
                                owner: projectile.owner,
                                friendly_damage: false,
                                reagent: None,
                            })
                        },
                        projectile::Effect::Vanish => server_emitter.emit(ServerEvent::Destroy {
                            entity,
                            cause: HealthSource::World,
                        }),
                        _ => {},
                    }
                }
            } else if let Some(dir) = velocities
                .get(entity)
                .and_then(|vel| vel.0.try_normalized())
            {
                ori.0 = dir.into();
            }

            if projectile.time_left == Duration::default() {
                server_emitter.emit(ServerEvent::Destroy {
                    entity,
                    cause: HealthSource::World,
                });
            }
            projectile.time_left = projectile
                .time_left
                .checked_sub(Duration::from_secs_f32(dt.0))
                .unwrap_or_default();
        }
        sys_metrics.projectile_ns.store(
            start_time.elapsed().as_nanos() as i64,
            std::sync::atomic::Ordering::Relaxed,
        );
    }
}
