
use std::path;
use image::RgbImage;

#[derive(Debug)]
pub struct Identicon {
    username: String,
    img: RgbImage,
}

impl Identicon {
    pub fn new(username: String, img: RgbImage) -> Self {
        Self { img, username }
    }

    pub fn save(&self) -> String {
        let filename = format!("{}.png", self.username);

        let file_path = path::Path::new(&filename);

        self.img
            .save_with_format(path::Path::new(&filename), image::ImageFormat::Png)
            .unwrap();

        file_path.to_str().unwrap().to_owned()
    }
}
