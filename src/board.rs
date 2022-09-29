use std::fmt::Debug;

use crate::minion::Minion;

pub struct Board {
    player1: PSide,
    player2: PSide,
}

impl Board {
    pub fn new() -> Board {
        Board {
            player1: PSide::new(),
            player2: PSide::new(),
        }
    }

    pub fn summon_minion(&mut self, minion: Minion, player: u8, slot: usize) -> Result<(), String> {
        let on_summon = minion.on_summon;

        match player {
            1 => self.player1.summon_minion(minion, slot),
            2 => self.player2.summon_minion(minion, slot),
            _ => Err("Invalid player number".to_owned()),
        }?;

        if let Some(on_summon) = on_summon {
            on_summon(self, player, slot);
        };

        Ok(())
    }

    pub fn minion_count(&self, player: u8) -> Result<usize, String> {
        match player {
            1 => Ok(self.player1.minion_count()),
            2 => Ok(self.player2.minion_count()),
            _ => Err("Invalid player number".to_owned()),
        }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.player1)?;
        write!(f, "{:?}", self.player2)?;

        Ok(())
    }
}

pub struct PSide {
    minionslots: Vec<BoardSlot>,
}

impl PSide {
    pub fn new() -> PSide {
        PSide {
            minionslots: vec![],
        }
    }

    pub fn minion_count(&self) -> usize {
        self.minionslots.len()
    }

    pub fn get_minions_on_summon(
        &self,
        slot: usize,
    ) -> Option<fn(&mut Board, side: u8, slot: usize)> {
        match self.minionslots.get(slot) {
            Some(BoardSlot::Minion(minion)) => minion.on_summon,
            Some(_) => None,
            None => None,
        }
    }

    pub fn summon_minion(&mut self, minion: Minion, slot: usize) -> Result<(), String> {
        if self.minionslots.len() >= 7 {
            return Err("No more minion slots available".to_owned());
        }

        if self.minionslots.len() < slot {
            return Err("Invalid slot number".to_owned());
        }

        self.minionslots.insert(slot, BoardSlot::Minion(minion));
        Ok(())
    }
}

impl Debug for PSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (_, slot) in self.minionslots.iter().enumerate() {
            if let BoardSlot::Minion(minion) = slot {
                write!(f, "{}\t", minion.name)?;
            }
        }

        writeln!(f, "")?;

        for (_, slot) in self.minionslots.iter().enumerate() {
            if let BoardSlot::Minion(minion) = slot {
                write!(f, "{} {}\t", minion.attack, minion.health)?;
            }
        }

        writeln!(f, "")?;

        Ok(())
    }
}

#[derive(Clone)]
pub enum BoardSlot {
    Minion(Minion),
    Unknown,
}
