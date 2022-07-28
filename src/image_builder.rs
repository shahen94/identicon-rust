use std::vec;
use sha2::{Digest, Sha512};

use crate::identicon::Identicon;

#[derive(Debug)]
pub struct ImageBuilder {
    pub username: String,
    hash: Option<Vec<u8>>,
    color: Option<Vec<u8>>,
    grid: Option<Vec<u8>>
}

impl ImageBuilder {
    pub fn new(username: String) -> Self {
        Self {
            username,
            hash: None,
            color: None,
            grid: None,
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
      }
    }

    pub fn set_grid(&self) -> Self {
      let hash = self.hash.clone().unwrap();

      let squares: u8 = 10;
      let length = squares / 2;

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

      result = result.iter().filter(|item| {
        return **item % 2 == 0;
      }).collect();
      
      Self {
        username: self.username.clone(),
        hash: self.hash.clone(),
        color: self.color.clone(),
        grid: Some(result),
      }
    }

    pub fn filter_odd_squares(&self) -> Self {
      todo!()
    }

    pub fn build_pixel_map(&self) -> Self {
      todo!()
    }

    pub fn draw_image(&self) -> Self {
      todo!()
    }

    pub fn build(&self) -> Identicon {
      todo!()
    }
}
