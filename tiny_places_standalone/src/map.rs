use vecmath::Vector2;


pub struct Map {
    pub decorations: Vec<MapObject>,
}


impl Map {
    pub fn new() -> Map {        
        Map {
            decorations: Vec::new(),
        }
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