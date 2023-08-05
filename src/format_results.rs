use crate::{
    battle::battle::{BattleResult, Belligerent},
    match_up::match_up::Battalion,
    BattleState,
};

fn format_battalion_state(battalion: &Battalion) -> String {
    let Battalion {
        name,
        count,
        position,
        ..
    } = battalion;
    format!("{name} - {count} at position {position}")
}

fn format_army_state(mut army_state: Vec<Battalion>) -> String {
    army_state.sort_by(|a, b| b.count.cmp(&a.count));
    let formatted_vec: String = army_state
        .iter()
        .map(|b| format_battalion_state(b))
        .collect::<Vec<String>>()
        .join("\n");

    format!("{formatted_vec}")
}

pub fn format_battle_state(battle_state: BattleState, battle_result: &BattleResult) -> String {
    let mut winning_army: (Belligerent, String);
    let mut losing_army: (Belligerent, String);
    if let Belligerent::WesternArmy = battle_result.winner.as_ref().unwrap() {
        winning_army = (
            Belligerent::WesternArmy,
            format_army_state(battle_state.army_1_state),
        );
        losing_army = (
            Belligerent::EasternArmy,
            format_army_state(battle_state.army_2_state),
        );
    } else {
        winning_army = (
            Belligerent::EasternArmy,
            format_army_state(battle_state.army_2_state),
        );
        losing_army = (
            Belligerent::WesternArmy,
            format_army_state(battle_state.army_1_state),
        );
    }

    format!(
        "\nWinner ({}):\n----------------------\n{}\n\nLoser ({}):\n----------------------\n{}\n",
        winning_army.0, winning_army.1, losing_army.0, losing_army.1
    )
}

pub fn format_outcome(battle_result: BattleResult) -> String {
    let result = format!(
        "Battle ID: {}\n{} Wins\n{}\nTick Count: {}",
        battle_result.id,
        battle_result.winner.unwrap().to_string(),
        battle_result.win_type.unwrap().to_string(),
        battle_result.tick_count
    );
    format!("\nBATTLE RESULTS:\n-------------\n{result}\n")
}
