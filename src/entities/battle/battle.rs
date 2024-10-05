use crate::entities::battalion::battalion::Battalion;

#[derive(Debug)]
pub struct Battle {
    pub army_1_state: Vec<Battalion>,
    pub army_2_state: Vec<Battalion>,
}
