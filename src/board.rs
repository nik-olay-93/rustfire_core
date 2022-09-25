use uuid::Uuid;

pub struct Board {
    pub player1: PSide,
    pub player2: PSide,
}

pub struct PSide {
    pub hero: HeroSlot,
    pub hero_power: HeroPowerSlot,
    pub mininions: [MinionSlot; 7],
    pub mana: u8,
    pub on_turn_start: Vec<Effect>,
    pub on_turn_end: Vec<Effect>,
}

pub enum Effect {
    Minion(u8),
}

pub enum HeroSlot {
    None,
    Hero,
}

pub enum HeroPowerSlot {
    None,
    HeroPower,
}

pub enum MinionSlot {
    None,
    Minion(Box<dyn Minion>),
}

pub trait Minion {
    fn id(&self) -> u32;
    fn uuid(&self) -> Uuid;

    fn base_cost(&self) -> u32;
    fn cost(&self) -> u8;

    fn base_attack(&self) -> u32;
    fn attack(&self) -> u8;

    fn base_health(&self) -> u32;
    fn current_health(&self) -> u8;
    fn max_health(&self) -> u8;

    fn restore_health(&mut self, health: u8);
    fn take_damage(&mut self, damage: u8);
    fn reduce_health(&mut self, health: u8);

    fn reduce_attack(&mut self, attack: u8);

    fn summon(&self, board: &mut Board, player: &mut PSide, enemy: &mut PSide, slot: usize);
    fn trigger_battlecry(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
    );
    fn trigger_deathrattle(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
    );

    fn on_attack(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
        target: usize,
    );
    fn on_attacked(
        &mut self,
        board: &mut Board,
        player: &mut PSide,
        enemy: &mut PSide,
        slot: usize,
        attacker: usize,
    );

    fn has_modifier(&self, modifier: Modifier) -> bool;
    fn modifiers(&self) -> Vec<Modifier>;
    fn add_modifier(&mut self, modifier: Modifier);
    fn remove_modifier(&mut self, modifier: Modifier);
    fn remove_all_modifiers(&mut self);
}

pub enum Modifier {
    Taunt,
    DivineShield,
    Charge,
    Windfury,
    Stealth,
    Poisonous,
    SpellDamage(u8),
    Rush,
}