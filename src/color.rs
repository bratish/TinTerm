#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub(crate) fn interpolate(&self, other: &Color, t: f32) -> Color {
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
        }
    }

    /// Create a new Color from a hex string
    /// Supports both 6-digit (e.g., "#FF0000" or "FF0000") and 3-digit (e.g., "#F00" or "F00") formats
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        // Remove '#' if present
        let hex = hex.trim_start_matches('#');
        
        // Validate hex length
        if hex.len() != 6 && hex.len() != 3 {
            return Err("Invalid hex color length. Expected 6 or 3 characters (e.g., FF0000 or F00)");
        }

        // Validate hex characters
        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Invalid hex color format. Expected hexadecimal characters (0-9, A-F)");
        }

        let (r, g, b) = if hex.len() == 3 {
            // Convert 3-digit format to 6-digit by duplicating each digit
            let r = u8::from_str_radix(&format!("{}{}", hex.chars().nth(0).unwrap(), hex.chars().nth(0).unwrap()), 16)
                .map_err(|_| "Failed to parse red component")?;
            let g = u8::from_str_radix(&format!("{}{}", hex.chars().nth(1).unwrap(), hex.chars().nth(1).unwrap()), 16)
                .map_err(|_| "Failed to parse green component")?;
            let b = u8::from_str_radix(&format!("{}{}", hex.chars().nth(2).unwrap(), hex.chars().nth(2).unwrap()), 16)
                .map_err(|_| "Failed to parse blue component")?;
            (r, g, b)
        } else {
            // Parse 6-digit format
            let r = u8::from_str_radix(&hex[0..2], 16)
                .map_err(|_| "Failed to parse red component")?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .map_err(|_| "Failed to parse green component")?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .map_err(|_| "Failed to parse blue component")?;
            (r, g, b)
        };

        Ok(Color { r, g, b })
    }

    /// Convert the color to a hex string
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
