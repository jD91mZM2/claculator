use super::prelude::*;

#[derive(Clone, Copy)]
pub struct Number;

impl Mode for Number {
    fn fmt<W: Write>(self, w: &mut W, cur: &Cursor, selected: bool) -> io::Result<()> {
        for (i, &n) in cur.buf.iter().enumerate() {
            if selected && i == cur.index { write!(w, "{}", Underline)?; }
            write!(w, "{}", n)?;
            if selected && i == cur.index { write!(w, "{}", NoUnderline)?; }
            write!(w, " ")?;
        }
        Ok(())
    }

    fn len(self, cur: &Cursor) -> usize {
        let mut len = 0;
        for &i in &cur.buf {
            let mut i = i; // make mutable
            loop {
                i /= 10;
                len += 1;
                if i == 0 {
                    break;
                }
            }
            len += 1; // space
        }
        len
    }

    fn add(self, cur: &mut Cursor, c: char) -> bool {
        if c == ' ' {
            cur.index += 1;
            cur.buf.insert(cur.index, 0);
            return true;
        }
        let digit = match c.to_digit(10) {
            Some(digit) => digit,
            None => return false
        };

        cur.buf[cur.index] = match cur.buf[cur.index].checked_mul(10).and_then(|i| i.checked_add(digit as u64)) {
            Some(i) => i,
            None => return false
        };
        true
    }

    fn pop(self, cur: &mut Cursor) {
        if cur.buf[cur.index] == 0 && cur.buf.len() > 1 {
            cur.buf.remove(cur.index);
            cur.index = cur.index.saturating_sub(1);
            return;
        }

        cur.buf[cur.index] /= 10;
    }
}
