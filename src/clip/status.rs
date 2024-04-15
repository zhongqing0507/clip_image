use std::error::Error;
use std::path::PathBuf;
use std::process;

use clap::Parser;
use image::{DynamicImage, ImageBuffer, Pixel, Rgb};
use image::{GenericImage,GenericImageView};
use once_cell::sync::Lazy;

use crate::cli::cli::Args;

use super::tools::create_bar;



pub static ARGS: Lazy<Args> = Lazy::new(|| {
    Args::parse()
});


#[derive(Debug,Clone)]
pub struct ClipInfo{
    pub image: String,
    pub file_name: String,
    pub file_stem: String,
    pub output: PathBuf,
    pub tile_width: usize,
    pub tile_height: usize
}

impl ClipInfo{
    pub fn new() -> Self{
        
        // ARGS.init().unwrap();

        Self { 
            image: ARGS.image.clone(), 
            file_name: ARGS.image_filename().to_string(), 
            file_stem: ARGS.image_file_stem().to_string(), 
            output: ARGS.save_path(), 
            tile_width: ARGS.tile_width, 
            tile_height: ARGS.tile_height,

        }
    }

}

#[derive(Debug,Clone)]
pub struct ImageInfo{
    pub image: DynamicImage,
    pub width: usize,
    pub height: usize,
    pub padded_flag: bool,
    pub padded_width: usize,
    pub padded_height: usize,
}


impl ImageInfo {
    

    pub fn new(image: &str) -> Self{

        let image = image::open(image).unwrap();

        ImageInfo{
            image: image.clone(),
            width: image.width() as usize,
            height: image.height() as usize,
            padded_flag: false,
            padded_width: 0,
            padded_height: 0,
        }
    }

    pub fn check_padded(&self, tile_width: usize, tile_height: usize) -> (bool, usize, usize) {

        if self.width % tile_width == 0 && self.height % tile_height == 0 {
            return (false, 0, 0);
        }else {
            let padded_width = (tile_width - (self.width % tile_width)) % tile_width;
            let padded_height = (tile_height- (self.height % tile_height)) % tile_height;
            return (true, padded_width, padded_height);
        }
    }


    pub fn padded_image(&mut self){
        if self.padded_flag {
            let width = self.width + self.padded_width;
            let height = self.height + self.padded_height;
            let mut padded_image:ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_pixel(width as u32, height as u32, Rgb([0,0,0]));
            image::imageops::overlay(&mut padded_image, &self.image.to_rgb8(), 0, 0);
            self.image =   padded_image.into();
            self.height = height;
            self.width = width;
            
        }
    }

    pub fn clip_image(&mut self, clip_info: &ClipInfo) -> Result<(), Box<dyn Error>>{

        let (tile_width, tile_height) = (clip_info.tile_width as u32, clip_info.tile_height as u32);
        let output_path = &clip_info.output;
        
        let length = (self.width as u64 / tile_width as u64) * (self.height as u64 / tile_height as u64);
        let bar = create_bar(length);
        bar.set_message("clipping image");
        for y in (0..self.height as u32 - tile_height + 1).step_by(tile_height as usize){
            for x in (0..self.width as u32 - tile_width + 1).step_by(tile_width as usize){
                // self.image.
                // 使用sub_image 获取裁剪区域
                let window = self.image.sub_image(x, y, tile_width, tile_height);
                let mut cropped_image = ImageBuffer::<Rgb<u8>, Vec<_>>::new(tile_width, tile_height);
                for (dx, dy, pixel) in window.pixels(){
                    cropped_image.put_pixel(dx, dy, pixel.to_rgb());
                }
                
                let output_filename = format!("{}_{:02}_{:02}.png", clip_info.file_stem, y/tile_height, x/tile_width);
    
                let output_path = output_path.join(output_filename);
                cropped_image.save(output_path.as_path()).map_err(|err| {
                    eprintln!("save image error: {}", err);
                    process::exit(1);
                }).unwrap();
                
                bar.inc(1);
            }
        }
        bar.finish_with_message("clipped");
        Ok(())
    }

}