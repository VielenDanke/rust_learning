use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }
}

#[derive(Debug)]
enum ParseColorError {
    RedChannelOutOfBounds,
    GreenChannelOutOfBounds,
    BlueChannelOutOfBounds,
    NoLeadingHash,
}

impl FromStr for Rgb {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.strip_prefix("#") {
            let red = u8::from_str_radix(&hex[0..2], 16)
                .or_else(|_| Err(ParseColorError::RedChannelOutOfBounds))?;
            let green = u8::from_str_radix(&hex[2..4], 16)
                .or_else(|_| Err(ParseColorError::GreenChannelOutOfBounds))?;
            let blue = u8::from_str_radix(&hex[4..6], 16)
                .or_else(|_| Err(ParseColorError::BlueChannelOutOfBounds))?;
            Ok(
                Rgb {
                    r: red,
                    g: green,
                    b: blue,
                }
            )
        } else {
            Err(ParseColorError::NoLeadingHash)
        }
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
