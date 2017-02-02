use std::io::prelude::*;
use std::fs::File;
use std::mem;
use std::marker::Copy;
pub struct Image{
    pub bytes:Vec<(u8,u8,u8,u8)>,
    pub dim:(u32,u32)
}

impl Image{
    pub fn new(bytes:Vec<(u8,u8,u8,u8)>,width:u32,height:u32)->Self{
        return Image{bytes:bytes,dim:(width,height)};
    }
    pub fn blank()->Self{
        return Image{bytes:Vec::new(),dim:(0,0)};
    }
}

impl Clone for Image{
    fn clone(&self)->Self{
        let mut bytes = Vec::new();
        for x in &self.bytes{
            bytes.push(*x);
        }
        return Image{bytes:bytes,dim:self.dim};
    }
}

pub fn load_image(bytes:&[u8],width:u32,height:u32)->Image{
    if bytes.len() % 3 == 0{
        let size = width*height*3;
        let mut image = Vec::with_capacity(size as usize);
        for i in 0..size{
            if i % 3  == 0{
                image.push((bytes[i as usize ],bytes[i as usize +1 ],bytes[i as usize +2 ],255 as u8));
            }
        }
        return Image::new(image,width,height);
    }else{
        let size = width*height*4;
        let mut image = Vec::with_capacity(size as usize);
        for i in 0..size{
            if i % 4  == 0{
                image.push((bytes[i as usize ],bytes[i as usize +1 ],bytes[i as usize +2 ],bytes[i as usize +3  ]));
            }
        }
        return Image::new(image,width,height);
    }
}

///Panics if the given bytes are to the right size.
pub fn reload_image(bytes:&[u8],image:&mut Image){
    if bytes.len() % 3 == 0{
        let size = image.dim.0*image.dim.1*3;
        if bytes.len() != size as usize{
            panic!("Image is not the right size");
        }
        image.bytes.clear();
        for i in 0..size{
            if i % 3  == 0{
                image.bytes.push((bytes[i as usize ],bytes[i as usize +1 ],bytes[i as usize +2 ],255 as u8));
            }
        }
    }else{
        let size = image.dim.0*image.dim.1*4;
        if bytes.len() != size as usize{
            panic!("Image is not the right size");
        }
        image.bytes.clear();
        for i in 0..size{
            if i % 4  == 0{
                image.bytes.push((bytes[i as usize ],bytes[i as usize +1 ],bytes[i as usize +2 ],bytes[i as usize +3  ]));
            }
        }
    }
}
