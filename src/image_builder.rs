use image::{ImageBuffer, RgbImage};
use sha2::{Digest, Sha512};
use std::vec;

use crate::identicon::Identicon;

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
}

#[derive(Debug)]
pub struct ImageBuilder {
    pub username: String,
    hash: Option<Vec<u8>>,
    color: Option<Vec<u8>>,
    grid: Option<Vec<u8>>,
    pixel_map: Option<Vec<(Point, Point)>>,
    img: Option<RgbImage>
}

const SQUARES: u8 = 10;
const SIZE: u8 = 20;

impl ImageBuilder {
    pub fn new(username: String) -> Self {
        Self {
            username,
            hash: None,
            color: None,
            grid: None,
            pixel_map: None,
            img: None,
        }
    }

    pub fn hash_username(&self) -> Self {
        let mut sha512 = Sha512::default();

        sha512.update(self.username.to_owned());
        let hash = sha512.finalize();

        Self {
            username: self.username.clone(),
            hash: Some(hash.to_vec()),
            color: self.color.clone(),
            grid: None,
            pixel_map: None,
            img: None,
        }
    }

    pub fn pick_color(&self) -> Self {
        let hash = self.hash.clone().unwrap();

        let length = hash.len();

        let r = hash[length - 3];
        let g = hash[length - 2];
        let b = hash[length - 1];

        Self {
            username: self.username.clone(),
            hash: self.hash.clone(),
            color: Some(vec![r, g, b]),
            grid: None,
            pixel_map: None,
            img: None,
        }
    }

    pub fn set_grid(&self) -> Self {
        let hash = self.hash.clone().unwrap();

        let length = SQUARES / 2;

        let mut grid: Vec<u8> = Vec::new();

        for i in 0..length {
            grid.push(hash[i as usize]);
        }

        let mut result: Vec<u8> = vec![];
        result.append(&mut grid.clone());
        result.append(&mut grid.clone());

        for index in 0..length {
            let reverse_index: usize = (length - index - 1).into();
            result[reverse_index] = grid[index as usize];
        }

        Self {
            username: self.username.clone(),
            hash: self.hash.clone(),
            color: self.color.clone(),
            grid: Some(result),
            pixel_map: None,
            img: None,
        }
    }

    pub fn filter_odd_squares(&self) -> Self {
        let filtered_grid = self
            .grid
            .clone()
            .unwrap()
            .iter()
            .filter(|item| {
                return **item % 2 == 0;
            })
            .map(|x| {
                return *x;
            })
            .collect::<Vec<u8>>();

        Self {
            username: self.username.clone(),
            hash: self.hash.clone(),
            color: self.color.clone(),
            grid: Some(filtered_grid),
            pixel_map: None,
            img: None,
        }
    }

    pub fn build_pixel_map(&self) -> Self {
        let grid = self.grid.clone().unwrap();
        let mut pixel_map: Vec<(Point, Point)> = Vec::new();
        let square_size = (SIZE as f32 / SQUARES as f32).round() as u8;

        for i in 0..grid.len() {
            let horizontal = (i as u8 % SQUARES) * square_size;
            let vertical = (i as u8 / SQUARES) * square_size;
            let top_left = Point {
                x: horizontal,
                y: vertical,
            };
            let bottom_right = Point {
                x: horizontal + square_size,
                y: vertical + square_size,
            };
            pixel_map.push((top_left, bottom_right));
        }

        Self {
            username: self.username.clone(),
            hash: self.hash.clone(),
            color: self.color.clone(),
            grid: self.grid.clone(),
            pixel_map: Some(pixel_map),
            img: None,
        }
    }

    pub fn draw_image(&self) -> Self {
        let mut img: RgbImage = ImageBuffer::new(SIZE as u32, SIZE as u32);
        let color = self.color.clone().unwrap();

        self.pixel_map.as_ref().unwrap().iter().for_each(|(x, y)| {
            let r = color[0];
            let g = color[1];
            let b = color[2];
            img.put_pixel(x.x as u32, x.y as u32, image::Rgb([r, g, b]));
            img.put_pixel(y.x as u32, y.y as u32, image::Rgb([r, g, b]));
        });

        Self {
            username: self.username.clone(),
            hash: self.hash.clone(),
            color: self.color.clone(),
            grid: self.grid.clone(),
            pixel_map: None,
            img: Some(img.clone()),
        }
    }

    pub fn build(&self) -> Identicon {
        Identicon::new(self.username.to_owned(), self.img.clone().unwrap().clone())
    }
}
