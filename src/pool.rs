///A pool for storing objects

#[derive(Copy,Clone,PartialEq)]
pub struct Handle<I:Copy + Clone + PartialEq>{
    self_index:usize,
    item_index:usize,
    age:usize,
    pub inner:I
}
impl<I:Copy+Clone+PartialEq> Handle<I>{
    pub fn inner(&mut self,inner:I){
        self.inner = inner;
    }
}
pub struct Pool<T,I:Copy + Clone + PartialEq>{
    pub items:Vec<T>,
    handle_index:Vec<usize>,
    handle:Vec<Handle<I>>,
    age:usize
}

impl<T,I:Copy + Clone + PartialEq> Pool<T,I>{
    pub fn new()->Self{
        return Pool::<T,I>{items:Vec::new(),handle_index:Vec::new(),handle:Vec::new(),age:0};
    }
    pub fn insert(&mut self,item:T,inner:I)->Handle<I>{
        self.handle_index.push(self.handle.len());
        let self_index = self.handle.len();
        self.handle.push(Handle{self_index:self_index,item_index:self.items.len(),age:self.age,inner:inner});
        self.items.push(item);
        self.age += 1;
        return self.handle[self.handle.len()-1];
    }

    pub fn get_mut(&mut self,handle:Handle<I>)-> Result<&mut T,&str> {
        if  handle.self_index >= self.handle.len(){
            return Result::Err("Handle does not exist!");
        }
        if self.handle[handle.self_index].age != handle.age{
            return Result::Err("Handle invalid");
        }
        return Result::Ok(&mut self.items[self.handle[handle.self_index].item_index]);
    }
    pub fn get(&self,handle:Handle<I>)-> Result<&T,&str>{
        if  handle.self_index >= self.handle.len(){
            return Result::Err("Handle does not exist!");
        }
        if self.handle[handle.self_index].age != handle.age{
            return Result::Err("Handle invalid");
        }
        return Result::Ok(&self.items[self.handle[handle.self_index].item_index]);
    }

    pub fn remove(&mut self,handle:Handle<I>)->Result<T,&str>{
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

    pub fn update_handle(&mut self,handle:Handle<I>)->Result<(),&str>{
        if  handle.self_index >= self.handle.len(){
            return Result::Err("Handle does not exist!");
        }
        if self.handle[handle.self_index].age != handle.age{
            return Result::Err("Handle invalid");
        }
        self.handle[handle.self_index].inner = handle.inner;
        return Result::Ok(());
    }
    pub fn check_handle(&self,handle:Handle<I>)->Result<(),&str>{
        if  handle.self_index >= self.handle.len(){
            return Result::Err("Handle does not exist!");
        }
        if self.handle[handle.self_index].age != handle.age{
            return Result::Err("Handle invalid");
        }
        return Result::Ok(());
    }
    pub fn get_handle(&self,handle:Handle<I>)->Result<Handle<I>,&str>{
        if  handle.self_index >= self.handle.len(){
            return Result::Err("Handle does not exist!");
        }
        if self.handle[handle.self_index].age != handle.age{
            return Result::Err("Handle invalid");
        }
        return Result::Ok(self.handle[handle.self_index]);
    }
}


#[test]
pub fn test_pool(){
    let mut pool : Pool<u32,u32> = Pool::new();
    pool.insert(1432432,0);
    pool.insert(1432432,0);
    pool.insert(1432432,0);
    pool.insert(1432432,0);
    pool.insert(1432432,0);
    let handle = pool.insert(10);
    pool.insert(1432432,0);
    pool.insert(1432432,0);
    pool.insert(1432432,0);
    let handle2 = pool.insert(20);
    pool.remove(handle);

    assert!(*pool.get(handle).unwrap() == *pool.get(handle2).unwrap());
}
