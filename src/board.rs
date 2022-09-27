use crate::minion::Minion;

pub struct Board<'a> {
    pub player1: PSide<'a>,
    pub player2: PSide<'a>,
    pub minions: Vec<Minion>,
}

impl<'a> Board<'a> {
    pub fn new() -> Self {
        Board {
            player1: PSide::new(),
            player2: PSide::new(),
            minions: vec![],
        }
    }
}

pub struct PSide<'a> {
    pub hero: HeroSlot,
    pub hero_power: HeroPowerSlot,
    pub minionslots: [MinionSlot<'a>; 7],
    pub mana: u8,
    pub max_mana: u8,
}

impl<'a> PSide<'a> {
    pub fn new() -> Self {
        Self {
            hero: HeroSlot::None,
            hero_power: HeroPowerSlot::None,
            // TODO: Come up with a better way to do this
            minionslots: [
                MinionSlot::None,
                MinionSlot::None,
                MinionSlot::None,
                MinionSlot::None,
                MinionSlot::None,
                MinionSlot::None,
                MinionSlot::None,
            ],
            mana: 0,
            max_mana: 0,
        }
    }
}

pub enum HeroSlot {
    None,
    Hero,
}

pub enum HeroPowerSlot {
    None,
    HeroPower,
}

pub enum MinionSlot<'a> {
    None,
    Minion(&'a mut Minion),
}
