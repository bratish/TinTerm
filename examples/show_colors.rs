use tinterm::Color;

fn main() {
    // Basic colors
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::RED.r, Color::RED.g, Color::RED.b, "RED");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::GREEN.r, Color::GREEN.g, Color::GREEN.b, "GREEN");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::BLUE.r, Color::BLUE.g, Color::BLUE.b, "BLUE");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::YELLOW.r, Color::YELLOW.g, Color::YELLOW.b, "YELLOW");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::MAGENTA.r, Color::MAGENTA.g, Color::MAGENTA.b, "MAGENTA");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::CYAN.r, Color::CYAN.g, Color::CYAN.b, "CYAN");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::WHITE.r, Color::WHITE.g, Color::WHITE.b, "WHITE");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::BLACK.r, Color::BLACK.g, Color::BLACK.b, "BLACK");

    println!("\n--- Pink Colors ---");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::PINK.r, Color::PINK.g, Color::PINK.b, "PINK");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::LIGHT_PINK.r, Color::LIGHT_PINK.g, Color::LIGHT_PINK.b, "LIGHT_PINK");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::HOT_PINK.r, Color::HOT_PINK.g, Color::HOT_PINK.b, "HOT_PINK");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::DEEP_PINK.r, Color::DEEP_PINK.g, Color::DEEP_PINK.b, "DEEP_PINK");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::PALE_VIOLET_RED.r, Color::PALE_VIOLET_RED.g, Color::PALE_VIOLET_RED.b, "PALE_VIOLET_RED");
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", Color::MEDIUM_VIOLET_RED.r, Color::MEDIUM_VIOLET_RED.g, Color::MEDIUM_VIOLET_RED.b, "MEDIUM_VIOLET_RED");

    // To make this more maintainable, let's create a macro to help with the printing
    macro_rules! print_color {
        ($color:expr, $name:expr) => {
            println!("\x1b[38;2;{};{};{}m{}\x1b[0m", $color.r, $color.g, $color.b, $name);
        };
    }

    println!("\n--- Red Colors ---");
    print_color!(Color::LIGHT_SALMON, "LIGHT_SALMON");
    print_color!(Color::SALMON, "SALMON");
    print_color!(Color::DARK_SALMON, "DARK_SALMON");
    print_color!(Color::LIGHT_CORAL, "LIGHT_CORAL");
    print_color!(Color::INDIAN_RED, "INDIAN_RED");
    print_color!(Color::CRIMSON, "CRIMSON");
    print_color!(Color::FIRE_BRICK, "FIRE_BRICK");
    print_color!(Color::DARK_RED, "DARK_RED");

    println!("\n--- Orange Colors ---");
    print_color!(Color::ORANGE_RED, "ORANGE_RED");
    print_color!(Color::TOMATO, "TOMATO");
    print_color!(Color::CORAL, "CORAL");
    print_color!(Color::DARK_ORANGE, "DARK_ORANGE");
    print_color!(Color::ORANGE, "ORANGE");

    println!("\n--- Yellow Colors ---");
    print_color!(Color::GOLD, "GOLD");
    print_color!(Color::YELLOW, "YELLOW");
    print_color!(Color::KHAKI, "KHAKI");
    print_color!(Color::DARK_KHAKI, "DARK_KHAKI");

    println!("\n--- Green Colors ---");
    print_color!(Color::LAWN_GREEN, "LAWN_GREEN");
    print_color!(Color::CHARTREUSE, "CHARTREUSE");
    print_color!(Color::LIME_GREEN, "LIME_GREEN");
    print_color!(Color::LIGHT_GREEN, "LIGHT_GREEN");
    print_color!(Color::PALE_GREEN, "PALE_GREEN");
    print_color!(Color::FOREST_GREEN, "FOREST_GREEN");
    print_color!(Color::SPRING_GREEN, "SPRING_GREEN");
    print_color!(Color::SEA_GREEN, "SEA_GREEN");
    print_color!(Color::MEDIUM_SEA_GREEN, "MEDIUM_SEA_GREEN");
    print_color!(Color::DARK_SEA_GREEN, "DARK_SEA_GREEN");
    print_color!(Color::OLIVE, "OLIVE");
    print_color!(Color::DARK_OLIVE_GREEN, "DARK_OLIVE_GREEN");

    // ... Continue with other color groups ...
    // I can provide the full example if you'd like, but this shows the pattern
    // Each color group would be printed with a header and then each color in that group
} 