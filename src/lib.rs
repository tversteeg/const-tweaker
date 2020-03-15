//! # Runtime `const` tweaking
//!
//! This library starts a web server at `http://127.0.0.1:9938` where you can change the values of `const` variables in your crate.
//!
//! ## Example
//! ```rust
//! const_tweaker::tweak! {
//!     VALUE: f64 = 0.0;
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize the server at 'http://127.0.0.1:9938'
//!     const_tweaker::run();
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

use anyhow::Result;
use async_std::task;
use dashmap::{mapref::multiple::RefMulti, DashMap};
use horrorshow::{html, owned_html, Raw, Render};
use serde::Deserialize;
use std::{fmt::Display, thread};
use tide::{Request, Response};

/// Macro for exposing a `const` value so it's value can be changed at runtime.
///
/// `f64` & `bool` are the types that are currently supported.
///
/// ```rust
/// const_tweaker::tweak! {
///     F64_VALUE: f64 = 0.0;
///     BOOL_VALUE: bool = false;
/// };
/// ```
#[macro_export]
macro_rules! tweak {
    ($name:ident : f64 = $default_value:expr; $($other_lines:tt)*) => {
        $crate::tweak!($name, f64, $default_value, $crate::__F64S, $($other_lines)*);
    };
    ($name:ident : bool = $default_value:expr; $($other_lines:tt)*) => {
        $crate::tweak!($name, bool, $default_value, $crate::__BOOLS, $($other_lines)*);
    };
    ($_name:ident : $type:ty = $_default_value:expr; $($other_lines:tt)*) => {
        compile_error!(concat!("const-tweaker doesn't support type: ", stringify!($type)));
    };
    ($name:ident, $type:ty, $default_value:expr, $map:expr, $($other_lines:tt)*) => {
        // Create a new type for this constant, inspired by lazy_static
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct $name { __private_field: () }
        impl $name {
            pub fn get(&self) -> $type {
                let key = concat!(file!(), "::", stringify!($name));
                // Try to get the value from the map
                match $map.get(key) {
                    // Return it if it succeeds
                    Some(value) => *value,
                    None => {
                        // Otherwise add the default value to the map and return that instead
                        let value = $default_value;
                        $map.insert(key, value);

                        value
                    }
                }
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self.get())
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self.get())
            }
        }
        impl std::ops::Deref for $name {
            type Target = $type;

            fn deref(&self) -> &'static $type {
                // Make what is returned static, this leaks the memory of the primitive which is a
                // workaround because Deref has to return a reference. I couldn't find another way
                // to return one while staying in the lifetime of the dashmap object.
                unsafe { std::mem::transmute::<&$type, &'static $type>(&self.get()) }
            }
        }
        #[doc(hidden)]
        static $name: $name = $name { __private_field: () };
        // Call it recursively for all other lines
        $crate::tweak!($($other_lines)*);
    };
    () => ()
}

lazy_static::lazy_static! {
    #[doc(hidden)]
    pub static ref __F64S: DashMap<&'static str, f64> = DashMap::new();
    pub static ref __BOOLS: DashMap<&'static str, bool> = DashMap::new();
}

#[derive(Debug, Deserialize)]
struct PostData<T> {
    key: String,
    value: T,
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
        h1 { : "Const Tweaker Web Interface" }
        p { : f64s() }
        p { : bools() }
        span(id="status"){ }
        script { : Raw(r#"
async function send(source, value, data_type) {
    // Change the label
    var label_element = document.getElementById(source + '_label');
    label_element.innerHTML = value;

    // Make the request
    fetch('/set/' + data_type, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({key: source, value: value})
    }).catch(err => {
        var status_element = document.getElementById('status');
        status.innerHTML = 'HTTP Error: ' + err.status;
    });
}
"#) }
    };

    Response::new(200)
        .body_string(format!("{}", body))
        .set_header("content-type", "text/html;charset=utf-8")
}

fn f64s() -> impl Render {
    // Render sliders
    owned_html! {
        @for ref_multi in __F64S.iter() {
            p {
                span { : ref_multi.key() }
                br {}
                input(type="range",
                    id=ref_multi.key(),
                    min="-100",
                    max="100",
                    defaultValue=ref_multi.value(),
                    // The value is a string, convert it to a number so it can be properly
                    // deserialized by serde
                    oninput=send(&ref_multi, "Number(this.value)", "f64"))
                { }
                span (id=format!("{}_label", ref_multi.key()))
                    { : ref_multi.value() }
            }
        }
    }
}

fn bools() -> impl Render {
    // Render checkboxes
    owned_html! {
        @ for ref_multi in __BOOLS.iter() {
            p {
                span { : ref_multi.key() }
                br {}
                input(type="checkbox",
                    id=ref_multi.key(),
                    value=ref_multi.value().to_string(),
                    onclick=send(&ref_multi, "this.checked", "bool"))
                    { }
                span (id=format!("{}_label", ref_multi.key()))
                    { : ref_multi.value().to_string() }
            }
        }
    }
}

/// The javascript call to send the updated data.
fn send<T>(ref_multi: &RefMulti<&str, T>, look_for: &str, data_type: &str) -> String
where
    T: Display,
{
    format!("send('{}', {}, '{}')", ref_multi.key(), look_for, data_type)
}

// Handle setting of values
async fn handle_set_f64(mut request: Request<()>) -> Response {
    let post_data: PostData<f64> = request.body_json().await.expect("Could not decode JSON");
    __F64S.alter(&*post_data.key, |_, _| post_data.value);

    Response::new(200)
}

async fn handle_set_bool(mut request: Request<()>) -> Response {
    let post_data: PostData<bool> = request.body_json().await.expect("Could not decode JSON");
    __BOOLS.alter(&*post_data.key, |_, _| post_data.value);

    Response::new(200)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
