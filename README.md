<h1 align="center">const-tweaker</h1>
<p align="center">
	Tweak const variables live from a web GUI.
</p>
	
<p align="center">
	<a href="https://github.com/tversteeg/const-tweaker/actions"><img src="https://github.com/tversteeg/const-tweaker/workflows/CI/badge.svg" alt="CI"/></a>
	<a href="https://crates.io/crates/const-tweaker"><img src="https://img.shields.io/crates/v/const-tweaker.svg" alt="Version"/></a>
	<a href="https://docs.rs/const-tweaker"><img src="https://img.shields.io/badge/api-rustdoc-blue.svg" alt="Rust Documentation"/></a>
	<img src="https://img.shields.io/crates/l/const-tweaker.svg" alt="License"/>
	<br/><br/>
	<img src="img/example.gif">
	<br/>
</p>

This library opens a web interface when the application is run, allowing you to change the values of constants in real time.
It's especially useful for gamedev where you want to tweak some variables without introducing a hot-reloading scripting language for it.

After running your application the web GUI to change constants is opened at [`127.0.0.1:9938`](http://127.0.0.1:9938).

## Example

```rust
// Create a slider to tweak 'VALUE' in the web GUI
#[const_tweaker::tweak]
const VALUE: f64 = 0.0;

fn main() {
	// Enter a GUI/Game loop
	loop {
		// Print the constant value that can be changed from the website
		println!("VALUE: {}", VALUE);

		// ...
	}
}
```
