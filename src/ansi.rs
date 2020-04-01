use style::{Colour, Style};

use std::fmt::{self, Display, Write};

impl Style {
    /// Write any bytes that go *before* a piece of text to the given writer.
    #[inline]
    pub(crate) fn write_prefix(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // If there are actually no styles here, then don’t write *any* codes
        // as the prefix. An empty ANSI code may not affect the terminal
        // output at all, but a user may just want a code-free string.
        if self.is_plain() {
            return Ok(());
        }

        // Write the codes’ prefix, then write numbers, separated by
        // semicolons, for each text style we want to apply.
        f.write_str("\x1B[")?;

        let mut written_anything = false;
        macro_rules! write_char {
            ($cond:ident, $c:expr) => {
                if self.$cond {
                    if written_anything {
                        f.write_char(';')?;
                    } else {
                        written_anything = true;
                    }
                    f.write_char($c)?;
                }
            };
        }
        macro_rules! write_chars {
            ($cond:ident => $c:expr) => { write_char!($cond, $c); };
            ($cond:ident => $c:expr, $($t:tt)+) => {
                write_char!($cond, $c);
                write_chars!($($t)+);
            };
        }

        write_chars!(
            is_bold => '1',
            is_dimmed => '2',
            is_italic => '3',
            is_underline => '4',
            is_blink => '5',
            is_reverse => '7',
            is_hidden => '8',
            is_strikethrough => '9'
        );

        // The foreground and background colours, if specified, need to be
        // handled specially because the number codes are more complicated.
        // (see `write_background_code` and `write_foreground_code`)
        if let Some(bg) = self.background {
            if written_anything {
                f.write_char(';')?;
            }
            written_anything = true;
            bg.write_background_code(f)?;
        }

        if let Some(fg) = self.foreground {
            if written_anything {
                f.write_char(';')?;
            }
            fg.write_foreground_code(f)?;
        }

        // All the codes end with an `m`, because reasons.
        f.write_char('m')
    }

    /// Write any bytes that go *after* a piece of text to the given writer.
    #[inline]
    pub(crate) fn write_suffix(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_plain() {
            Ok(())
        } else {
            f.write_str(RESET)
        }
    }
}

/// The code to send to reset all styles and return to `Style::default()`.
pub static RESET: &str = "\x1B[0m";

impl Colour {
    #[inline]
    fn write_foreground_code(self, f: &mut fmt::Formatter) -> fmt::Result {
        use Colour::*;
        match self {
            Black => f.write_str("30"),
            Red => f.write_str("31"),
            Green => f.write_str("32"),
            Yellow => f.write_str("33"),
            Blue => f.write_str("34"),
            Purple => f.write_str("35"),
            Cyan => f.write_str("36"),
            White => f.write_str("37"),
            Fixed(num) => {
                f.write_str("38;5;")?;
                num.fmt(f)
            }
            RGB(r, g, b) => {
                f.write_str("38;2;")?;
                r.fmt(f)?;
                f.write_char(';')?;
                g.fmt(f)?;
                f.write_char(';')?;
                b.fmt(f)
            }
        }
    }

