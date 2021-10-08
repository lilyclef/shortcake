use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn get_arc(u: usize) -> f64 {
    if (u as i64 - 6) % 3 == 0 {
        ((u as i64 - 6) / 3) as f64 * 0.25 * PI
    } else {
        (((u as i64 - 6) as f64) * 0.25 / 3.0) * PI
    }
}

fn fill_circle(context: &web_sys::CanvasRenderingContext2d, ci: usize, from: usize, to: usize) {
    let color_list = ["#B5EAEA", "#EDF6E5", "#FFBCBC", "#F38BA0"];
    context.begin_path();
    context.move_to(250.0, 250.0);
    context.set_fill_style(&color_list[ci].into());
    context
        .arc(250.0, 250.0, 200.0, get_arc(from), get_arc(to))
        .unwrap();
    context.close_path();
    context.fill();
}

fn set_text(
    context: &web_sys::CanvasRenderingContext2d,
    scale_lst: &Vec<(&str, f64, f64)>,
    text: &str,
    from: usize,
    to: usize,
) {
    context.set_font("30px Hachi Maru Pop");
    let color_letters = "#797979";
    context.set_fill_style(&color_letters.into());
    let mut x = (scale_lst[from].1 + scale_lst[to].1 + 250.0 + scale_lst[(from + to) / 2].1) / 4.0;
    let mut y = (scale_lst[from].2 + scale_lst[to].2 + 250.0 + scale_lst[(from + to) / 2].2) / 4.0;
    if to - from >= 12 {
        x = 500.0 - x;
        y = 500.0 - y;
    };
    context.fill_text(&text, x, y).unwrap();
}

fn set_scale(
    context: &web_sys::CanvasRenderingContext2d,
    scale_lst: &Vec<(&str, f64, f64)>,
    color: &str,
) {
    context.set_fill_style(&color.into());
    scale_lst.into_iter().for_each(|item| {
        let font = if ["0", "6", "12", "18"].contains(&item.0) {
            "15px Hachi Maru Pop"
        } else {
            "10px Hachi Maru Pop"
        };
        context.set_font(font);
        context.fill_text(item.0, item.1, item.2).unwrap()
    });
}

#[allow(dead_code)]
fn set_line(context: &web_sys::CanvasRenderingContext2d, color_letters: &str) {
    context.begin_path();
    context.set_stroke_style(&color_letters.into());
    context.move_to(250.0, 0.0);
    context.line_to(250.0, 500.0);
    context.move_to(0.0, 250.0);
    context.line_to(500.0, 250.0);
    context.stroke();
}

fn convert_img(document: &web_sys::Document, canvas: HtmlCanvasElement) {
    let canvas_url = canvas.to_data_url().unwrap();
    let img = document
        .get_element_by_id("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();
    img.set_src(&canvas_url);
}

fn set_sign(context: &web_sys::CanvasRenderingContext2d, color: &str) {
    context.set_font("8px Hachi Maru Pop");
    context.set_fill_style(&color.into());
    context
        .fill_text("#今日の一日ぐらふ", 390.0, 460.0)
        .unwrap();
    context.fill_text("@lilybrevec", 390.0, 480.0).unwrap();
}

pub fn set_img(document: &web_sys::Document, data: &Vec<(&str, usize, usize)>) {
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

    let scale_lst = vec![
        ("0", 244.0, 42.0),
        ("1", 299.0, 50.0),
        ("2", 352.0, 73.0),
        ("3", 401.0, 111.0),
        ("4", 432.0, 155.0),
        ("5", 451.0, 204.0),
        ("6", 457.0, 254.0),
        ("7", 451.0, 305.0),
        ("8", 432.0, 355.0),
        ("9", 400.0, 400.0),
        ("10", 355.0, 431.0),
        ("11", 300.0, 458.0),
        ("12", 243.0, 470.0),
        ("13", 190.0, 460.0),
        ("14", 138.0, 440.0),
        ("15", 94.0, 410.0),
        ("16", 57.0, 370.0),
        ("17", 36.0, 312.0),
        ("18", 25.0, 254.0),
        ("19", 36.0, 198.0),
        ("20", 57.0, 145.0),
        ("21", 92.0, 100.0),
        ("22", 133.0, 68.0),
        ("23", 190.0, 48.0),
    ];

    // Test cases
    fill_circle(&context, 0, 0, 24);
    let mut i = 1;
    data.into_iter().for_each(|item| {
        fill_circle(&context, i % 4, item.1, item.2);
        i += 1;
    });
    data.into_iter().for_each(|item| {
        set_text(&context, &scale_lst, item.0, item.1, item.2);
    });

    let color_letters = "#797979";
    set_sign(&context, &color_letters);
    set_scale(&context, &scale_lst, &color_letters);
    //set_line(&context, &color_letters);
    convert_img(document, canvas);
}

pub fn start() {
    let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
    let test_data = vec![
        ("睡眠", 2, 9),
        ("お仕事", 9, 18),
        ("Rust", 18, 20),
        ("生活", 20, 23),
    ];
    set_img(&document, &test_data);
    // https://rustwasm.github.io/wasm-bindgen/examples/closures.html
    // set_onclickを設定したいがうまく行っていないの図
    /* let a = Closure::wrap(Box::new(move || {
    }) as Box<dyn FnMut()>);
    let input =  document.get_element_by_id("input-start")
      .expect("should have #input-start on the page")
      .dyn_ref::<HtmlElement>()
      .expect("#input-start be an `HtmlElement`");
     input.set_onclick(Some(a.as_ref().unchecked_ref()));
     */
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    start();
    Ok(())
}
