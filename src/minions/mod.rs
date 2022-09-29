use uuid::Uuid;

use crate::minion::Minion;

pub fn generate_test_minion() -> Minion {
    Minion {
        id: "1".to_owned(),
        uuid: Uuid::new_v4(),

        card_id: "1".to_owned(),

        name: "test".to_owned(),

        attack: 1,
        health: 1,
        cost: 1,

        on_summon: None,
    }
}
