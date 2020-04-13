mod sub_module;

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

// Default values for integer sliders
#[const_tweaker::tweak]
const I8_VALUE_DEFAULT: i8 = 0;
#[const_tweaker::tweak]
const U8_VALUE_DEFAULT: u8 = 0;
#[const_tweaker::tweak]
const I16_VALUE_DEFAULT: i16 = 0;
#[const_tweaker::tweak]
const U16_VALUE_DEFAULT: u16 = 0;
#[const_tweaker::tweak]
const I32_VALUE_DEFAULT: i32 = 0;
#[const_tweaker::tweak]
const U32_VALUE_DEFAULT: u32 = 0;
#[const_tweaker::tweak]
const I64_VALUE_DEFAULT: i64 = 0;
#[const_tweaker::tweak]
const U64_VALUE_DEFAULT: u64 = 0;
#[const_tweaker::tweak]
const USIZE_VALUE_DEFAULT: usize = 0;

// Checkbox
#[const_tweaker::tweak]
const BOOL_VALUE: bool = false;

// Text input
#[const_tweaker::tweak]
const STRING_VALUE: &str = "Hello";

fn main() {
    // Print the constant value times every second
    loop {
        dbg!(
            F32_VALUE_CUSTOM,
            F32_VALUE_DEFAULT,
            F64_VALUE_CUSTOM,
            F64_VALUE_DEFAULT,
            I8_VALUE_DEFAULT,
            U8_VALUE_DEFAULT,
            I16_VALUE_DEFAULT,
            U16_VALUE_DEFAULT,
            I32_VALUE_DEFAULT,
            U32_VALUE_DEFAULT,
            I64_VALUE_DEFAULT,
            U64_VALUE_DEFAULT,
            USIZE_VALUE_DEFAULT,
            BOOL_VALUE,
            STRING_VALUE,
        );

        thread::sleep(Duration::from_secs(1));
    }
}
