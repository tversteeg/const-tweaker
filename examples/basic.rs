use anyhow::Result;
use std::{thread, time::Duration};

// Custom slider minimum value, maximum value & step size
#[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.1)]
const F32_VALUE_CUSTOM: f32 = 0.0;

// Default values for slider
#[const_tweaker::tweak]
const F32_VALUE_DEFAULT: f32 = 0.0;

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

// Value that will be accessed after 10 seconds, which will trigger a page refresh
#[const_tweaker::tweak]
const DELAYED_VALUE: &str = "Delayed";

fn main() -> Result<()> {
    // Run the tweaker server only when in debug mode
    #[cfg(debug_assertions)]
    const_tweaker::run()?;

    let mut countdown = 10i32;

    // Print the constant value times every second
    loop {
        dbg!(
            F32_VALUE_CUSTOM,
            F32_VALUE_DEFAULT,
            F64_VALUE_CUSTOM,
            F64_VALUE_DEFAULT,
            BOOL_VALUE,
            STRING_VALUE
        );

        // Wait for 10 seconds before printing the delayed value once
        countdown -= 1;
        if countdown == 0 {
            dbg!(DELAYED_VALUE);
        }

        thread::sleep(Duration::from_secs(1));
    }
}
