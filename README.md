# const-tweaker
Tweak const variables live from a web GUI.

![CI](https://github.com/tversteeg/const-tweaker/workflows/CI/badge.svg)
<a href="https://crates.io/crates/const-tweaker"><img src="https://img.shields.io/crates/v/const-tweaker.svg" alt="Version"/></a>
<a href="https://docs.rs/const-tweaker"><img src="https://img.shields.io/badge/api-rustdoc-blue.svg" alt="Rust Documentation"/></a>
<img src="https://img.shields.io/crates/l/const-tweaker.svg" alt="License"/>

This library opens a web interface when the application is run, allowing you to change the values of constants in real time.
It's especially useful for gamedev where you want to tweak some variables without introducing a hot-reloading scripting language for it.

The server is opened at [`127.0.0.1:9938`](http://127.0.0.1:9938).

## Example

```rust
const_tweaker::tweak! {
    VALUE: f64 = 0.0;
}

fn main() {
	// Initialize the server at 'http://127.0.0.1:9938'
	const_tweaker::run().expect("Could not run server");

	// Enter a GUI/Game loop
	loop {
		// Print the constant value that can be changed from the website
		println!("VALUE: {}", VALUE);

		// ...
	}
}
```
