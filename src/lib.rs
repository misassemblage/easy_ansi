use core::fmt;

struct Sgr(&'static str);

impl Sgr {}

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
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const DEFAULT: &str = "\x1b[39m";

// BACKGROUND COLORS
pub const BG_BLACK: &str = "\x1b[40m";
pub const BG_RED: &str = "\x1b[41m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BG_BLUE: &str = "\x1b[44m";
pub const BG_MAGENTA: &str = "\x1b[45m";
pub const BG_CYAN: &str = "\x1b[46m";
pub const BG_WHITE: &str = "\x1b[47m";
pub const BG_DEFAULT: &str = "\x1b[49m";

// BRIGHT FOREGROUND COLORS
pub const BR_BLACK: &str = "\x1b[90m";
pub const BR_RED: &str = "\x1b[91m";
pub const BR_GREEN: &str = "\x1b[92m";
pub const BR_YELLOW: &str = "\x1b[93m";
pub const BR_BLUE: &str = "\x1b[94m";
pub const BR_MAGENTA: &str = "\x1b[95m";
pub const BR_CYAN: &str = "\x1b[96m";
pub const BR_WHITE: &str = "\x1b[97m";

// BRIGHT BACKGROUND COLORS
pub const BG_BRBLACK: &str = "\x1b[100m";
pub const BG_BRRED: &str = "\x1b[101m";
pub const BG_BR_GREEN: &str = "\x1b[102m";
pub const BG_BR_YELLOW: &str = "\x1b[103m";
pub const BG_BR_BLUE: &str = "\x1b[104m";
pub const BG_BR_MAGENTA: &str = "\x1b[105m";
pub const BG_BR_CYAN: &str = "\x1b[106m";
pub const BG_BR_WHITE: &str = "\x1b[107m";

pub mod extended_colors {
    use core::fmt;

    // Prefixes
    const BG_EXTENDED: &str = "\x1b[48;";
    const FG_EXTENDED: &str = "\x1b[38;";

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum Websafe {
        Fg(u8),
        Bg(u8),
    }

    impl fmt::Display for Websafe {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Websafe::Fg(n) => write!(f, "{FG_EXTENDED}5;{n}m"),
                Websafe::Bg(n) => write!(f, "{BG_EXTENDED}5;{n}m"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum TrueColor {
        Fg(u8, u8, u8),
        Bg(u8, u8, u8),
    }

    impl fmt::Display for TrueColor {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TrueColor::Fg(r, g, b) => write!(f, "{FG_EXTENDED}2;{r};{g};{b}m"),
                TrueColor::Bg(r, g, b) => write!(f, "{BG_EXTENDED}2;{r};{g};{b}m"),
            }
        }
    }
}
