pub enum Resource{
    Wood,
    Ore,
    Wheat,
    Wool,
    Brick,
    Desert,
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