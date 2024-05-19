use engine::Mob;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn draw_game(width: i64, height: i64, box_size: i64, mobs: Vec<Mob>) {
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

    context.clear_rect(0.0, 0.0, width as f64, height as f64);

    for x in 0..=(width / box_size) {
        let mut start = (x * box_size) as f64;
        start += 0.5;

        context.begin_path();
        context.set_stroke_style(&JsValue::from("black"));
        context.move_to(start, 0.0);

        context.line_to(start, height as f64);
        context.stroke();
    }

    for y in 0..=(height / box_size) {
        let mut start = (y * box_size) as f64;
        start += 0.5;

        context.begin_path();
        context.set_stroke_style(&JsValue::from("black"));
        context.move_to(0.0, start);

        context.line_to(width as f64, start);
        context.stroke();
    }

    let mob_path_colors = [
        "red", "green", "blue", "pink", "purple", "orange", "yellow", "cyan", "magenta", "lime",
        "brown", "olive", "navy", "teal", "maroon", "aqua", "fuchsia",
    ];
    for mob in mobs {
        let path = mob.path();
        for (i, coord) in path.iter().enumerate() {
            let x = (coord.x * box_size) as f64;
            let y = (coord.y * box_size) as f64;
            let offset = (box_size / 2) as f64;

            context.set_stroke_style(&JsValue::from(mob_path_colors[i % mob_path_colors.len()]));

            if i == 0 {
                context.begin_path();
                context.move_to(x + offset, y + offset);
            } else if i == (path.len() - 1) {
                context.stroke();
            } else {
                context.line_to(x + offset, y + offset);
            }
        }
    }
}
