use crate::coord::{Coord, CoordError};
use rand::prelude::SliceRandom;

#[derive(Debug)]
pub struct Board {
    pub width: u8,
    pub height: u8,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            width: 8,
            height: 8,
        }
    }
}

impl Board {
    fn random_y_coord(&self) -> Result<Coord, CoordError> {
        let available_coords = self.y_coords();

        if available_coords.is_empty() {
            return Err(CoordError::NoAvailableCoords);
        }

        let random_tile: Option<&Coord> = available_coords.choose(&mut rand::thread_rng());

        if random_tile.is_none() {
            return Err(CoordError::NoAvailableCoords);
        }

        Ok(random_tile.unwrap().to_owned())
    }

    pub fn random_start_coord(&self) -> Result<Coord, CoordError> {
        let mut coord = self.random_y_coord()?;

        coord.x = 0;

        Ok(coord)
    }

    pub fn random_end_coord(&self) -> Result<Coord, CoordError> {
        let mut coord = self.random_y_coord()?;

        coord.x = self.width;

        Ok(coord)
    }

    fn y_coords(&self) -> Vec<Coord> {
        (0..self.height).map(|y| Coord { x: 0, y }).collect()
    }

    pub fn random_step(&self, from: &Coord, towards: &Coord) -> Result<Coord, CoordError> {
        if from == towards {
            return Err(CoordError::NoAvailableCoords);
        }

        let is_x_direction = rand::random();

        if is_x_direction && from.x != towards.x {
            println!(
                "Random step is x direction - from {:?} towards {:?}",
                from.x, towards.x
            );

            let step_value: i8 = if towards.x > from.x { 1 } else { -1 };

            println!("Step value: {}", step_value);

            return Ok(Coord {
                x: from.x.wrapping_add_signed(step_value),
                y: from.y,
            });
        } else if from.y != towards.y {
            println!(
                "Random step is y direction - from {:?} towards {:?}",
                from.y, towards.y
            );

            let step_value: i8 = if towards.y > from.y { 1 } else { -1 };

            println!("Step value: {}", step_value);

            return Ok(Coord {
                x: from.x,
                y: from.y.wrapping_add_signed(step_value),
            });
        }

        Ok(from.to_owned())
    }
}
