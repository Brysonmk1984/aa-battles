use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Nation {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub gold: i32,
    pub is_npc: bool,
}
