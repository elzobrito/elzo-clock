//! Fonte de dígitos “bricks” com blocos sólidos `█`.
//!
//! Mesma ideia da fonte bricks do tclock/clock-tui (MIT): cada linha é uma
//! sequência off/on de colunas, escalada por `size`, preenchida com `█`
//! (não `#`), para o relógio ficar liso.

const GLYPH_COLS: u16 = 6;
const GLYPH_ROWS: u16 = 5;
const BLOCK: char = '█';
const CHAR_SPACING: usize = 1;

/// Returns rendered lines for a time string such as `22:49:08`.
pub fn render_time(time: &str, scale: u8) -> Vec<String> {
    let size = scale.clamp(1, 3) as u16;
    let height = (GLYPH_ROWS.saturating_mul(size)) as usize;
    let mut rows = vec![String::new(); height];

    for (i, ch) in time.chars().enumerate() {
        if i > 0 {
            for row in &mut rows {
                row.push_str(&" ".repeat(CHAR_SPACING * size as usize));
            }
        }
        let glyph_rows = render_char(ch, size);
        for (r, line) in glyph_rows.iter().enumerate() {
            if r < rows.len() {
                rows[r].push_str(line);
            }
        }
    }
    rows
}

pub fn digit_height(scale: u8) -> u16 {
    let size = scale.clamp(1, 3) as u16;
    GLYPH_ROWS.saturating_mul(size)
}

fn render_char(c: char, size: u16) -> Vec<String> {
    let matrix = char_matrix(c).unwrap_or_else(empty_matrix);
    let mut out = Vec::with_capacity((GLYPH_ROWS * size) as usize);
    for row in matrix {
        let line = row_to_string(&row, size);
        for _ in 0..size {
            out.push(line.clone());
        }
    }
    out
}

/// Odd indices = off length, even indices = on length (after first off).
/// Pattern starts with `on = false`, then toggles per segment.
/// Examples (size=1):
///   [0, 6]       -> ██████
///   [2, 2]       ->   ██
///   [0, 2, 2, 2] -> ██  ██
fn row_to_string(row: &[u16], size: u16) -> String {
    let mut s = String::new();
    let mut on = false;
    for &len in row {
        let n = (len as usize).saturating_mul(size as usize);
        if on {
            for _ in 0..n {
                s.push(BLOCK);
            }
        } else {
            for _ in 0..n {
                s.push(' ');
            }
        }
        on = !on;
    }
    // pad to full glyph width so spacing stays even
    let target = (GLYPH_COLS as usize).saturating_mul(size as usize);
    while s.chars().count() < target {
        s.push(' ');
    }
    s
}

fn empty_matrix() -> [Vec<u16>; 5] {
    [vec![], vec![], vec![], vec![], vec![]]
}

fn char_matrix(c: char) -> Option<[Vec<u16>; 5]> {
    match c {
        '0' => Some([
            vec![0, 6],
            vec![0, 2, 2, 2],
            vec![0, 2, 2, 2],
            vec![0, 2, 2, 2],
            vec![0, 6],
        ]),
        '1' => Some([vec![0, 4], vec![2, 2], vec![2, 2], vec![2, 2], vec![0, 6]]),
        '2' => Some([vec![0, 6], vec![4, 2], vec![0, 6], vec![0, 2], vec![0, 6]]),
        '3' => Some([vec![0, 6], vec![4, 2], vec![0, 6], vec![4, 2], vec![0, 6]]),
        '4' => Some([
            vec![0, 2, 2, 2],
            vec![0, 2, 2, 2],
            vec![0, 6],
            vec![4, 2],
            vec![4, 2],
        ]),
        '5' => Some([vec![0, 6], vec![0, 2], vec![0, 6], vec![4, 2], vec![0, 6]]),
        '6' => Some([
            vec![0, 6],
            vec![0, 2],
            vec![0, 6],
            vec![0, 2, 2, 2],
            vec![0, 6],
        ]),
        '7' => Some([vec![0, 6], vec![4, 2], vec![4, 2], vec![4, 2], vec![4, 2]]),
        '8' => Some([
            vec![0, 6],
            vec![0, 2, 2, 2],
            vec![0, 6],
            vec![0, 2, 2, 2],
            vec![0, 6],
        ]),
        '9' => Some([
            vec![0, 6],
            vec![0, 2, 2, 2],
            vec![0, 6],
            vec![4, 2],
            vec![0, 6],
        ]),
        ':' => Some([vec![], vec![2, 2], vec![], vec![2, 2], vec![]]),
        '.' => Some([vec![], vec![], vec![], vec![], vec![2, 2]]),
        '-' => Some([vec![], vec![], vec![0, 6], vec![], vec![]]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_solid_blocks() {
        let lines = render_char('0', 1);
        assert_eq!(lines.len(), 5);
        assert!(lines[0].contains('█'));
        assert!(!lines[0].contains('#'));
        assert_eq!(lines[0].chars().filter(|&c| c == '█').count(), 6);
    }

    #[test]
    fn every_supported_glyph_renders_at_each_scale() {
        for ch in "0123456789:.-".chars() {
            for scale in 1..=3 {
                let lines = render_char(ch, scale);
                assert_eq!(lines.len(), (GLYPH_ROWS * scale) as usize);
                assert!(lines
                    .iter()
                    .all(|line| line.chars().count() == (GLYPH_COLS * scale) as usize));
            }
        }
    }

    #[test]
    fn unknown_glyph_renders_as_blank_space() {
        let lines = render_char('x', 1);
        assert!(lines.iter().all(|line| line.trim().is_empty()));
    }
}
