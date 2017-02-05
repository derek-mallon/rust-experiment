use pool::Handle;
use pool::Pool;
use utils::*;
use position::PositionSystem;

basic_pub_all!{
    pub struct PhysicsObject{
        pub pos_handle:Handle,
        pub x_speed:f32,
        pub y_speed:f32,
    }
}


pub struct PhysicsSystem{
    physics_objects:Pool,
    time_step:f32,
}

impl PhysicsSystem{
    pub fn new(){
        PhysicsSystem{physics_objects:Pool,time_step:1.0};
    }

    pub fn update(&self,pos_system:&mut PositionSystem){
        for object in &self.physics_objects.items{
            pos_system.get_mut(object.pos_handle).unwrap().vec4.translate(object.x_speed,object.y_speed)
        }
    }

    pub fn add_physics_object(&mut self,physics_object:PhysicsObject)->Handle{
        return self.physics_objects.insert(physics_object);
    }

    pub fn remove_physics_object(&mut self,handle:Handle){
        self.physics_objects.remove(handle).unwrap();
    }
}
