//! # Runtime `const` tweaking
//!
//! This library starts a web server at `http://127.0.0.1:9938` where you can change the values of `const` variables in your crate.
//!
//! `bool`, `&str`, `f32`, `f64`, `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128` and `usize` are the types that are currently supported.
//!
//! ## Example
//! ```rust
//! // Tweak `VALUE` when running in debug mode
//! // This will render a slider in the web GUI because the type here is a `f64`
//! #[const_tweaker::tweak]
//! const VALUE: f64 = 0.0;
//!
//! // Enter a GUI/Game loop
//! loop {
//!     // ...
//!
//!     // Print the constant value that can be changed from the website.
//!     println!("VALUE: {}", VALUE);
//! #   break;
//! }
//! ```
//!
//! Some widgets have customizable options, as seen in the examples below:
//!
//! `f32` & `f64`:
//! ```rust
//! // Spawns a slider
//! #[const_tweaker::tweak]
//! const DEFAULT_VALUE: f64 = 0.0;
//!
//! // Spawns a slider with 10 steps from 0-1
//! #[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.1)]
//! const CUSTOM_VALUE: f32 = 0.0;
//! ```
//!
//! `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128` & `usize`:
//! ```rust
//! // Spawns a slider with 90 steps from 100-1000
//! #[const_tweaker::tweak(min = 100, max = 1000, step = 10)]
//! const CUSTOM_VALUE: i64 = 0;
//! ```
//!
//! `bool`:
//! ```rust
//! // Spawns a checkbox
//! #[const_tweaker::tweak]
//! const DEFAULT_VALUE: bool = true;
//! ```
//!
//! `&str`
//! ```rust
//! // Spaws a textbox
//! #[const_tweaker::tweak]
//! const DEFAULT_VALUE: &str = "Hi";
//! ```

#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    nonstandard_style,
    unused,
    clippy::all
)]
// Ignore the lazy_static warning about the mutex
#![allow(clippy::mutex_atomic)]

use async_std::task;
use dashmap::DashMap;
use horrorshow::{html, owned_html, Raw, Render};
use serde::{de::DeserializeOwned, Deserialize};
use std::{cmp::Ordering, fmt::Display, string::ToString, sync::Mutex, thread};
use tide::{Request, Response};

pub use const_tweaker_attribute::tweak;
#[doc(hidden)]
pub use ctor::ctor;

