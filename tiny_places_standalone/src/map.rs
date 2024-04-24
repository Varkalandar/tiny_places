#[path = "tileset.rs"]
mod tileset;

use tileset::TileSet;

pub struct Map {
    decorations: TileSet
}


impl Map {
    pub fn new() -> Map {        
        Map {
            decorations: TileSet::load("../tiny_places_client/resources/objects/map_objects.tica"),
        }
    }
}