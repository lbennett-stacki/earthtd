#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Clone)]
pub enum CoordError {
    NoAvailableCoords,
}
