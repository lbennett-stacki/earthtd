use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone)]
pub enum CoordError {
    NoAvailableCoords,
}
