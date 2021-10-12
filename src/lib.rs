use std::f64::consts::PI;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, HtmlElement, HtmlImageElement, HtmlInputElement};

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

fn fill_circle(
    context: &web_sys::CanvasRenderingContext2d,
    ci: usize,
    from: usize,
    to: usize,
) -> Result<(), JsValue> {
    let color_list = [
        "#9AD3BC", "#EDF6E5", "#FFBCBC", "#F38BA0", "#B5EAEA", "#87A8A4",
    ];
    let ci = ci % color_list.len();
    context.begin_path();
    context.move_to(250.0, 250.0);
    context.set_fill_style(&color_list[ci].into());
    context
        .arc(250.0, 250.0, 200.0, get_arc(from), get_arc(to))
        .unwrap();
    context.close_path();
    context.fill();
    Ok(())
}

fn set_text(
    context: &web_sys::CanvasRenderingContext2d,
    scale_lst: &Vec<(&str, f64, f64)>,
    text: &str,
    from: usize,
    to: usize,
) -> Result<(), JsValue> {
    context.set_font("30px Hachi Maru Pop");
    let color_letters = "#797979";
    context.set_fill_style(&color_letters.into());
    let from_to = ((from + to) / 2) % 24;
    let from = from % 24;
    let to = to % 24;
    let x = (scale_lst[from].1 + scale_lst[to].1 + 250.0 + scale_lst[from_to].1) / 4.0;
    let y = (scale_lst[from].2 + scale_lst[to].2 + 250.0 + scale_lst[from_to].2) / 4.0;
    context.fill_text(&text, x, y).unwrap();
    Ok(())
}

fn set_scale(
    context: &web_sys::CanvasRenderingContext2d,
    scale_lst: &Vec<(&str, f64, f64)>,
    color: &str,
) -> Result<(), JsValue> {
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
    Ok(())
}

#[allow(dead_code)]
fn set_line(
    context: &web_sys::CanvasRenderingContext2d,
    color_letters: &str,
) -> Result<(), JsValue> {
    context.begin_path();
    context.set_stroke_style(&color_letters.into());
    context.move_to(250.0, 0.0);
    context.line_to(250.0, 500.0);
    context.move_to(0.0, 250.0);
    context.line_to(500.0, 250.0);
    context.stroke();
    Ok(())
}

fn convert_img(img: &HtmlImageElement, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let canvas_url = canvas.to_data_url().unwrap();
    img.set_src(&canvas_url);
    Ok(())
}

fn set_sign(context: &web_sys::CanvasRenderingContext2d, color: &str) -> Result<(), JsValue> {
    context.set_font("8px Hachi Maru Pop");
    context.set_fill_style(&color.into());
    context.fill_text("#今日の一日ぐらふ", 390.0, 460.0)
}

fn set_img(
    context: &web_sys::CanvasRenderingContext2d,
    data: &Vec<(String, usize, usize)>,
) -> Result<(), JsValue> {
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

    fill_circle(&context, 0, 0, 24).expect("Err first fill_circle");
    let mut i = 1;
    data.into_iter().for_each(|item| {
        let ci = if i == data.len() - 1 { 0 } else { i };
        fill_circle(&context, ci, item.1, item.2).expect("Err each Fill_circle");
        i += 1;
    });
    data.into_iter().for_each(|item| {
        set_text(&context, &scale_lst, &item.0, item.1, item.2).expect("Err each set_text");
    });

    let color_letters = "#797979";
    set_sign(&context, &color_letters).expect("Err set_sign");
    set_scale(&context, &scale_lst, &color_letters).expect("Err set_scale");
    //set_line(&context, &color_letters);
    Ok(())
}

pub fn start(document: &web_sys::Document) -> Result<(), JsValue> {
    let test_data = vec![
        ("睡眠".to_string(), 2, 9),
        ("お仕事".to_string(), 9, 18),
        ("Rust".to_string(), 18, 20),
        ("生活".to_string(), 20, 26),
    ];
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let img = document
        .get_element_by_id("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    set_img(&context, &test_data)
        .and_then(|_| convert_img(&img, &canvas))
        .unwrap();

    let document_cloned = document.clone();
    let closure = Closure::wrap(Box::new(move || {
        context.clear_rect(0.0, 0.0, 500.0, 500.0);
        let mut data: Vec<(String, usize, usize)> = Vec::new();
        let mut before_str = String::new();
        let mut before_hour = 0;
        let mut input_val: String;
        let mut first_hour = 0;
        let mut first = true;
        for i in 0..24 {
            let input_id = &format!("input_hour_{}", i);
            input_val = document_cloned
                .get_element_by_id(input_id)
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
            if !input_val.is_empty() {
                if !before_str.is_empty() {
                    data.push((before_str, before_hour, i));
                }
                if first {
                    first_hour = i;
                    first = false;
                }
                before_hour = i;
                before_str = input_val.to_string();
            }
        }
        if !before_str.is_empty() {
            data.push((before_str, before_hour, first_hour + 24));
        }
        set_img(&context, &data)
            .and_then(|_| convert_img(&img, &canvas))
            .unwrap();
    }) as Box<dyn FnMut()>);

    document
        .get_element_by_id("input_start")
        .expect("should have #input_start on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#input_start be an `HtmlElement`")
        .set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    Ok(())
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
    start(&document).unwrap();
    Ok(())
}
