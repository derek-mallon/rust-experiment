extern crate glium;
use image::*;
use std::collections::HashMap;
use hotswap::Hotswap;
use std::io::prelude::*;
use std::fs::File;
use glium::Surface;
use position::PositionSystem;
use pool::Pool;
use pool::Handle;
use utils::clamp;

#[derive(Copy,Clone)]
struct Vertex{
    position: [f32;2],
    tex_cords:[f32;2],

}

implement_vertex!(Vertex,position,tex_cords);

#[derive(Copy,Clone)]
struct VertexPos{
    position: [f32;2],
}

implement_vertex!(VertexPos,position);

#[derive(Copy,Clone)]
pub enum PrimativeType{
    Square(f32,f32),
    Rectangle(f32,f32,f32,f32),
    Circle(f32),
    Disk(f32),
    Line(f32),
    Triangle(f32,f32),
}

#[derive(Copy,Clone)]
pub struct Color{
    pub r:f32,
    pub g:f32,
    pub b:f32,
    pub a:f32
}

impl Color{
    pub fn new(r:f32,g:f32,b:f32,a:f32)->Self{
        return Color{r:r,g:g,b:b,a:a};
    }
}

struct Frame{
    dim:(u32,u32,u32,u32),
    texture_index:usize,
    va:Vec<Vertex>
}



impl Frame{
    //dim (x,y,x+width,y+height) i.e bottom left and top right.
    pub fn new(texture_index:usize,dim:(u32,u32,u32,u32),render_system:&RenderSystem)->Self{
        
        let dim_gl = (render_system.pixels_to_gl_cords_x(dim.0),render_system.pixels_to_gl_cords_y(dim.1),render_system.pixels_to_gl_cords_x(dim.2),render_system.pixels_to_gl_cords_y(dim.3));

        let width = dim_gl.2-dim_gl.0;
        let height = dim_gl.3-dim_gl.1;

        let vertex1 = Vertex { position:[-width/2.0,height/2.0],tex_cords:[dim_gl.0,dim_gl.3]}; // Top-left
        let vertex2 = Vertex { position:[width/2.0,height/2.0],tex_cords:[dim_gl.2,dim_gl.3]};  //Top right
        let vertex3 = Vertex { position:[width/2.0,-height/2.0],tex_cords:[dim_gl.2,dim_gl.1]}; //Bottom right
        let vertex4 = Vertex { position:[width/2.0,-height/2.0],tex_cords:[dim_gl.2,dim_gl.1]}; //Bottom right
        let vertex5 = Vertex { position:[-width/2.0,-height/2.0],tex_cords:[dim_gl.0,dim_gl.1]}; //Bottom left
        let vertex6 = Vertex { position:[-width/2.0,height/2.0],tex_cords:[dim_gl.0,dim_gl.3]}; //Top left
        
        let texture_rec = vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
        
        return Frame{dim:dim,texture_index:texture_index,va:texture_rec};
    }
}


struct Animation{
    frames:Vec<Frame>,
    current_frame:usize
}

impl Animation{
    pub fn new(frames:Vec<Frame>)->Self{
        return Animation{frames:frames,current_frame:0};
    }

}


pub struct Sprite{
    animations:HashMap<String,Animation>,
    current_animation:usize,
    hotswap:Hotswap
}

impl Sprite{
    pub fn new(path_to_sprite_sheet:&str,texture_index:usize,render_system:&RenderSystem)->Self{
        let mut hotswap = Hotswap::new(String::from(path_to_sprite_sheet));
        let mut animations = HashMap::new();
        hotswap.reload(&mut |ref mut file|{
            let mut string = String::new();
            file.read_to_string(&mut string);
            let lines = string.split("\n");
            let mut name = String::new();
            let mut frame_coords :Vec<(u32,u32,u32,u32)>= Vec::new();
            for line  in lines{
                let columns = line.split(" ").collect::<Vec<&str>>();
                if columns.len() == 1{
                    if name.len() != 0{
                        let mut frames = Vec::new();
                        for i in &frame_coords{
                            frames.push(Frame::new(texture_index,*i,render_system));
                        }
                        animations.insert(String::from(name.as_str()),Animation::new(frames));
                        name.clear();
                        frame_coords.clear();
                    }
                    name.push_str(columns[0]);
                }
                if columns.len() == 4{
                    frame_coords.push((columns[0].parse::<u32>().unwrap(),columns[1].parse::<u32>().unwrap(),columns[2].parse::<u32>().unwrap(),columns[3].parse::<u32>().unwrap()));
                }
            }
        });
        return Sprite{animations:animations,current_animation:0,hotswap:hotswap};
    }
}

