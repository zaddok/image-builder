use std::{collections::HashMap, fs};

use image::{self, DynamicImage, ImageBuffer, Rgba};
use imageproc::{drawing::{draw_text_mut, draw_filled_rect, draw_filled_rect_mut}, rect::Rect};
use rusttype::{Font, Scale};

mod colors;
use colors::Colors;

pub struct Position {
  pub x: i32,
  pub y: i32,
}
pub struct Size {
  pub width: u32,
  pub height: u32,
}
pub struct Text<'a> {
  pub content: &'a str,
  pub size: u32,
  pub position: Position,
  pub custom_font: Option<&'a str>,
}
pub enum Color<'a> {
  Name(&'a str),
  Rgba([u8; 4]),
}

pub struct Image<'a> {
  image: ImageBuffer<Rgba<u8>, Vec<u8>>,
  default_font: Font<'a>,
  fonts: HashMap<&'a str, Font<'a>>,
  colors: Colors<'a, 'a>,
}
impl<'a> Image<'a> {
  pub fn new(width: u32, height: u32) -> Image<'a> {
    let image = DynamicImage::new_rgb16(width, height);
    let image = draw_filled_rect(
      &image,
      Rect::at(0, 0).of_size(width, height),
      image::Rgba([255u8, 255u8, 255u8, 255u8])
    );
    let default_font = Vec::from(include_bytes!("Roboto-Regular.ttf") as &[u8]);
    let default_font = Font::try_from_vec(default_font).unwrap();

    let fonts = HashMap::new();
    let colors = Colors::new();

    Image { image, fonts, default_font, colors }
  }

  pub fn add_custom_font(&mut self, name: &'a str, path: &'a str) {
    let custom_font: Vec<u8> = fs::read(path)
      .expect(&format!("It was not possible to meet the file in the path \"{}\"", path));
    self.fonts.insert(name, Font::try_from_vec(custom_font).unwrap());
  }

  pub fn print_text(&mut self, text: Text<'a>) {
    let scale = Scale { x: text.size as f32, y: text.size as f32 };
    let font = match text.custom_font {
      None => &self.default_font,
      Some(key) => match &self.fonts.get(key) {
        None => &self.default_font,
        Some(font) => font
      }
    };
    draw_text_mut(
      &mut self.image,
      image::Rgba([0u8, 0u8, 0u8, 255u8]),
      text.position.x, text.position.y, scale, font,
      text.content
    );
  }

  pub fn print_rect(&mut self, position: Position, size: Size, color: Color) {
    let color = match &color {
      Color::Rgba(color) => color,
      Color::Name(name) => {
        match self.colors.get(name) {
          Some(color) => color,
          None => panic!("The color \"{}\" is unknown", name),
        }
      }
    };
    let rect = Rect::at(position.x, position.y).of_size(size.width, size.height);
    draw_filled_rect_mut(&mut self.image, rect, Rgba(color.clone()));
  }

  pub fn save(&self, path: &str) {
    self.image.save(path).unwrap();
  }
}
