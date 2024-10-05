use std::collections::HashMap;

use crate::{
    entities::{
        army::Army, game_defaults::GameDefaults, nation::Nation,
        nation_army::nation_army::NationArmy,
    },
    enums::{ArmorType, ArmyName, StartingDirection, WeaponType},
    match_up::create_mocks::create_mock_generic_battalion,
    util::create_hash_of_defaults,
};

use super::nation_army::NationArmyMock;

pub struct GameDefaultsMocks;

impl GameDefaultsMocks {
    pub fn generate_weapon_armor_hash() -> HashMap<String, f64> {
        HashMap::from([
            ("piercing-unarmored".to_string(), 1.0),
            ("piercing-leather".to_string(), 0.75),
            ("piercing-chain".to_string(), 0.6),
            ("piercing-plate".to_string(), 0.1),
            ("crushing-unarmored".to_string(), 0.25),
            ("crushing-leather".to_string(), 0.50),
            ("crushing-chain".to_string(), 0.75),
            ("crushing-plate".to_string(), 1.0),
            ("blunt-unarmored".to_string(), 0.75),
            ("blunt-leather".to_string(), 0.75),
            ("blunt-chain".to_string(), 0.5),
            ("blunt-plate".to_string(), 0.25),
            ("edged-unarmored".to_string(), 1.0),
            ("edged-leather".to_string(), 0.75),
            ("edged-chain".to_string(), 0.5),
            ("edged-plate".to_string(), 0.25),
            ("magic-unarmored".to_string(), 0.25),
            ("magic-leather".to_string(), 0.50),
            ("magic-chain".to_string(), 1.0),
            ("magic-plate".to_string(), 0.75),
        ])
    }

    pub fn generate_mock_army_defaults() -> HashMap<ArmyName, Army> {
        let mut vec_of_mock_defaults = vec![
            Army {
                id: 1,
                name: ArmyName::PeacekeeperMonks,
                count: 1,
                shield_rating: 0.0,
                flying: false,
                range: 5,
                attack_speed: 1,
                accuracy: 1.0,
                aoe: 0.00,
                spread: 1.00,
                weapon_type: WeaponType::Blunt,
                armor_type: ArmorType::Unarmored,
                agility: 0.25,
                speed: 5,
            },
            Army {
                id: 2,
                name: ArmyName::ImperialLegionnaires,
                count: 1,
                shield_rating: 0.75,
                flying: false,
                range: 10,
                attack_speed: 1,
                accuracy: 1.0,
                aoe: 0.00,
                spread: 1.00,
                weapon_type: WeaponType::Edged,
                armor_type: ArmorType::Plate,
                agility: 0.25,
                speed: 5,
            },
            Army {
                id: 3,
                name: ArmyName::NorthWatchLongbowmen,
                count: 1,
                shield_rating: 0.0,
                flying: false,
                range: 150,
                attack_speed: 1,
                accuracy: 0.75,
                aoe: 0.00,
                spread: 1.00,
                weapon_type: WeaponType::Piercing,
                armor_type: ArmorType::Unarmored,
                agility: 0.25,
                speed: 5,
            },
            Army {
                id: 4,
                name: ArmyName::HighbornCavalry,
                count: 1,
                shield_rating: 0.5,
                flying: false,
                range: 10,
                attack_speed: 2,
                accuracy: 0.9,
                aoe: 0.00,
                spread: 1.00,
                weapon_type: WeaponType::Edged,
                armor_type: ArmorType::Plate,
                agility: 0.25,
                speed: 10,
            },
            Army {
                id: 5,
                name: ArmyName::RoninImmortals,
                count: 1,
                shield_rating: 0.0,
                flying: false,
                range: 5,
                attack_speed: 2,
                accuracy: 1.0,
                aoe: 0.00,
                spread: 1.00,
                weapon_type: WeaponType::Edged,
                armor_type: ArmorType::Chain,
                agility: 0.25,
                speed: 5,
            },
            Army {
                id: 6,
                name: ArmyName::ShinobiMartialArtists,
                count: 1,
                shield_rating: 0.0,
                flying: false,
                range: 5,
                attack_speed: 3,
                accuracy: 1.0,
                aoe: 0.00,
                spread: 1.00,
                weapon_type: WeaponType::Blunt,
                armor_type: ArmorType::Leather,
                agility: 0.5,
                speed: 5,
            },
            Army {
                id: 7,
                name: ArmyName::AmazonianHuntresses,
                count: 1,
                shield_rating: 0.0,
                flying: false,
                range: 5,
                attack_speed: 1,
                accuracy: 1.0,
                aoe: 0.00,
                spread: 1.00,
                weapon_type: WeaponType::Piercing,
                armor_type: ArmorType::Leather,
                agility: 0.25,
                speed: 5,
            },
            Army {
                id: 8,
                name: ArmyName::AvianCliffDwellers,
                count: 1,
                shield_rating: 0.26,
                flying: true,
                range: 10,
                attack_speed: 1,
                accuracy: 0.9,
                aoe: 0.00,
                spread: 1.00,
                weapon_type: WeaponType::Edged,
                armor_type: ArmorType::Leather,
                agility: 0.25,
                speed: 5,
            },
            Army {
                id: 9,
                name: ArmyName::MagiEnforcers,
                count: 1,
                shield_rating: 0.0,
                flying: false,
                range: 15,
                attack_speed: 1,
                accuracy: 1.0,
                aoe: 0.00,
                spread: 1.00,
                weapon_type: WeaponType::Magic,
                armor_type: ArmorType::Chain,
                agility: 0.1,
                speed: 5,
            },
            Army {
                id: 10,
                name: ArmyName::SkullClanDeathCultists,
                count: 1,
                shield_rating: 0.0,
                flying: false,
                range: 100,
                attack_speed: 1,
                accuracy: 0.75,
                aoe: 3.00,
                spread: 1.00,
                weapon_type: WeaponType::Magic,
                armor_type: ArmorType::Unarmored,
                agility: 0.25,
                speed: 5,
            },
            Army {
                id: 17,
                name: ArmyName::BarbariansOfTheOuterSteppe,
                count: 1,
                shield_rating: 0.0,
                flying: false,
                range: 100,
                attack_speed: 1,
                accuracy: 0.75,
                aoe: 0.00,
                spread: 2.00,
                weapon_type: WeaponType::Crushing,
                armor_type: ArmorType::Chain,
                agility: 0.25,
                speed: 10,
            },
            Army {
                id: 18,
                name: ArmyName::OathSwornKnights,
                count: 1,
                shield_rating: 0.5,
                flying: false,
                range: 5,
                attack_speed: 1,
                accuracy: 0.75,
                aoe: 0.00,
                spread: 1.00,
                weapon_type: WeaponType::Edged,
                armor_type: ArmorType::Plate,
                agility: 0.25,
                speed: 5,
            },
            Army {
                id: 19,
                name: ArmyName::MinuteMenMilitia,
                count: 1,
                shield_rating: 0.0,
                flying: false,
                range: 5,
                attack_speed: 1,
                accuracy: 0.75,
                aoe: 0.00,
                spread: 1.00,
                weapon_type: WeaponType::Edged,
                armor_type: ArmorType::Unarmored,
                agility: 0.25,
                speed: 5,
            },
            Army {
                id: 20,
                name: ArmyName::DeathDealerAssassins,
                count: 1,
                shield_rating: 0.0,
                flying: false,
                range: 10,
                attack_speed: 3,
                accuracy: 1.0,
                aoe: 0.00,
                spread: 3.00,
                weapon_type: WeaponType::Edged,
                armor_type: ArmorType::Unarmored,
                agility: 0.35,
                speed: 10,
            },
            Army {
                id: 21,
                name: ArmyName::ElvenArchers,
                count: 1,
                shield_rating: 0.0,
                flying: false,
                range: 100,
                attack_speed: 2,
                accuracy: 0.9,
                aoe: 0.00,
                spread: 2.00,
                weapon_type: WeaponType::Piercing,
                armor_type: ArmorType::Leather,
                agility: 0.25,
                speed: 10,
            },
            Army {
                id: 22,
                name: ArmyName::CastlegateCrossbowmen,
                count: 1,
                shield_rating: 0.0,
                flying: false,
                range: 75,
                attack_speed: 2,
                accuracy: 0.75,
                aoe: 0.00,
                spread: 1.00,
                weapon_type: WeaponType::Piercing,
                armor_type: ArmorType::Chain,
                agility: 0.15,
                speed: 5,
            },
        ];

        create_hash_of_defaults(vec_of_mock_defaults)
    }

