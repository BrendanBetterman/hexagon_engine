
//extern crate glium;
#[allow(unused_imports)]
use glium::glutin::dpi::Position;
use glium::texture::SrgbTexture2d;
use glium::{glutin, Surface,Display};
use glium::vertex::VertexBufferAny;
use glutin::event_loop::EventLoop;
use image::ImageBuffer;
use rand::Rng;
use std::io::Cursor;
use std::str::Bytes;
use std::usize;
use crate::support::tile::HexTile;
use super::support::camera::CameraState;
use super::support;

pub struct Renderer{
    //event_loop: EventLoop<()>,
    display: Display,
    diffuse_texture: glium::texture::SrgbTexture2d,
    vertex_buffer: Vec<VertexBufferAny>,
    tiles:Vec<HexTile>,
}
impl Renderer{
    pub fn new(event_loop:&EventLoop<()>) -> Renderer{
        let display = create_display(&event_loop);
        let diffuse_texture = create_diffuse_texture(&display,&include_bytes!("textures/Texture.png"));
        
        let mut vex_buff = Vec::new();
        let mut ra =rand::thread_rng();
        let seed = ra.gen_range(0..100000);

        let mut tiles= Vec::new();
        for x in 0..5{
            for z in 0..5{
                let tile = support::tile::HexTile::new(x as f32*13.856*2.0+(13.856*(z%2)as f32),z as f32*24.0,support::tile::Resource::Wood,0);
                vex_buff.push(support::make_hex_chunk(&display, include_bytes!("models/HexTile.obj"), &tile, seed));
                tiles.push(tile);
            }
        }

        return Renderer{
            //event_loop: event_loop,
            display: display,
            diffuse_texture: diffuse_texture,
            vertex_buffer: vex_buff,
            tiles: tiles
        };
    }
    pub fn render_frame(&mut self,camera:&CameraState){
        
        let program = program!(&self.display,
            140 => {
                vertex: include_str!("shaders/Vertex.vert"),
                fragment: include_str!("shaders/Fragment.frag"),
            },
        ).unwrap();
         // building the uniforms
         let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
            diffuse_tex: &self.diffuse_texture,
            rot_x_matrix: camera.get_rot_x(),
            rot_y_matrix: camera.get_rot_y(),
            // get objects rotation
        };
        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };
        let mut target = self.display.draw();
        if camera.clicked{
            
        }
        target.clear_color_and_depth((0.68,0.88,0.9, 0.0), 1.0);
        for i in 0..self.vertex_buffer.len(){
            target.draw(&self.vertex_buffer[i],
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &program, &uniforms, &params).unwrap();
        }
        
        
        target.finish().unwrap();

    }
}

pub fn create_display(event_loop:&EventLoop<()>) -> Display {
    let wb = glutin::window::WindowBuilder::new().with_title("Catan")
    .with_inner_size(glutin::dpi::LogicalSize::new(1280,720))
    .with_position(glutin::dpi::LogicalPosition::new(320,180));
    
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    glium::Display::new(wb, cb, &event_loop).unwrap()
}
pub fn create_diffuse_texture<const N: usize>(display: &Display,image_path: &'static [u8; N]) -> glium::texture::SrgbTexture2d{
    let image = image::load(Cursor::new(image_path),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::SrgbTexture2d::new(display, image).unwrap()
}