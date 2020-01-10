mod color;
mod gradient;
mod text_modifier;
mod color_presets;

pub use color::Color;
pub use gradient::Gradient;
pub use text_modifier::TextModifier;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_gradient() {
        let result = "test".gradient(Color::RED, Color::BLUE, None);
        assert!(!result.is_empty());
        assert!(result.contains("\x1b[38;2;"));
        assert!(result.contains("\x1b[39m")); // Check reset sequence
    }

    #[test]
    fn test_basic_gradient_bg() {
        let result = "test".gradient_bg(Color::RED, Color::BLUE, None);
        assert!(!result.is_empty());
        assert!(result.contains("\x1b[48;2;"));
        assert!(result.contains("\x1b[49m")); // Check reset sequence
    }

    #[test]
    fn test_empty_string() {
        let result = "".gradient(Color::RED, Color::BLUE, None);
        assert!(result.is_empty());
        
        let result_bg = "".gradient_bg(Color::RED, Color::BLUE, None);
        assert!(result_bg.is_empty());
    }

    #[test]
    fn test_multiline_block_true() {
        let text = "line1\nline2";
        let result = text.gradient(Color::RED, Color::BLUE, Some(true));
        assert!(result.contains("\n"));
        // Each line should start with its own color sequence
        let lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("\x1b[38;2;"));
        assert!(lines[1].starts_with("\x1b[38;2;"));
    }

    #[test]
    fn test_multiline_block_false() {
        let text = "line1\nline2";
        let result = text.gradient(Color::RED, Color::BLUE, Some(false));
        assert!(result.contains("\n"));
        // Color should continue across lines
        let lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("\x1b[38;2;"));
        assert!(lines[1].contains("\x1b[38;2;")); // Second line should contain gradient but not start with it
    }

    #[test]
    fn test_multiline_bg_block_false() {
        let text = "line1\nline2";
        let result = text.gradient_bg(Color::RED, Color::BLUE, Some(false));
        assert!(result.contains("\n"));
        // Should have background reset at end of each line
        assert!(result.contains("\x1b[49m\n"));
        // Color should continue across lines
        let lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("\x1b[48;2;"));
        assert!(lines[0].ends_with("\x1b[49m"));
    }

    #[test]
    fn test_gradient_chaining() {
        // Test FG then BG
        let result = "test"
            .gradient(Color::RED, Color::BLUE, None)
            .gradient_bg(Color::GREEN, Color::YELLOW, None);
        
        // Should contain both FG and BG sequences
        assert!(result.contains("\x1b[38;2;")); // FG
        assert!(result.contains("\x1b[48;2;")); // BG
        // Should contain both reset sequences
        assert!(result.contains("\x1b[39m")); // FG reset
        assert!(result.contains("\x1b[49m")); // BG reset
        
        // Test BG then FG
        let result = "test"
            .gradient_bg(Color::GREEN, Color::YELLOW, None)
            .gradient(Color::RED, Color::BLUE, None);
        
        // Should contain both FG and BG sequences
        assert!(result.contains("\x1b[38;2;")); // FG
        assert!(result.contains("\x1b[48;2;")); // BG
        // Should contain both reset sequences
        assert!(result.contains("\x1b[39m")); // FG reset
        assert!(result.contains("\x1b[49m")); // BG reset
    }

    #[test]
    fn test_with_existing_ansi() {
        // Test with existing bold text
        let bold_text = "test".bold();
        let result = bold_text.gradient(Color::RED, Color::BLUE, None);
        assert!(result.contains("\x1b[1m")); // Bold sequence preserved
        assert!(result.contains("\x1b[38;2;")); // FG color added
        
        // Test with existing color
        let colored_text = "test".fg(Color::RED);
        let result = colored_text.gradient_bg(Color::GREEN, Color::BLUE, None);
        assert!(result.contains("\x1b[38;2;")); // Original FG color preserved
        assert!(result.contains("\x1b[48;2;")); // BG gradient added
    }

    #[test]
    fn test_special_characters() {
        let text = "test 👍 emoji";
        let result = text.gradient(Color::RED, Color::BLUE, None);
        assert!(!result.is_empty());
        assert!(result.contains("👍"));
        
        let result_bg = text.gradient_bg(Color::RED, Color::BLUE, None);
        assert!(!result_bg.is_empty());
        assert!(result_bg.contains("👍"));
    }

    #[test]
    fn test_color_interpolation() {
        let color1 = Color::new(255, 0, 0);
        let color2 = Color::new(0, 0, 255);
        let mid = color1.interpolate(&color2, 0.5);
        assert_eq!(mid.r, 127);
        assert_eq!(mid.b, 127);
    }

    #[test]
    fn test_mixed_content() {
        let text = "line1\nline2";
        let result = text.gradient_bg(Color::RED, Color::BLUE, Some(false));
        assert!(result.contains("\x1b[49m\n")); // Check background reset at newlines
        assert!(result.contains("\n")); // Empty line preserved
        
        let lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("\x1b[48;2;")); // First line has background
    }
}