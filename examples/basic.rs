use anyhow::Result;
use std::{thread, time::Duration};

const_tweaker::tweak! {
    F64_VALUE: f64 = 0.0;
    BOOL_VALUE: bool = false;
}

fn main() -> Result<()> {
    // Run the tweaker server only when in debug mode
    #[cfg(debug_assertions)]
    const_tweaker::run()?;

    // Print the constant value times every second
    loop {
        dbg!(F64_VALUE);
        dbg!(BOOL_VALUE);

        thread::sleep(Duration::from_secs(1));
    }
}
