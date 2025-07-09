#![cfg_attr(not(test), no_std)]
use core::convert;
use core::fmt;

//RESET
pub const RESET: &str = "\x1b[0m";

// STYLES
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";

pub const UNDERLINE: &str = "\x1b[4m";
pub const UL: &str = "\x1b[4m";

pub const STRIKETHROUGH: &str = "\x1b[9m";
pub const ST: &str = "\x1b[9m";

// STYLE RESETS
pub const NORMAL_INTENSITY: &str = "\x1b[22m";
pub const NOIN: &str = "\x1b[22m";

pub const NO_UNDERLINE: &str = "\x1b[24m";
pub const NOUL: &str = "\x1b[24m";

pub const NO_STRIKETHROUGH: &str = "\x1b[29m";
pub const NOST: &str = "\x1b[29m";

pub const NO_COLOR: &str = "\x1b[39m\x1b[49m"; //DEFAULT + BG_DEFAULT
pub const NC: &str = "\x1b[39m\x1b[49m";

// FOREGROUND COLORS
pub const BLACK: Fg<Basic> = Fg(Basic("\x1b[30m"));
pub const RED: Fg<Basic> = Fg(Basic("\x1b[31m"));
pub const GREEN: Fg<Basic> = Fg(Basic("\x1b[32m"));
pub const YELLOW: Fg<Basic> = Fg(Basic("\x1b[33m"));
pub const BLUE: Fg<Basic> = Fg(Basic("\x1b[34m"));
pub const MAGENTA: Fg<Basic> = Fg(Basic("\x1b[35m"));
pub const CYAN: Fg<Basic> = Fg(Basic("\x1b[36m"));
pub const WHITE: Fg<Basic> = Fg(Basic("\x1b[37m"));
pub const DEFAULT: Fg<Basic> = Fg(Basic("\x1b[39m"));

// BACKGROUND COLORS
pub const BG_BLACK: Bg<Basic> = Bg(Basic("\x1b[40m"));
pub const BG_RED: Bg<Basic> = Bg(Basic("\x1b[41m"));
pub const BG_GREEN: Bg<Basic> = Bg(Basic("\x1b[42m"));
pub const BG_YELLOW: Bg<Basic> = Bg(Basic("\x1b[43m"));
pub const BG_BLUE: Bg<Basic> = Bg(Basic("\x1b[44m"));
pub const BG_MAGENTA: Bg<Basic> = Bg(Basic("\x1b[45m"));
pub const BG_CYAN: Bg<Basic> = Bg(Basic("\x1b[46m"));
pub const BG_WHITE: Bg<Basic> = Bg(Basic("\x1b[47m"));
pub const BG_DEFAULT: Bg<Basic> = Bg(Basic("\x1b[49m"));

// BRIGHT FOREGROUND COLORS
pub const BR_BLACK: Fg<Basic> = Fg(Basic("\x1b[90m"));
pub const BR_RED: Fg<Basic> = Fg(Basic("\x1b[91m"));
pub const BR_GREEN: Fg<Basic> = Fg(Basic("\x1b[92m"));
pub const BR_YELLOW: Fg<Basic> = Fg(Basic("\x1b[93m"));
pub const BR_BLUE: Fg<Basic> = Fg(Basic("\x1b[94m"));
pub const BR_MAGENTA: Fg<Basic> = Fg(Basic("\x1b[95m"));
pub const BR_CYAN: Fg<Basic> = Fg(Basic("\x1b[96m"));
pub const BR_WHITE: Fg<Basic> = Fg(Basic("\x1b[97m"));

// BRIGHT BACKGROUND COLORS
pub const BG_BRBLACK: Bg<Basic> = Bg(Basic("\x1b[100m"));
pub const BG_BRRED: Bg<Basic> = Bg(Basic("\x1b[101m"));
pub const BG_BR_GREEN: Bg<Basic> = Bg(Basic("\x1b[102m"));
pub const BG_BR_YELLOW: Bg<Basic> = Bg(Basic("\x1b[103m"));
pub const BG_BR_BLUE: Bg<Basic> = Bg(Basic("\x1b[104m"));
pub const BG_BR_MAGENTA: Bg<Basic> = Bg(Basic("\x1b[105m"));
pub const BG_BR_CYAN: Bg<Basic> = Bg(Basic("\x1b[106m"));
pub const BG_BR_WHITE: Bg<Basic> = Bg(Basic("\x1b[107m"));

