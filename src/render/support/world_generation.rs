use glium::{self, implement_vertex, Display};
use glium::vertex::VertexBufferAny;
use obj;
use noise::{NoiseFn, Perlin};
use super::tile::{HexTile, Resource};

pub struct WorldGen{
    pub smoothness: f64,
    pub slope: f64,
    pub noise: Perlin,
}
impl WorldGen {
    pub fn new(smoothness: f64, slope: f64, noise: Perlin)->WorldGen{
        WorldGen{
            smoothness,
            slope,
            noise,
        }
    }
}


pub fn sq(num:f32)->f32{
    num * num
}

pub fn mix_gen(position: [f32;3],h1:f32,h2:f32)->f32{
    let mut distance:f32 = 0.0;
    for i in 0..3{
        distance += position[i] * position[i];
    }
    let blend = f32::max(-1.618*sq(distance / 200.0)+1.0,0.0);
    //println!("distance {} blend {}",distance,blend);
    h2 * blend as f32 + h1 * (1.0-blend as f32)
}

pub fn generate_height_mountain(position: [f32;3],tile: &HexTile,perlin:&Perlin)->f32{
    let smoothness = 0.075;
    let slope = 177.5;
    let baseheight = 0.55;
    ((perlin.get([((position[0] + tile.x)) as f64 * smoothness,1.0,((position[2] + tile.z)) as f64 * smoothness]) + baseheight) * slope) as f32
    //150.0
}

pub fn generate_height_standard(position: [f32;3],tile: &HexTile,perlin:&Perlin)->f32{
    let smoothness = 0.035;
    let slope = 57.5;
    let baseheight = 0.5;
    ((perlin.get([((position[0] + tile.x)) as f64 * smoothness,1.0,((position[2] + tile.z)) as f64 * smoothness]) + baseheight) * slope) as f32
    //25.0
}

pub fn generate_height(position: [f32;3],tile: &HexTile,perlin:&Perlin)->f32{
    if tile.resource == Resource::Ore{
        let h1 = generate_height_standard(position, tile, perlin);
        let h2 = generate_height_mountain(position, tile, perlin);
        return mix_gen(position, h1, h2);
    }else{
        generate_height_standard(position, tile, perlin)
    }

}

pub fn generate_normal(position: [f32;3],tile: &HexTile,perlin:&Perlin)->[f32;3]{
    let offset = 0.25;
    let mut normal = [0.0,0.0,0.0];
    normal[0] = generate_height([position[0]-offset,position[1],position[2]], tile, perlin) 
                    - generate_height([position[0],position[1],position[2]], tile, perlin);
    normal[2] = generate_height([position[0],position[1],position[2]-offset], tile, perlin)
                    - generate_height([position[0],position[1],position[2]], tile, perlin);
    normal[1] = 1.0;

    normalize(normal)
}
pub fn normalize(normal: [f32;3])->[f32;3]{
    let min_v = -1.0;
    let max_v = 1.0;
    let mut out = [0.0,0.0,0.0];
    for i in 0..3{
        out[i] = (normal[i]-min_v)/(max_v-min_v);
        print!("Norm: {} ",out[i]);    
    }
    println!("");
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
                            if normal[0] <0.1 && normal[1] >0.1 && normal[2] <0.1{
                                let tmp_normal = normal[1];
                                normal = generate_normal(position,tile,&perlin);
                                normal[1]=tmp_normal;
                            }
                            let height = generate_height(position,tile,&perlin);
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
