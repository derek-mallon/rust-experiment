///A pool for storing objects

#[derive(Copy,Clone,Hash,Eq,PartialEq)]
pub struct Handle{
    self_index:usize,
    item_index:usize,
    age:usize,
}

pub struct Pool<T>{
    pub items:Vec<T>,
    handle_index:Vec<usize>,
    handle:Vec<Handle>,
    age:usize
}

impl<T> Pool<T>{
    pub fn new()->Self{
        return Pool::<T>{items:Vec::new(),handle_index:Vec::new(),handle:Vec::new(),age:0};
    }

    pub fn insert(&mut self,item:T)->Handle{
        self.handle_index.push(self.handle.len());
        let self_index = self.handle.len();
        self.handle.push(Handle{self_index:self_index,item_index:self.items.len(),age:self.age});
        self.items.push(item);
        self.age += 1;
        return self.handle[self.handle.len()-1];
    }

    pub fn get_mut(&mut self,handle:Handle)-> Result<&mut T,&str> {
        if  handle.self_index >= self.handle.len(){
            return Result::Err("Handle does not exist!");
        }
        if self.handle[handle.self_index].age != handle.age{
            return Result::Err("Handle invalid");
        }
        return Result::Ok(&mut self.items[self.handle[handle.self_index].item_index]);
    }
    pub fn get(&self,handle:Handle)-> Result<&T,&str>{
        if  handle.self_index >= self.handle.len(){
            return Result::Err("Handle does not exist!");
        }
        if self.handle[handle.self_index].age != handle.age{
            return Result::Err("Handle invalid");
        }
        return Result::Ok(&self.items[self.handle[handle.self_index].item_index]);
    }

    pub fn remove(&mut self,handle:Handle)->Result<T,&str>{
        if  handle.self_index >= self.handle.len(){
            return Result::Err("Handle does not exist!");
        }
        if self.handle[handle.self_index].age != handle.age{
            return Result::Err("Handle invalid");
        }

        let handle_index = self.handle_index.pop().unwrap();
        self.handle[handle_index].item_index = self.handle[handle.self_index].item_index;
        return Result::Ok(self.items.swap_remove(self.handle[handle.self_index].item_index));
    }

    pub fn check_handle(&self,handle:Handle)->Result<(),&str>{
        if  handle.self_index >= self.handle.len(){
            return Result::Err("Handle does not exist!");
        }
        if self.handle[handle.self_index].age != handle.age{
            return Result::Err("Handle invalid");
        }
        return Result::Ok(());
    }
    pub fn update_items(&mut self,closure:&mut FnMut(&mut T)){
        for mut item in &mut self.items{
            closure(&mut item);
        }
    }
    pub fn read_all_items(&self,closure:&Fn(&T)){
        for item in &self.items{
            closure(&item);
        }
    }
}


#[test]
pub fn test_pool(){
    let mut pool : Pool<u32,u32> = Pool::new();
    pool.insert(1432432);
    pool.insert(1432432);
    pool.insert(1432432);
    pool.insert(1432432);
    pool.insert(1432432);
    let handle = pool.insert(10);
    pool.insert(1432432);
    pool.insert(1432432);
    pool.insert(1432432);
    let handle2 = pool.insert(20);
    *pool.get_mut(handle2).unwrap() = 1;
    println!("{}",pool.get(handle2).unwrap());
    pool.remove(handle);

    assert!(*pool.get(handle).unwrap() == *pool.get(handle2).unwrap());
}