// Error
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColorError {
    OutOfRange,
    InvalidFormat,
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorError::OutOfRange => f.write_str("Value out of expected u24 range."),
            ColorError::InvalidFormat => f.write_str("Invalid color format."),
        }
    }
}
///
///
///
/// STICK WITH IT NOW, OK?!!!
/// JUST IMPLEMENT. THE STRUCTURE IS OK.
///-------------------------------------
/// TO DO:
///-------------------------------------
/// > impl basic Display for each Ansi
/// > impl Convert for each Ansi to handle future degredation
///   > work out quantization? easy lookup? idk.
/// > clean up, test!
/// > commit. commit! immediately!
/// > environmental variable detection and caching with OnceLock?
///
/// this is Claude's example...
/*
"static DETECTED_CAPABILITY: OnceLock<RenderTarget> = OnceLock::new();

fn detect_terminal_capability() -> RenderTarget {

    Check COLORTERM, TERM, NO_COLOR, FORCE_COLOR, etc.
    Check if stdout.is_terminal()
    Return appropriate capability
}
*/
/// > advanced Display impl for each Ansi that matches on cached env capability
///
///
pub trait Ansi {}

// Extended Color Formats
const BG_EXTENDED: &str = "\x1b[48;";
const FG_EXTENDED: &str = "\x1b[38;";
const UL_EXTENDED: &str = "\x1b[58;";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TrueColor(u8, u8, u8);
impl Ansi for TrueColor {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Extended(u8);
impl Ansi for Extended {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Basic(&'static str);
impl Ansi for Basic {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Fg<T: Ansi>(T);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Bg<T: Ansi>(T);

impl convert::From<TrueColor> for Extended {
    fn from(color: TrueColor) -> Self {
        color
    }
}

/*macro_rules! impl_enums {
    ($(($enum_type:ident, $variant:ident)),*) => {
        $(
            impl fmt::Display for $enum_type {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self {
                        Fg::FgTrueColor(r, g, b) => write!(f, "{FG_EXTENDED}2;{r};{g};{b}m"),
                        Fg::FgWebColor(n) => write!(f, "{FG_EXTENDED}5;{n}m"),
                        Fg::FgCodeColor(code) => f.write_str(code),
                    }
                }
            }
            impl convert::From<(u8, u8, u8)> for $enum_type {
                fn from(rgb: (u8, u8, u8)) -> Self {
                    let (r, g, b) = rgb;
                    Self::$variant(r, g, b)
                }
            }
            impl convert::TryFrom<u32> for $enum_type {
                type Error = ColorError;

                fn try_from(value: u32) -> Result<Self, Self::Error> {
                    let 0x0..=0xFFFFFF = value else {
                        return Err(ColorError::OutOfRange);
                    };
                    let r = ((value >> 16) & 0xFF) as u8;
                    let g = ((value >> 8) & 0xFF) as u8;
                    let b = (value & 0xFF) as u8;
                    Ok(Self::$variant(r, g, b))
                }
            }

        )*
    };
}*/

//impl_enums!((Fg, FgTrueColor), (Bg, BgTrueColor), (Ul, UlTrueColor));

#[test]
fn truecolor() {
    let bg_intorange = Bgr::try_from(0xFF4F01).unwrap();
    dbg!(bg_intorange);
    assert_eq!(bg_intorange, Bg::BgTrueColor(0xFF, 0x4F, 0x00))
}

/*macro_rules! truecolor_fg {
( $hex:literal ) => {{
        const _: () =
        const _: () = assert!($hex <= 0xFFFFFF, "Expected value 0x000000 - 0xFFFFFF");



      }};
}*/
/*
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TrueColorBg(pub u8, pub u8, pub u8);

impl fmt::Display for TrueColorBg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrueColorBg(r, g, b) => write!(f, "{BG_EXTENDED}2;{r};{g};{b}m"),
        }
    }
}

impl convert::TryFrom<u32> for TrueColorBg {
    type Error = ColorError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let 0x0..=0xFFFFFF = value else {
            return Err(ColorError::OutOfRange);
        };
        let r = ((value >> 16) & 0xFF) as u8;
        let g = ((value >> 8) & 0xFF) as u8;
        let b = (value & 0xFF) as u8;
        Ok(Self(r, g, b))
    }
}
const ul_white = "\x1b[58;5;255m";*/
