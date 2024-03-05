use std::collections::HashMap;

pub struct TickMocks;

impl TickMocks {
    pub fn generate_weapon_armor_hash<'a>() -> HashMap<String, f64> {
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
}
