use super::*;
use crate::audio::sfx::SfxEvent;
use common::{
    comp::{
        item::tool::ToolCategory, CharacterAbilityType, CharacterState, Item, ItemConfig, Loadout,
    },
    states,
};
use std::time::{Duration, Instant};

#[test]
fn maps_wield_while_equipping() {
    let mut loadout = Loadout::default();

    loadout.active_item = Some(ItemConfig {
        item: Item::new_from_asset_expect("common.items.weapons.axe.starter_axe"),
        ability1: None,
        ability2: None,
        ability3: None,
        block_ability: None,
        dodge_ability: None,
    });

    let result = CombatEventMapper::map_event(
        &CharacterState::Equipping(states::equipping::Data {
            time_left: Duration::from_millis(10),
        }),
        &PreviousEntityState {
            event: SfxEvent::Idle,
            time: Instant::now(),
            weapon_drawn: false,
        },
        Some(&loadout),
    );

    assert_eq!(result, SfxEvent::Wield(ToolCategory::Axe));
}

#[test]
fn maps_unwield() {
    let mut loadout = Loadout::default();

    loadout.active_item = Some(ItemConfig {
        item: Item::new_from_asset_expect("common.items.weapons.bow.starter_bow"),
        ability1: None,
        ability2: None,
        ability3: None,
        block_ability: None,
        dodge_ability: None,
    });

    let result = CombatEventMapper::map_event(
        &CharacterState::default(),
        &PreviousEntityState {
            event: SfxEvent::Idle,
            time: Instant::now(),
            weapon_drawn: true,
        },
        Some(&loadout),
    );

    assert_eq!(result, SfxEvent::Unwield(ToolCategory::Bow));
}

#[test]
fn maps_basic_melee() {
    let mut loadout = Loadout::default();

    loadout.active_item = Some(ItemConfig {
        item: Item::new_from_asset_expect("common.items.weapons.axe.starter_axe"),
        ability1: None,
        ability2: None,
        ability3: None,
        block_ability: None,
        dodge_ability: None,
    });

    let result = CombatEventMapper::map_event(
        &CharacterState::BasicMelee(states::basic_melee::Data {
            buildup_duration: Duration::default(),
            recover_duration: Duration::default(),
            knockback: 0.0,
            base_healthchange: 10,
            range: 1.0,
            max_angle: 1.0,
            exhausted: false,
        }),
        &PreviousEntityState {
            event: SfxEvent::Idle,
            time: Instant::now(),
            weapon_drawn: true,
        },
        Some(&loadout),
    );

    assert_eq!(
        result,
        SfxEvent::Attack(CharacterAbilityType::BasicMelee, ToolCategory::Axe)
    );
}

#[test]
fn matches_ability_stage() {
    let mut loadout = Loadout::default();

    loadout.active_item = Some(ItemConfig {
        item: Item::new_from_asset_expect("common.items.weapons.sword.starter_sword"),
        ability1: None,
        ability2: None,
        ability3: None,
        block_ability: None,
        dodge_ability: None,
    });

    let result = CombatEventMapper::map_event(
        &CharacterState::ComboMelee(states::combo_melee::Data {
            static_data: states::combo_melee::StaticData {
                num_stages: 1,
                stage_data: vec![states::combo_melee::Stage {
                    stage: 1,
                    base_damage: 100,
                    max_damage: 120,
                    damage_increase: 10,
                    knockback: 10.0,
                    range: 4.0,
                    angle: 30.0,
                    base_buildup_duration: Duration::from_millis(500),
                    base_swing_duration: Duration::from_millis(200),
                    base_recover_duration: Duration::from_millis(400),
                    forward_movement: 0.5,
                }],
                initial_energy_gain: 0,
                max_energy_gain: 100,
                energy_increase: 20,
                speed_increase: 0.05,
                max_speed_increase: 1.8,
                is_interruptible: true,
            },
            stage: 1,
            combo: 0,
            timer: Duration::default(),
            stage_section: states::utils::StageSection::Swing,
            next_stage: false,
        }),
        &PreviousEntityState {
            event: SfxEvent::Idle,
            time: Instant::now(),
            weapon_drawn: true,
        },
        Some(&loadout),
    );

    assert_eq!(
        result,
        SfxEvent::Attack(
            CharacterAbilityType::ComboMelee(states::utils::StageSection::Swing, 1),
            ToolCategory::Sword
        )
    );
}

#[test]
fn ignores_different_ability_stage() {
    let mut loadout = Loadout::default();

    loadout.active_item = Some(ItemConfig {
        item: Item::new_from_asset_expect("common.items.weapons.sword.starter_sword"),
        ability1: None,
        ability2: None,
        ability3: None,
        block_ability: None,
        dodge_ability: None,
    });

    let result = CombatEventMapper::map_event(
        &CharacterState::ComboMelee(states::combo_melee::Data {
            static_data: states::combo_melee::StaticData {
                num_stages: 1,
                stage_data: vec![states::combo_melee::Stage {
                    stage: 1,
                    base_damage: 100,
                    max_damage: 120,
                    damage_increase: 10,
                    knockback: 10.0,
                    range: 4.0,
                    angle: 30.0,
                    base_buildup_duration: Duration::from_millis(500),
                    base_swing_duration: Duration::from_millis(200),
                    base_recover_duration: Duration::from_millis(400),
                    forward_movement: 0.5,
                }],
                initial_energy_gain: 0,
                max_energy_gain: 100,
                energy_increase: 20,
                speed_increase: 0.05,
                max_speed_increase: 1.8,
                is_interruptible: true,
            },
            stage: 1,
            combo: 0,
            timer: Duration::default(),
            stage_section: states::utils::StageSection::Swing,
            next_stage: false,
        }),
        &PreviousEntityState {
            event: SfxEvent::Idle,
            time: Instant::now(),
            weapon_drawn: true,
        },
        Some(&loadout),
    );

    assert_ne!(
        result,
        SfxEvent::Attack(
            CharacterAbilityType::ComboMelee(states::utils::StageSection::Swing, 2),
            ToolCategory::Sword
        )
    );
}
