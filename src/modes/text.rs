use super::prelude::*;
use std::ptr;
use termion::color;

#[derive(Clone, Copy)]
crate struct Text;

fn to_string(cur: &Cursor) -> Option<String> {
    let mut bytes = Vec::with_capacity(cur.buf.len());

    for &n in &cur.buf {
        if n >= 32 && n <= std::u8::MAX as u64 {
            bytes.push(n as u8);
        }
    }

    // I kinda want to do from_utf8_lossy, but it does a SECOND memory
    // allocation...
    String::from_utf8(bytes).ok()
}

impl Mode for Text {
    fn fmt<W: Write>(self, w: &mut W, cur: &Cursor, selected: bool) -> io::Result<()> {
        let string = match to_string(cur) {
            Some(string) => string,
            None => {
                write!(w, "{}Invalid UTF-8{}", color::Fg(color::Red), color::Fg(color::Reset))?;
                return Ok(());
            }
        };

        let mut chars = string.chars().peekable();
        let mut i = 0;
        while let Some(c) = chars.peek() {
            if selected && i + c.len_utf8() > cur.index {
                break;
            }
            i += c.len_utf8();
            chars.next();
        }
        // i now points to the start of the selected codepoint

        w.write_all(&string.as_bytes()[..i])?;

        if let Some(c) = chars.next() {
            write!(w, "{}", Underline)?;
            w.write_all(&string.as_bytes()[i..][..c.len_utf8()])?;
            i += c.len_utf8();
            write!(w, "{}", NoUnderline)?;

            w.write_all(&string.as_bytes()[i..])?;
        }

        Ok(())
    }

    fn len(self, cur: &Cursor) -> usize {
        to_string(cur)
            .map(|string| string.chars().count())
            .unwrap_or("Invalid UTF-8".len())
    }

    fn add(self, cur: &mut Cursor, c: char) -> bool {
        if c as u32 > std::u8::MAX as u32 {
            return false;
        }
        if cur.buf[cur.index] == 0 {
            // We're at a 0 and could use this byte instead of adding a new
            // one. But considering we're kinda dealing with unicode, it's a
            // lot easier to just remove and re-add it.
            cur.buf.remove(cur.index);
            cur.index = cur.index.saturating_sub(1);
        } else {
            cur.index += 1;
        }

        let len = c.len_utf8();
        let mut bytes = [0; 4];
        c.encode_utf8(&mut bytes);

        let mut insert = [0; 4];
        for (i, b) in bytes.iter().enumerate() {
            insert[i] = *b as u64;
        }

        // Insert multiple
        unsafe {
            cur.buf.reserve(len);

            let old_len = cur.buf.len();
            let new_len = old_len + len;
            cur.buf.set_len(new_len);

            let p = cur.buf.as_mut_ptr().offset(cur.index as isize);
            ptr::copy(p, p.offset(len as isize), old_len - cur.index);
        }
        cur.buf[cur.index..][..len].copy_from_slice(&insert[..len]);

        cur.index += len - 1;

        true
    }

    fn pop(self, cur: &mut Cursor) {
        if cur.buf.len() == 1 {
            cur.buf[0] = 0;
            return;
        }

        cur.buf.remove(cur.index);
        cur.index = cur.index.saturating_sub(1);
    }
}
