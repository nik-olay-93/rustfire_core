use std::fmt::Debug;

use uuid::Uuid;

use crate::minion::Minion;

pub struct Board {
    player1: PSide,
    player2: PSide,
    minions: Vec<(Uuid, u8, usize)>,
}

impl Board {
    /// Constructs an empty board.
    ///
    /// # Example
    ///
    /// ```
    /// use rustfire_core::board::Board;
    ///
    /// let board = Board::new();
    /// ```
    pub fn new() -> Board {
        Board {
            player1: PSide::new(),
            player2: PSide::new(),
            minions: Vec::new(),
        }
    }

    /// Counts the number of minions on one side of the board.
    ///
    /// # Arguments
    ///
    /// * `player` - The side of the board to count the minions on.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The number of minions on the side of the board.
    /// * `Err(String)` - The error message, if player is not 1 or 2.
    ///
    /// # Example
    ///
    /// ```
    /// use rustfire_core::board::Board;
    ///
    /// let board = Board::new();
    /// assert_eq!(board.minion_count(1), Ok(0));
    /// ```
    pub fn minion_count(&self, player: u8) -> Result<usize, String> {
        match player {
            1 => Ok(self.player1.minion_count()),
            2 => Ok(self.player2.minion_count()),
            _ => Err("Invalid player number".to_owned()),
        }
    }

    /// Summons given minion to the board, triggering its summoning effects.
    ///
    /// # Arguments
    ///
    /// * `minion` - The minion to summon.
    /// * `player` - The side of the board to summon the minion to.
    /// * `slot` - The slot to summon the minion to.
    ///
    /// # Returns
    ///
    /// * `Ok(Uuid)` - The UUID of the minion summoned.
    /// * `Err(String)` - The error message, if player is not 1 or 2, or if the slot is not empty or out of bounds.
    pub fn summon_minion(
        &mut self,
        minion: Minion,
        player: u8,
        slot: usize,
    ) -> Result<Uuid, String> {
        let on_summon = minion.on_summon;

        let uuid = match player {
            1 => self.player1.summon_minion(minion, slot),
            2 => self.player2.summon_minion(minion, slot),
            _ => Err("Invalid player number".to_owned()),
        }?;

        self.minions.push((uuid, player, slot));

        if let Some(on_summon) = on_summon {
            on_summon(self, player, slot);
        };

        Ok(uuid)
    }

    /// Triggers all dead minions' deathrattles.
    pub fn deathrattle_dead(&mut self) {
        let mut dead_minions = Vec::new();

        for min in self.minions.iter() {
            let side;
            match min.1 {
                1 => side = &mut self.player1,
                2 => side = &mut self.player2,
                _ => continue,
            }

            let minion = match side.get_minion(min.2) {
                Some(minion) => minion,
                None => continue,
            };

            if minion.health <= 0 {
                dead_minions.push(min.clone());
            }
        }
    }

    /// Removes all dead minions from the board.
    pub fn remove_dead(&mut self) {
        let mut dead_minions = Vec::new();

        for min in self.minions.iter() {
            let side;
            match min.1 {
                1 => side = &mut self.player1,
                2 => side = &mut self.player2,
                _ => continue,
            }

            let minion = match side.get_minion(min.2) {
                Some(minion) => minion,
                None => continue,
            };

            if minion.health <= 0 {
                dead_minions.push(min.clone());
            }
        }

        for min in dead_minions.iter() {
            let side;
            match min.1 {
                1 => side = &mut self.player1,
                2 => side = &mut self.player2,
                _ => continue,
            }

            side.remove_minion(min.2).unwrap_or_default();

            self.minions.retain(|x| x.0 != min.0);
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

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PSide {
    minionslots: Vec<BoardSlot>,
}

impl PSide {
    /// Constructs an empty side of the board.
    pub fn new() -> PSide {
        PSide {
            minionslots: vec![],
        }
    }

    /// Returns the number of minions on the side of the board.
    pub fn minion_count(&self) -> usize {
        self.minionslots.len()
    }

    /// Returns mutable reference to minion in given slot.
    ///
    /// # Arguments
    ///
    /// * `slot` - The slot to get the minion from.
    ///
    /// # Returns
    ///
    /// * `Some(&mut Minion)` - The minion in the slot.
    /// * `None` - If the slot is empty or out of bounds.
    pub fn get_minion(&mut self, slot: usize) -> Option<&mut Minion> {
        if let Some(BoardSlot::Minion(minion)) = self.minionslots.get_mut(slot) {
            Some(minion)
        } else {
            None
        }
    }

    /// Summons given minion to the side of the board.
    ///
    /// # Arguments
    ///
    /// * `minion` - The minion to summon.
    /// * `slot` - The slot to summon the minion to.
    ///
    /// # Returns
    ///
    /// * `Ok(Uuid)` - The UUID of the minion summoned.
    /// * `Err(String)` - The error message, if the slot is not empty or out of bounds.
    pub fn summon_minion(&mut self, mut minion: Minion, slot: usize) -> Result<Uuid, String> {
        let uuid = Uuid::new_v4();

        if self.minionslots.len() >= 7 {
            return Err("No more minion slots available".to_owned());
        }

        if self.minionslots.len() < slot {
            return Err("Invalid slot number".to_owned());
        }

        minion.uuid = uuid;
        self.minionslots.insert(slot, BoardSlot::Minion(minion));
        Ok(uuid)
    }

    /// Removes minion from given slot.
    pub fn remove_minion(&mut self, slot: usize) -> Result<(), String> {
        if self.minionslots.len() < slot {
            return Err("Invalid slot number".to_owned());
        }

        self.minionslots.remove(slot);
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

        writeln!(f)?;

        for (_, slot) in self.minionslots.iter().enumerate() {
            if let BoardSlot::Minion(minion) = slot {
                write!(f, "{} {}\t", minion.attack, minion.health)?;
            }
        }

        writeln!(f)?;

        Ok(())
    }
}

impl Default for PSide {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub enum BoardSlot {
    Minion(Minion),
    Unknown,
}
