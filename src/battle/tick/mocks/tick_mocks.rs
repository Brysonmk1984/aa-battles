use std::collections::HashMap;

pub struct TickMocks;

impl TickMocks {
    pub fn generate_weapon_armor_hash<'a>() -> HashMap<&'a str, f64> {
        HashMap::from([
            ("piercing-unarmored", 1.0),
            ("piercing-leather", 0.75),
            ("piercing-chain", 0.6),
            ("piercing-plate", 0.1),
            ("crushing-unarmored", 0.25),
            ("crushing-leather", 0.50),
            ("crushing-chain", 0.75),
            ("crushing-plate", 1.0),
            ("blunt-unarmored", 0.75),
            ("blunt-leather", 0.75),
            ("blunt-chain", 0.5),
            ("blunt-plate", 0.25),
            ("edged-unarmored", 1.0),
            ("edged-leather", 0.75),
            ("edged-chain", 0.5),
            ("edged-plate", 0.25),
            ("magic-unarmored", 0.25),
            ("magic-leather", 0.50),
            ("magic-chain", 1.0),
            ("magic-plate", 0.75),
        ])
    }
}
