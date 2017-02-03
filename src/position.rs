///Position system
use pool::{Pool,Handle};
use utils::*;
use utils::clamp;

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

basic_pub_all!{
    pub struct PositionInner{
        pub render_handle:Option<Handle<u32>>
    }
}




pub type PositionHandle = Handle<PositionInner>;

pub struct PositionSystem{
    pool:Pool<Vec2,PositionInner>,
    map:Vec<Vec<Vec<PositionHandle>>>,
    cell_size:f32,
    width:f32,
    height:f32
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
        return PositionSystem{pool:Pool::new(),map:map,cell_size:cell_size,width:width as f32,height:height as f32};
    }

    pub fn insert(&mut self,pos:Vec2)->PositionHandle{
        let mut result = self.pool.insert(pos,PositionInner::new(Option::None));
        let row = clamp((pos.y / self.cell_size).floor() + self.height/2.0,0.0,self.height) as u32;
        let width = clamp((pos.x / self.cell_size).floor() + self.width/2.0,0.0,self.width) as u32;
        self.map[row as usize][width as usize].push(result);
        return result;
    }

    pub fn get_mut(&mut self,handle:PositionHandle)->Result<&mut Vec2,&str>{
       return self.pool.get_mut(handle);
    }

    pub fn get(&self,handle:PositionHandle)-> Result<&Vec2,&str> {
        return self.pool.get(handle);
    }

    pub fn get_bucket_from_id(&self,handle:PositionHandle)->Result<&[PositionHandle],&str>{
        match self.pool.get(handle) {
            Result::Err(s) => {return Result::Err(s);},
            Result::Ok(pos) => {
                let row = clamp((pos.y / self.cell_size).floor() + self.height/2.0,0.0,self.height) as u32;
                let width = clamp((pos.x / self.cell_size).floor() + self.width/2.0,0.0,self.width) as u32;
                return Result::Ok(self.map[row as usize][width as usize].as_slice());
            }
        }
    }

    pub fn get_bucket(&self,x:f32,y:f32)->&[PositionHandle]{
        let row = clamp((y / self.cell_size).floor() + self.height/2.0,0.0,self.height) as u32;
        let width = clamp((x / self.cell_size).floor() + self.width/2.0,0.0,self.width) as u32;
        return self.map[row as usize][width as usize].as_slice();
    }

    pub fn remove(&mut self,handle:PositionHandle)->Result<(),&str>{
        match self.pool.remove(handle){
            Result::Err(s) => {return Result::Err(s);},
            Result::Ok(pos) => {
                let row = clamp((pos.y / self.cell_size).floor() + self.height/2.0,0.0,self.height) as u32;
                let width = clamp((pos.x / self.cell_size).floor() + self.width/2.0,0.0,self.width) as u32;
                for i in 0..self.map[row as usize][width as usize].len(){
                    if self.map[row as usize][width as usize][i as usize] == handle{
                        self.map[row as usize][width as usize].swap_remove(i as usize);
                    }
                }
                return Result::Ok(());
            }

        }
    }

    //Warning! make sure the handle you use for this is up to date because it will overwrite the
    //old inner if you dont, use the marco
    //version of this.
    pub fn update_handle(&mut self,handle:PositionHandle,render_handle:Option<Handle<u32>>)->Result<(),&str>{
        let mut updated_handle = handle;
        let mut inner = updated_handle.inner;
        match render_handle {
            Some(handle)=>{
                inner.render_handle(Option::Some(handle));
            },
            None =>()
        }
        updated_handle.inner(inner);
        match self.pool.update_handle(updated_handle){
            Result::Err(s) => {return Result::Err(s);},
            Result::Ok(..) => {return Result::Ok(());}
        }
    }
    pub fn get_handle(&self,handle:PositionHandle)->Result<PositionHandle,&str>{
        return self.pool.get_handle(handle);
    }
}

macro_rules! update_handle{
    ($pos_system:expr , $handle:expr, $render_handle:expr ) =>{
            let handle_copy = $handle;
            let render_handle_copy = $render_handle;
            let mut new_handle = $pos_system.get_handle(handle_copy).unwrap();
            $pos_system.update_handle(new_handle,render_handle_copy);
    }
}
