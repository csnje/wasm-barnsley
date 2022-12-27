// A WebAssembly implementation of the Barnsley fern.

use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

const CANVAS_WIDTH: u32 = 600;
const CANVAS_HEIGHT: u32 = 600;

// Range of fractal
const MIN_X: f64 = -2.1820;
const MAX_X: f64 = 2.6558;
const MIN_Y: f64 = 0.0;
const MAX_Y: f64 = 9.9983;

const COLOR: &str = "rgba(0,127,0,0.2)";
const RADIUS: f64 = 0.005;

// Number of points to add per animation frame
const POINTS_PER_FRAME: usize = 500;

struct Position {
    x: f64,
    y: f64,
}

impl Position {
    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context
            .arc(self.x, self.y, RADIUS, 0.0, std::f64::consts::TAU)
            .unwrap();
        context.fill();
    }
}

struct Data {
    position: Position,
    context: CanvasRenderingContext2d,
    request_animation_frame_handle: AnimationFrame,
    _timestamp: f64,
}

static mut DATA: Option<Data> = None;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have window");
    let document = window.document().expect("should have window");

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(CANVAS_WIDTH);
    canvas.set_height(CANVAS_HEIGHT);
    document.body().unwrap().append_child(&canvas)?;

    let context = canvas
        .get_context("2d")?
        .expect("should have 2d context")
        .dyn_into::<CanvasRenderingContext2d>()?;
    context.scale(
        CANVAS_WIDTH as f64 / (MAX_X - MIN_X),
        -(CANVAS_WIDTH as f64) / (MAX_Y - MIN_Y),
    )?;
    context.translate(-MIN_X, -MAX_Y)?;
    context.set_fill_style(&JsValue::from_str(COLOR));

    let position = Position { x: 0.0, y: 0.0 };
    position.draw(&context);

    let request_animation_frame_handle = request_animation_frame(on_animation_frame);

    unsafe {
        DATA = Some(Data {
            position,
            context,
            request_animation_frame_handle,
            _timestamp: 0.0,
        });
    }

    Ok(())
}

fn update(position: &Position) -> Position {
    let random = js_sys::Math::random();
    if random < 0.01 {
        Position {
            x: 0.0,
            y: 0.16 * position.y,
        }
    } else if random < 0.86 {
        Position {
            x: 0.85 * position.x + 0.04 * position.y,
            y: -0.04 * position.x + 0.85 * position.y + 1.6,
        }
    } else if random < 0.93 {
        Position {
            x: 0.2 * position.x - 0.26 * position.y,
            y: 0.23 * position.x + 0.22 * position.y + 1.6,
        }
    } else {
        Position {
            x: -0.15 * position.x + 0.28 * position.y,
            y: 0.26 * position.x + 0.24 * position.y + 0.44,
        }
    }
}

fn on_animation_frame(_timestamp: f64) {
    let data = unsafe { DATA.as_mut().unwrap() };

    for _ in 0..POINTS_PER_FRAME {
        data.position = update(&data.position);
        data.position.draw(&data.context);
    }

    data.request_animation_frame_handle = request_animation_frame(on_animation_frame);
}