    #[inline]
    fn write_background_code(self, f: &mut fmt::Formatter) -> fmt::Result {
        use Colour::*;
        match self {
            Black => f.write_str("40"),
            Red => f.write_str("41"),
            Green => f.write_str("42"),
            Yellow => f.write_str("43"),
            Blue => f.write_str("44"),
            Purple => f.write_str("45"),
            Cyan => f.write_str("46"),
            White => f.write_str("47"),
            Fixed(num) => {
                f.write_str("48;5;")?;
                num.fmt(f)
            }
            RGB(r, g, b) => {
                f.write_str("48;2;")?;
                r.fmt(f)?;
                f.write_char(';')?;
                g.fmt(f)?;
                f.write_char(';')?;
                b.fmt(f)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use style::{Colour::*, Style};

    macro_rules! test {
        ($name: ident: $style: expr; $input: expr => $result: expr) => {
            #[test]
            fn $name() {
                assert_eq!($style.paint($input).to_string(), $result.to_string());
            }
        };
    }

    test!(plain:                 Style::default();                  "text/plain" => "text/plain");
    test!(red:                   Red;                               "hi" => "\x1B[31mhi\x1B[0m");
    test!(black:                 Black.normal();                    "hi" => "\x1B[30mhi\x1B[0m");
    test!(yellow_bold:           Yellow.bold();                     "hi" => "\x1B[1;33mhi\x1B[0m");
    test!(yellow_bold_2:         Yellow.normal().bold();            "hi" => "\x1B[1;33mhi\x1B[0m");
    test!(blue_underline:        Blue.underline();                  "hi" => "\x1B[4;34mhi\x1B[0m");
    test!(green_bold_ul:         Green.bold().underline();          "hi" => "\x1B[1;4;32mhi\x1B[0m");
    test!(green_bold_ul_2:       Green.underline().bold();          "hi" => "\x1B[1;4;32mhi\x1B[0m");
    test!(purple_on_white:       Purple.on(White);                  "hi" => "\x1B[47;35mhi\x1B[0m");
    test!(purple_on_white_2:     Purple.normal().on(White);         "hi" => "\x1B[47;35mhi\x1B[0m");
    test!(yellow_on_blue:        Style::new().on(Blue).fg(Yellow);  "hi" => "\x1B[44;33mhi\x1B[0m");
    test!(yellow_on_blue_2:      Cyan.on(Blue).fg(Yellow);          "hi" => "\x1B[44;33mhi\x1B[0m");
    test!(cyan_bold_on_white:    Cyan.bold().on(White);             "hi" => "\x1B[1;47;36mhi\x1B[0m");
    test!(cyan_ul_on_white:      Cyan.underline().on(White);        "hi" => "\x1B[4;47;36mhi\x1B[0m");
    test!(cyan_bold_ul_on_white: Cyan.bold().underline().on(White); "hi" => "\x1B[1;4;47;36mhi\x1B[0m");
    test!(cyan_ul_bold_on_white: Cyan.underline().bold().on(White); "hi" => "\x1B[1;4;47;36mhi\x1B[0m");
    test!(fixed:                 Fixed(100);                        "hi" => "\x1B[38;5;100mhi\x1B[0m");
    test!(fixed_on_purple:       Fixed(100).on(Purple);             "hi" => "\x1B[45;38;5;100mhi\x1B[0m");
    test!(fixed_on_fixed:        Fixed(100).on(Fixed(200));         "hi" => "\x1B[48;5;200;38;5;100mhi\x1B[0m");
    test!(rgb:                   RGB(70,130,180);                   "hi" => "\x1B[38;2;70;130;180mhi\x1B[0m");
    test!(rgb_on_blue:           RGB(70,130,180).on(Blue);          "hi" => "\x1B[44;38;2;70;130;180mhi\x1B[0m");
    test!(blue_on_rgb:           Blue.on(RGB(70,130,180));          "hi" => "\x1B[48;2;70;130;180;34mhi\x1B[0m");
    test!(rgb_on_rgb:            RGB(70,130,180).on(RGB(5,10,15));  "hi" => "\x1B[48;2;5;10;15;38;2;70;130;180mhi\x1B[0m");
    test!(bold:                  Style::new().bold();               "hi" => "\x1B[1mhi\x1B[0m");
    test!(underline:             Style::new().underline();          "hi" => "\x1B[4mhi\x1B[0m");
    test!(bunderline:            Style::new().bold().underline();   "hi" => "\x1B[1;4mhi\x1B[0m");
    test!(dimmed:                Style::new().dimmed();             "hi" => "\x1B[2mhi\x1B[0m");
    test!(italic:                Style::new().italic();             "hi" => "\x1B[3mhi\x1B[0m");
    test!(blink:                 Style::new().blink();              "hi" => "\x1B[5mhi\x1B[0m");
    test!(reverse:               Style::new().reverse();            "hi" => "\x1B[7mhi\x1B[0m");
    test!(hidden:                Style::new().hidden();             "hi" => "\x1B[8mhi\x1B[0m");
    test!(stricken:              Style::new().strikethrough();      "hi" => "\x1B[9mhi\x1B[0m");
}
