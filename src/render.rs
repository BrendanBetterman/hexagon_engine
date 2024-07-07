
//extern crate glium;
#[allow(unused_imports)]
use glium::glutin::dpi::Position;
use glium::{glutin, Surface,Display};
use glutin::event_loop::EventLoop;
use rand::Rng;
use std::io::Cursor;

use crate::support::load_wavefront;
use crate::support::mesh::Mesh;
use crate::support::tile::{num_resource, HexTile};
use super::support::camera::CameraState;
use super::support;

pub struct Renderer{
    //event_loop: EventLoop<()>,
    display: Display,
    mesh_buffer: Vec<Mesh>,
    tiles:Vec<HexTile>,
}
impl Renderer{
    pub fn new(event_loop:&EventLoop<()>) -> Renderer{
        let display = create_display(&event_loop);
        
        let mut mesh_buffer = Vec::new();
        let mut ra =rand::thread_rng();
        let seed = ra.gen_range(0..100000);

        let mut tiles= Vec::new();
        let map = [
            [0,2],[0,3],[0,4],
            [1,1],[1,2],[1,3],[1,4],
            [2,1],[2,2],[2,3],[2,4],[2,5],
            [3,1],[3,2],[3,3],[3,4],
            [4,2],[4,3],[4,4]
            ];
        let tile_mesh = include_bytes!("models/HexTile.obj");

        let grass = include_bytes!("textures/Grass.png");
        let sand = include_bytes!("textures/Sand.png");
        let ore = include_bytes!("textures/Ore.png");
        for i in 0..map.len(){
                let vert = 27.7128;
                let tile = support::tile::HexTile::new(map[i][1] as f32*vert+(vert/2.0*(map[i][0]%2)as f32)-80.0,map[i][0] as f32*24.0-80.0,
                num_resource(ra.gen_range(0..6)),
                0);
                mesh_buffer.push(
                match tile.resource{
                    support::tile::Resource::Wood => Mesh::new(
                        support::make_hex_chunk(&display,tile_mesh , &tile, seed), 
                        create_diffuse_texture(&display,&grass)),
                    support::tile::Resource::Ore => Mesh::new(
                        support::make_hex_chunk(&display,tile_mesh , &tile, seed), 
                        create_diffuse_texture(&display,&ore)),
                        support::tile::Resource::Desert => Mesh::new(
                            support::make_hex_chunk(&display,tile_mesh , &tile, seed), 
                            create_diffuse_texture(&display,&sand)),
                    _=> Mesh::new(
                        support::make_hex_chunk(&display, tile_mesh, &tile, seed), 
                        create_diffuse_texture(&display,&grass)),
                }
                );
                tiles.push(tile);
        }
        mesh_buffer.push(Mesh::new(load_wavefront(&display, include_bytes!("models/Water.obj"),50.0,[0.0,0.0,0.0]),
        create_diffuse_texture(&display,&include_bytes!("textures/Water.png"))));
        mesh_buffer.push(Mesh::new(load_wavefront(&display, include_bytes!("models/Tree.obj"),10.0,[0.0,50.0,0.0]),
        create_diffuse_texture(&display,&include_bytes!("textures/Leaves.png"))));
/*
        for x in 0..5{
            for z in 0..5{
                let vert = 27.7128 + 1.0;
                let tile = support::tile::HexTile::new(x as f32*vert+(vert/2.0*(z%2)as f32),z as f32*25.0,support::tile::Resource::Wood,0);
                
                mesh_buffer.push(Mesh::new(
                    support::make_hex_chunk(&display, include_bytes!("models/HexTile.obj"), &tile, seed), 
                    create_diffuse_texture(&display,&include_bytes!("textures/Texture.png"))));
                tiles.push(tile);
            }
        }
*/
        return Renderer{
            //event_loop: event_loop,
            display: display,
            mesh_buffer: mesh_buffer,
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
        for i in 0..self.mesh_buffer.len(){
            // building the uniforms
            let uniforms = uniform! {
                persp_matrix: camera.get_perspective(),
                view_matrix: camera.get_view(),
                diffuse_tex: &self.mesh_buffer[i].diffuse_texture,
                rot_x_matrix: camera.get_rot_x(),
                rot_y_matrix: camera.get_rot_y(),
                // get objects rotation
            };
            target.draw(&self.mesh_buffer[i].vertex_buffer,
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