
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