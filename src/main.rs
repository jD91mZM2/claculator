#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use failure::Error;
use std::io::{self, prelude::*};
use termion::{
    clear,
    cursor,
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    style
};

crate struct Cursor {
    /// The inner numbers that will be formatted in different ways
    crate buf: Vec<u64>,
    /// The cursor position in the buffer
    crate index: usize
}

mod modes;
use self::modes::*;

fn main() -> Result<(), Error> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode()?;

    let stdin = io::stdin();
    let mut keys = stdin.lock().keys();

    macro_rules! print {
        ($fmt:expr) => {
            write!(stdout, $fmt)?;
        };
        ($fmt:expr, $($args:expr),+) => {
            write!(stdout, $fmt, $($args),+)?;
        };
    }
    macro_rules! println {
        () => { println!(""); };
        ($fmt:expr) => { write!(stdout, concat!($fmt, "\r\n"))?; };
        ($fmt:expr, $($args:expr),+) => { write!(stdout, concat!($fmt, "\r\n"), $($args),+)?; };
    }

    let mut mode = DynMode::Number;
    let mut cur = Cursor {
        buf: vec![0],
        index: 0
    };

    loop {
        let (width, _height) = termion::terminal_size()?;

        let mut lines = 0;

        macro_rules! entry {
            ($mode:ident, $prefix:expr) => {
                if mode == DynMode::$mode {
                    print!("{}", style::Bold);
                }
                print!(concat!($prefix, ": "));
                $mode.fmt(&mut stdout, &cur, mode == DynMode::$mode)?;
                println!();
                if mode == DynMode::$mode {
                    print!("{}", style::Reset);
                }

                let wrappings = ($prefix.len() + 2 + $mode.len(&cur) - 1) as u16 / width;
                lines += 1 + wrappings;
            }
        }

        entry!(Number, "Number");
        entry!(Binary, "Binary");
        entry!(Octal, "Octal");
        entry!(Hex, "Hex");
        entry!(Utf8, "UTF-8");
        entry!(Utf32, "UTF-32");

        match keys.next() {
            None => break,
            Some(result) => match result? {
                Key::Backspace => mode.pop(&mut cur),
                Key::Char(c) => if !mode.add(&mut cur, c) && c == 'q' {
                    break;
                },
                Key::Ctrl('c') => break,
                Key::Down => mode = match mode {
                    DynMode::Number => DynMode::Binary,
                    DynMode::Binary => DynMode::Octal,
                    DynMode::Octal => DynMode::Hex,
                    DynMode::Hex => DynMode::Utf8,
                    DynMode::Utf8 => DynMode::Utf32,
                    DynMode::Utf32 => DynMode::Number
                },
                Key::Up => mode = match mode {
                    DynMode::Number => DynMode::Utf8,
                    DynMode::Binary => DynMode::Number,
                    DynMode::Octal => DynMode::Binary,
                    DynMode::Hex => DynMode::Octal,
                    DynMode::Utf8 => DynMode::Hex,
                    DynMode::Utf32 => DynMode::Utf8
                },
                Key::Left => cur.index = cur.index.saturating_sub(1),
                Key::Right => cur.index = (cur.index + 1).min(cur.buf.len() - 1),
                _ => ()
            }
        }

        print!("{}{}", cursor::Up(lines), clear::AfterCursor);
    }
    Ok(())
}