    pub fn generate_aoe_spread_hash() -> HashMap<i32, Vec<(f64, i32)>> {
        let spread_one_vec = vec![
            (0.0, 1),
            (0.5, 2),
            (1.0, 5),
            (1.5, 9),
            (2.0, 13),
            (2.5, 20),
            (3.0, 33),
        ];
        let spread_two_vec = vec![
            (0.0, 1),
            (0.5, 1),
            (1.0, 2),
            (1.5, 3),
            (2.0, 5),
            (2.5, 7),
            (3.0, 9),
        ];
        let spread_three_vec = vec![
            (0.0, 1),
            (0.5, 1),
            (1.0, 1),
            (1.5, 2),
            (2.0, 2),
            (2.5, 3),
            (3.0, 5),
        ];
        let hash = HashMap::from([
            (1, spread_one_vec),
            (2, spread_two_vec),
            (3, spread_three_vec),
        ]);
        hash
    }

    pub fn generate_example_competitors(
        east_count: i32,
        west_count: i32,
    ) -> ((Nation, Vec<NationArmy>), (Nation, Vec<NationArmy>)) {
        let competitor_one = (
            Nation {
                ..Default::default()
            },
            vec![NationArmy::from(NationArmyMock::new(east_count))],
        );
        let competitor_two = (
            Nation {
                ..Default::default()
            },
            vec![NationArmy::from(NationArmyMock::new(west_count))],
        );

        (competitor_one, competitor_two)
    }
}

pub fn get_game_defaults() -> GameDefaults {
    GameDefaults {
        weapons_vs_armor: GameDefaultsMocks::generate_weapon_armor_hash(),
        aoe_vs_spread: GameDefaultsMocks::generate_aoe_spread_hash(),
        army_defaults: GameDefaultsMocks::generate_mock_army_defaults(),
        environment: String::from("test"),
    }
}

pub fn get_competitors(
    east_count: i32,
    west_count: i32,
) -> ((Nation, Vec<NationArmy>), (Nation, Vec<NationArmy>)) {
    GameDefaultsMocks::generate_example_competitors(east_count, west_count)
}
