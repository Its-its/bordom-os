
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScanCodeExtension {
    #[default]
    Default,
    Extended,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyCode {
    Escape,

    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,

    Minus,
    Equal,

    Backspace,
    LeftTab,

    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,

    LeftBracket,
    RightBracket,
    Enter,
    LeftControl,

    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,

    SemiColon,
    Apostrophe,
    Tilde,

    LeftShift,
    BackSlash,

    Z,
    X,
    C,
    V,
    B,
    N,
    M,

    Comma,
    Period,
    ForwardSlash,
    RightShift,
    UnknownUnknown,
    LeftCommand,
    Space,
    CapsLock,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // NumPadAsterisk = 55,
    // NumPadHome = 71,
    // NumPadArrowUp,
    // NumPadPgUp,
    // NumPadMinus,
    // NumPadArrowLeft,
    // NumPadNumPad5,
    // NumPadArrowRight,
    // NumPadPlus,
    // NumPadEnd,
    // NumPadArrowDown,
    // NumPadPgDn,
    // Insert,
    // Delete,
    // NumPadSlash = 224,
    // NumPadEnter = 224,

    // Note that scancodes with extended byte (E0) generates two different interrupts: the first containing the E0 byte, the second containing the scancode
    Extended(ExtendedKeyCode),
    Unknown(u8),
}

impl KeyCode {
    pub fn from_scan_code(ext: ScanCodeExtension, value: u8) -> Self {
        use KeyCode::*;

        match ext {
            ScanCodeExtension::Default => (),
            ScanCodeExtension::Extended => return Self::Extended(ExtendedKeyCode::from_scan_code(value))
        }

        match value {
            1 => Escape,

            2 => One,
            3 => Two,
            4 => Three,
            5 => Four,
            6 => Five,
            7 => Six,
            8 => Seven,
            9 => Eight,
            10 => Nine,
            11 => Zero,

            12 => Minus,
            13 => Equal,

            14 => Backspace,
            15 => LeftTab,

            16 => Q,
            17 => W,
            18 => E,
            19 => R,
            20 => T,
            21 => Y,
            22 => U,
            23 => I,
            24 => O,
            25 => P,

            26 => LeftBracket,
            27 => RightBracket,
            28 => Enter,
            29 => LeftControl,

            30 => A,
            31 => S,
            32 => D,
            33 => F,
            34 => G,
            35 => H,
            36 => J,
            37 => K,
            38 => L,

            39 => SemiColon,
            40 => Apostrophe,
            41 => Tilde,

            42 => LeftShift,
            43 => BackSlash,

            44 => Z,
            45 => X,
            46 => C,
            47 => V,
            48 => B,
            49 => N,
            50 => M,

            51 => Comma,
            52 => Period,
            53 => ForwardSlash,
            54 => RightShift,
            55 => UnknownUnknown,
            56 => LeftCommand,
            57 => Space,
            58 => CapsLock,
            59 => F1,
            60 => F2,
            61 => F3,
            62 => F4,
            63 => F5,
            64 => F6,
            65 => F7,
            66 => F8,
            67 => F9,
            68 => F10,
            // 69...132
            133 => F11,
            134 => F12,

            _ => Unknown(value),
        }
    }

    pub fn to_byte_unicode(self, use_alternative: bool) -> Option<u8> {
        Some(match self {
            KeyCode::Unknown(_) => 0,
            KeyCode::Extended(ExtendedKeyCode::Unknown(_)) => 0,

            KeyCode::Extended(_) => return None,
            KeyCode::Escape => return None,
            KeyCode::LeftControl => return None,
            KeyCode::RightShift => return None,
            KeyCode::UnknownUnknown => return None,
            KeyCode::LeftCommand => return None,
            KeyCode::LeftTab => return None,
            KeyCode::LeftShift => return None,
            KeyCode::CapsLock => return None,

            KeyCode::F1 => return None,
            KeyCode::F2 => return None,
            KeyCode::F3 => return None,
            KeyCode::F4 => return None,
            KeyCode::F5 => return None,
            KeyCode::F6 => return None,
            KeyCode::F7 => return None,
            KeyCode::F8 => return None,
            KeyCode::F9 => return None,
            KeyCode::F10 => return None,
            KeyCode::F11 => return None,
            KeyCode::F12 => return None,

            KeyCode::Backspace => 8,
            KeyCode::Enter => 10,
            KeyCode::Space => 32,
            KeyCode::Apostrophe => if use_alternative { 34 } else { 39 },
            KeyCode::Comma => if use_alternative { 60 } else { 44 },
            KeyCode::Minus => if use_alternative { 95 } else { 45 },
            KeyCode::Period => if use_alternative { 62 } else { 46 },
            KeyCode::ForwardSlash => if use_alternative { 63 } else { 47 },
            KeyCode::Zero => 48,
            KeyCode::One => 49,
            KeyCode::Two => 50,
            KeyCode::Three => 51,
            KeyCode::Four => 52,
            KeyCode::Five => 53,
            KeyCode::Six => 54,
            KeyCode::Seven => 55,
            KeyCode::Eight => 56,
            KeyCode::Nine => 57,
            KeyCode::SemiColon => if use_alternative { 58 } else { 59 },
            KeyCode::Equal => if use_alternative { 43 } else { 61 },
            KeyCode::LeftBracket => if use_alternative { 123 } else { 91 },
            KeyCode::BackSlash => if use_alternative { 124 } else { 92 },
            KeyCode::RightBracket => if use_alternative { 125 } else { 93 },
            KeyCode::Tilde => if use_alternative { 126 } else { 96 },
            KeyCode::A => if use_alternative { 65 } else { 97 },
            KeyCode::B => if use_alternative { 66 } else { 98 },
            KeyCode::C => if use_alternative { 67 } else { 99 },
            KeyCode::D => if use_alternative { 68 } else { 100 },
            KeyCode::E => if use_alternative { 69 } else { 101 },
            KeyCode::F => if use_alternative { 70 } else { 102 },
            KeyCode::G => if use_alternative { 71 } else { 103 },
            KeyCode::H => if use_alternative { 72 } else { 104 },
            KeyCode::I => if use_alternative { 73 } else { 105 },
            KeyCode::J => if use_alternative { 74 } else { 106 },
            KeyCode::K => if use_alternative { 75 } else { 107 },
            KeyCode::L => if use_alternative { 76 } else { 108 },
            KeyCode::M => if use_alternative { 77 } else { 109 },
            KeyCode::N => if use_alternative { 78 } else { 110 },
            KeyCode::O => if use_alternative { 79 } else { 111 },
            KeyCode::P => if use_alternative { 80 } else { 112 },
            KeyCode::Q => if use_alternative { 81 } else { 113 },
            KeyCode::R => if use_alternative { 82 } else { 114 },
            KeyCode::S => if use_alternative { 83 } else { 115 },
            KeyCode::T => if use_alternative { 84 } else { 116 },
            KeyCode::U => if use_alternative { 85 } else { 117 },
            KeyCode::V => if use_alternative { 86 } else { 118 },
            KeyCode::W => if use_alternative { 87 } else { 119 },
            KeyCode::X => if use_alternative { 88 } else { 120 },
            KeyCode::Y => if use_alternative { 89 } else { 121 },
            KeyCode::Z => if use_alternative { 90 } else { 122 },

        })
    }
}




#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExtendedKeyCode {
    MultiMediaPreviousTrack = 0x10,
    MultiMediaNextTrack = 0x19,

    KeypadEnter = 0x1C,
    RightControl = 0x1D,
    Mute = 0x20,

    MultiMediaCalculator = 0x21,
    MultiMediaPlay = 0x22,
    MultiMediaStop = 0x24,
    MultiMediaVolumeDown = 0x2E,
    MultiMediaVolumeUp = 0x30,
    MultiMediaWWWHome = 0x32,

    KeypadForwardSlash = 0x35,
    RightAlt = 0x38,
    Home = 0x47,
    CursorUp = 0x48,
    PageUp = 0x49,
    CursorLeft = 0x4B,
    CursorRight = 0x4D,
    End = 0x4F,
    CursorDown = 0x50,
    PageDown = 0x51,
    Insert = 0x52,
    Delete = 0x53,
    LeftGUI = 0x5B,
    RightGUI = 0x5C,
    Apps = 0x5D,
    Power = 0x5E,
    Sleep = 0x5F,
    Wake = 0x63,

    MultiMediaWWWSearch = 0x65,
    MultiMediaWWWFavorites = 0x66,
    MultiMediaWWWRefresh = 0x67,
    MultiMediaWWWStop = 0x68,
    MultiMediaWWWForward = 0x69,
    MultiMediaWWWBack = 0x6A,
    MultiMediaMyComputer = 0x6B,
    MultiMediaEmail = 0x6C,
    MultiMediaMediaSelect = 0x6D,

    // 0x90..0xED

    Unknown(u8),
}

