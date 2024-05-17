use crate::coord::Coord;

#[derive(Default, Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub start_coord: Coord,
    pub end_coord: Coord,
    pub speed: u8,
    pub health: u8,
    pub attack: u8,
    pub defense: u8,
    pub path: Vec<Coord>,
}
