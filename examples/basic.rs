use anyhow::Result;
use std::{thread, time::Duration};

// Custom slider minimum value, maximum value & step size
#[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.1)]
const F64_VALUE_CUSTOM: f64 = 0.0;

// Default values for slider
#[const_tweaker::tweak]
const F64_VALUE_DEFAULT: f64 = 0.0;

// Checkbox
#[const_tweaker::tweak]
const BOOL_VALUE: bool = false;

// Text input
#[const_tweaker::tweak]
const STRING_VALUE: &str = "Hello";

fn main() -> Result<()> {
    // Run the tweaker server only when in debug mode
    #[cfg(debug_assertions)]
    const_tweaker::run()?;

    // Print the constant value times every second
    loop {
        dbg!(
            F64_VALUE_CUSTOM,
            F64_VALUE_DEFAULT,
            BOOL_VALUE,
            STRING_VALUE
        );

        thread::sleep(Duration::from_secs(1));
    }
}
