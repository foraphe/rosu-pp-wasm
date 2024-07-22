use crate::{
    model::beatmap::Beatmap,
    Difficulty,
};

use wasm_bindgen::prelude::*;

/// Calculates online SR locally
#[wasm_bindgen]
pub fn calculate_sr(str: &[u8], mods: u32) -> f32{
    let map = Beatmap::from_bytes(str).unwrap();
    Difficulty::new()
                .mods(mods)
                .calculate(&map)
                .stars() 
    as f32
}
