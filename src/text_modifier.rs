use crate::color::Color;

pub trait TextModifier {
    fn color(&self, color: Color) -> String;
    fn background_color(&self, color: Color) -> String;
    fn fg(&self, color: Color) -> String;
    fn bg(&self, color: Color) -> String;
    fn bold(&self) -> String;
    fn dim(&self) -> String;
    fn italic(&self) -> String;
    fn underline(&self) -> String;
    fn blink(&self) -> String;
    fn reverse(&self) -> String;
    fn hidden(&self) -> String;
    fn strikethrough(&self) -> String;
    fn bright(&self) -> String;
}

impl TextModifier for str {
    fn color(&self, color: Color) -> String {
        // self.fg(color)
        format!("\x1b[38;2;{};{};{}m{}\x1b[39m", color.r, color.g, color.b, self)
    }

    fn background_color(&self, color: Color) -> String {
        self.bg(color)
    }

    fn fg(&self, color: Color) -> String {
        format!("\x1b[38;2;{};{};{}m{}\x1b[39m", color.r, color.g, color.b, self)
    }

    fn bg(&self, color: Color) -> String {
        format!("\x1b[48;2;{};{};{}m{}\x1b[49m", color.r, color.g, color.b, self)
    }

    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[22m", self)
    }

    fn dim(&self) -> String {
        format!("\x1b[2m{}\x1b[22m", self)
    }

    fn italic(&self) -> String {
        format!("\x1b[3m{}\x1b[23m", self)
    }

    fn underline(&self) -> String {
        format!("\x1b[4m{}\x1b[24m", self)
    }

    fn blink(&self) -> String {
        format!("\x1b[5m{}\x1b[25m", self)
    }

    fn reverse(&self) -> String {
        format!("\x1b[7m{}\x1b[27m", self)
    }

    fn hidden(&self) -> String {
        format!("\x1b[8m{}\x1b[28m", self)
    }

    fn strikethrough(&self) -> String {
        format!("\x1b[9m{}\x1b[29m", self)
    }

    fn bright(&self) -> String {
        format!("\x1b[1m{}\x1b[22m", self)
    }
} 