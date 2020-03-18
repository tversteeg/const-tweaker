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
//! // Spawns a slider with 10 steps from 0-10
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

use anyhow::Result;
use async_std::task;
use dashmap::DashMap;
use horrorshow::{html, owned_html, Raw, Render};
use serde::Deserialize;
use std::thread;
use tide::{Request, Response};

pub use const_tweaker_attribute::tweak;

/// Type representing the const field with metadata.
#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
pub enum Field {
    F64 {
        value: f64,
        /// Minimum value of slider.
        min: f64,
        /// Maximum value of slider.
        max: f64,
        /// Step increase of slider.
        step: f64,
    },
    Bool {
        value: bool,
    },
}

impl Field {
    /// Set a f64 value when the field matches the proper variant.
    pub fn set_f64(&mut self, new_value: f64) -> &Self {
        match self {
            Field::F64 { ref mut value, .. } => {
                *value = new_value;
                self
            }
            _ => panic!("Unexpected type, please report an issue"),
        }
    }

    /// Set a bool value when the field matches the proper variant.
    pub fn set_bool(&mut self, new_value: bool) -> &Self {
        match self {
            Field::Bool { ref mut value, .. } => {
                *value = new_value;
                self
            }
            _ => panic!("Unexpected type, please report an issue"),
        }
    }

    /// Create a HTML widget from this field with it's metadata.
    pub fn to_html_widget(&self, key: &str) -> String {
        match self {
            Field::F64 {
                value,
                min,
                max,
                step,
            } => {
                (owned_html! {
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
                            oninput=send(key, "Number(this.value)", "f64"))
                        { }
                    }
                    div (class="column is-narrow") {
                        span (id=format!("{}_label", key), class="is-small")
                        { : value }
                    }
                })
                .to_string()
            }
            Field::Bool { value } => (owned_html! {
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
            })
            .to_string(),
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
            app.at("/set/f64").post(handle_set_f64);
            app.at("/set/bool").post(handle_set_bool);
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
        div (class="container") {
            h1 (class="title") { : "Const Tweaker Web Interface" }
            p { : widgets() }
            div (class="notification is-danger") {
                span(id="status") { }
            }
        }
        script { : Raw(include_str!("send.js")) }
    };

    Response::new(200)
        .body_string(format!("{}", body))
        .set_header("content-type", "text/html;charset=utf-8")
}

/// Render all widgets.
fn widgets() -> impl Render {
    owned_html! {
        @for ref_multi in DATA.iter() {
            div (class="columns box") {
                div (class="column is-narrow") {
                    span (class="tag") { : ref_multi.key() }
                }
                : Raw(ref_multi.value().to_html_widget(ref_multi.key()))
            }
        }
    }
}

/// The javascript call to send the updated data.
fn send(key: &str, look_for: &str, data_type: &str) -> String {
    format!("send('{}', {}, '{}')", key, look_for, data_type)
}

// Handle setting of values
async fn handle_set_f64(mut request: Request<()>) -> Response {
    let post_data: PostData<f64> = request.body_json().await.expect("Could not decode JSON");
    DATA.get_mut(&*post_data.key)
        .expect("Could not get item from map")
        .set_f64(post_data.value);

    Response::new(200)
}

async fn handle_set_bool(mut request: Request<()>) -> Response {
    let post_data: PostData<bool> = request.body_json().await.expect("Could not decode JSON");
    DATA.get_mut(&*post_data.key)
        .expect("Could not get item from map")
        .set_bool(post_data.value);

    Response::new(200)
}
