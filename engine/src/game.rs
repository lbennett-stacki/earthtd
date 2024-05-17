use std::default::Default;

use crate::{
    board::Board,
    coord::{Coord, CoordError},
    mob::Mob,
};

#[derive(Default, Debug)]
pub struct Game {
    pub board: Board,
    pub mobs: Vec<Mob>,
}

impl Game {
    pub fn start(&mut self) -> &Self {
        self.play_round();

        self
    }

    fn play_round(&mut self) -> &Self {
        let _game = self.generate_mobs();

        for mob in &self.mobs {
            println!(
                "{} is starting at {:?} and ending at {:?}",
                mob.name, mob.start_coord, mob.end_coord
            );
        }

        self
    }
}

impl Game {
    fn generate_mobs(&mut self) -> Result<(), CoordError> {
        for i in 0..10 {
            let start_coord = self.board.random_start_coord()?;
            let end_coord = self.board.random_end_coord()?;
            let mut mob = Mob {
                name: format!("Mob_{}", i),
                start_coord,
                end_coord,
                path: vec![],
                ..Default::default()
            };

            mob.path = self.generate_mob_path(&mob)?;

            self.mobs.push(mob);
        }

        Ok(())
    }

    fn generate_mob_path(&mut self, mob: &Mob) -> Result<Vec<Coord>, CoordError> {
        let mut steps = vec![mob.start_coord];

        while let Some(last_step) = steps.last() {
            if *last_step == mob.end_coord {
                break;
            }

            let next_step = self.board.random_step(last_step, &mob.end_coord)?;
            steps.push(next_step);
        }

        Ok(steps)
    }
}
