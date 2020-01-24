/// Represents colors within the WASM SVG GRAPHICS lib
#[derive(Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    /// Constructor for color
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    /// Constructor for color from hex
    ///
    /// # Example
    ///
    /// ```
    /// // 6 character definition
    /// let black_coral = Color::from_hex("565676");
    ///
    /// // 7 character definition
    /// let pale_lavender = Color::from_hex("#d8dcff");
    ///
    /// // Generated with https://coolors.co/
    /// ```
    pub fn from_hex(input_string: &str) -> Option<Color> {
        // Invalid hex length
        if input_string.len() < 6 || input_string.len() > 7 {
            return None
        }

        // Invalid 7 character hex string
        if input_string.len() == 7 && input_string.as_bytes()[0] != b'#' {
            return None
        }

        // Get purely the numbers
        let hex_string: &str = match input_string.len() {
                7 => &input_string[1..],
                _ => &input_string[..],
        };



        let red_result = u8::from_str_radix(&hex_string[..2], 16);
        let green_result = u8::from_str_radix(&hex_string[2..4], 16);
        let blue_result = u8::from_str_radix(&hex_string[4..], 16);

        match (red_result, green_result, blue_result) {
            (Ok(red), Ok(green), Ok(blue)) => Some(Color::new(red, green, blue)),
            _ => None
        }
    }

    /// Returns the red component of the color
    ///
    /// # Example
    ///
    /// ```
    /// let pastel_purple = Color::new(172, 159, 187);
    ///
    /// println!("{}", pastel_purple.r()); // 172
    /// ```
    pub fn r(&self) -> u8 {
        self.red
    }

    /// Returns the red component of the color
    ///
    /// # Example
    ///
    /// ```
    /// let pastel_purple = Color::new(172, 159, 187);
    ///
    /// println!("{}", pastel_purple.red()); // 172
    /// ```
    pub fn red(&self) -> u8 {
        self.red
    }

    /// Returns the green component of the color
    ///
    /// # Example
    ///
    /// ```
    /// let pastel_purple = Color::new(172, 159, 187);
    ///
    /// println!("{}", pastel_purple.g()); // 159
    /// ```
    pub fn g(&self) -> u8 {
        self.green
    }

    /// Returns the green component of the color
    ///
    /// # Example
    ///
    /// ```
    /// let pastel_purple = Color::new(172, 159, 187);
    ///
    /// println!("{}", pastel_purple.green()); // 159
    /// ```
    pub fn green(&self) -> u8 {
        self.green
    }

    /// Returns the blue component of the color
    ///
    /// # Example
    ///
    /// ```
    /// let pastel_purple = Color::new(172, 159, 187);
    ///
    /// println!("{}", pastel_purple.b()); // 187
    /// ```
    pub fn b(&self) -> u8 {
        self.blue
    }

    /// Returns the blue component of the color
    ///
    /// # Example
    ///
    /// ```
    /// let pastel_purple = Color::new(172, 159, 187);
    ///
    /// println!("{}", pastel_purple.blue()); // 187
    /// ```
    pub fn blue(&self) -> u8 {
        self.blue
    }

    /// Returns a tuple with red, green and blue, respectively
    ///
    /// # Example
    ///
    /// ```
    /// let pastel_purple = Color::new(172, 159, 187);
    ///
    /// println!("{}", pastel_purple.rgb()); // (172, 159, 187)
    /// ```
    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.r(), self.g(), self.b())
    }

    /// Returns a string a string containing the u8 variant of the colors
    ///
    /// # Example
    ///
    /// ```
    /// let pastel_purple = Color::new(172, 159, 187);
    ///
    /// println!("{}", pastel_purple.to_rgb_string()); // rgb(172, 159, 187)
    /// ```
    pub fn to_rgb_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r(), self.g(), self.b())
    }

    /// Returns a string a string containing the hex variant of the colors
    ///
    /// # Example
    ///
    /// ```
    /// let pastel_purple = Color::new(172, 159, 187);
    ///
    /// println!("{}", pastel_purple.to_hex_string()); // #ac9fbb
    /// ```
    pub fn to_hex_string(&self) -> String {
        let red_hex = format!("{:02x}", self.r());
        let green_hex = format!("{:02x}", self.g());
        let blue_hex = format!("{:02x}", self.b());

        format!("#{}{}{}", red_hex, green_hex, blue_hex)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r() == other.r() &&
        self.g() == other.g() &&
        self.b() == other.b()
    }
}

