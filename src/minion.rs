use uuid::Uuid;

use crate::board::Board;

#[derive(Clone)]
pub struct Minion {
    pub id: String,
    pub uuid: Uuid,

    pub card_id: String,

    pub name: String,

    pub cost: u128,
    pub attack: u128,
    pub health: u128,

    pub on_summon: Option<fn(&mut Board, side: u8, slot: usize)>,
}

impl Minion {
    pub fn new() -> Minion {
        Minion {
            id: "".to_owned(),
            uuid: Uuid::new_v4(),

            card_id: "".to_owned(),

            name: "".to_owned(),

            cost: 0,
            attack: 0,
            health: 0,

            on_summon: None,
        }
    }

    pub fn test() -> Minion {
        let mut min = Minion::new();
        min.id = "1".to_owned();
        min.card_id = "1".to_owned();
        min.name = "test".to_owned();

        min.cost = 1;
        min.attack = 1;
        min.health = 1;

        min
    }
}
