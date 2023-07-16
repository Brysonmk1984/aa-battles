use crate::{
    match_up::match_up::{Battalion, BattleArmy},
    service::query::Army,
};

use super::tick::run_tick::run_tick;

#[derive(Debug)]
pub struct BattleResult {
    id: i32,
    winner: i32,
    loser: i32,
}

#[derive(Debug)]
struct Battle {
    army_1_state: Vec<Battalion>,
    army_2_state: Vec<Battalion>,
}

pub fn run_battle(battle_tuple: (BattleArmy, BattleArmy)) -> BattleResult {
    //println!("{battle_tuple:?}");
    //dbg!(battle_tuple);

    let battle = Battle {
        army_1_state: battle_tuple.0.full_army,
        army_2_state: battle_tuple.1.full_army,
    };

    run_tick(battle.army_1_state, battle.army_2_state);

    //run_battle_update(battle);

    // return results
    BattleResult {
        id: 1,
        winner: 1,
        loser: 2,
    }
}

fn run_battle_update(mut battle: Battle) {
    battle.army_1_state[0].position = 999;

    println!("{battle:?}")
}

type Position = i32;

pub trait Marching {
    fn march(position: Position, speed: i32) -> Position;
}

enum WeaponType {
    Piercing,
    Edged,
    Crushing,
    Concussive,
    Magic,
}

enum ArmorType {
    Unarmored,
    Leather,
    ChainMail,
    PlateMail,
}

struct DefenderStats {
    defender_armor_type: ArmorType,
    defender_shield_rating: f32,
    defender_speed: i32,
    defender_agility: f32,
}

enum AttackOutcome {
    Hit,
    Miss,
    Blocked,
    Evaded,
}

trait Attacking {
    fn attack(weapon_type: WeaponType, defender_stats: DefenderStats) -> AttackOutcome;
}
