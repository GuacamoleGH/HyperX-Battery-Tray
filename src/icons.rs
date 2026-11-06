//! Crisp, anti-aliased tray icons using Segoe UI Bold subpixel alpha font tables.

use crate::{fonts::AA_FONT_MAX16, settings::DisplayMode};

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
        // Digit 1 (3px wide), rows 1..=14
        for r in 1..=14 {
            blend_pixel(p, cur_x + 1, sy + r, rgb, 255);
            if (2..=5).contains(&r) {
                blend_pixel(p, cur_x, sy + r, rgb, 180);
            }
            if r >= 12 {
                blend_pixel(p, cur_x, sy + r, rgb, 255);
                blend_pixel(p, cur_x + 2, sy + r, rgb, 255);
            }
        }
        cur_x += 4;
        // Digit 0 (5px wide)
        let rows0 = &AA_FONT_MAX16[0];
        for r in 0..16 {
            for col in 1..6 {
                let a = rows0[r][col];
                blend_pixel(p, cur_x + col as i32 - 1, sy + r as i32, rgb, a);
            }
        }
        cur_x += 6;
        // Digit 0 (5px wide)
        for r in 0..16 {
            for col in 1..6 {
                let a = rows0[r][col];
                blend_pixel(p, cur_x + col as i32 - 1, sy + r as i32, rgb, a);
            }
        }
        return;
    }
    if text == "--" {
        // Centered vertically on rows 7..=8 (center 7.5, perfectly centered in 16px)
        for r in 7..=8 {
            for c in 0..5 {
                let a = if c == 0 || c == 4 { 180 } else { 255 };
                blend_pixel(p, sx + c, sy + r, rgb, a);
                blend_pixel(p, sx + c + 8, sy + r, rgb, a);
            }
        }
        return;
    }
    let mut cur_x = sx;
    for ch in text.bytes() {
        if ch.is_ascii_digit() {
            let d = (ch - b'0') as usize;
            let rows = &AA_FONT_MAX16[d];
            for r in 0..16 {
                for col in 0..7 {
                    let a = rows[r][col];
                    blend_pixel(p, cur_x + col as i32, sy + r as i32, rgb, a);
                }
            }
            cur_x += 8;
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
    let white = [250, 252, 255];
    let text = percent.map(|v| v.to_string()).unwrap_or_else(|| "--".into());

    match mode {
        DisplayMode::Number => {
            // Huge, bold, smooth anti-aliased Segoe UI numbers, perfectly centered vertically and horizontally!
            let num_w = if text == "100" {
                15
            } else if text == "--" {
                13
            } else if text.len() == 2 {
                15
            } else {
                7
            };
            let sx = (s - num_w) / 2;
            let sy = (s - 16) / 2;
            draw_large_num(&mut p, &text, sx, sy, white);
        }
        DisplayMode::Icon => {
            // Battery only (Full-size, clean, solid fill)
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
            for m in [DisplayMode::Icon, DisplayMode::Number] {
                for v in [None, Some(0), Some(20), Some(50), Some(100)] {
                    assert!(make_icon(s, m, v, 20).rgba.iter().any(|x| *x != 0));
                }
            }
        }
    }

    #[test]
    fn number_is_vertically_centered_in_16x16() {
        for v in [0, 50, 64, 99, 100] {
            let p = make_icon(16, DisplayMode::Number, Some(v), 20);
            // Rows 0 and 15 must have 0 alpha (perfect 1px top and bottom margin)
            let row0_alpha: u32 = (0..16).map(|x| p.rgba[((0 * 16 + x) * 4 + 3) as usize] as u32).sum();
            let row15_alpha: u32 = (0..16).map(|x| p.rgba[((15 * 16 + x) * 4 + 3) as usize] as u32).sum();
            assert_eq!(row0_alpha, 0, "value {v} should have top margin on row 0");
            assert_eq!(row15_alpha, 0, "value {v} should have bottom margin on row 15");
            // Rows 1 and 14 must have non-zero alpha (active glyph height is 14)
            let row1_alpha: u32 = (0..16).map(|x| p.rgba[((1 * 16 + x) * 4 + 3) as usize] as u32).sum();
            let row14_alpha: u32 = (0..16).map(|x| p.rgba[((14 * 16 + x) * 4 + 3) as usize] as u32).sum();
            assert!(row1_alpha > 0, "value {v} should be active on row 1");
            assert!(row14_alpha > 0, "value {v} should be active on row 14");
        }
    }
}
