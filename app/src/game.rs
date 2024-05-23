use engine::Mob;
use leptos_use::UseMouseReturn;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn draw_game(
    width: i64,
    height: i64,
    box_size: i64,
    mobs: Vec<Mob>,
    mouse_x: f64,
    mouse_y: f64,
) {
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

    let cols = width / box_size;
    let rows = height / box_size;

    for y in 0..=rows {
        for x in 0..cols {
            let x_b = (x * box_size) as f64;
            let y_b = (y * box_size) as f64;
            context.begin_path();
            context.set_line_width(1 as f64);
            context.set_stroke_style(&JsValue::from("rgba(0,0,0,0.1)"));
            context.rect(x_b, y_b, box_size as f64, box_size as f64);
            context.stroke();

            let x_t = (mouse_x / box_size as f64).floor();
            let y_t = (mouse_y / box_size as f64).floor();
            if x_t == x as f64 && y_t == y as f64 {
                context.set_fill_style(&JsValue::from("red"));
                context.fill_rect(x_b, y_b, box_size as f64, box_size as f64);
                context.fill();
            }
        }
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
            context.set_line_width((box_size / 4) as f64);

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
