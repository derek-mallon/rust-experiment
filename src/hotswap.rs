/// Allows for hotswaping in variables during run time with a file
/// Not for release only as a dev tool.

use std::time::SystemTime;
use std::io::prelude::*;
use std::fs::File;

pub struct Hotswap{
    path:String,
    updated:bool,
}

impl Hotswap{
    pub fn new(path:String)->Self{
        return Hotswap{path:path.clone(),updated:false};
    }

    pub fn check_update(&mut self){
        match File::open(self.path.as_str()){
            Result::Ok(file) => {
                let time = file.metadata().unwrap().modified().unwrap();
                let now = SystemTime::now();
                if now.duration_since(time).unwrap().as_secs() == 0 && self.updated{
                    self.updated = false;
                }
            },
            Result::Err(..) => {return;}
        }
    }
    pub fn reload(&mut self,closure:&mut FnMut(&mut File)){
        if self.updated == false{
            match File::open(self.path.as_str()){
                Result::Ok(mut file) => {
                    closure(&mut file);
                    self.updated == true;
                },
                Result::Err(..) => {return;}
            }
        }
    }
}
