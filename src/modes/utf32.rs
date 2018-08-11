use super::prelude::*;

#[derive(Clone, Copy)]
crate struct Utf32;

fn chars<'a>(buf: &'a [u64]) -> impl Iterator<Item = char> + 'a {
    buf.iter()
        .filter(|&&c| c <= std::u32::MAX as u64)
        .filter_map(|&c| std::char::from_u32(c as u32))
}

impl Mode for Utf32 {
    fn fmt<W: Write>(self, w: &mut W, cur: &Cursor, selected: bool) -> io::Result<()> {
        for (i, c) in chars(&cur.buf).enumerate() {
            if selected && i == cur.index {
                write!(w, "{}", Underline)?;
            }
            let mut bytes = [0; 4];
            c.encode_utf8(&mut bytes);
            w.write_all(&bytes)?;
            if selected && i == cur.index {
                write!(w, "{}", NoUnderline)?;
            }
        }
        Ok(())
    }
    fn len(self, cur: &Cursor) -> usize {
        cur.buf.iter()
            .filter(|&&c| c <= std::u32::MAX as u64)
            .filter(|&&c| std::char::from_u32(c as u32).is_some())
            .count()
    }
    fn add(self, cur: &mut Cursor, c: char) -> bool {
        if cur.buf[cur.index] == 0 {
            cur.buf[cur.index] = c as u32 as u64;
        } else {
            cur.buf.insert(cur.index + 1, c as u32 as u64);
            cur.index += 1;
        }
        true
    }
    fn pop(self, cur: &mut Cursor) {
        if cur.buf.len() <= 1 {
            cur.buf[0] = 0;
        } else {
            cur.buf.remove(cur.index);
            cur.index = cur.index.saturating_sub(1);
        }
    }
}