pub enum RenderType{
    Primative(PrimativeType,Color),
    Sprite(Sprite)    
}

pub struct Renderable{
    t:RenderType,
    pos:Handle,
    program:String
}

impl Renderable{
    pub fn new(item:RenderType,pos:Handle,program:&str)->Self{
        return Renderable{t:item,pos:pos,program:String::from(program)};
    }
}
pub struct Program{
    pub program:glium::Program,
    vert_hotswap:Hotswap,
    frag_hotswap:Hotswap,
}

impl Program{
    pub fn new(vert_path:&str,frag_path:&str,display:&glium::backend::Facade)->Self{
        let mut vert_hotswap = Hotswap::new(String::from(vert_path));
        let mut vert_source = String::new();
        vert_hotswap.reload(&mut |ref mut file|{
            file.read_to_string(&mut vert_source);
        });
        let mut frag_hotswap = Hotswap::new(String::from(frag_path));
        let mut frag_source = String::new();
        frag_hotswap.reload(&mut |ref mut file|{
            file.read_to_string(&mut frag_source);
        });

        let program = glium::Program::from_source(display,vert_source.as_str(),frag_source.as_str(),None).unwrap();

        return Program{program:program,vert_hotswap:vert_hotswap,frag_hotswap:frag_hotswap};
        
    }
}

const SQUARE :usize = 0;
const TRIANGLE :usize = 0;

pub struct Texture{
    texture:glium::texture::SrgbTexture2d,
    hotswap:Hotswap
}

//TODO: make this have less allocations.
impl Texture{
    pub fn new(path:&str,display:&glium::backend::Facade)->Self{
        let mut hotswap = Hotswap::new(String::from(path));
        let split = path.split("_").collect::<Vec<&str>>();
        let mut image = Image::blank();
        {
            hotswap.reload(&mut |ref mut file|{
                let mut image_raw = Vec::new();
                file.read_to_end(&mut image_raw);
                image = load_image(&image_raw,split[0].parse::<u32>().unwrap(),split[1].parse::<u32>().unwrap());
            });
        }
        let texture  = glium::texture::SrgbTexture2d::new(display,glium::texture::RawImage2d::from_raw_rgba(image.bytes,image.dim)).unwrap();
        return Texture{texture:texture,hotswap:hotswap};
    }
}
pub struct RenderSystem{
    vb:glium::VertexBuffer<Vertex>,
    vb_p:glium::VertexBuffer<VertexPos>,
    textures:HashMap<String,Texture>,
    programs:HashMap<String,Program>,
    va:HashMap<String,Vec<VertexPos>>,
    renderables:Pool<Renderable>,
    unit_x_pixels:f32,
    unit_y_pixels:f32,
    unit_offset_x:f32,
    unit_offset_y:f32
    //scale_y:f32,
    //scale_x:f32,
}

const units_per_screen_width :f32 = 16.0;
const units_per_screen_height :f32 = 9.0;
const ideal_unit_size_in_pixels :f32 = 100.0;


