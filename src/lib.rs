use std::{fmt::Display, vec};

use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageResult, Pixel};

pub struct TextualImage {
    img: DynamicImage,
    chars: Vec<char>,
}

#[derive(Debug)]
pub struct Result(Vec<char>);

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tmp = String::new();

        for c in self.0.iter() {
            tmp.push(*c);
        }

        write!(f, "{}", tmp)
    }
}

impl TextualImage {
    pub fn new(filename: &str, chars: Option<Vec<char>>) -> ImageResult<TextualImage> {
        let img = image::open(filename)?;

        Ok(TextualImage {
            img: img,
            chars: if let Some(v) = chars {
                v
            } else {
                vec!['W', '@', '#', '8', '&', '*', 'o', ':', '.', ' ']
            },
        })
    }

    pub fn set_chars(&mut self, chars: Vec<char>) {
        self.chars = chars;
    }

    pub fn resize(&mut self, width: u32) {
        let (ori_width, ori_height) = self.img.dimensions();
        let target_height = self.get_target_height(ori_width, ori_height, width);
        self.img = self.img
            .resize(width, target_height, FilterType::CatmullRom);
    }

    fn get_target_height(&self, ori_width: u32, ori_height: u32, target_width: u32) -> u32 {
        let mut target_height = ori_height;
        if target_width < ori_width {
            target_height =
                (target_height as f64 / (ori_width as f64 / target_width as f64)).round() as u32;
        }
        target_height
    }

    pub fn display(&self) -> Result {
        let mut ouput = Vec::new();

        let (width, height) = self.img.dimensions();
        for i in 0..height {
            for j in 0..width {
                let piexl = self.img.get_pixel(j, i);
                let channels = piexl.channels();
                ouput.push(self.chars[self.calculate_index(channels[0], channels[1], channels[2])]);
            }
            ouput.push('\n');
        }

        Result(ouput)
    }

    fn calculate_index(&self, r: u8, g: u8, b: u8) -> usize {
        let scale = 0.2126 * r as f64 + 0.7152 * g as f64 + 0.0722 * b as f64;
        (scale / ((255 / self.chars.len()) as f64 + 0.5)).floor() as usize
    }
}
