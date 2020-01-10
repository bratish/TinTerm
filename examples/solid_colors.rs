use tinterm::*;

fn main() {
    let text = "Solid Colors Example";

    // Apply solid foreground color
    let fg_colored_text = text.fg(Color::RED);
    println!("Foreground: {}", fg_colored_text);

    // Apply solid background color
    let bg_colored_text = text.bg(Color::GREEN);
    println!("Background: {}", bg_colored_text);

    // Combine both foreground and background
    let combined_colored_text = text.fg(Color::BLUE).bg(Color::YELLOW);
    println!("Combined: {}", combined_colored_text);
} 