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
    let scale = (s / 16).max(1);
    let border = [225, 230, 238, 255];
    let bg_inner = [28, 30, 35, 220];
    let text_col = if percent.is_some() {
        [245, 248, 255, 255]
    } else {
        [175, 180, 190, 255]
    };

    match mode {
        DisplayMode::IconPercent => {
            // Horizontal compact battery on top, percentage centered below
            let bw = 12 * scale;
            let bh = 6 * scale;
            let bx = (s - (bw + 2 * scale)) / 2;
            let by = scale;

            rect(&mut p, bx, by, bw, bh, bg_inner);
            rect(&mut p, bx, by, bw, scale, border);
            rect(&mut p, bx, by + bh - scale, bw, scale, border);
            rect(&mut p, bx, by, scale, bh, border);
            rect(&mut p, bx + bw - scale, by, scale, bh, border);

            let tip_h = 2 * scale;
            let tip_y = by + (bh - tip_h) / 2;
            rect(&mut p, bx + bw, tip_y, scale, tip_h, border);

            if let Some(v) = percent {
                let ix = bx + scale;
                let iy = by + scale;
                let iw = bw - 2 * scale;
                let ih = bh - 2 * scale;
                let fill_w = ((iw * (v.min(100) as i32) + 50) / 100).min(iw);
                if fill_w > 0 {
                    rect(&mut p, ix, iy, fill_w, ih, color);
                }
            }

            let text = percent.map(|v| v.to_string()).unwrap_or_else(|| "--".into());
            let num_w = if text == "100" { 9 } else { text.len() as i32 * 4 - 1 } * scale;
            let num_x = (s - num_w) / 2;
            let extra_y = if s >= 20 { scale } else { 0 };
            let num_y = by + bh + scale + extra_y;
            small_number(&mut p, &text, num_x, num_y, scale, text_col);
        }
        DisplayMode::VerticalPercent => {
            // Vertical sleek battery on left, percentage on right
            let bw = 6 * scale;
            let bh = 13 * scale;
            let bx = 0;
            let by = 2 * scale;

            let tip_w = 2 * scale;
            let tip_h = scale;
            let tip_x = bx + (bw - tip_w) / 2;
            rect(&mut p, tip_x, by - tip_h, tip_w, tip_h, border);

            rect(&mut p, bx, by, bw, bh, bg_inner);
            rect(&mut p, bx, by, bw, scale, border);
            rect(&mut p, bx, by + bh - scale, bw, scale, border);
            rect(&mut p, bx, by, scale, bh, border);
            rect(&mut p, bx + bw - scale, by, scale, bh, border);

            if let Some(v) = percent {
                let ix = bx + scale;
                let iy = by + scale;
                let iw = bw - 2 * scale;
                let ih = bh - 2 * scale;
                let fill_h = ((ih * (v.min(100) as i32) + 50) / 100).min(ih);
                if fill_h > 0 {
                    rect(&mut p, ix, iy + ih - fill_h, iw, fill_h, color);
                }
            }

            let text = percent.map(|v| v.to_string()).unwrap_or_else(|| "--".into());
            let num_w = if text == "100" { 9 } else { text.len() as i32 * 4 - 1 } * scale;
            let avail_left = bx + bw + scale;
            let avail_w = s - avail_left;
            let num_x = avail_left + (avail_w - num_w) / 2;
            let num_y = (s - 5 * scale) / 2;
            small_number(&mut p, &text, num_x, num_y, scale, text_col);
        }
        DisplayMode::Icon => {
            // Battery only, centered
            let bw = 12 * scale;
            let bh = 8 * scale;
            let bx = (s - (bw + 2 * scale)) / 2;
            let by = (s - bh) / 2;

            rect(&mut p, bx, by, bw, bh, bg_inner);
            rect(&mut p, bx, by, bw, scale, border);
            rect(&mut p, bx, by + bh - scale, bw, scale, border);
            rect(&mut p, bx, by, scale, bh, border);
            rect(&mut p, bx + bw - scale, by, scale, bh, border);

            let tip_h = 4 * scale;
            let tip_y = by + (bh - tip_h) / 2;
            rect(&mut p, bx + bw, tip_y, scale, tip_h, border);

            if let Some(v) = percent {
                let ix = bx + scale;
                let iy = by + scale;
                let iw = bw - 2 * scale;
                let ih = bh - 2 * scale;
                let fill_w = ((iw * (v.min(100) as i32) + 50) / 100).min(iw);
                if fill_w > 0 {
                    rect(&mut p, ix, iy, fill_w, ih, color);
                }
            }
        }
        DisplayMode::Number => {
            // Percentage only, centered
            let text = percent.map(|v| v.to_string()).unwrap_or_else(|| "--".into());
            let num_w = if text == "100" { 9 } else { text.len() as i32 * 4 - 1 } * scale;
            let num_x = (s - num_w) / 2;
            let num_y = (s - 5 * scale) / 2;
            small_number(&mut p, &text, num_x, num_y, scale, text_col);
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
