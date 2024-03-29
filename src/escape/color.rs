use super::CSI;

pub const RESET: CSI = CSI("0m");

pub const BLACK_FOREGROUND: CSI = CSI("30m");
pub const BLACK_BACKGROUND: CSI = CSI("40m");
pub const BRIGHT_BLACK_FOREGROUND: CSI = CSI("90m");
pub const BRIGHT_BLACK_BACKGROUND: CSI = CSI("100m");

pub const RED_FOREGROUND: CSI = CSI("31m");
pub const RED_BACKGROUND: CSI = CSI("41m");
pub const BRIGHT_RED_FOREGROUND: CSI = CSI("91m");
pub const BRIGHT_RED_BACKGROUND: CSI = CSI("101m");

pub const GREEN_FOREGROUND: CSI = CSI("32m");
pub const GREEN_BACKGROUND: CSI = CSI("42m");
pub const BRIGHT_GREEN_FOREGROUND: CSI = CSI("92m");
pub const BRIGHT_GREEN_BACKGROUND: CSI = CSI("102m");

pub const YELLOW_FOREGROUND: CSI = CSI("33m");
pub const YELLOW_BACKGROUND: CSI = CSI("43m");
pub const BRIGHT_YELLOW_FOREGROUND: CSI = CSI("93m");
pub const BRIGHT_YELLOW_BACKGROUND: CSI = CSI("103m");

pub const BLUE_FOREGROUND: CSI = CSI("34m");
pub const BLUE_BACKGROUND: CSI = CSI("44m");
pub const BRIGHT_BLUE_FOREGROUND: CSI = CSI("94m");
pub const BRIGHT_BLUE_BACKGROUND: CSI = CSI("104m");

pub const MAGENTA_FOREGROUND: CSI = CSI("35m");
pub const MAGENTA_BACKGROUND: CSI = CSI("45m");
pub const BRIGHT_MAGENTA_FOREGROUND: CSI = CSI("95m");
pub const BRIGHT_MAGENTA_BACKGROUND: CSI = CSI("105m");

pub const CYAN_FOREGROUND: CSI = CSI("36m");
pub const CYAN_BACKGROUND: CSI = CSI("46m");
pub const BRIGHT_CYAN_FOREGROUND: CSI = CSI("96m");
pub const BRIGHT_CYAN_BACKGROUND: CSI = CSI("106m");

pub const WHITE_FOREGROUND: CSI = CSI("37m");
pub const WHITE_BACKGROUND: CSI = CSI("47m");
pub const BRIGHT_WHITE_FOREGROUND: CSI = CSI("97m");
pub const BRIGHT_WHITE_BACKGROUND: CSI = CSI("107m");

pub const DEFAULT_FOREGROUND: CSI = CSI("39m");
pub const DEFAULT_BACKGROUND: CSI = CSI("49m");
