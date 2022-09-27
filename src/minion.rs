use uuid::Uuid;

use crate::board::{Board, PSide};

#[derive(Clone)]
pub struct Minion {
    pub id: String,
    pub uuid: Option<Uuid>,

    pub name: String,

    pub base_cost: u8,
    pub cost: u8,

    pub base_attack: u8,
    pub attack: u8,

    pub base_health: u8,
    pub health: u8,

    pub base_modifiers: Vec<MinionModifier>,
    pub buffs: Vec<MinionBuff>,

    pub summon: Option<
        fn(&mut Minion, board: &mut Board, player: &mut PSide, enemy: &mut PSide, slot: usize),
    >,
    pub battlecry: Option<
        fn(&mut Minion, board: &mut Board, player: &mut PSide, enemy: &mut PSide, slot: usize),
    >,
    pub deathrattle: Option<
        fn(&mut Minion, board: &mut Board, player: &mut PSide, enemy: &mut PSide, slot: usize),
    >,
    pub remove: Option<
        fn(&mut Minion, board: &mut Board, player: &mut PSide, enemy: &mut PSide, slot: usize),
    >,

    pub before_attack: Option<
        fn(
            &mut Minion,
            board: &mut Board,
            player: &mut PSide,
            enemy: &mut PSide,
            slot: usize,
            target: usize,
        ),
    >,
    pub after_attack: Option<
        fn(
            &mut Minion,
            board: &mut Board,
            player: &mut PSide,
            enemy: &mut PSide,
            slot: usize,
            target: usize,
        ),
    >,

    pub before_attacked: Option<
        fn(
            &mut Minion,
            board: &mut Board,
            player: &mut PSide,
            enemy: &mut PSide,
            slot: usize,
            attacker: usize,
        ),
    >,
    pub after_attacked: Option<
        fn(
            &mut Minion,
            board: &mut Board,
            player: &mut PSide,
            enemy: &mut PSide,
            slot: usize,
            attacker: usize,
        ),
    >,

    pub before_another_attack: Option<
        fn(
            &mut Minion,
            board: &mut Board,
            player: &mut PSide,
            enemy: &mut PSide,
            slot: usize,
            target: usize,
        ),
    >,
    pub after_another_attack: Option<
        fn(
            &mut Minion,
            board: &mut Board,
            player: &mut PSide,
            enemy: &mut PSide,
            slot: usize,
            target: usize,
        ),
    >,
}

impl Minion {
    pub fn trigger_summon(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
    ) {
        if let Some(summon) = self.summon {
            summon(self, board, player, enemy, slot);
        }
    }
    pub fn trigger_battlecry(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
    ) {
        if let Some(battlecry) = self.battlecry {
            battlecry(self, board, player, enemy, slot);
        }
    }
    pub fn trigger_deathrattle(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
    ) {
        if let Some(deathrattle) = self.deathrattle {
            deathrattle(self, board, player, enemy, slot);
        }
    }
    pub fn trigger_remove(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
    ) {
        if let Some(remove) = self.remove {
            remove(self, board, player, enemy, slot);
        }
    }

    pub fn trigger_before_attack(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
        target: usize,
    ) {
        if let Some(before_attack) = self.before_attack {
            before_attack(self, board, player, enemy, slot, target);
        }
    }
    pub fn trigger_after_attack(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
        target: usize,
    ) {
        if let Some(after_attack) = self.after_attack {
            after_attack(self, board, player, enemy, slot, target);
        }
    }

    pub fn trigger_before_attacked(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
        attacker: usize,
    ) {
        if let Some(before_attacked) = self.before_attacked {
            before_attacked(self, board, player, enemy, slot, attacker);
        }
    }
    pub fn trigger_after_attacked(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
        attacker: usize,
    ) {
        if let Some(after_attacked) = self.after_attacked {
            after_attacked(self, board, player, enemy, slot, attacker);
        }
    }

    pub fn trigger_before_another_attack(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
        target: usize,
    ) {
        if let Some(before_another_attack) = self.before_another_attack {
            before_another_attack(self, board, player, enemy, slot, target);
        }
    }
    pub fn trigger_after_another_attack(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
        target: usize,
    ) {
        if let Some(after_another_attack) = self.after_another_attack {
            after_another_attack(self, board, player, enemy, slot, target);
        }
    }
}

#[derive(Clone)]
pub enum MinionModifier {
    Taunt,
    DivineShield,
    Charge,
    Windfury,
    Stealth,
    Poisonous,
    SpellDamage(u8),
    Rush,
}

#[derive(Clone)]
pub enum MinionBuff {
    Attack(u8),
    Health(u8),
    AttackAndHealth((u8, u8)),
    Modifier(MinionModifier),
}