impl RenderSystem{
    pub fn new(width:u32,height:u32,display:&glium::backend::Facade,texture_paths:Vec<&str>,program_paths:&str)->Self{
        
        let unit_x_pixels = width as f32/units_per_screen_width;
        let unit_y_pixels = -(height as f32)/units_per_screen_height;
        //let scale_x = unit_x_pixels/ideal_unit_size_in_pixels;
        //let scale_y = unit_y_pixels/ideal_unit_size_in_pixels;

        let unit_offset_x = 8.0;
        let unit_offset_y = 4.5;
        let vertex1 = VertexPos { position:[-1.0,1.0]}; // Top-left
        let vertex2 = VertexPos { position:[1.0,1.0]};  //Top right
        let vertex3 = VertexPos { position:[1.0,-1.0]}; //Bottom right

        let vertex4 = VertexPos { position:[1.0,-1.0]}; //Bottom right
        let vertex5 = VertexPos { position:[-1.0,-1.0]}; //Bottom left
        let vertex6 = VertexPos { position:[-1.0,1.0]}; //Top left

        let square = vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
        let triangle = vec![vertex1,vertex2,vertex3];
        let mut va = HashMap::new();
        va.insert(String::from("square"),square);
        va.insert(String::from("triangle"),triangle);
        let mut vb_p = glium::VertexBuffer::dynamic(display,&va["square"]).unwrap();

        let vertex1 = Vertex { position:[-0.1,0.10],tex_cords:[0.0,0.0]}; // Top-left
        let vertex2 = Vertex { position:[0.1,0.1],tex_cords:[0.0,0.0]};  //Top right
        let vertex3 = Vertex { position:[0.1,-0.1],tex_cords:[0.0,0.0]}; //Bottom right
        let triangle = vec![vertex1,vertex2,vertex3];
        let mut vb = glium::VertexBuffer::dynamic(display,&triangle).unwrap();
        let mut textures = HashMap::new();

        for path in texture_paths {
            let texture = Texture::new(path,display);
            textures.insert(String::from(path),texture);
        }

        let mut program_path_src = String::new();
        let mut file = File::open(program_paths).unwrap();
        let mut programs = HashMap::new();
        file.read_to_string(&mut program_path_src);
        let split = program_path_src.split("\n").collect::<Vec<&str>>();
        for line in split{
            let  collumns = line.split(" ").collect::<Vec<&str>>();
            if collumns.len() == 3 {
                programs.insert(String::from(collumns[0]),Program::new(collumns[1],collumns[2],display));
            }
        }

        return RenderSystem{vb_p:vb_p,vb:vb,textures:textures,programs:programs,va:va,renderables:Pool::new(),unit_x_pixels:unit_x_pixels,unit_y_pixels:unit_y_pixels,/*scale_x:scale_x,scale_y:scale_y,*/unit_offset_x:unit_offset_x,unit_offset_y:unit_offset_y};
    }

    pub fn render(&mut self,display:&glium::backend::glutin_backend::GlutinFacade,pos_system:&PositionSystem){

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let mut target = display.draw();
        target.clear_color(0.0,0.0,0.0,1.0);

        for renderable in &self.renderables.items{
            match renderable.t{
                RenderType::Primative(pt,color) => {
                    match pt{
                        PrimativeType::Square(width,height) =>{
                            self.vb_p.write(self.va.get("square").unwrap());
                            let pos = pos_system.get(renderable.pos).unwrap();
                            let uniforms = uniform! {
                                scale: [
                                    [self.world_cords_to_gl_cords_x(width),0.0,0.0,0.0],
                                    [0.0,self.world_cords_to_gl_cords_y(height),0.0,0.0],
                                    [0.0,0.0,1.0,0.0],
                                    [0.0,0.0,0.0,1.0 as f32]
                                ],
                                translate: [
                                    [1.0,0.0,0.0,0.0],
                                    [0.0,1.0,0.0,0.0],
                                    [0.0,0.0,1.0,0.0],
                                    [self.world_cords_to_gl_cords_x(pos.vec4.x),self.world_cords_to_gl_cords_y(pos.vec4.y),0.0,1.0]
                                ],
                                color: [color.r,color.g,color.b,color.a]
                            };
                            target.draw(&self.vb_p,&indices,&self.programs.get(renderable.program.as_str()).unwrap().program,&uniforms,&Default::default()).unwrap();
                        },
                        _=>{}
                    }
                },
                _ => {}
            }
        }
        target.finish().unwrap();
    }

    pub fn add_renderable(&mut self,renderable:Renderable)->Handle{
        return self.renderables.insert(renderable);
    }

    pub fn remove_renderable(&mut self,handle:Handle){
        self.renderables.remove(handle).uwrap();
    }

    pub fn pixels_to_world_cords_x(&self,x:u32)->f32{
        return  x as f32  / self.unit_x_pixels - self.unit_offset_x;
    }

    pub fn pixels_to_world_cords_y(&self,y:u32)->f32{
        return  y as f32  / self.unit_y_pixels + self.unit_offset_y;
    }
    
    pub fn pixels_to_gl_cords_x(&self,y:u32)->f32{
        return self.pixels_to_world_cords_x(y)/16.0;
    }

    pub fn pixels_to_gl_cords_y(&self,y:u32)->f32{
        return self.pixels_to_world_cords_y(y)/9.0;
    }
    pub fn world_cords_to_gl_cords_x(&self,x:f32)->f32{
        return x/8.0;
    }
    pub fn world_cords_to_gl_cords_y(&self,y:f32)->f32{
        return y/4.5;
    }

}
