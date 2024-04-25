use std::collections::HashMap;

use vecmath::Vector2;

#[path = "tileset.rs"]
mod tileset;

use tileset::TileSet;

use self::tileset::Tile;

pub struct Map {
    pub decoration_tiles: TileSet,
    pub decorations: Vec<MapObject>,
}


impl Map {
    pub fn new() -> Map {        
        Map {
            decoration_tiles: TileSet::load("../tiny_places_client/resources/objects", "map_objects.tica"),
            decorations: Vec::new(),
        }
    }
    
    pub fn add_decoration(&mut self, id: usize, position: Vector2<f64>, scale: f64) {
        let deco = MapObject::new(id, position, scale);        
        self.decorations.push(deco);    
    }
}


pub struct MapObject {
    pub id: usize,
    pub position: Vector2<f64>,
    pub scale: f64,    
}


impl MapObject {
    
    pub fn new(id: usize, position: Vector2<f64>, scale: f64) -> MapObject {
        MapObject { 
            id, 
            position, 
            scale
        }
    }
}