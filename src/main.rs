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
use hotswap::Hotswap;
use std::io::prelude::*;
use std::fs::File;
use position::PositionSystem;
use position::Vec2;
use rendering::PrimativeType;
use rendering::RenderType;
use rendering::Renderable;

#[derive(Copy,Clone)]
struct Vertex{
    position: [f32;2],
    tex_cords:[f32;2],
}



implement_vertex!(Vertex,position,tex_cords);

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


    use glium::DisplayBuild;
    use glium::Surface;
    use std::collections::HashMap;

    let display  = glium::glutin::WindowBuilder::new().with_dimensions(1600,900).build_glium().unwrap();

    let dim = display.get_window().unwrap().get_inner_size_pixels().unwrap();
    println!("{} {}",dim.0,dim.1);
    let texture_paths = vec!["100_100_test.data"];
    let program_paths = "program.data";
    let mut render_system = rendering::RenderSystem::new(dim.0,dim.1,&display,texture_paths,program_paths);
    let mut pos_system = PositionSystem::new(16,9,1.0);
    let square = Renderable::new(RenderType::Primative(PrimativeType::Square(10.0,10.0),rendering::Color::new(1.0,0.0,0.0,1.0)),pos_system.insert(Vec2::new(1.0,-1.0)),"geometry");
    let square2 = Renderable::new(RenderType::Primative(PrimativeType::Square(10.0,10.0),rendering::Color::new(0.0,1.0,0.0,1.0)),pos_system.insert(Vec2::new(-1.0,1.0)),"geometry");
    let square3 = Renderable::new(RenderType::Primative(PrimativeType::Square(10.0,10.0),rendering::Color::new(0.0,0.0,1.0,1.0)),pos_system.insert(Vec2::new(-1.0,-1.0)),"geometry");
    add_renderable!(render_system;pos_system;[square,square2]);
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
    loop{
        if dragging{
            let check = pos_system.get_bucket(render_system.pixels_to_world_cords_x(mouse_x),render_system.pixels_to_world_cords_y(mouse_y));
            for i in check{

            }
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
    }
}

