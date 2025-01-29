#[derive(PartialEq)]
pub enum Glyphs {
    None,
    Space,
    CurlyLeft,
    CurlyRight,
    Backslash,
    Hash,
    Bang,
    Equal,
    Colon,
}

pub static DOUBLE_BACKSLASH: &str = "\\\\";

impl Glyphs {
    pub fn value(&self) -> u8 {
        self.as_char() as u8
    }

    pub fn as_char(&self) -> char {
        match *self {
            Glyphs::None => '\0',
            Glyphs::Space => ' ',
            Glyphs::Backslash => '\\',
            Glyphs::CurlyLeft => '{',
            Glyphs::CurlyRight => '}',
            Glyphs::Bang => '!',
            Glyphs::Hash => '#',
            Glyphs::Equal => '=',
            Glyphs::Colon => ':',
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            Glyphs::None => "\0",
            Glyphs::Space => " ",
            Glyphs::Backslash => "\\",
            Glyphs::CurlyLeft => "{",
            Glyphs::CurlyRight => "}",
            Glyphs::Bang => "!",
            Glyphs::Hash => "#",
            Glyphs::Equal => "=",
            Glyphs::Colon => ":",
        }
    }

    pub fn from(char: u8) -> Glyphs {
        match char {
            val if val == ' ' as u8 => Glyphs::Space,
            val if val == '\\' as u8 => Glyphs::Backslash,
            val if val == '{' as u8 => Glyphs::CurlyLeft,
            val if val == '}' as u8 => Glyphs::CurlyRight,
            val if val == '!' as u8 => Glyphs::Bang,
            val if val == '#' as u8 => Glyphs::Hash,
            val if val == '=' as u8 => Glyphs::Equal,
            val if val == ':' as u8 => Glyphs::Colon,
            _ => Glyphs::None,
        }
    }
}