impl ExtendedKeyCode {
    pub fn from_scan_code(value: u8) -> Self {
        use ExtendedKeyCode::*;

        match value {
            0x10 => MultiMediaPreviousTrack,
            0x19 => MultiMediaNextTrack,

            0x1C => KeypadEnter,
            0x1D => RightControl,
            0x20 => Mute,

            0x21 => MultiMediaCalculator,
            0x22 => MultiMediaPlay,
            0x24 => MultiMediaStop,
            0x2E => MultiMediaVolumeDown,
            0x30 => MultiMediaVolumeUp,
            0x32 => MultiMediaWWWHome,

            0x35 => KeypadForwardSlash,
            0x38 => RightAlt,
            0x47 => Home,
            0x48 => CursorUp,
            0x49 => PageUp,
            0x4B => CursorLeft,
            0x4D => CursorRight,
            0x4F => End,
            0x50 => CursorDown,
            0x51 => PageDown,
            0x52 => Insert,
            0x53 => Delete,
            0x5B => LeftGUI,
            0x5C => RightGUI,
            0x5D => Apps,
            0x5E => Power,
            0x5F => Sleep,
            0x63 => Wake,

            0x65 => MultiMediaWWWSearch,
            0x66 => MultiMediaWWWFavorites,
            0x67 => MultiMediaWWWRefresh,
            0x68 => MultiMediaWWWStop,
            0x69 => MultiMediaWWWForward,
            0x6A => MultiMediaWWWBack,
            0x6B => MultiMediaMyComputer,
            0x6C => MultiMediaEmail,
            0x6D => MultiMediaMediaSelect,

            _ => Unknown(value)
        }
    }
}