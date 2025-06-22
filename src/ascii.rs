use image::{DynamicImage, ImageBuffer, Rgb, imageops::FilterType};
use ratatui::style::Style;

/// ASCII character sets from darkest to lightest
pub const ASCII_CHARS_DENSE: &[char] =
    &['@', '#', 'S', '%', '?', '*', '+', ';', ':', ',', '.', ' '];
pub const ASCII_CHARS_SIMPLE: &[char] = &['@', '#', '*', '+', '-', '.', ' '];
pub const ASCII_CHARS_BLOCKS: &[char] = &['█', '▉', '▊', '▋', '▌', '▍', '▎', '▏', ' '];
pub const ASCII_CHARS_MINIMAL: &[char] = &['█', '▓', '▒', '░', ' '];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CharacterSet {
    Dense,
    Simple,
    Blocks,
    Minimal,
}

impl CharacterSet {
    pub fn chars(&self) -> &'static [char] {
        match self {
            CharacterSet::Dense => ASCII_CHARS_DENSE,
            CharacterSet::Simple => ASCII_CHARS_SIMPLE,
            CharacterSet::Blocks => ASCII_CHARS_BLOCKS,
            CharacterSet::Minimal => ASCII_CHARS_MINIMAL,
        }
    }

    pub fn next(&self) -> CharacterSet {
        match self {
            CharacterSet::Dense => CharacterSet::Simple,
            CharacterSet::Simple => CharacterSet::Blocks,
            CharacterSet::Blocks => CharacterSet::Minimal,
            CharacterSet::Minimal => CharacterSet::Dense,
        }
    }

    pub fn previous(&self) -> CharacterSet {
        match self {
            CharacterSet::Dense => CharacterSet::Minimal,
            CharacterSet::Simple => CharacterSet::Dense,
            CharacterSet::Blocks => CharacterSet::Simple,
            CharacterSet::Minimal => CharacterSet::Blocks,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            CharacterSet::Dense => "Dense",
            CharacterSet::Simple => "Simple",
            CharacterSet::Blocks => "Blocks",
            CharacterSet::Minimal => "Minimal",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColoredChar {
    pub ch: char,
    pub style: Style,
}

#[derive(Debug, Clone)]
pub struct AsciiConverter {
    character_set: CharacterSet,
    width: u32,
    height: u32,
    color_enabled: bool,
    scale_factor: f32,
}

impl AsciiConverter {
    pub fn new(character_set: CharacterSet, width: u32, height: u32) -> Self {
        Self {
            character_set,
            width,
            height,
            color_enabled: false,
            scale_factor: 1.0,
        }
    }

    pub fn new_dense(width: u32, height: u32) -> Self {
        Self::new(CharacterSet::Dense, width, height)
    }

    pub fn new_simple(width: u32, height: u32) -> Self {
        Self::new(CharacterSet::Simple, width, height)
    }

    pub fn new_blocks(width: u32, height: u32) -> Self {
        Self::new(CharacterSet::Blocks, width, height)
    }

    /// Convert image to ASCII art with optimized performance
    pub fn convert_image(&self, image: &DynamicImage) -> Vec<String> {
        let (target_width, target_height) = self.get_scaled_dimensions();

        // Use Triangle filtering for better quality while still being faster than Lanczos3
        let resized = image.resize_exact(target_width, target_height, FilterType::Triangle);

        // Convert to grayscale
        let gray = resized.to_luma8();

        let chars = self.character_set.chars();
        let mut result = Vec::with_capacity(target_height as usize);

        for y in 0..target_height {
            let mut line = String::with_capacity(target_width as usize);
            for x in 0..target_width {
                let pixel = gray.get_pixel(x, y);
                let brightness = pixel[0] as f32 / 255.0;
                let char_index = ((1.0 - brightness) * (chars.len() - 1) as f32) as usize;
                let char_index = char_index.min(chars.len() - 1);
                line.push(chars[char_index]);
            }
            result.push(line);
        }

        result
    }

    /// Convert image to colored ASCII art with optimized performance
    pub fn convert_image_colored(&self, image: &DynamicImage) -> Vec<Vec<ColoredChar>> {
        let (target_width, target_height) = self.get_scaled_dimensions();

        // Use Triangle filtering for better quality while still being faster than Lanczos3
        let resized = image.resize_exact(target_width, target_height, FilterType::Triangle);
        let rgb_image = resized.to_rgb8();

        let chars = self.character_set.chars();
        let mut result = Vec::with_capacity(target_height as usize);

        for y in 0..target_height {
            let mut line = Vec::with_capacity(target_width as usize);
            for x in 0..target_width {
                let pixel = rgb_image.get_pixel(x, y);
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

                // Corrected brightness calculation using proper luminance formula
                let brightness = (77 * r as u32 + 150 * g as u32 + 29 * b as u32) / 256;
                let char_index = ((255 - brightness) * (chars.len() - 1) as u32 / 255) as usize;
                let char_index = char_index.min(chars.len() - 1);

                let style = if self.color_enabled {
                    Style::default().fg(ratatui::style::Color::Rgb(r, g, b))
                } else {
                    Style::default()
                };

                line.push(ColoredChar {
                    ch: chars[char_index],
                    style,
                });
            }
            result.push(line);
        }

        result
    }

    /// Convert raw RGB frame to ASCII art with optimized performance
    pub fn convert_rgb_frame(
        &self,
        frame: &[u8],
        frame_width: u32,
        frame_height: u32,
    ) -> Vec<String> {
        if frame.len() != (frame_width * frame_height * 3) as usize {
            return vec!["Invalid frame data".to_string()];
        }

        // Create image buffer from raw RGB data
        let img_buffer = match ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(
            frame_width,
            frame_height,
            frame.to_vec(),
        ) {
            Some(buffer) => buffer,
            None => return vec!["Failed to create image buffer".to_string()],
        };

        let dynamic_image = DynamicImage::ImageRgb8(img_buffer);
        self.convert_image(&dynamic_image)
    }

    /// Convert raw RGB frame to colored ASCII art with optimized performance
    pub fn convert_rgb_frame_colored(
        &self,
        frame: &[u8],
        frame_width: u32,
        frame_height: u32,
    ) -> Vec<Vec<ColoredChar>> {
        if frame.len() != (frame_width * frame_height * 3) as usize {
            return vec![vec![ColoredChar {
                ch: 'E',
                style: Style::default(),
            }]];
        }

        // Create image buffer from raw RGB data
        let img_buffer = match ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(
            frame_width,
            frame_height,
            frame.to_vec(),
        ) {
            Some(buffer) => buffer,
            None => {
                return vec![vec![ColoredChar {
                    ch: 'E',
                    style: Style::default(),
                }]];
            }
        };

        let dynamic_image = DynamicImage::ImageRgb8(img_buffer);
        self.convert_image_colored(&dynamic_image)
    }

    /// Fast RGB frame to ASCII conversion with improved quality sampling
    pub fn convert_rgb_frame_direct(
        &self,
        frame: &[u8],
        frame_width: u32,
        frame_height: u32,
    ) -> Vec<Vec<ColoredChar>> {
        if frame.len() != (frame_width * frame_height * 3) as usize {
            return vec![vec![ColoredChar {
                ch: 'E',
                style: Style::default(),
            }]];
        }

        let (target_width, target_height) = self.get_scaled_dimensions();
        let chars = self.character_set.chars();
        let mut result = Vec::with_capacity(target_height as usize);

        // Calculate scaling factors
        let x_scale = frame_width as f32 / target_width as f32;
        let y_scale = frame_height as f32 / target_height as f32;

        for y in 0..target_height {
            let mut line = Vec::with_capacity(target_width as usize);
            for x in 0..target_width {
                // Use 2x2 sampling for better quality when downscaling
                let src_x = (x as f32 * x_scale) as u32;
                let src_y = (y as f32 * y_scale) as u32;

                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                let mut samples = 0u32;

                // Sample a small area instead of single pixel
                for dy in 0..2 {
                    for dx in 0..2 {
                        let sample_x = (src_x + dx).min(frame_width - 1);
                        let sample_y = (src_y + dy).min(frame_height - 1);
                        let pixel_idx = ((sample_y * frame_width + sample_x) * 3) as usize;

                        if pixel_idx + 2 < frame.len() {
                            r_sum += frame[pixel_idx] as u32;
                            g_sum += frame[pixel_idx + 1] as u32;
                            b_sum += frame[pixel_idx + 2] as u32;
                            samples += 1;
                        }
                    }
                }

                if samples > 0 {
                    let r = (r_sum / samples) as u8;
                    let g = (g_sum / samples) as u8;
                    let b = (b_sum / samples) as u8;

                    // Corrected brightness calculation using proper luminance formula
                    let brightness = (77 * r as u32 + 150 * g as u32 + 29 * b as u32) / 256;
                    let char_index = ((255 - brightness) * (chars.len() - 1) as u32 / 255) as usize;
                    let char_index = char_index.min(chars.len() - 1);

                    let style = if self.color_enabled {
                        Style::default().fg(ratatui::style::Color::Rgb(r, g, b))
                    } else {
                        Style::default()
                    };

                    line.push(ColoredChar {
                        ch: chars[char_index],
                        style,
                    });
                } else {
                    line.push(ColoredChar {
                        ch: ' ',
                        style: Style::default(),
                    });
                }
            }
            result.push(line);
        }

        result
    }

    // Getters and setters
    pub fn character_set(&self) -> CharacterSet {
        self.character_set
    }

    pub fn set_character_set(&mut self, character_set: CharacterSet) {
        self.character_set = character_set;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn color_enabled(&self) -> bool {
        self.color_enabled
    }

    pub fn set_color_enabled(&mut self, enabled: bool) {
        self.color_enabled = enabled;
    }

    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    pub fn set_scale_factor(&mut self, factor: f32) {
        self.scale_factor = factor.clamp(0.1, 2.0);
    }

    pub fn increase_scale(&mut self) {
        self.scale_factor = (self.scale_factor + 0.1).min(2.0);
    }

    pub fn decrease_scale(&mut self) {
        self.scale_factor = (self.scale_factor - 0.1).max(0.1);
    }

    fn get_scaled_dimensions(&self) -> (u32, u32) {
        let width = (self.width as f32 * self.scale_factor) as u32;
        let height = (self.height as f32 * self.scale_factor) as u32;
        (width.max(1), height.max(1))
    }

    pub fn toggle_color(&mut self) {
        self.color_enabled = !self.color_enabled;
    }

    pub fn next_character_set(&mut self) {
        self.character_set = self.character_set.next();
    }

    pub fn previous_character_set(&mut self) {
        self.character_set = self.character_set.previous();
    }
}

impl Default for AsciiConverter {
    fn default() -> Self {
        Self::new_dense(80, 24)
    }
}
