use noise::Perlin;
use Resource::*;
pub enum Resource{
    Wood,
    Ore,
    Wheat,
    Wool,
    Brick,
    Desert,
}
pub fn num_resource(number:u8)-> Resource{
    match number{
        0=>Wood,
        1=>Ore,
        2=>Wheat,
        3=>Wool,
        4=>Brick,
        _=>Desert,
    }
}

pub struct HexTile{
    pub x:f32,
    pub z:f32,
    pub resource: Resource,
    pub number: u8,
}
impl HexTile{
    pub fn new(x:f32,z:f32,resource:Resource,number:u8)->HexTile{
        HexTile{
            x,
            z,
            resource,
            number,
        }
    }
}

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