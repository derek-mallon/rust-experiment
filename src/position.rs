///Position system
use pool::{Pool,Handle};

#[derive(Copy,Clone,PartialEq)]
pub struct Vec2{
    pub x:f32,
    pub y:f32
}
impl Vec2{
    pub fn new(x:f32,y:f32)->Self{
        return Vec2{x:x,y:y};
    }
    pub fn origin()->Self{
        return Vec2{x:0.0 as f32,y:0.0 as f32};
    }
}
pub struct PositionSystem{
    pool:Pool<Vec2>,
    map:Vec<Vec<Vec<Handle>>>,
    cell_size:f32
}

impl PositionSystem{
    pub fn new(width:u32,height:u32,cell_size:f32)->Self{
        let mut map = Vec::with_capacity(height as usize);
        for i in 0..height{
            map.push(Vec::with_capacity(width as usize));
            for x in 0..width{
                map[i as usize].push(Vec::new());
            }
        }
        return PositionSystem{pool:Pool::new(),map:map,cell_size:cell_size};
    }
    pub fn insert(&mut self,pos:Vec2)->Handle{
        let mut result = self.pool.insert(pos);
        let row = (pos.y / self.cell_size).floor() as u32;
        let width = (pos.x / self.cell_size).floor() as u32;
        self.map[row as usize][width as usize].push(result);
        return result;
    }

    pub fn get(&self,handle:Handle)-> Result<&Vec2,&str> {
        return self.pool.get(handle);
    }
    pub fn get_bucket_from_id(&self,handle:Handle)->Result<&[Handle],&str>{
        match self.pool.get(handle) {
            Result::Err(s) => {return Result::Err(s);},
            Result::Ok(vec) => {
                let row = (vec.y / self.cell_size).floor() as u32;
                let width = (vec.x / self.cell_size).floor() as u32;
                return Result::Ok(self.map[row as usize][width as usize].as_slice());
            }
        }
    }

    pub fn get_bucket(&self,x:f32,y:f32)->&[Handle]{
        let row = (y / self.cell_size).floor() as u32;
        let width = (x / self.cell_size).floor() as u32;
        return self.map[row as usize][width as usize].as_slice();
    }

    pub fn remove(&mut self,handle:Handle)->Result<(),&str>{
        match self.pool.remove(handle){
            Result::Err(s) => {return Result::Err(s);},
            Result::Ok(vec) => {
                let row = (vec.y / self.cell_size).floor() as u32;
                let width = (vec.x / self.cell_size).floor() as u32;
                for i in 0..self.map[row as usize][width as usize].len(){
                    if self.map[row as usize][width as usize][i as usize] == handle{
                        self.map[row as usize][width as usize].swap_remove(i as usize);
                    }
                }
                return Result::Ok(());
            }

        }
    }
    
}
