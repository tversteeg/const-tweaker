//! # Runtime `const` tweaking
//!
//! This library starts a web server at `http://127.0.0.1:9938` where you can change the values of `const` variables in your crate.
//!
//! `f64` & `bool` are the types that are currently supported.
//!
//! ## Example
//! ```rust
//! // Tweak `VALUE` when running in debug mode
//! // This will render a slider in the web GUI because the type here is a `f64`
//! #[const_tweaker::tweak]
//! const VALUE: f64 = 0.0;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize the server at 'http://127.0.0.1:9938' when running in debug mode
//!     #[cfg(debug_assertions)]
//!     const_tweaker::run()?;
//!
//!     // Enter a GUI/Game loop
//!     loop {
//!         // ...
//!
//!         // Print the constant value that can be changed from the website.
//!         println!("VALUE: {}", VALUE);
//! #       break;
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! Some widgets have customizable options, as seen in the examples below:
//!
//! `f64`:
//! ```rust
//! // Spawns a slider
//! #[const_tweaker::tweak]
//! const DEFAULT_VALUE: f64 = 0.0;
//!
//! // Spawns a slider with 10 steps from 0-1
//! #[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.1)]
//! const CUSTOM_VALUE: f64 = 0.0;
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

use anyhow::Result;
use async_std::task;
use dashmap::DashMap;
use horrorshow::{html, owned_html, Raw, Render};
use serde::{de::DeserializeOwned, Deserialize};
use std::{cmp::Ordering, string::ToString, thread};
use tide::{Request, Response};

pub use const_tweaker_attribute::tweak;

/// Type representing the const field with metadata.
#[doc(hidden)]
#[derive(Debug)]
pub enum Field {
    F32 {
        value: f32,
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
            | Field::Bool { module, .. }
            | Field::String { module, .. } => &*module,
        }
    }

    /// The file with line number.
    pub fn file(&self) -> String {
        match self {
            Field::F32 { file, line, .. }
            | Field::F64 { file, line, .. }
            | Field::Bool { file, line, .. }
            | Field::String { file, line, .. } => format!("{}:{}", file, line),
        }
    }

    /// Just the line number in the file.
    pub fn line_number(&self) -> u32 {
        match self {
            Field::F32 { line, .. }
            | Field::F64 { line, .. }
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
            } => Field::render_float(key, (*value).into(), *min, *max, *step, "f32").to_string(),
            Field::F64 {
                value,
                min,
                max,
                step,
                ..
            } => Field::render_float(key, *value, *min, *max, *step, "f64").to_string(),
            Field::Bool { value, .. } => Field::render_bool(key, *value).to_string(),
            Field::String { value, .. } => Field::render_string(key, value).to_string(),
        }
    }

    /// Render the float widget for both float types.
    fn render_float<'a>(
        key: &'a str,
        value: f64,
        min: f64,
        max: f64,
        step: f64,
        http_path: &'a str,
    ) -> impl Render + ToString + 'a {
        owned_html! {
            div (class="column") {
                input (type="range",
                    id=key,
                    min=min,
                    max=max,
                    step=step,
                    defaultValue=value,
                    style="width: 100%",
                    // The value is a string, convert it to a number so it can be properly
                    // deserialized by serde
                    oninput=send(key, "Number(this.value)", http_path))
                { }
            }
            div (class="column is-narrow") {
                span (id=format!("{}_label", key), class="is-small")
                { : value }
            }
        }
    }

    /// Render the bool widget.
    fn render_bool<'a>(key: &'a str, value: bool) -> impl Render + ToString + 'a {
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
}

/// Launch the `const` tweaker web service.
///
/// This will launch a web server at `http://127.0.01:9938`.
pub fn run() -> Result<()> {
    // Run a blocking web server in a new thread
    thread::spawn(|| {
        task::block_on(async {
            let mut app = tide::new();
            app.at("/").get(main_site);
            app.at("/set/f32").post(|r| handle_set_value(r, set_f32));
            app.at("/set/f64").post(|r| handle_set_value(r, set_f64));
            app.at("/set/bool").post(|r| handle_set_value(r, set_bool));
            app.at("/set/string")
                .post(|r| handle_set_value(r, set_string));
            app.listen("127.0.0.1:9938").await
        })
        .expect("Running web server failed");
    });

    Ok(())
}

/// Build the actual site.
async fn main_site(_: Request<()>) -> Response {
    let body = html! {
        style { : include_str!("bulma.css") }
        style { : "* { font-family: sans-serif}" }
        body {
            // Title
            div (class="container box") {
                h1 (class="title is-1") { : "Const Tweaker Web Interface" }
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
            }
        }
    }
}

/// Render a module of widgets.
fn render_module<'a>(module: &'a str) -> impl Render + 'a {
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
