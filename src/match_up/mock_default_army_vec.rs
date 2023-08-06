use std::collections::HashMap;

use crate::{service::query::Army, util::create_hash_of_defaults};

pub fn get_mock_defaults() -> HashMap<&'static str, Army> {
    let mut vec_of_mock_defaults = vec![
        Army {
            id: 7,
            name: "Amazonian Huntresses".to_string(),
            lore: "TBD".to_string(),
            count: 1,
            shield_rating: 0.0,
            flying: false,
            range: 5,
            attack_speed: 0.0,
            accuracy: 1.0,
            aoe: false,
            weapon_type: "piercing".to_string(),
            armor_type: "unarmored".to_string(),
            agility: 0.25,
            speed: 5,
        },
        Army {
            id: 8,
            name: "Avian Cliff Dwellers".to_string(),
            lore: "TBD".to_string(),
            count: 1,
            shield_rating: 0.26,
            flying: true,
            range: 10,
            attack_speed: 0.0,
            accuracy: 0.9,
            aoe: false,
            weapon_type: "piercing".to_string(),
            armor_type: "unarmored".to_string(),
            agility: 0.25,
            speed: 5,
        },
        Army {
            id: 4,
            name: "Highborn Cavalry".to_string(),
            lore: "TBD".to_string(),
            count: 1,
            shield_rating: 0.5,
            flying: false,
            range: 10,
            attack_speed: 0.0,
            accuracy: 0.9,
            aoe: false,
            weapon_type: "piercing".to_string(),
            armor_type: "unarmored".to_string(),
            agility: 0.25,
            speed: 10,
        },
        Army {
            id: 2,
            name: "Imperial Legionnaires".to_string(),
            lore: "TBD".to_string(),
            count: 1,
            shield_rating: 0.75,
            flying: false,
            range: 10,
            attack_speed: 0.0,
            accuracy: 1.0,
            aoe: false,
            weapon_type: "piercing".to_string(),
            armor_type: "unarmored".to_string(),
            agility: 0.25,
            speed: 5,
        },
        Army {
            id: 9,
            name: "Magi Enforcers".to_string(),
            lore: "TBD".to_string(),
            count: 1,
            shield_rating: 0.0,
            flying: false,
            range: 15,
            attack_speed: 0.0,
            accuracy: 1.0,
            aoe: false,
            weapon_type: "piercing".to_string(),
            armor_type: "unarmored".to_string(),
            agility: 0.1,
            speed: 5,
        },
        Army {
            id: 3,
            name: "North Watch Longbowmen".to_string(),
            lore: "TBD".to_string(),
            count: 1,
            shield_rating: 0.0,
            flying: false,
            range: 150,
            attack_speed: 0.0,
            accuracy: 0.75,
            aoe: false,
            weapon_type: "piercing".to_string(),
            armor_type: "unarmored".to_string(),
            agility: 0.25,
            speed: 5,
        },
        Army {
            id: 1,
            name: "Peacekeeper Monks".to_string(),
            lore: "TBD".to_string(),
            count: 1,
            shield_rating: 0.0,
            flying: false,
            range: 5,
            attack_speed: 0.0,
            accuracy: 1.0,
            aoe: false,
            weapon_type: "piercing".to_string(),
            armor_type: "unarmored".to_string(),
            agility: 0.5,
            speed: 5,
        },
        Army {
            id: 5,
            name: "Rōnin Immortals".to_string(),
            lore: "TBD".to_string(),
            count: 1,
            shield_rating: 0.0,
            flying: false,
            range: 5,
            attack_speed: 0.0,
            accuracy: 1.0,
            aoe: false,
            weapon_type: "piercing".to_string(),
            armor_type: "unarmored".to_string(),
            agility: 0.25,
            speed: 5,
        },
        Army {
            id: 6,
            name: "Shinobi Assassins".to_string(),
            lore: "TBD".to_string(),
            count: 1,
            shield_rating: 0.0,
            flying: false,
            range: 5,
            attack_speed: 0.0,
            accuracy: 1.0,
            aoe: false,
            weapon_type: "piercing".to_string(),
            armor_type: "unarmored".to_string(),
            agility: 0.5,
            speed: 5,
        },
        Army {
            id: 10,
            name: "Skull Clan Death Cultists".to_string(),
            lore: "TBD".to_string(),
            count: 1,
            shield_rating: 0.0,
            flying: false,
            range: 100,
            attack_speed: 0.0,
            accuracy: 0.75,
            aoe: false,
            weapon_type: "piercing".to_string(),
            armor_type: "unarmored".to_string(),
            agility: 0.25,
            speed: 5,
        },
    ];
    vec_of_mock_defaults.sort_by(|a, b| a.name.cmp(&b.name));

    create_hash_of_defaults(vec_of_mock_defaults)
}