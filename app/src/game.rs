use std::f64::consts::PI;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn draw_game() {
    use wasm_bindgen::JsCast;

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    let width = 100;
    let height = 100;

    let box_size = 10;

    for x in 0..width {
        let mut start: f64 = (x * box_size).try_into().unwrap();
        start += 0.5;

        context.move_to(start, 0.0);

        let h: f64 = (height).try_into().unwrap();
        context.line_to(start, h);
    }

    for y in 0..height {
        let mut start: f64 = (y * box_size).try_into().unwrap();
        start += 0.5;

        context.move_to(0.0, start);

        let w: f64 = (width).try_into().unwrap();
        context.line_to(w, start);
    }

    context.stroke();
}
