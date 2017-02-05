///Position system
use pool::{Pool,Handle};
use utils::*;
use utils::clamp;
use std::collections::HashSet;

basic_pub_all!{
    pub struct Vec4{
        pub x:f32,
        pub y:f32,
        pub half_width:f32,
        pub half_height:f32
    }
}

impl Vec4{
    pub fn origin()->Self{
        return Vec4{x:0.0,y:0.0,half_width:0.0,half_height:0.0};
    }
    pub fn aabb(v1:Vec4,v2:Vec4)->bool{
        return (v1.x - v2.x).abs() < (v1.half_width + v2.half_width) && (v1.y - v2.y).abs() < (v1.half_height + v2.half_height);
    }
    pub fn translate(&mut self,x:f32,y:f32){
        self.x += x;
        self.y += y;
    }
}


basic_pub_all_no_copy_no_eq!{
    pub struct Position{
        pub vec4:Vec4,
        pub locations:Vec<Vec<Option<Handle>>>,
        pub renderable_handle:Option<Handle>,
        pub physics_handle:Option<Handle>
    }
}



pub struct PositionSystem{
    pool:Pool<Position>,
    map:Vec<Vec<Pool<Handle>>>,
    cell_size:f32,
    width:usize,
    height:usize
}

pub fn get_over_all_pos(width:usize,height:usize,cell_size:f32,pos:Vec4,closure:&mut FnMut(usize,usize)){
    let width = width as i32;
    let height = height as i32;
    let top = clamp((((pos.y+pos.half_height) / cell_size).floor() + height as f32/2.0) as i32,0,height-1) as usize;
    let right = clamp((((pos.x+pos.half_width) / cell_size).floor() + width as f32/2.0) as i32,0,width-1) as usize;
    let bottom = clamp((((pos.y-pos.half_height) / cell_size).floor() + height as f32/2.0) as i32,0,height-1) as usize;
    let left = clamp((((pos.x-pos.half_width) / cell_size).floor() + width as f32/2.0) as i32,0,width-1) as usize;
    for y in bottom..top+1{
        for x in left..right+1{
            closure(x,y);
        }
    }
}
    impl PositionSystem{
    pub fn new(width:usize,height:usize,cell_size:f32)->Self{
        let mut map = Vec::with_capacity(height as usize);
        for i in 0..height{
            map.push(Vec::with_capacity(width as usize));
            for x in 0..width{
                map[i as usize].push(Pool::new());
            }
        }
        return PositionSystem{pool:Pool::new(),map:map,cell_size:cell_size,width:width,height:height};
    }

    pub fn insert(&mut self,pos:Vec4)->Handle{
        let mut result = self.pool.insert(Position::new(pos,Vec::new(),Option::None,Option::None));
        let mut vec = Vec::with_capacity(self.height);
        for y in 0..self.height{
            vec.push(Vec::with_capacity(self.width));
            for x in 0..self.width{
                vec[y].push(Option::None);
            }
        }
        get_over_all_pos(self.width,self.height,self.cell_size,pos,&mut |x,y|{
            vec[y][x] = Option::Some(self.map[y][x].insert(result));
        });
        {
            self.pool.get_mut(result).unwrap().locations(vec);
        }
        return result;
    }

    pub fn get_mut(&mut self,handle:Handle)->Result<&mut Position,&str>{
       return self.pool.get_mut(handle);
    }


    pub fn get(&self,handle:Handle)-> Result<&Position,&str> {
        return self.pool.get(handle);
    }

    pub fn get_bucket_from_id(&self,handle:Handle)->Result<Vec<Handle>,&str>{
        match self.pool.get(handle) {
            Result::Err(s) => {return Result::Err(s);},
            Result::Ok(pos) => {
                let mut collection = Vec::new();
                let mut set = HashSet::new();
                get_over_all_pos(self.width,self.height,self.cell_size,pos.vec4,&mut |x,y|{
                        for handle in &self.map[y][x].items{
                            if !set.contains(handle){
                                set.insert(*handle);
                                collection.push(*handle);
                            }
                        }
                });
                return Result::Ok(collection);
            }
        }
    }

    pub fn get_bucket(&mut self,x:f32,y:f32)->Vec<Handle>{
        let mut collection = Vec::new();
        let mut set = HashSet::new();
        get_over_all_pos(self.width,self.height,self.cell_size,Vec4::new(x,y,0.01,0.01),&mut |x,y|{
            for handle in &self.map[y][x].items{
                if !set.contains(handle){
                    set.insert(*handle);
                    collection.push(*handle);
                }
            }
        });
        return collection;
    }

    pub fn get_location(&mut self,x:f32,y:f32)->Vec<Handle>{
        let mut collection = Vec::new();
        let mut set = HashSet::new();
        let vec = Vec4::new(x,y,0.01,0.01);
        get_over_all_pos(self.width,self.height,self.cell_size,vec,&mut |x,y|{
            for handle in &self.map[y][x].items{
                if Vec4::aabb(self.pool.get(*handle).unwrap().vec4,vec){
                    if !set.contains(handle){
                        set.insert(*handle);
                        collection.push(*handle);
                    }
                }
            }
        });
        return collection
    }

    pub fn remove(&mut self,handle:Handle,render_system:&mut RenderSystem,physics_system:&mut PhysicsSystem)->Result<(),&str>{
        match self.pool.remove(handle){
            Result::Err(s) => {return Result::Err(s);},
            Result::Ok(position) => {
                let width = self.width as i32;
                let height = self.height as i32;
                let pos = position.vec4;
                match position.renderable_handle{
                    Option::Some(handle) =>{
                        render_system.remove_renderable(handle);
                    },
                    Option::None() => ();
                }
                match position.physics_handle{
                    Option::Some(handle) =>{
                        physics_system.remove_physics_object(handle);
                    },
                    Option::None() => ();
                }

                let top = clamp((((pos.y+pos.half_height) / self.cell_size).floor() + height as f32/2.0) as i32,0,height) as usize;
                let right = clamp((((pos.x+pos.half_width) / self.cell_size).floor() + width as f32/2.0) as i32,0,width) as usize;
                let bottom = clamp((((pos.y-pos.half_height) / self.cell_size).floor() + height as f32/2.0) as i32,0,height) as usize;
                let left = clamp((((pos.x-pos.half_width) / self.cell_size).floor() + width as f32/2.0) as i32,0,width) as usize;
                for y in bottom..top+1{
                    for x in left..right+1{
                        match position.locations[y][x]{
                            Option::Some(loc_handle) =>{
                                self.map[y][x].remove(loc_handle);
                            },
                            Option::None =>()
                        }
                    }
                }
                return Result::Ok(());
            }

        }
    }

    pub fn update(&mut self,handle:Handle)->Result<(),&str>{
        match self.pool.get_mut(handle) {
            Result::Err(s) => {return Result::Err(s);},
            Result::Ok(mut position) => {
                for y in 0..self.height{
                    for x in 0..self.width{
                        match position.locations[y][x]{
                            Option::Some(loc_handle) =>{
                                self.map[y][x].remove(loc_handle);
                            },
                            Option::None =>()
                        }
                    }
                }
                let width = self.width as i32;
                let height = self.height as i32;
                let pos = position.vec4;
                let top = clamp((((pos.y+pos.half_height) / self.cell_size).floor() + height as f32/2.0) as i32,0,height) as usize;
                let right = clamp((((pos.x+pos.half_width) / self.cell_size).floor() + width as f32/2.0) as i32,0,width) as usize;
                let bottom = clamp((((pos.y-pos.half_height) / self.cell_size).floor() + height as f32/2.0) as i32,0,height) as usize;
                let left = clamp((((pos.x-pos.half_width) / self.cell_size).floor() + width as f32/2.0) as i32,0,width) as usize;
                for y in bottom..top+1{
                    for x in left..right+1{
                            position.locations[y][x] = Option::Some(self.map[y][x].insert(handle));
                    }
                }
                return Result::Ok(());
            }
        }
    }

} 
