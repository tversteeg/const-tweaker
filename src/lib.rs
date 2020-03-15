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
use dashmap::DashMap;
use std::thread;

/// Macro for exposing a `const` value so it's value can be changed at runtime.
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
        unimplemented!("const-tweaker doesn't support type ", stringify!($type));
    };
    ($name:ident, $type:ty, $default_value:expr, $map:expr, $($other_lines:tt)*) => {
        // Create a new type for this constant, inspired by lazy_static
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct $name { __private_field: () }

        impl $name {
            pub fn get(&self) -> $type {
                // Try to get the value from the map
                match $map.get(stringify!($name)) {
                    // Return it if it succeeds
                    Some(value) => *value,
                    None => {
                        // Otherwise add the default value to the map and return that instead
                        let value = $default_value;
                        $map.insert(stringify!($name), value);

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

        impl std::ops::Deref for $name {
            type Target = $type;

            fn deref(&self) -> &'static $type {
                // Make what is returned static, this leaks the memory of the primitive which is a
                // workaround because Deref has to return a reference. I couldn't find another way
                // to return one while staying in the lifetime of the dashmap object.
                unsafe {
                    std::mem::transmute::<&$type, &'static $type>(&self.get())
                }
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

/// Launch the `const` tweaker web service.
///
/// This will launch a web server at `http://127.0.01:9938`.
pub fn run() -> Result<()> {
    // Run a blocking web server in a new thread
    thread::spawn(|| {
        task::block_on(async {
            let mut app = tide::new();
            app.at("/").get(|_| async move { "Hello world" });
            app.listen("127.0.0.1:9938").await
        })
        .expect("Running web server failed");
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
