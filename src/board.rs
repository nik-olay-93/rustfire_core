use std::fmt::Error;

use uuid::Uuid;

use crate::minion::Minion;

pub struct Board {
    pub player1: PSide,
    pub player2: PSide,
    pub minions: Vec<Uuid>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            player1: PSide::new(),
            player2: PSide::new(),
            minions: vec![],
        }
    }

    pub fn summon_minion(&mut self, minion: &Minion, player: usize) -> Result<usize, Error> {
        match player {
            1..=2 => {
                let mut new_minion = minion.clone();
                let uuid = Uuid::new_v4();
                new_minion.uuid = Some(uuid);
                match player {
                    1 => {
                        let slot = self.player1.summon_minion(new_minion)?;
                        self.minions.push(uuid);
                        Ok(slot)
                    }
                    2 => {
                        let slot = self.player2.summon_minion(new_minion)?;
                        self.minions.push(uuid);
                        Ok(slot)
                    }
                    _ => Err(Error),
                }
            }
            _ => Err(Error {}),
        }
    }
}

pub struct PSide {
    pub hero: HeroSlot,
    pub hero_power: HeroPowerSlot,
    pub minionslots: Vec<MinionSlot>,
    pub mana: u8,
    pub max_mana: u8,
}

impl PSide {
    pub fn new() -> Self {
        Self {
            hero: HeroSlot::None,
            hero_power: HeroPowerSlot::None,
            // TODO: Come up with a better way to do this
            minionslots: vec![
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

    pub fn summon_minion(&mut self, minion: Minion) -> Result<usize, Error> {
        for (i, slot) in self.minionslots.iter_mut().enumerate() {
            if let MinionSlot::None = slot {
                *slot = MinionSlot::Minion(minion);
                return Ok(i);
            }
        }
        Err(Error {})
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

pub enum MinionSlot {
    None,
    Minion(Minion),
}
