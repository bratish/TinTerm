use tinterm::*;

fn main() {
    let text = "Gradient Colors Example";

    // Apply foreground gradient
    let fg_gradient_text = text.gradient(Color::RED, Color::BLUE, None);
    println!("Foreground Gradient:\n{}", fg_gradient_text);

    // Apply background gradient
    let bg_gradient_text = text.gradient_bg(Color::GREEN, Color::YELLOW, None);
    println!("Background Gradient:\n{}", bg_gradient_text);

    // Multiline gradient with block true
    let multiline_text = "Line 1\nLine 2\nLine 3";
    let multiline_gradient = multiline_text.gradient(Color::CYAN, Color::MAGENTA, Some(true));
    println!("Multiline Gradient (Block true):\n{}", multiline_gradient);

    // Multiline gradient with block false
    let multiline_gradient_block_false = multiline_text.gradient(Color::CYAN, Color::MAGENTA, Some(false));
    println!("Multiline Gradient (Block false):\n{}", multiline_gradient_block_false);
} 