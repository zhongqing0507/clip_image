use clap::Parser;
use std::{fs, path::{Path, PathBuf}};

#[derive(Parser, Debug)]
#[clap(name = "clip image")]
pub struct Args{

    // #[clap(short, long)]
    pub image: String,

    #[clap(short('O'), long)]
    pub output: Option<String>,

    #[clap(short('W'), long, default_value_t = 32)]
    pub tile_width: usize,


    #[clap(short('H'), long, default_value_t = 32)]
    pub tile_height: usize,

}

impl Args{

    pub fn save_path(&self) -> PathBuf{
        match &self.output{
            Some(output) => {
                let output = Path::new(output);
                fs::create_dir_all(output)
                    .map_err(|err| {
                        eprintln!("Falied to create output directory, {}", err);
                        std::process::exit(1);
                    }).unwrap();
                output.to_path_buf()
            },
            None => {
                Path::new(&format!("clip_{}",self.image_file_stem())).to_path_buf()
            }
        }
    }


    pub fn image_filename(&self) -> &str{
        let file = Path::new(&self.image);
        let filename = file.file_name()
            .and_then(|os_str| os_str.to_str())
            .ok_or_else(|| {
                eprintln!("please give me a image file");
                std::process::exit(1);
            }).unwrap();
        filename
    }

    pub fn image_file_stem(&self) -> &str{

        let file = Path::new(&self.image);
        let file_stem = file.file_stem()
            .and_then(|stem_osstr| stem_osstr.to_str())
            .unwrap();

        file_stem
    }

}