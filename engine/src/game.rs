use crate::{
    board::Board,
    coord::{Coord, CoordError},
    mob::Mob,
};

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    mobs: Vec<Mob>,
}

impl Game {
    pub fn new(width: i64, height: i64, box_size: i64) -> Self {
        Game {
            board: Board::new(width, height, box_size),
            mobs: vec![],
        }
    }

    pub fn mobs(&self) -> Vec<Mob> {
        self.mobs.clone()
    }

    pub fn set_mobs(&mut self, mobs: Vec<Mob>) {
        self.mobs = mobs;
    }

    pub fn start(&mut self) {
        self.play_round();
    }

    pub fn next_round(&mut self) {
        self.mobs.clear();
        self.play_round();
    }

    pub fn play_round(&mut self) -> &Self {
        let _game = self.generate_mobs();

        self
    }

    fn generate_mobs(&mut self) -> Result<(), CoordError> {
        for i in 0..3 {
            let start = self.board.random_start_coord()?;
            let end = self.board.random_end_coord()?;
            let mob = Mob::new(format!("Mob_{}", i), self.generate_mob_path(&start, &end)?);

            self.mobs.push(mob);
        }

        Ok(())
    }

    fn generate_mob_path(&mut self, start: &Coord, end: &Coord) -> Result<Vec<Coord>, CoordError> {
        let mut steps = vec![start.to_owned()];

        while let Some(last_step) = steps.last() {
            if last_step == end {
                break;
            }

            let next_step = self.board.random_step(last_step, end)?;
            steps.push(next_step);
        }

        Ok(steps)
    }
}
