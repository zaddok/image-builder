use std::fs;
use std::io::Cursor;

use image::io::Reader as ImageReader;
use image::DynamicImage;
use image_builder::{colors, FilterType, Image, Picture, Rect, Text};

fn main() {
    let width = 600;
    let height = 280;
    let mut image = Image::new(width, height, colors::GRAY);

    let roboto_bold = fs::read("fonts/Roboto/Roboto-Bold.ttf").unwrap();
    image.add_custom_font("Roboto bold", roboto_bold);

    image.add_rect(
        Rect::new()
            .size(width - 10, height - 10)
            .position(5, 5)
            .color(colors::PURPLE),
    );

    image.add_rect(
        Rect::new()
            .size(width - 30, height - 30)
            .position(15, 15)
            .color(colors::GRAY),
    );

    let logo = fs::read("logo.png").unwrap();
    let img: DynamicImage = ImageReader::new(Cursor::new(logo))
        .with_guessed_format()
        .expect("jpg or png")
        .decode()
        .unwrap();
    image.add_picture(
        Picture::new(img)
            .resize(134, 83, FilterType::Triangle)
            .crop(41, 143, 536, 332)
            .position(233, 30),
    );

    image.add_text(
        Text::new("Image Builder")
            .size(90)
            .font("Roboto bold")
            .position(60, 125)
            .color([0, 0, 0, 100]),
    );

    image.add_text(
        Text::new("Image Builder")
            .size(90)
            .font("Roboto bold")
            .position(55, 120)
            .color(colors::PURPLE),
    );

    image.add_text(
        Text::new("This image was built using this library.")
            .size(30)
            .position(80, 220)
            .color(colors::ORANGE),
    );

    image.save("example.png");
}
