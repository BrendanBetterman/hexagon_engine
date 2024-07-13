use glium::{self, implement_vertex, Display};
use glium::vertex::VertexBufferAny;
use obj;
use std::io::Cursor;

pub struct Mesh {
    pub vertex_buffer: glium::vertex::VertexBufferAny,
    pub diffuse_texture: glium::texture::SrgbTexture2d,
}

impl Mesh {
    pub fn new(
        vertex_buffer: glium::vertex::VertexBufferAny,
        diffuse_texture: glium::texture::SrgbTexture2d) -> Mesh
    {
        Mesh {
            vertex_buffer: vertex_buffer,
            diffuse_texture: diffuse_texture,
        }
    }

}

pub fn load_wavefront(display: &Display, data: &[u8],scale:f32,pos: [f32;3]) -> VertexBufferAny {
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
                        position[0] = position[0] * scale + pos[0];
                        position[1] = position[1] * scale + pos[1];
                        position[2] = position[2] * scale + pos[2];
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

pub fn create_diffuse_texture<const N: usize>(display: &Display,image_path: &'static [u8; N]) -> glium::texture::SrgbTexture2d{
    let image = image::load(Cursor::new(image_path),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::SrgbTexture2d::new(display, image).unwrap()
}