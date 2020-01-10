use tinterm::*;

fn main() {
    let text = "Test String";
    
    // Test 1: FG then BG
    let result = text
        .gradient(Color::WHITE, Color::BLACK, None)
        .gradient_bg(Color::MAGENTA, Color::CYAN, None);
    
    println!("\nTest 1 - FG then BG raw escape sequences:");
    for byte in result.bytes() {
        print!("{:02X} ", byte);
    }
    println!("\nTest 1 visual result: {}", result);

    // Test 2: BG then FG
    let result2 = text
        .gradient_bg(Color::MAGENTA, Color::CYAN, None)
        .gradient(Color::WHITE, Color::BLACK, None);
    
    println!("\nTest 2 - BG then FG raw escape sequences:");
    for byte in result2.bytes() {
        print!("{:02X} ", byte);
    }
    println!("\nTest 2 visual result: {}", result2);
} 