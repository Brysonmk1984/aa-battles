use super::battle_result::BattleResult;

impl BattleResult {
    /**
     * Formats the final tally and outcome to be printed to the command line and the log
     */
    pub fn format_outcome(&self) -> String {
        let result = format!(
            "Battle ID: {}\n{} Wins\n{}\nTick Count: {}",
            self.id,
            self.winner.as_ref().unwrap().to_string(),
            self.win_type.as_ref().unwrap().to_string(),
            self.tick_count
        );
        format!("\nBATTLE RESULTS:\n-------------\n{result}\n")
    }
}
