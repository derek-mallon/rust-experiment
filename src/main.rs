#[macro_use]
extern crate glium;
mod pool;
mod hotswap;
mod image;
mod rendering;
mod position;
mod utils;
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

fn main() {


    let mut bytes =  Vec::new();
    File::open("test.data").unwrap().read_to_end(&mut bytes);
    let image = image::load_image(bytes.as_slice(),100,100);
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
    let square = Renderable::new(RenderType::Primative(PrimativeType::Square(10.0,10.0),rendering::Color::new(1.0,0.0,0.0,1.0)),pos_system.insert(Vec2::new(1.0,0.0)),"geometry");
    render_system.add_renderable(square);
    loop{
        render_system.render(&display,&pos_system);
        for ev in display.poll_events(){
            match ev{
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::MouseMoved(x,y) =>{
                    println!("{} {}",x,y);
                    println!("({} {})",render_system.pixels_to_gl_cords_x(x as u32),render_system.pixels_to_gl_cords_y(y as u32));
                }
                _=>()
            }
        }
    }
}

