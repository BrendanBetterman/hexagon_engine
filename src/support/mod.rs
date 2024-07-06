#![allow(dead_code)]
use std::time::{Duration, Instant};
use glium::glutin::dpi::Position;
use glium::{self, Display};
use glium::vertex::VertexBufferAny;
use glium::glutin::event_loop::{EventLoop, ControlFlow};
use glium::glutin::event::{Event, StartCause};
use obj;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use crate::support::tile::HexTile;
pub mod camera;
pub mod tile;
pub mod mesh;
pub enum Action {
    Stop,
    Continue,
}
pub fn start_loop<F>(event_loop: EventLoop<()>, mut callback: F)->! where F: 'static + FnMut(&Vec<Event<'_, ()>>) -> Action {
    let mut events_buffer = Vec::new();
    let mut next_frame_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        let run_callback = match event.to_static() {
            Some(Event::NewEvents(cause)) => {
                match cause {
                    StartCause::ResumeTimeReached { .. } | StartCause::Init => {
                        true
                    },
                    _ => false
                }
            },
            Some(event) => {
                events_buffer.push(event);
                false
            }
            None => {
                // Ignore this event.
                false
            },
        };

        let action = if run_callback {
            let action = callback(&events_buffer);
            next_frame_time = Instant::now() + Duration::from_nanos(0);//16666667
            // TODO: Add back the old accumulator loop in some way

            events_buffer.clear();
            action
        } else {
            Action::Continue
        };

        match action {
            Action::Continue => {
                *control_flow = ControlFlow::WaitUntil(next_frame_time);
            },
            Action::Stop => *control_flow = ControlFlow::Exit
        }
    })
}
#[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        texture: [f32; 2],
    }

pub fn generate_height(position: [f32;3],tile: &HexTile,perlin:&Perlin)->f32{
    let smoothness = 0.035;
    let slope = 57.5;
    let baseheight = 1.0;
    ((perlin.get([((position[0] + tile.x)) as f64 * smoothness,1.0,((position[2] + tile.z)) as f64 * smoothness]) + baseheight) * slope) as f32
}

pub fn generate_normal(position: [f32;3],tile: &HexTile,perlin:&Perlin)->[f32;3]{
    let offset = 0.15;
    let mut normal = [0.0,0.0,0.0];
    normal[0] = generate_height([position[0]-offset,position[1],position[2]], tile, perlin) - generate_height([position[0]+offset,position[1],position[2]], tile, perlin);
    normal[2] = generate_height([position[0],position[1],position[2]-offset], tile, perlin) - generate_height([position[0],position[1],position[2]+offset], tile, perlin);
    normal[1] = 2.0;

    normalize(normal)
}
pub fn normalize(normal: [f32;3])->[f32;3]{
    let mut min_v = normal[0];
    let mut max_v = normal[1];
    for i in normal{
        if i < min_v{
            min_v = i;
        } else {
            max_v = i;  
        }
    }
    let mut out = [0.0,0.0,0.0];
    for i in 0..3{
        out[i] = (normal[i]-min_v)/(max_v-min_v);
    }
    out
}
pub fn make_hex_chunk(display: &Display, data: &[u8],tile: &HexTile,seed:u32) -> VertexBufferAny {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        texture: [f32; 2],
    }
    let scale = 10.0;
    let perlin = Perlin::new(seed);
    implement_vertex!(Vertex, position, normal, texture);

    let mut data = ::std::io::BufReader::new(data);
    let data = obj::ObjData::load_buf(&mut data).unwrap();

    let mut vertex_data = Vec::new();

    for object in data.objects.iter() {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                obj::SimplePolygon(indices) => {
                    for v in indices.iter() {
                        // modify vertex with noise
                        let mut position = data.position[v.0];
                        let texture = v.1.map(|index| data.texture[index]);
                        let normal = v.2.map(|index| data.normal[index]);
                        let texture = texture.unwrap_or([0.0, 0.0]);
                        let mut normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                        if !(position[1] < 0.0) {
                            let height = generate_height(position,tile,&perlin);
                            if normal[0] <0.5 && normal[1] >0.5 && normal[2] <0.5{
                                normal = generate_normal(position,tile,&perlin);
                            }
                            position = [(position[0]+tile.x)*scale,height,(position[2]+tile.z)*scale];
                        }else{
                            position = [(position[0]+tile.x)*scale,position[1]*scale,(position[2]+tile.z)*scale];
                        }
                        
                        vertex_data.push(Vertex {
                            position,
                            normal,
                            texture,
                        })
                    }
                },
            }
        }
    }

    glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into()
}


/// Returns a vertex buffer that should be rendered as `TrianglesList`.
pub fn load_wavefront(display: &Display, data: &[u8]) -> VertexBufferAny {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        texture: [f32; 2],
    }

    implement_vertex!(Vertex, position, normal, texture);

    let mut data = ::std::io::BufReader::new(data);
    let data = obj::ObjData::load_buf(&mut data).unwrap();

    let mut vertex_data = Vec::new();

    for object in data.objects.iter() {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                obj::SimplePolygon(indices) => {
                    for v in indices.iter() {
                        let mut position = data.position[v.0];
                        position[0] *= 10.0;
                        position[2] *= 10.0;
                        let texture = v.1.map(|index| data.texture[index]);
                        let normal = v.2.map(|index| data.normal[index]);

                        let texture = texture.unwrap_or([0.0, 0.0]);
                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                        vertex_data.push(Vertex {
                            position,
                            normal,
                            texture,
                        })
                    }
                },
            }
        }
    }

    glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into()
}
