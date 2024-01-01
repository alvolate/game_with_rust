use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::wasm_bindgen::JsCast;
mod game_agents;
use game_agents::environment::Environment;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
static CELL_SIZE:u32 = 10;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// This is like the `main` function, except for JavaScript.

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let env = Environment::new(8);



    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.set_width(CELL_SIZE * env.width);
    canvas.set_height(CELL_SIZE * env.width);

    let context= canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>()?;    
    draw_env(&context, &env);
    draw_snake(&context, &env);

    let h1 = document.create_element("h1").unwrap();
    h1.set_inner_html("Hello World");
    alert("Hello, this is an alert from Rust!");

    // Append the h1 element to the document body
    document.body().unwrap().append_child(&h1).unwrap();

    

    Ok(())
}

fn draw_env(context: &web_sys::CanvasRenderingContext2d, env: &Environment){
    for x in 0..=env.width{
        context.move_to((CELL_SIZE * x) as f64 , 0.0);
        context.line_to((CELL_SIZE * x) as f64, (CELL_SIZE * env.width) as f64);
        context.close_path();
    }
    for y in 0..=env.width{
        context.move_to( 0.0,(CELL_SIZE * y) as f64);
        context.line_to( (CELL_SIZE * env.width) as f64,(CELL_SIZE * y) as f64,);
        context.close_path();
    }
    context.stroke();
}

fn draw_snake(context:&web_sys::CanvasRenderingContext2d, env: &Environment){
    let head = env.get_snakehead();
    let col = head % CELL_SIZE;
    let row = ((head / CELL_SIZE)as f64).floor();
    log(&format!("col: {}, row: {}", col, row));

    context.begin_path();
    context.fill_rect((col * CELL_SIZE) as f64, row * (CELL_SIZE) as f64, CELL_SIZE as f64, CELL_SIZE as f64);
    context.close_path();
    context.stroke();
}