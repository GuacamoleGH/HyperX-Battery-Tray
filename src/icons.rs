//! Crisp raster icons with no external fonts or assets.
use crate::settings::DisplayMode;

#[derive(Debug, Clone)]
pub struct IconPixels {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

fn put(p: &mut IconPixels, x: i32, y: i32, c: [u8; 4]) {
    if x >= 0 && y >= 0 && x < p.width as i32 && y < p.height as i32 {
        let i = ((y as u32 * p.width + x as u32) * 4) as usize;
        p.rgba[i..i + 4].copy_from_slice(&c);
    }
}

fn rect(p: &mut IconPixels, x: i32, y: i32, w: i32, h: i32, c: [u8; 4]) {
    for yy in y..y + h {
        for xx in x..x + w {
            put(p, xx, yy, c);
        }
    }
}

const FONT3X5: [[u8; 5]; 10] = [
    [7, 5, 5, 5, 7], // 0
    [2, 6, 2, 2, 7], // 1
    [7, 1, 7, 4, 7], // 2
    [7, 1, 7, 1, 7], // 3
    [5, 5, 7, 1, 1], // 4
    [7, 4, 7, 1, 7], // 5
    [7, 4, 7, 5, 7], // 6
    [7, 1, 2, 2, 2], // 7
    [7, 5, 7, 5, 7], // 8
    [7, 5, 7, 1, 7], // 9
];

const FONT4X7: [[u8; 7]; 10] = [
    [15, 9, 9, 9, 9, 9, 15],  // 0
    [6, 14, 6, 6, 6, 6, 15],  // 1
    [15, 1, 1, 15, 8, 8, 15], // 2
    [15, 1, 1, 15, 1, 1, 15], // 3
    [9, 9, 9, 15, 1, 1, 1],   // 4
    [15, 8, 8, 15, 1, 1, 15], // 5
    [15, 8, 8, 15, 9, 9, 15], // 6
    [15, 1, 2, 2, 4, 4, 4],   // 7
    [15, 9, 9, 15, 9, 9, 15], // 8
    [15, 9, 9, 15, 1, 1, 15], // 9
];

const FONT5X12: [[u8; 12]; 10] = [
    [14, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 14], // 0
    [6, 14, 6, 6, 6, 6, 6, 6, 6, 6, 6, 31],           // 1
    [14, 17, 1, 1, 2, 4, 8, 16, 16, 16, 31, 31],     // 2
    [14, 17, 1, 1, 14, 14, 1, 1, 1, 1, 17, 14],       // 3
    [17, 17, 17, 17, 17, 31, 31, 1, 1, 1, 1, 1],      // 4
    [31, 31, 16, 16, 30, 31, 1, 1, 1, 1, 17, 14],    // 5
    [14, 17, 16, 16, 30, 17, 17, 17, 17, 17, 17, 14], // 6
    [31, 31, 1, 2, 2, 4, 4, 8, 8, 16, 16, 16],        // 7
    [14, 17, 17, 17, 14, 14, 17, 17, 17, 17, 17, 14], // 8
    [14, 17, 17, 17, 17, 15, 1, 1, 1, 1, 17, 14],    // 9
];

fn draw_small_digit(p: &mut IconPixels, d: u8, x: i32, y: i32, c: [u8; 4]) {
    if d > 9 {
        return;
    }
    for (row, bits) in FONT3X5[d as usize].iter().enumerate() {
        for col in 0..3 {
            if bits & (1 << (2 - col)) != 0 {
                put(p, x + col, y + row as i32, c);
            }
        }
    }
}

fn draw_small_num(p: &mut IconPixels, text: &str, x: i32, y: i32, c: [u8; 4]) {
    let mut cur_x = x;
    if text == "100" {
        rect(p, cur_x, y, 1, 5, c);
        cur_x += 2;
        draw_small_digit(p, 0, cur_x, y, c);
        cur_x += 4;
        draw_small_digit(p, 0, cur_x, y, c);
        return;
    }
    for ch in text.bytes() {
        if ch.is_ascii_digit() {
            draw_small_digit(p, ch - b'0', cur_x, y, c);
            cur_x += 4;
        } else if ch == b'-' {
            rect(p, cur_x, y + 2, 3, 1, c);
            cur_x += 4;
        }
    }
}

fn draw_med_digit(p: &mut IconPixels, d: u8, x: i32, y: i32, c: [u8; 4]) {
    if d > 9 {
        return;
    }
    for (row, bits) in FONT4X7[d as usize].iter().enumerate() {
        for col in 0..4 {
            if bits & (1 << (3 - col)) != 0 {
                put(p, x + col, y + row as i32, c);
            }
        }
    }
}

fn draw_med_num(p: &mut IconPixels, text: &str, x: i32, y: i32, c: [u8; 4]) {
    let mut cur_x = x;
    if text == "100" {
        rect(p, cur_x, y, 1, 7, c);
        cur_x += 2;
        draw_med_digit(p, 0, cur_x, y, c);
        cur_x += 5;
        draw_med_digit(p, 0, cur_x, y, c);
        return;
    }
    for ch in text.bytes() {
        if ch.is_ascii_digit() {
            draw_med_digit(p, ch - b'0', cur_x, y, c);
            cur_x += 5;
        } else if ch == b'-' {
            rect(p, cur_x, y + 3, 3, 1, c);
            cur_x += 4;
        }
    }
}

fn draw_large_digit(p: &mut IconPixels, d: u8, x: i32, y: i32, c: [u8; 4]) {
    if d > 9 {
        return;
    }
    for (row, bits) in FONT5X12[d as usize].iter().enumerate() {
        for col in 0..5 {
            if bits & (1 << (4 - col)) != 0 {
                put(p, x + col, y + row as i32, c);
            }
        }
    }
}

fn draw_large_num(p: &mut IconPixels, text: &str, x: i32, y: i32, c: [u8; 4]) {
    let mut cur_x = x;
    if text == "100" {
        rect(p, cur_x, y, 2, 12, c);
        cur_x += 3;
        draw_large_digit(p, 0, cur_x, y, c);
        cur_x += 6;
        draw_large_digit(p, 0, cur_x, y, c);
        return;
    }
    for ch in text.bytes() {
        if ch.is_ascii_digit() {
            draw_large_digit(p, ch - b'0', cur_x, y, c);
            cur_x += 6;
        } else if ch == b'-' {
            rect(p, cur_x, y + 5, 4, 2, c);
            cur_x += 5;
        }
    }
}

pub fn make_icon(size: u32, mode: DisplayMode, percent: Option<u8>, low: u8) -> IconPixels {
    let mut p = IconPixels {
        width: size,
        height: size,
        rgba: vec![0; (size * size * 4) as usize],
    };

    let color = match percent {
        None => [125, 130, 140, 255],
        Some(v) if v <= low => [238, 68, 68, 255],
        Some(v) if v <= 40 => [245, 158, 11, 255],
        _ => [52, 211, 110, 255],
    };

    let s = size as i32;
    let border = [235, 238, 245, 255];
    let bg_inner = [25, 28, 34, 230];
    let text_col = if percent.is_some() {
        [245, 248, 255, 255]
    } else {
        [175, 180, 190, 255]
    };

    match mode {
        DisplayMode::IconPercent => {
            // Horizontal: LARGE BATTERY + NUMBER BELOW
            let bw = if s <= 18 { 14 } else { (s * 82) / 100 };
            let bh = if s <= 18 { 9 } else { (s * 50) / 100 };
            let tip_w = 1;
            let tip_h = (bh * 5) / 10;
            let gap = 1;
            let text_h = 5;
            let total_w = bw + tip_w;
            let total_h = bh + gap + text_h;

            let bx = (s - total_w) / 2;
            let by = (s - total_h) / 2;

            rect(&mut p, bx, by, bw, bh, bg_inner);
            rect(&mut p, bx, by, bw, 1, border);
            rect(&mut p, bx, by + bh - 1, bw, 1, border);
            rect(&mut p, bx, by, 1, bh, border);
            rect(&mut p, bx + bw - 1, by, 1, bh, border);

            let tip_y = by + (bh - tip_h) / 2;
            rect(&mut p, bx + bw, tip_y, tip_w, tip_h, border);

            if let Some(v) = percent {
                let iw = bw - 2;
                let ih = bh - 2;
                let fill_w = ((iw * (v.min(100) as i32) + 50) / 100).min(iw);
                if fill_w > 0 {
                    rect(&mut p, bx + 1, by + 1, fill_w, ih, color);
                }
            }

            let text = percent.map(|v| v.to_string()).unwrap_or_else(|| "--".into());
            let num_w = if text == "100" { 9 } else { text.len() as i32 * 4 - 1 };
            let num_x = (s - num_w) / 2;
            let num_y = by + bh + gap;
            draw_small_num(&mut p, &text, num_x, num_y, text_col);
        }
        DisplayMode::VerticalPercent => {
            // Vertical: LARGE BATTERY + TALLER 7PX NUMBERS
            let bw = if s <= 18 { 6 } else { 7 };
            let bh = if s <= 18 { 14 } else { 17 };
            let tip_w = 2;
            let tip_h = 1;

            let text = percent.map(|v| v.to_string()).unwrap_or_else(|| "--".into());
            let num_w = if text == "100" {
                10
            } else if text.len() == 2 {
                9
            } else if text == "--" {
                7
            } else {
                4
            };
            let gap = 1;
            let total_w = bw + gap + num_w;
            let total_h = bh + tip_h;

            let bx = ((s - total_w) / 2).max(0);
            let by = (s - total_h) / 2 + tip_h;

            let tip_x = bx + (bw - tip_w) / 2;
            rect(&mut p, tip_x, by - tip_h, tip_w, tip_h, border);

            rect(&mut p, bx, by, bw, bh, bg_inner);
            rect(&mut p, bx, by, bw, 1, border);
            rect(&mut p, bx, by + bh - 1, bw, 1, border);
            rect(&mut p, bx, by, 1, bh, border);
            rect(&mut p, bx + bw - 1, by, 1, bh, border);

            if let Some(v) = percent {
                let iw = bw - 2;
                let ih = bh - 2;
                let fill_h = ((ih * (v.min(100) as i32) + 50) / 100).min(ih);
                if fill_h > 0 {
                    rect(&mut p, bx + 1, by + ih - fill_h + 1, iw, fill_h, color);
                }
            }

            let num_x = bx + bw + gap;
            let num_y = (s - 7) / 2;
            draw_med_num(&mut p, &text, num_x, num_y, text_col);
        }
        DisplayMode::Icon => {
            // Battery only
            let bw = if s <= 18 { 13 } else { 16 };
            let bh = if s <= 18 { 10 } else { 12 };
            let tip_w = 2;
            let tip_h = 4;
            let total_w = bw + tip_w;
            let bx = (s - total_w) / 2;
            let by = (s - bh) / 2;

            rect(&mut p, bx, by, bw, bh, bg_inner);
            rect(&mut p, bx, by, bw, 1, border);
            rect(&mut p, bx, by + bh - 1, bw, 1, border);
            rect(&mut p, bx, by, 1, bh, border);
            rect(&mut p, bx + bw - 1, by, 1, bh, border);

            let tip_y = by + (bh - tip_h) / 2;
            rect(&mut p, bx + bw, tip_y, tip_w, tip_h, border);

            if let Some(v) = percent {
                let iw = bw - 2;
                let ih = bh - 2;
                let fill_w = ((iw * (v.min(100) as i32) + 50) / 100).min(iw);
                if fill_w > 0 {
                    rect(&mut p, bx + 1, by + 1, fill_w, ih, color);
                }
            }
        }
        DisplayMode::Number => {
            // Number only: 12PX TALL, BIG & BOLD!
            let text = percent.map(|v| v.to_string()).unwrap_or_else(|| "--".into());
            let num_h = 12;
            let num_w = if text == "100" {
                12
            } else if text.len() == 2 {
                11
            } else if text == "--" {
                9
            } else {
                5
            };

            let num_x = (s - num_w) / 2;
            let num_y = (s - num_h) / 2;
            draw_large_num(&mut p, &text, num_x, num_y, text_col);
        }
    }

    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icons_have_pixels() {
        for s in 16..65 {
            for m in [
                DisplayMode::IconPercent,
                DisplayMode::VerticalPercent,
                DisplayMode::Icon,
                DisplayMode::Number,
            ] {
                for v in [None, Some(0), Some(20), Some(50), Some(100)] {
                    assert!(make_icon(s, m, v, 20).rgba.iter().any(|x| *x != 0));
                }
            }
        }
    }

    #[test]
    fn one_hundred_fits() {
        let p = make_icon(16, DisplayMode::Number, Some(100), 20);
        assert_eq!(p.width, 16);
    }
}