/// Type representing the const field with metadata.
#[doc(hidden)]
#[derive(Debug)]
pub enum Field {
    F32 {
        value: f32,
        /// Minimum value of slider.
        min: f32,
        /// Maximum value of slider.
        max: f32,
        /// Step increase of slider.
        step: f32,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
    F64 {
        value: f64,
        /// Minimum value of slider.
        min: f64,
        /// Maximum value of slider.
        max: f64,
        /// Step increase of slider.
        step: f64,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
    I8 {
        value: i8,
        /// Minimum value of slider.
        min: i8,
        /// Maximum value of slider.
        max: i8,
        /// Step increase of slider.
        step: i8,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
    U8 {
        value: u8,
        /// Minimum value of slider.
        min: u8,
        /// Maximum value of slider.
        max: u8,
        /// Step increase of slider.
        step: u8,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
    I16 {
        value: i16,
        /// Minimum value of slider.
        min: i16,
        /// Maximum value of slider.
        max: i16,
        /// Step increase of slider.
        step: i16,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
    U16 {
        value: u16,
        /// Minimum value of slider.
        min: u16,
        /// Maximum value of slider.
        max: u16,
        /// Step increase of slider.
        step: u16,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
    I32 {
        value: i32,
        /// Minimum value of slider.
        min: i32,
        /// Maximum value of slider.
        max: i32,
        /// Step increase of slider.
        step: i32,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
    U32 {
        value: u32,
        /// Minimum value of slider.
        min: u32,
        /// Maximum value of slider.
        max: u32,
        /// Step increase of slider.
        step: u32,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
    I64 {
        value: i64,
        /// Minimum value of slider.
        min: i64,
        /// Maximum value of slider.
        max: i64,
        /// Step increase of slider.
        step: i64,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
    U64 {
        value: u64,
        /// Minimum value of slider.
        min: u64,
        /// Maximum value of slider.
        max: u64,
        /// Step increase of slider.
        step: u64,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
    Usize {
        value: usize,
        /// Minimum value of slider.
        min: usize,
        /// Maximum value of slider.
        max: usize,
        /// Step increase of slider.
        step: usize,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
    Bool {
        value: bool,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
    String {
        value: String,

        /// Rust module location.
        module: String,
        /// Rust file location.
        file: String,
        /// Rust line number in file.
        line: u32,
    },
}

impl Field {
    /// The full module path where the constant lives.
    pub fn module_path(&self) -> &str {
        match self {
            Field::F32 { module, .. }
            | Field::F64 { module, .. }
            | Field::I8 { module, .. }
            | Field::U8 { module, .. }
            | Field::I16 { module, .. }
            | Field::U16 { module, .. }
            | Field::I32 { module, .. }
            | Field::U32 { module, .. }
            | Field::I64 { module, .. }
            | Field::U64 { module, .. }
            | Field::Usize { module, .. }
            | Field::Bool { module, .. }
            | Field::String { module, .. } => &*module,
        }
    }

    /// The file with line number.
    pub fn file(&self) -> String {
        match self {
            Field::F32 { file, line, .. }
            | Field::F64 { file, line, .. }
            | Field::I8 { file, line, .. }
            | Field::U8 { file, line, .. }
            | Field::I16 { file, line, .. }
            | Field::U16 { file, line, .. }
            | Field::I32 { file, line, .. }
            | Field::U32 { file, line, .. }
            | Field::I64 { file, line, .. }
            | Field::U64 { file, line, .. }
            | Field::Usize { file, line, .. }
            | Field::Bool { file, line, .. }
            | Field::String { file, line, .. } => format!("{}:{}", file, line),
        }
    }

    /// Just the line number in the file.
    pub fn line_number(&self) -> u32 {
        match self {
            Field::F32 { line, .. }
            | Field::F64 { line, .. }
            | Field::I8 { line, .. }
            | Field::U8 { line, .. }
            | Field::I16 { line, .. }
            | Field::U16 { line, .. }
            | Field::I32 { line, .. }
            | Field::U32 { line, .. }
            | Field::I64 { line, .. }
            | Field::U64 { line, .. }
            | Field::Usize { line, .. }
            | Field::Bool { line, .. }
            | Field::String { line, .. } => *line,
        }
    }

    /// Create a HTML widget from this field with it's metadata.
    pub fn to_html_widget(&self, key: &str) -> String {
        match self {
            Field::F32 {
                value,
                min,
                max,
                step,
                ..
            } => Field::render_slider(key, *value, *min, *max, *step, "f32").to_string(),
            Field::F64 {
                value,
                min,
                max,
                step,
                ..
            } => Field::render_slider(key, *value, *min, *max, *step, "f64").to_string(),
            Field::I8 {
                value,
                min,
                max,
                step,
                ..
            } => Field::render_slider(key, *value, *min, *max, *step, "i8").to_string(),
            Field::U8 {
                value,
                min,
                max,
                step,
                ..
            } => Field::render_slider(key, *value, *min, *max, *step, "u8").to_string(),
            Field::I16 {
                value,
                min,
                max,
                step,
                ..
            } => Field::render_slider(key, *value, *min, *max, *step, "i16").to_string(),
            Field::U16 {
                value,
                min,
                max,
                step,
                ..
            } => Field::render_slider(key, *value, *min, *max, *step, "u16").to_string(),
            Field::I32 {
                value,
                min,
                max,
                step,
                ..
            } => Field::render_slider(key, *value, *min, *max, *step, "i32").to_string(),
            Field::U32 {
                value,
                min,
                max,
                step,
                ..
            } => Field::render_slider(key, *value, *min, *max, *step, "u32").to_string(),
            Field::I64 {
                value,
                min,
                max,
                step,
                ..
            } => Field::render_slider(key, *value, *min, *max, *step, "i64").to_string(),
            Field::U64 {
                value,
                min,
                max,
                step,
                ..
            } => Field::render_slider(key, *value, *min, *max, *step, "u64").to_string(),
            Field::Usize {
                value,
                min,
                max,
                step,
                ..
            } => Field::render_slider(key, *value, *min, *max, *step, "usize").to_string(),
            Field::Bool { value, .. } => Field::render_bool(key, *value).to_string(),
            Field::String { value, .. } => Field::render_string(key, value).to_string(),
        }
    }

    /// Render a slider widget for the number types.
    fn render_slider<'a, T>(
        key: &'a str,
        value: T,
        min: T,
        max: T,
        step: T,
        http_path: &'a str,
    ) -> impl Render + ToString + 'a
    where
        T: Display + 'a,
    {
        owned_html! {
            div (class="column") {
                input (type="range",
                    id=key.to_string(),
                    min=min.to_string(),
                    max=max.to_string(),
                    step=step.to_string(),
                    defaultValue=value.to_string(),
                    style="width: 100%",
                    // The value is a string, convert it to a number so it can be properly
                    // deserialized by serde
                    oninput=send(key, "Number(this.value)", http_path))
                { }
            }
            div (class="column is-narrow") {
                span (id=format!("{}_label", key), class="is-small")
                { : value.to_string() }
            }
        }
    }

    /// Render the bool widget.
    fn render_bool(key: &str, value: bool) -> impl Render + ToString + '_ {
        owned_html! {
            div (class="column") {
                input (type="checkbox",
                    id=key,
                    value=value.to_string(),
                    onclick=send(key, "this.checked", "bool"))
                { }
            }
            div (class="column is-narrow") {
                span (id=format!("{}_label", key))
                { : value.to_string() }
            }
        }
    }

    /// Render the string widget.
    fn render_string<'a>(key: &'a str, value: &'a str) -> impl Render + ToString + 'a {
        owned_html! {
            div (class="column") {
                input (type="text",
                    id=key,
                    value=value,
                    style="width: 100%",
                    onchange=send(key, "this.value", "string"))
                { }
            }
            div (class="column is-narrow") {
                span (id=format!("{}_label", key))
                { : value.to_string() }
            }
        }
    }
}

/// A struct used for deserializing POST request JSON data.
#[derive(Debug, Deserialize)]
struct PostData<T> {
    key: String,
    value: T,
}

lazy_static::lazy_static! {
    /// The list of fields with their data.
    #[doc(hidden)]
    pub static ref DATA: DashMap<&'static str, Field> = DashMap::new();
    /// The last known size of the DATA map, used to detect whether the page should refresh.
    static ref LAST_MAP_SIZE: Mutex<usize> = Mutex::new(0);
}

/// Launch the `const` tweaker web service.
///
/// This will launch a web server at `http://127.0.01:9938`.
#[ctor::ctor]
fn run() {
    // Run a blocking web server in a new thread
    thread::spawn(|| {
        task::block_on(async {
            let mut app = tide::new();
            // The main site
            app.at("/").get(main_site);
            // Whether the page should be refreshed or not
            app.at("/should_refresh").get(should_refresh);

            // Setting the data
            app.at("/set/f32").post(|r| handle_set_value(r, set_f32));
            app.at("/set/f64").post(|r| handle_set_value(r, set_f64));
            app.at("/set/bool").post(|r| handle_set_value(r, set_bool));
            app.at("/set/string")
                .post(|r| handle_set_value(r, set_string));
            app.listen("127.0.0.1:9938").await
        })
        .expect("Running web server failed");
    });
}

/// Build the actual site.
async fn main_site(_: Request<()>) -> Response {
    // Set LAST_MAP_SIZE to it's initial value
    let mut last_map_size = LAST_MAP_SIZE.lock().unwrap();
    *last_map_size = DATA.len();

    let body = html! {
        style { : include_str!("bulma.css") }
        style { : "* { font-family: sans-serif}" }
        body {
            // Title
            section (class="hero is-primary") {
                div (class="hero-body") {
                    div (class="container") {
                        h1 (class="title is-1") { : "Const Tweaker Web Interface" }
                    }
                }
            }
            // All the widgets
            : render_widgets();
            // The error message
            div (class="container") {
                div (class="notification is-danger") {
                    span(id="status") { }
                }
            }
        }
        script { : Raw(include_str!("send.js")) }
    };

    Response::new(200)
        .body_string(format!("{}", body))
        .set_header("content-type", "text/html;charset=utf-8")
}

/// Render all widgets.
fn render_widgets() -> impl Render {
    owned_html! {
        // All modules go in their own panels
        @for module in modules().into_iter() {
            section (class="section") {
                div (class="container box") {
                    h3 (class="title is-3") { : format!("Module: \"{}\"", module) }
                    : render_module(&module)
                }

                // The textbox to copy the output from
                div (class="container box") {
                    h4 (class="title is-4") { : "Changes" }
                    div (class="columns") {
                        div (class="column") {
                            textarea (
                                class="textarea",
                                style="font-family: monospace",
                                id=format!("{}_output", module.replace("::", "_")),
                                readonly,
                                placeholder="No changes")
                        }
                        div (class="column is-narrow control") {
                            button (class="button is-link", onclick=format!("copy_text(\"{}\")", module)) {
                                : "Copy"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Render a module of widgets.
fn render_module(module: &str) -> impl Render {
    let mut data = DATA
        .iter()
        .filter(|kv| kv.value().module_path() == module)
        .collect::<Vec<_>>();

    data.sort_by(|a, b| {
        a.value()
            .line_number()
            .partial_cmp(&b.value().line_number())
            .unwrap_or(Ordering::Equal)
    });

    owned_html! {
        // All widgets go into their own column box
        @for ref_multi in data.iter() {
            : render_widget(ref_multi.key(), ref_multi.value())
        }
    }
}

/// Render a single widget.
fn render_widget<'a>(key: &'a str, field: &'a Field) -> impl Render + 'a {
    owned_html! {
        div (class="columns") {
            div (class="column is-narrow") {
                // module::CONSTANT
                span (class="is-small") { : key }

                br {}
                // file:line
                span (class="tag") { : field.file() }
            }
            : Raw(field.to_html_widget(key))
        }
    }
}

/// The javascript call to send the updated data.
fn send(key: &str, look_for: &str, data_type: &str) -> String {
    format!(
        "send('{}', {}, '{}')",
        key.replace("\\", "\\\\"),
        look_for,
        data_type
    )
}

/// Whether the webpage should refresh itself or not.
async fn should_refresh(_request: Request<()>) -> Response {
    let mut last_map_size = LAST_MAP_SIZE.lock().unwrap();

    if *last_map_size == DATA.len() {
        // Don't need to do anything, just send an empty response
        Response::new(200)
    } else {
        // There is a size mismatch of the map, reload the page
        *last_map_size = DATA.len();

        Response::new(200).body_string("refresh".to_string())
    }
}

/// Handle setting of values.
async fn handle_set_value<T, F>(mut request: Request<()>, set_value: F) -> Response
where
    T: DeserializeOwned,
    F: Fn(&mut Field, T),
{
    let post_data: PostData<T> = request.body_json().await.expect("Could not decode JSON");
    set_value(
        &mut DATA
            .get_mut(&*post_data.key)
            .expect("Could not get item from map"),
        post_data.value,
    );

    Response::new(200)
}

/// Set a f32 value when the field matches the proper variant.
fn set_f32(field: &mut Field, new_value: f32) {
    match field {
        Field::F32 { ref mut value, .. } => {
            *value = new_value;
        }
        _ => panic!("Unexpected type, please report an issue"),
    }
}

/// Set a f64 value when the field matches the proper variant.
fn set_f64(field: &mut Field, new_value: f64) {
    match field {
        Field::F64 { ref mut value, .. } => {
            *value = new_value;
        }
        _ => panic!("Unexpected type, please report an issue"),
    }
}

/// Set a bool value when the field matches the proper variant.
fn set_bool(field: &mut Field, new_value: bool) {
    match field {
        Field::Bool { ref mut value, .. } => {
            *value = new_value;
        }
        _ => panic!("Unexpected type, please report an issue"),
    }
}

/// Set a string value when the field matches the proper variant.
fn set_string(field: &mut Field, new_value: String) {
    match field {
        Field::String { ref mut value, .. } => {
            *value = new_value;
        }
        _ => panic!("Unexpected type, please report an issue"),
    }
}

/// Get a list of all modules.
fn modules() -> Vec<String> {
    let mut modules: Vec<_> = DATA
        .iter()
        .map(|kv| kv.value().module_path().to_string())
        .collect::<_>();

    // Remove duplicate entries
    modules.sort();
    modules.dedup();

    modules
}
