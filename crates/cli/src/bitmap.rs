use crate::{PIXEL_OFF_COLOR, PIXEL_ON_COLOR};
const BORDER_COLOR: u32 = PIXEL_ON_COLOR;

#[derive(Debug)]
pub struct Bitmap {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl Bitmap {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        let size = (width * height) / 8;
        Bitmap {
            width,
            height,
            data: vec![false; size],
        }
    }

    pub fn load_from_slice(&mut self, src: &[u8]) {
        let pixel_count = self.width * self.height;
        if self.data.len() != pixel_count {
            self.data.resize(pixel_count, false);
        }
        self.data.fill(false);

        let mut pixel_index = 0usize;
        for &byte in src {
            for bit in (0..8).rev() {
                if pixel_index >= pixel_count {
                    return;
                }
                let set = ((byte >> bit) & 1) == 1;
                self.data[pixel_index] = set;
                pixel_index += 1;
            }
        }
    }

    pub fn render(&mut self, framebuffer: &mut [u32], width: usize, height: usize) {
        let fb_len = width.saturating_mul(height);
        if fb_len == 0 || framebuffer.is_empty() {
            return;
        }
        if self.width == 0 || self.height == 0 {
            return;
        }

        let clear_len = fb_len.min(framebuffer.len());
        framebuffer[..clear_len].fill(BORDER_COLOR);

        // Fill full window width (no left/right bars), keep aspect ratio.
        // Remaining top/bottom area is left as white border.
        let target_w = width;
        let target_h = self.height.saturating_mul(target_w) / self.width;
        if target_h == 0 {
            return;
        }

        let (y_off, crop_top) = if target_h <= height {
            ((height - target_h) / 2, 0)
        } else {
            (0, (target_h - height) / 2)
        };

        for y in 0..height {
            let scaled_y = if target_h <= height {
                if y < y_off || y >= y_off + target_h {
                    continue;
                }
                y - y_off
            } else {
                y + crop_top
            };
            let src_y = scaled_y.saturating_mul(self.height) / target_h;

            for x in 0..width {
                let src_x = x.saturating_mul(self.width) / target_w;
                let idx = src_y.saturating_mul(self.width) + src_x;
                let set = self.data.get(idx).copied().unwrap_or(false);
                let color = if set { PIXEL_ON_COLOR } else { PIXEL_OFF_COLOR };

                let fb_idx = y.saturating_mul(width) + x;
                if fb_idx < framebuffer.len() {
                    framebuffer[fb_idx] = color;
                }
            }
        }
    }
}
