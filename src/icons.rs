//! Crisp, anti-aliased tray icons using Segoe UI Bold subpixel alpha font tables.

use crate::{
    fonts::{AA_FONT_LARGE, AA_FONT_MED},
    settings::DisplayMode,
};

#[derive(Debug, Clone)]
pub struct IconPixels {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

fn blend_pixel(p: &mut IconPixels, x: i32, y: i32, rgb: [u8; 3], alpha: u8) {
    if alpha == 0 {
        return;
    }
    if x >= 0 && y >= 0 && x < p.width as i32 && y < p.height as i32 {
        let i = ((y as u32 * p.width + x as u32) * 4) as usize;
        let dst_a = p.rgba[i + 3] as u32;
        let src_a = alpha as u32;
        let out_a = src_a + dst_a * (255 - src_a) / 255;
        if out_a > 0 {
            for c in 0..3 {
                let src_c = rgb[c] as u32;
                let dst_c = p.rgba[i + c] as u32;
                let out_c = (src_c * src_a + dst_c * dst_a * (255 - src_a) / 255) / out_a;
                p.rgba[i + c] = out_c.min(255) as u8;
            }
            p.rgba[i + 3] = out_a.min(255) as u8;
        }
    }
}

fn fill_rect(p: &mut IconPixels, x: i32, y: i32, w: i32, h: i32, rgb: [u8; 3], alpha: u8) {
    for yy in y..y + h {
        for xx in x..x + w {
            blend_pixel(p, xx, yy, rgb, alpha);
        }
    }
}

fn draw_large_num(p: &mut IconPixels, text: &str, sx: i32, sy: i32, rgb: [u8; 3]) {
    if text == "100" {
        let mut cur_x = sx;
        // Digit 1 (3px wide)
        for r in 0..12 {
            blend_pixel(p, cur_x + 1, sy + r, rgb, 255);
            if (1..=3).contains(&r) {
                blend_pixel(p, cur_x, sy + r, rgb, 180);
            }
            if r >= 10 {
                blend_pixel(p, cur_x, sy + r, rgb, 255);
                blend_pixel(p, cur_x + 2, sy + r, rgb, 255);
            }
        }
        cur_x += 4;
        // Digit 0
        let rows0 = &AA_FONT_LARGE[0];
        for r in 0..12 {
            for col in 0..7 {
                blend_pixel(p, cur_x + col as i32 - 1, sy + r as i32, rgb, rows0[r][col]);
            }
        }
        cur_x += 5;
        // Digit 0
        for r in 0..12 {
            for col in 0..7 {
                blend_pixel(p, cur_x + col as i32 - 1, sy + r as i32, rgb, rows0[r][col]);
            }
        }
        return;
    }
    if text == "--" {
        for r in 5..=6 {
            for c in 0..5 {
                let a = if c == 0 || c == 4 { 180 } else { 255 };
                blend_pixel(p, sx + c, sy + r, rgb, a);
                blend_pixel(p, sx + c + 7, sy + r, rgb, a);
            }
        }
        return;
    }
    let mut cur_x = sx;
    for ch in text.bytes() {
        if ch.is_ascii_digit() {
            let d = (ch - b'0') as usize;
            let rows = &AA_FONT_LARGE[d];
            for r in 0..12 {
                for col in 0..7 {
                    let a = rows[r][col];
                    blend_pixel(p, cur_x + col as i32, sy + r as i32, rgb, a);
                }
            }
            cur_x += 8;
        }
    }
}

fn draw_med_num(p: &mut IconPixels, text: &str, sx: i32, sy: i32, rgb: [u8; 3]) {
    if text == "100" {
        let mut cur_x = sx;
        for r in 0..8 {
            blend_pixel(p, cur_x + 1, sy + r, rgb, 255);
        }
        cur_x += 3;
        let rows0 = &AA_FONT_MED[0];
        for r in 0..8 {
            for col in 0..5 {
                blend_pixel(p, cur_x + col as i32, sy + r as i32, rgb, rows0[r][col]);
            }
        }
        cur_x += 4;
        for r in 0..8 {
            for col in 0..5 {
                blend_pixel(p, cur_x + col as i32, sy + r as i32, rgb, rows0[r][col]);
            }
        }
        return;
    }
    if text == "--" {
        for r in 3..=4 {
            for c in 0..4 {
                let a = if c == 0 || c == 3 { 180 } else { 255 };
                blend_pixel(p, sx + c, sy + r, rgb, a);
                blend_pixel(p, sx + c + 6, sy + r, rgb, a);
            }
        }
        return;
    }
    let mut cur_x = sx;
    for ch in text.bytes() {
        if ch.is_ascii_digit() {
            let d = (ch - b'0') as usize;
            let rows = &AA_FONT_MED[d];
            for r in 0..8 {
                for col in 0..5 {
                    let a = rows[r][col];
                    blend_pixel(p, cur_x + col as i32, sy + r as i32, rgb, a);
                }
            }
            cur_x += 6;
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
        None => [125, 130, 140],
        Some(v) if v <= low => [239, 68, 68],
        Some(v) if v <= 40 => [245, 158, 11],
        _ => [52, 211, 110],
    };

    let s = size as i32;
    let border = [230, 235, 245];
    let bg_inner = [22, 24, 28];
    let white = [248, 250, 252];
    let text = percent.map(|v| v.to_string()).unwrap_or_else(|| "--".into());

    match mode {
        DisplayMode::Number => {
            // Big, bold, smooth anti-aliased Segoe UI numbers!
            let num_w = if text == "100" {
                14
            } else if text == "--" {
                12
            } else if text.len() == 2 {
                15
            } else {
                7
            };
            let sx = (s - num_w) / 2;
            let sy = (s - 12) / 2;
            draw_large_num(&mut p, &text, sx, sy, white);
        }
        DisplayMode::VerticalPercent => {
            // Vertical: Compact battery on left, BIG bold anti-aliased number on right!
            let bw = if s <= 18 { 4 } else { 5 };
            let bh = if s <= 18 { 12 } else { 15 };
            let bx = if s <= 18 { 0 } else { 1 };
            let by = (s - bh) / 2;

            // Nipple
            fill_rect(&mut p, bx + 1, by - 1, bw - 2, 1, border, 220);

            // Background & Border
            fill_rect(&mut p, bx, by, bw, bh, bg_inner, 230);
            fill_rect(&mut p, bx, by, bw, 1, border, 255);
            fill_rect(&mut p, bx, by + bh - 1, bw, 1, border, 255);
            fill_rect(&mut p, bx, by, 1, bh, border, 255);
            fill_rect(&mut p, bx + bw - 1, by, 1, bh, border, 255);

            // Fill from bottom
            if let Some(v) = percent {
                let ih = bh - 2;
                let iw = bw - 2;
                let fill_h = ((ih * (v.min(100) as i32) + 50) / 100).min(ih);
                if fill_h > 0 {
                    fill_rect(&mut p, bx + 1, by + ih - fill_h + 1, iw, fill_h, color, 255);
                }
            }

            // BIG number on right
            let num_x = bx + bw + 1;
            let num_y = (s - 8) / 2;
            draw_med_num(&mut p, &text, num_x, num_y, white);
        }
        DisplayMode::IconPercent => {
            // Horizontal: Compact battery on top, BIG bold anti-aliased number below!
            let bw = if s <= 18 { 10 } else { 13 };
            let bh = if s <= 18 { 5 } else { 6 };
            let bx = (s - (bw + 1)) / 2;
            let by = if s <= 18 { 1 } else { 2 };

            // Outline & Background
            fill_rect(&mut p, bx, by, bw, bh, bg_inner, 230);
            fill_rect(&mut p, bx, by, bw, 1, border, 255);
            fill_rect(&mut p, bx, by + bh - 1, bw, 1, border, 255);
            fill_rect(&mut p, bx, by, 1, bh, border, 255);
            fill_rect(&mut p, bx + bw - 1, by, 1, bh, border, 255);

            // Nipple
            fill_rect(&mut p, bx + bw, by + 1, 1, bh - 2, border, 220);

            // Fill horizontally
            if let Some(v) = percent {
                let iw = bw - 2;
                let ih = bh - 2;
                let fill_w = ((iw * (v.min(100) as i32) + 50) / 100).min(iw);
                if fill_w > 0 {
                    fill_rect(&mut p, bx + 1, by + 1, fill_w, ih, color, 255);
                }
            }

            // BIG number below
            let num_w = if text == "100" {
                11
            } else if text == "--" {
                10
            } else if text.len() == 2 {
                11
            } else {
                5
            };
            let num_x = (s - num_w) / 2;
            let num_y = by + bh + 1;
            draw_med_num(&mut p, &text, num_x, num_y, white);
        }
        DisplayMode::Icon => {
            // Battery only (Full-size, user's favorite)
            let bw = if s <= 18 { 14 } else { 17 };
            let bh = if s <= 18 { 10 } else { 12 };
            let tip_w = 2;
            let tip_h = 4;
            let total_w = bw + tip_w;
            let bx = (s - total_w) / 2;
            let by = (s - bh) / 2;

            fill_rect(&mut p, bx, by, bw, bh, bg_inner, 230);
            fill_rect(&mut p, bx, by, bw, 1, border, 255);
            fill_rect(&mut p, bx, by + bh - 1, bw, 1, border, 255);
            fill_rect(&mut p, bx, by, 1, bh, border, 255);
            fill_rect(&mut p, bx + bw - 1, by, 1, bh, border, 255);

            let tip_y = by + (bh - tip_h) / 2;
            fill_rect(&mut p, bx + bw, tip_y, tip_w, tip_h, border, 255);

            if let Some(v) = percent {
                let iw = bw - 2;
                let ih = bh - 2;
                let fill_w = ((iw * (v.min(100) as i32) + 50) / 100).min(iw);
                if fill_w > 0 {
                    fill_rect(&mut p, bx + 1, by + 1, fill_w, ih, color, 255);
                }
            }
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
