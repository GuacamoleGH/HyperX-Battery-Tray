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

const FONT3: [[u8; 5]; 10] = [
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

const FONT5_BOLD: [[u8; 7]; 10] = [
    [14, 17, 17, 17, 17, 17, 14], // 0
    [4, 12, 4, 4, 4, 4, 14],      // 1
    [14, 17, 1, 2, 4, 8, 31],     // 2
    [30, 1, 1, 14, 1, 1, 30],     // 3
    [2, 6, 10, 18, 31, 2, 2],     // 4
    [31, 16, 16, 30, 1, 1, 30],   // 5
    [14, 16, 16, 30, 17, 17, 14], // 6
    [31, 1, 2, 4, 8, 8, 8],       // 7
    [14, 17, 17, 14, 17, 17, 14], // 8
    [14, 17, 17, 15, 1, 1, 14],   // 9
];

fn small_digit(p: &mut IconPixels, d: u8, x: i32, y: i32, scale: i32, c: [u8; 4]) {
    if d > 9 {
        return;
    }
    for (row, bits) in FONT3[d as usize].iter().enumerate() {
        for col in 0..3 {
            if bits & (1 << (2 - col)) != 0 {
                rect(p, x + col * scale, y + row as i32 * scale, scale, scale, c);
            }
        }
    }
}

fn small_number(p: &mut IconPixels, text: &str, x: i32, y: i32, scale: i32, c: [u8; 4]) {
    let mut cur_x = x;
    if text == "100" {
        rect(p, cur_x, y, scale, 5 * scale, c);
        cur_x += 2 * scale;
        small_digit(p, 0, cur_x, y, scale, c);
        cur_x += 4 * scale;
        small_digit(p, 0, cur_x, y, scale, c);
        return;
    }
    for ch in text.bytes() {
        if ch.is_ascii_digit() {
            small_digit(p, ch - b'0', cur_x, y, scale, c);
            cur_x += 4 * scale;
        } else if ch == b'-' {
            rect(p, cur_x, y + 2 * scale, 3 * scale, scale, c);
            cur_x += 4 * scale;
        }
    }
}

fn bold_number(p: &mut IconPixels, text: &str, x: i32, y: i32, scale: i32, c: [u8; 4]) {
    let mut cur_x = x;
    if text == "100" {
        rect(p, cur_x, y, 2 * scale, 7 * scale, c);
        cur_x += 3 * scale;
        small_digit(p, 0, cur_x, y + scale, scale, c);
        cur_x += 4 * scale;
        small_digit(p, 0, cur_x, y + scale, scale, c);
        return;
    }
    for ch in text.bytes() {
        if ch.is_ascii_digit() {
            let d = (ch - b'0') as usize;
            for (row, bits) in FONT5_BOLD[d].iter().enumerate() {
                for col in 0..5 {
                    if bits & (1 << (4 - col)) != 0 {
                        rect(
                            p,
                            cur_x + col * scale,
                            y + row as i32 * scale,
                            scale,
                            scale,
                            c,
                        );
                    }
                }
            }
            cur_x += 6 * scale;
        } else if ch == b'-' {
            rect(p, cur_x, y + 3 * scale, 4 * scale, 2 * scale, c);
            cur_x += 5 * scale;
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
    let border = [232, 236, 245, 255];
    let bg_inner = [25, 28, 34, 230];
    let text_col = if percent.is_some() {
        [245, 248, 255, 255]
    } else {
        [175, 180, 190, 255]
    };

    match mode {
        DisplayMode::IconPercent => {
            let (bw, bh, tip_w, tip_h, border_th) = if s <= 18 {
                (13, 8, 1, 4, 1)
            } else if s <= 22 {
                (16, 9, 2, 4, 1)
            } else if s <= 28 {
                (19, 11, 2, 5, 1)
            } else {
                (25, 14, 3, 6, 2)
            };

            let font_scale = if s >= 30 { 2 } else { 1 };
            let text_h = 5 * font_scale;
            let gap = 1;
            let total_h = bh + gap + text_h;
            let total_w = bw + tip_w;

            let bx = (s - total_w) / 2;
            let by = (s - total_h) / 2;

            rect(&mut p, bx, by, bw, bh, bg_inner);
            rect(&mut p, bx, by, bw, border_th, border);
            rect(&mut p, bx, by + bh - border_th, bw, border_th, border);
            rect(&mut p, bx, by, border_th, bh, border);
            rect(&mut p, bx + bw - border_th, by, border_th, bh, border);

            let tip_y = by + (bh - tip_h) / 2;
            rect(&mut p, bx + bw, tip_y, tip_w, tip_h, border);

            if let Some(v) = percent {
                let ix = bx + border_th;
                let iy = by + border_th;
                let iw = bw - 2 * border_th;
                let ih = bh - 2 * border_th;
                let fill_w = ((iw * (v.min(100) as i32) + 50) / 100).min(iw);
                if fill_w > 0 {
                    rect(&mut p, ix, iy, fill_w, ih, color);
                }
            }

            let text = percent.map(|v| v.to_string()).unwrap_or_else(|| "--".into());
            let num_w = if text == "100" { 9 } else { text.len() as i32 * 4 - 1 } * font_scale;
            let num_x = (s - num_w) / 2;
            let num_y = by + bh + gap;
            small_number(&mut p, &text, num_x, num_y, font_scale, text_col);
        }
        DisplayMode::VerticalPercent => {
            let (bw, bh, tip_w, tip_h, border_th) = if s <= 18 {
                (6, 13, 2, 1, 1)
            } else if s <= 22 {
                (7, 16, 3, 1, 1)
            } else if s <= 28 {
                (8, 20, 4, 2, 1)
            } else {
                (12, 26, 4, 2, 2)
            };

            let font_scale = if s >= 30 { 2 } else { 1 };
            let text = percent.map(|v| v.to_string()).unwrap_or_else(|| "--".into());
            let num_w = if text == "100" { 9 } else { text.len() as i32 * 4 - 1 } * font_scale;
            let gap = 2;
            let total_w = bw + gap + num_w;
            let total_h = bh + tip_h;

            let bx = ((s - total_w) / 2).max(0);
            let by = (s - total_h) / 2 + tip_h;

            let tip_x = bx + (bw - tip_w) / 2;
            rect(&mut p, tip_x, by - tip_h, tip_w, tip_h, border);

            rect(&mut p, bx, by, bw, bh, bg_inner);
            rect(&mut p, bx, by, bw, border_th, border);
            rect(&mut p, bx, by + bh - border_th, bw, border_th, border);
            rect(&mut p, bx, by, border_th, bh, border);
            rect(&mut p, bx + bw - border_th, by, border_th, bh, border);

            if let Some(v) = percent {
                let ix = bx + border_th;
                let iy = by + border_th;
                let iw = bw - 2 * border_th;
                let ih = bh - 2 * border_th;
                let fill_h = ((ih * (v.min(100) as i32) + 50) / 100).min(ih);
                if fill_h > 0 {
                    rect(&mut p, ix, iy + ih - fill_h, iw, fill_h, color);
                }
            }

            let num_x = bx + bw + gap;
            let num_y = (s - 5 * font_scale) / 2;
            small_number(&mut p, &text, num_x, num_y, font_scale, text_col);
        }
        DisplayMode::Icon => {
            let (bw, bh, tip_w, tip_h, border_th) = if s <= 18 {
                (13, 10, 2, 4, 1)
            } else if s <= 22 {
                (16, 12, 2, 5, 1)
            } else if s <= 28 {
                (20, 15, 3, 6, 1)
            } else {
                (25, 18, 4, 8, 2)
            };

            let total_w = bw + tip_w;
            let bx = (s - total_w) / 2;
            let by = (s - bh) / 2;

            rect(&mut p, bx, by, bw, bh, bg_inner);
            rect(&mut p, bx, by, bw, border_th, border);
            rect(&mut p, bx, by + bh - border_th, bw, border_th, border);
            rect(&mut p, bx, by, border_th, bh, border);
            rect(&mut p, bx + bw - border_th, by, border_th, bh, border);

            let tip_y = by + (bh - tip_h) / 2;
            rect(&mut p, bx + bw, tip_y, tip_w, tip_h, border);

            if let Some(v) = percent {
                let ix = bx + border_th;
                let iy = by + border_th;
                let iw = bw - 2 * border_th;
                let ih = bh - 2 * border_th;
                let fill_w = ((iw * (v.min(100) as i32) + 50) / 100).min(iw);
                if fill_w > 0 {
                    rect(&mut p, ix, iy, fill_w, ih, color);
                }
            }
        }
        DisplayMode::Number => {
            let text = percent.map(|v| v.to_string()).unwrap_or_else(|| "--".into());
            let font_scale = if s >= 28 { 2 } else { 1 };
            let num_h = 7 * font_scale;
            let num_w = if text == "100" {
                10 * font_scale
            } else if text.len() == 2 {
                11 * font_scale
            } else if text == "--" {
                9 * font_scale
            } else {
                5 * font_scale
            };

            let num_x = (s - num_w) / 2;
            let num_y = (s - num_h) / 2;
            bold_number(&mut p, &text, num_x, num_y, font_scale, text_col);
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
