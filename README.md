# Tinterm: Tinted Terminal with Colors and Gradients

A Rust library for adding beautiful colors and gradients to your terminal output with an ergonomic API.

## Features

- 🎨 RGB color support for both foreground and background
- 🌈 Smooth color gradients
- ⚡ Simple and intuitive API
- 📝 Text styling (bold, italic, underline, etc.)
- 🔄 Chainable methods
- 📚 Comprehensive documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tinterm = "0.1.0"
```

## Quick Start

```rust
use tinterm::*;
// Simple colored text
println!("Hello".color(Color::RED));
// Background color
println!("World".background_color(Color::BLUE));
// Gradient text
println!("Rainbow".gradient(Color::RED, Color::BLUE, None));
// Styled text
println!("Bold and Blue".bold().color(Color::BLUE));
```

## Color Methods

### Basic Coloring

```rust
// Using descriptive names
"Text".color(Color::RED);
"Text".background_color(Color::BLUE);
// Using short aliases
"Text".fg(Color::RED);
"Text".bg(Color::BLUE);
```

### Gradients

```rust
// Foreground gradient
"Gradient Text".gradient(Color::RED, Color::BLUE, None);
// Background gradient
"Gradient Background".gradient_bg(Color::RED, Color::BLUE, None);
// Multiline gradient with block mode
let text = "Line 1\nLine 2";
text.gradient(Color::RED, Color::BLUE, Some(true));
```


### Text Styling

```rust
"This line is bold".bold();
"This line is italic".italic();
"This line is underline".underline();
"This line is strikethrough".strikethrough();
"This line is dim".dim();
"This line is blink".blink();
"This line is reverse".reverse();
"This line is hidden".hidden();
"This line is bright".bright();
```

### Method Chaining

```rust
"Styled Text"
.bold()
.color(Color::RED)
.background_color(Color::BLUE);
```


## Predefined Colors

The library comes with several predefined colors:

```rust
Color::RED
Color::GREEN
Color::BLUE
// ... and more
```

You can also create custom colors:

```rust
// Using RGB values (0-255)
let custom_color = Color::new(255, 128, 0);

// Using hex values
let hex_color = Color::from_hex("#FF8000").unwrap();
// or without the hash
let hex_color = Color::from_hex("FF8000").unwrap();

// Using them in text
println!("Custom RGB".color(custom_color));
println!("Custom Hex".color(hex_color));
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.