use crate::{PIXEL_OFF_COLOR, PIXEL_ON_COLOR};

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

        let clear_len = fb_len.min(framebuffer.len());
        framebuffer[..clear_len].fill(PIXEL_OFF_COLOR);

        let x_off = if width > self.width {
            (width - self.width) / 2
        } else {
            0
        };
        let y_off = if height > self.height {
            (height - self.height) / 2
        } else {
            0
        };

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let set = self.data.get(idx).copied().unwrap_or(false);
                let color = if set { PIXEL_ON_COLOR } else { PIXEL_OFF_COLOR };

                let fb_x = x + x_off;
                let fb_y = y + y_off;
                if fb_x < width && fb_y < height {
                    let fb_idx = fb_y * width + fb_x;
                    if fb_idx < framebuffer.len() {
                        framebuffer[fb_idx] = color;
                    }
                }
            }
        }
    }
}