impl Eq for Color {}

impl Copy for Color { }

impl Clone for Color {
    fn clone(&self) -> Self {
        Self::new(self.r(), self.g(), self.b())
    }
}

pub mod default {
    use super::Color;

    pub const BLACK: Color = Color { red:0, green:0, blue:0 };
    pub const WHITE: Color = Color { red:255, green:255, blue:255 };
}

pub enum TransparentableColor {
    Color(Color),
    Transparent,
}

impl TransparentableColor {
    /// Returns either the hex of 'color' or "transparent" for Color(color) and Transparent, respectively
    pub fn to_string(&self) -> String {
        match self {
            TransparentableColor::Color(color) => color.to_hex_string(),
            TransparentableColor::Transparent => String::from("transparent")
        }
    }
}

impl PartialEq for TransparentableColor {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TransparentableColor::Transparent, TransparentableColor::Transparent) => true,
            (TransparentableColor::Color(color1), TransparentableColor::Color(color2)) => color1 == color2,
            _ => false
        }
    }
}

impl Eq for TransparentableColor {}

impl Copy for TransparentableColor { }

impl Clone for TransparentableColor {
    fn clone(&self) -> Self {
        match self {
            TransparentableColor::Transparent => TransparentableColor::Transparent,
            TransparentableColor::Color(color) => TransparentableColor::Color(color.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    //use super::default;

    #[test]
    fn test_from_hex() {
        let color_option = Color::from_hex("f7ebec");

        match color_option {
            Some(color) => assert_eq!(color.rgb(), (247, 235, 236)),
            None => assert!(false)
        }

        let color_option = Color::from_hex("#f7ebec");

        match color_option {
            Some(color) => assert_eq!(color.rgb(), (247, 235, 236)),
            None => assert!(false)
        }

        let color_option = Color::from_hex("000000");

        match color_option {
            Some(color) => assert_eq!(color.rgb(), (0, 0, 0)),
            None => assert!(false)
        }

        let color_option = Color::from_hex("ffffff");

        match color_option {
            Some(color) => assert_eq!(color.rgb(), (255, 255, 255)),
            None => assert!(false)
        }

        let color_option = Color::from_hex("a");
        assert_eq!(color_option, None);

        let color_option = Color::from_hex("abcdefg");
        assert_eq!(color_option, None);

        let color_option = Color::from_hex("abcdefgh");
        assert_eq!(color_option, None);

        let color_option = Color::from_hex("#bcdefgh");
        assert_eq!(color_option, None);

        let color_option = Color::from_hex("#12345z");
        assert_eq!(color_option, None);

        let color_option = Color::from_hex("#123z56");
        assert_eq!(color_option, None);

        let color_option = Color::from_hex("#1z3456");
        assert_eq!(color_option, None);
    }

    #[test]
    fn test_to_rgb_string() {
        let color = Color::new(247, 235, 236);
        assert_eq!(color.to_rgb_string(), "rgb(247, 235, 236)");

        let color = Color::new(0, 0, 0);
        assert_eq!(color.to_rgb_string(), "rgb(0, 0, 0)");

        let color = Color::new(255, 255, 255);
        assert_eq!(color.to_rgb_string(), "rgb(255, 255, 255)");
    }

    #[test]
    fn test_to_hex_string() {
        let color = Color::new(247, 235, 236);
        assert_eq!(color.to_hex_string(), "#f7ebec");

        let color = Color::new(0, 0, 0);
        assert_eq!(color.to_hex_string(), "#000000");

        let color = Color::new(255, 255, 255);
        assert_eq!(color.to_hex_string(), "#ffffff");
    }

    #[test]
    fn test_eq() {
        let color1 = Color::new(247, 235, 236);
        assert_eq!(color1, color1);
        let color2 = Color::new(247, 235, 236);
        assert_eq!(color1, color2);

        let color1 = Color::new(0,0,0);
        assert_ne!(color1, color2);
    }
}