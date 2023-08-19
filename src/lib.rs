use std::{vec, fmt::Display};

use image::{DynamicImage, ImageResult, GenericImageView, Pixel, imageops::FilterType};

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
            chars: if let Some(v) = chars { v } else { vec!['W', '@', '#', '8', '&', '*', 'o', ':', '.', ' '] },
        })
    }

    pub fn set_chars(&mut self, chars: Vec<char>) {
        self.chars = chars;
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.img.resize(width, height, FilterType::CatmullRom);
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
        let scale =  0.2126 * r as f64 + 0.7152 * g as f64 + 0.0722 * b as f64;
        (scale / ((255 / self.chars.len()) as f64 + 0.5)).floor() as usize
    }

}
