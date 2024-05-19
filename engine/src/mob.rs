use crate::coord::Coord;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Mob {
    name: String,
    path: Vec<Coord>,
    pub speed: u8,
    pub health: u8,
    pub attack: u8,
    pub defense: u8,
}

#[wasm_bindgen]
impl Mob {
    pub fn new(name: String, path: Vec<Coord>) -> Self {
        Self {
            name,
            path,
            attack: 1,
            speed: 1,
            defense: 1,
            health: 1,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> Vec<Coord> {
        self.path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_path(&mut self, path: Vec<Coord>) {
        self.path = path;
    }
}
