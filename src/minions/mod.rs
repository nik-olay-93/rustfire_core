use crate::minion::Minion;

pub fn generate_test_minion() -> Minion {
    Minion {
        id: "1".to_owned(),
        uuid: None,
        name: "Test".to_owned(),
        base_cost: 1,
        cost: 1,
        base_attack: 1,
        attack: 1,
        base_health: 1,
        health: 1,
        base_modifiers: vec![],
        buffs: vec![],
        summon: None,
        battlecry: None,
        deathrattle: None,
        remove: None,
        before_attack: None,
        after_attack: None,
        before_attacked: None,
        after_attacked: None,
        before_another_attack: None,
        after_another_attack: None,
    }
}
