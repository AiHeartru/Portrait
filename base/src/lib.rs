mod portrait;

use std::fs::create_dir;
use std::path::Path;
use std::error::Error;
use std::time::Duration;
use portrait::*;

pub struct Base ();

impl Base {
    pub fn gen_portrait() -> String {
        let portrait = Portrait::new();
        portrait.gen_portrait([35u8, 37u8, 40u8])
    }

    // 检查是否存在文件夹，否则创建
    pub fn create_folder_if_not_exist() {
        let path = "static";
        let p = Path::new(path);
        if !p.exists() {
            println!("Creating a directory...");
            create_dir(path).unwrap();
        }
    }

    // 清空文件夹
    pub fn clear_folder() {
        let path = "static";
        let p = Path::new(path);
        if p.exists() {
            println!("Clearing a directory...");
            std::fs::remove_dir_all(path).unwrap();
        }
    }

    // 上一次的生成时间
    pub fn last_file_time() -> Result<Duration, Box<dyn Error>> {
        let path = "static";
        let p = Path::new(path);
        let t: Duration;
        t = if p.exists() {
            println!("Checking the latest file time...");
            let mut files = std::fs::read_dir(path)?;
            let file = files.next().expect("no file")?;
            let meta = file.metadata()?;
            let time = meta.modified()?;
            println!("Time spent: {:?}", time.elapsed()?);
            time.elapsed()?
        } else {
            println!("Creating a directory...");
            create_dir(path).unwrap();
            Duration::new(0, 0)
        };
        Ok(t)
    }
}