#[macro_use]
extern crate glium;
#[macro_use]
mod utils;

mod pool;
mod hotswap;
mod image;
#[macro_use]
mod position;
mod rendering;
mod time;
use hotswap::Hotswap;
use std::io::prelude::*;
use std::fs::File;
use position::PositionSystem;
use position::Vec4;
use rendering::PrimativeType;
use rendering::RenderType;
use rendering::Renderable;
use pool::Handle;
use std::mem::drop;
use std::ops::Drop;


struct Test{
    item:u32,
}

impl Drop for Test{
    fn drop(&mut self){
        println!("test");
    }
}




macro_rules! add_renderable{
    (
        $(
            $x:expr ; $z:expr ; [$($y:expr),*]
         );*
    ) =>{
    $($($x.add_renderable($y,&mut $z));*)*
    }
}
fn main() {

    {
        let test = Test{item:0};
    }

    use glium::DisplayBuild;
    use glium::Surface;
    use std::collections::HashMap;

    let display  = glium::glutin::WindowBuilder::new().with_dimensions(1600,900).build_glium().unwrap();

    let dim = display.get_window().unwrap().get_inner_size_pixels().unwrap();
    println!("{} {}",dim.0,dim.1);
    let texture_paths = vec!["100_100_test.data"];
    let program_paths = "program.data";
    let mut render_system = rendering::RenderSystem::new(dim.0,dim.1,&display,texture_paths,program_paths);
    let mut pos_system = PositionSystem::new(16,9,10.0);
    let pos_handle = pos_system.insert(Vec4::new(0.0,0.0,0.5,0.5));
    let render_handle = render_system.add_renderable(Renderable::new(RenderType::Primative(PrimativeType::Square(0.5,0.5),rendering::Color::new(0.0,1.0,0.0,1.0)),pos_handle,"geometry"));
    pos_system.get_mut(pos_handle).unwrap().renderable_handle(Option::Some(render_handle));
    let pos_handle = pos_system.insert(Vec4::new(4.0,2.0,0.5,0.5));
    let render_handle = render_system.add_renderable(Renderable::new(RenderType::Primative(PrimativeType::Square(0.5,0.5),rendering::Color::new(0.0,1.0,0.0,1.0)),pos_handle,"geometry"));
    pos_system.get_mut(pos_handle).unwrap().renderable_handle(Option::Some(render_handle));
    //let square2 = ;
    //let square = Renderable::new(RenderType::Primative(PrimativeType::Square(10.0,10.0),rendering::Color::new(1.0,0.0,0.0,1.0)),pos_system.insert(Vec2::new(1.0,-1.0)),"geometry");
    //add_renderable!(render_system;pos_system;[square2]);
    //let square4 = Renderable::new(RenderType::Primative(PrimativeType::Square(10.0,10.0),rendering::Color::new(0.0,0.0,0.0,1.0)),pos_system.insert(Vec2::new(0.0,1.0)),"geometry");
    //let square5 = Renderable::new(RenderType::Primative(PrimativeType::Square(10.0,10.0),rendering::Color::new(0.5,0.5,0.0,1.0)),pos_system.insert(Vec2::new(1.0,1.0)),"geometry");
    //let square6 = Renderable::new(RenderType::Primative(PrimativeType::Square(10.0,10.0),rendering::Color::new(0.0,0.5,0.5,1.0)),pos_system.insert(Vec2::new(-1.0,1.0)),"geometry");
    //render_system.add_renderable(square);
    //render_system.add_renderable(square2);
    //render_system.add_renderable(square3);
    /*
    render_system.add_renderable(square4);
    render_system.add_renderable(square5);
    render_system.add_renderable(square6);
    */
    let mut dragging = false;
    let mut mouse_x = 0;
    let mut mouse_y = 0;
    let mut time :u64 = 0;
    loop{
        println!("{} {}, {}",render_system.pixels_to_world_cords_x(mouse_x),render_system.pixels_to_world_cords_y(mouse_y),Vec4::aabb(Vec4::new(0.0,0.0,0.5,0.5),Vec4::new(render_system.pixels_to_world_cords_x(mouse_x),render_system.pixels_to_world_cords_y(mouse_y),0.01,0.01)));
        let mut found : Vec<Handle> = Vec::new();
        {
            if dragging{
                let check = {
                    pos_system.get_location(render_system.pixels_to_world_cords_x(mouse_x),render_system.pixels_to_world_cords_y(mouse_y))
                };
                for handle in check{
                    found.push(handle.clone());
                }
            }
        }
        for handle in found{
            {
                pos_system.get_mut(handle).unwrap().vec4.x(render_system.pixels_to_world_cords_x(mouse_x));
            }
            {
                pos_system.get_mut(handle).unwrap().vec4.y(render_system.pixels_to_world_cords_y(mouse_y));
            }
            {
                pos_system.update(handle);
            }
        }
        pos_system.get_mut(pos_handle).unwrap().vec4.translate(-0.01,0.0);
        {
            pos_system.update(pos_handle);
        }

        render_system.render(&display,&pos_system);
        for ev in display.poll_events(){
            match ev{
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::MouseInput(state,button) => {
                    match state {
                        glium::glutin::ElementState::Released =>{
                            dragging = false;
                        },
                        glium::glutin::ElementState::Pressed =>{
                            dragging = true;
                        }
                    }
                },
                glium::glutin::Event::MouseMoved(x,y) => {mouse_x = x as u32;mouse_y = y as u32;}
                _=>()
            }
        }
        time = time::do_time(time)
    }
}

