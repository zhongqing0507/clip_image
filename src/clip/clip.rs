
use std::error::Error;
use anyhow::Result;
use super::status::{ClipInfo, ImageInfo};

pub struct ClipManager{
    pub clip_info: ClipInfo,
    pub image_info: ImageInfo,
}


impl ClipManager {
    pub fn new() -> Self{
        let clip_info = ClipInfo::new();
        let image_info = ImageInfo::new(&clip_info.image.clone());

        Self { 
            clip_info, 
            image_info
        }
    }
    pub fn run(&mut self) -> Result<(), Box<dyn  Error>>{

        (self.image_info.padded_flag, self.image_info.padded_width, self.image_info.padded_height) = 
            self.image_info.check_padded(self.clip_info.tile_width, self.clip_info.tile_height);

        if self.image_info.padded_flag{
            self.image_info.padded_image();
        }

        self.image_info.clip_image(&self.clip_info)?;
        Ok(())
    }

}