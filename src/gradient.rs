use crate::color::Color;

pub trait Gradient {
    fn gradient(&self, start_color: Color, end_color: Color, block: Option<bool>) -> String;
    fn gradient_bg(&self, start_color: Color, end_color: Color, block: Option<bool>) -> String;
}

impl Gradient for str {
    fn gradient(&self, start_color: Color, end_color: Color, block: Option<bool>) -> String {
        apply_gradient(self, start_color, end_color, block.unwrap_or(false))
    }

    fn gradient_bg(&self, start_color: Color, end_color: Color, block: Option<bool>) -> String {
        apply_gradient_bg(self, start_color, end_color, block.unwrap_or(false))
    }
}

fn apply_gradient(text: &str, start_color: Color, end_color: Color, block: bool) -> String {
    let mut result = String::new();
    let mut visible_chars = 0;
    let mut total_visible = 0;
    
    // First pass: count visible characters
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            while let Some(next) = chars.next() {
                if next == 'm' {
                    break;
                }
            }
            continue;
        }
        if c != '\n' {
            total_visible += 1;
        }
    }
    
    // Second pass: apply gradient
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            let mut seq = String::from(c);
            while let Some(&next) = chars.peek() {
                seq.push(next);
                chars.next();
                if next == 'm' {
                    break;
                }
            }
            
            // Only skip foreground color sequences
            if !seq.contains("[38;2;") {
                result.push_str(&seq);
            }
            continue;
        }
        
        if c == '\n' {
            result.push(c);
            if block {
                visible_chars = 0;
            }
            continue;
        }
        
        let progress = if total_visible > 1 {
            visible_chars as f32 / (total_visible - 1) as f32
        } else {
            0.0
        };
        
        let color = start_color.interpolate(&end_color, progress);
        result.push_str(&format!("\x1b[38;2;{};{};{}m", color.r, color.g, color.b));
        result.push(c);
        
        visible_chars += 1;
    }
    
    result.push_str("\x1b[39m"); // Reset foreground color
    result
}

fn apply_gradient_bg(text: &str, start_color: Color, end_color: Color, block: bool) -> String {
    let mut result = String::new();
    let mut visible_chars = 0;
    let mut total_visible = 0;
    
    // First pass: count visible characters
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            while let Some(next) = chars.next() {
                if next == 'm' {
                    break;
                }
            }
            continue;
        }
        if c != '\n' {
            total_visible += 1;
        }
    }
    
    // Second pass: apply gradient
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            let mut seq = String::from(c);
            while let Some(&next) = chars.peek() {
                seq.push(next);
                chars.next();
                if next == 'm' {
                    break;
                }
            }
            
            // Only skip background color sequences
            if !seq.contains("[48;2;") {
                result.push_str(&seq);
            }
            continue;
        }
        
        if c == '\n' {
            result.push_str("\x1b[49m"); // Reset background color at the end of each line
            result.push(c);
            if block {
                visible_chars = 0;
            }
            continue;
        }
        
        let progress = if total_visible > 1 {
            visible_chars as f32 / (total_visible - 1) as f32
        } else {
            0.0
        };
        
        let color = start_color.interpolate(&end_color, progress);
        result.push_str(&format!("\x1b[48;2;{};{};{}m", color.r, color.g, color.b));
        result.push(c);
        
        visible_chars += 1;
    }
    
    result.push_str("\x1b[49m"); // Reset background color
    result
} 