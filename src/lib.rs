// A WebAssembly implementation of the Barnsley fern.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 800;

// Range of fractal
const MIN_X: f64 = -2.1820;
const MAX_X: f64 = 2.6558;
const MIN_Y: f64 = 0.0;
const MAX_Y: f64 = 9.9983;

const FILL_STYLE: &str = "rgba(0,127,0,0.2)";
const ARC_RADIUS: f64 = 0.002;

// Number of iterations per animation frame
const ITERATIONS_PER_FRAME: usize = 1000;

struct Position {
    x: f64,
    y: f64,
}

impl Position {
    fn update(&mut self) {
        let random = js_sys::Math::random();
        let position = if random < 0.01 {
            Position {
                x: 0.0,
                y: 0.16 * self.y,
            }
        } else if random < 0.86 {
            Position {
                x: 0.85 * self.x + 0.04 * self.y,
                y: -0.04 * self.x + 0.85 * self.y + 1.6,
            }
        } else if random < 0.93 {
            Position {
                x: 0.2 * self.x - 0.26 * self.y,
                y: 0.23 * self.x + 0.22 * self.y + 1.6,
            }
        } else {
            Position {
                x: -0.15 * self.x + 0.28 * self.y,
                y: 0.26 * self.x + 0.24 * self.y + 0.44,
            }
        };
        self.x = position.x;
        self.y = position.y;
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context
            .arc(self.x, self.y, ARC_RADIUS, 0.0, std::f64::consts::TAU)
            .unwrap();
        context.fill();
    }
}

fn window() -> Window {
    web_sys::window().expect("should have window")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register request animation frame callback");
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let document = window().document().expect("should have document");

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
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
    context.set_fill_style(&JsValue::from_str(FILL_STYLE));

    let mut position = std::cell::RefCell::new(Position { x: 0.0, y: 0.0 });

    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        for _ in 0..ITERATIONS_PER_FRAME {
            position.get_mut().update();
            position.get_mut().draw(&context);
        }
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
