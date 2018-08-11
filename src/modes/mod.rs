crate mod prelude {
    crate use std::io::{self, Write};
    crate use super::{super::Cursor, Mode};
    crate use termion::style::{Underline, NoUnderline};
}

crate mod binary;
crate mod number;
crate mod utf8;
crate mod utf32;
crate use self::binary::{Binary, Octal, Hex};
crate use self::number::Number;
crate use self::utf8::Utf8;
crate use self::utf32::Utf32;

use self::prelude::*;

crate trait Mode: Copy {
    fn fmt<W: Write>(self, w: &mut W, cur: &Cursor, selected: bool) -> io::Result<()>;
    fn len(self, cur: &Cursor) -> usize;
    fn add(self, cur: &mut Cursor, c: char) -> bool;
    fn pop(self, cur: &mut Cursor);
}

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
crate enum DynMode {
    Number,
    Binary,
    Octal,
    Hex,
    Utf8,
    Utf32
}

impl Mode for DynMode {
    fn fmt<W: Write>(self, _w: &mut W, _cur: &Cursor, _selected: bool) -> io::Result<()> {
        unimplemented!("this should never be called");
    }
    fn len(self, _cur: &Cursor) -> usize {
        unimplemented!("this should never be called");
    }

    fn add(self, cur: &mut Cursor, c: char) -> bool {
        match self {
            DynMode::Number => Number.add(cur, c),
            DynMode::Binary => Binary.add(cur, c),
            DynMode::Octal => Octal.add(cur, c),
            DynMode::Hex => Hex.add(cur, c),
            DynMode::Utf8 => Utf8.add(cur, c),
            DynMode::Utf32 => Utf32.add(cur, c),
        }
    }

    fn pop(self, cur: &mut Cursor) {
        match self {
            DynMode::Number => Number.pop(cur),
            DynMode::Binary => Binary.pop(cur),
            DynMode::Octal => Octal.pop(cur),
            DynMode::Hex => Hex.pop(cur),
            DynMode::Utf8 => Utf8.pop(cur),
            DynMode::Utf32 => Utf32.pop(cur)
        }
    }
}
