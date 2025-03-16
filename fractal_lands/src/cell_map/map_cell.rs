// the item grid size of a map cell. I.e. a cell can hold SUB*SUB items 
pub const CELL_SUB: u32 = 11;

const K_DECO: u8 = 1;        // decorations only consist of their image (graphical tile) id
const K_ITEM: u8 = 2;        // items are indices for the items table (kept in the map data)
const K_CURRENCY: u8 = 4;    // currency are stackable items, i.e. all items of a stack must be identical


pub struct MapCell {
    value: [u16; (CELL_SUB * CELL_SUB) as usize],
    kind:  [u8; (CELL_SUB * CELL_SUB) as usize],

    floor: u16,            // tile id of the floor graphics
    walls: [u16; 4],       // tile ids of the wall graphics
}

impl MapCell {

    pub fn new() -> MapCell {
        MapCell {
            value: [0u16; (CELL_SUB * CELL_SUB) as usize],
            kind:  [0u8; (CELL_SUB * CELL_SUB) as usize],
            floor: 0,
            walls: [0u16; 4],
        }
    }

    pub fn get_item(&self, i: u32, j: u32) -> Option<u32> {
        let n = index(i, j);

        let t = self.kind[n];

        if t == K_ITEM {
            return Some(self.value[n] as u32);
        }

        return None;
    }

    pub fn put_item(&mut self, i: u32, j: u32, item: u32) {
        let n = index(i, j);

        self.kind[n] = K_ITEM;
        self.value[n] = item as u16;
    }
}



fn index(i: u32, j: u32) -> usize {

    if i >= CELL_SUB {
        panic!("Cell coordinate i out of bounds: {}", i);
    }

    if j >= CELL_SUB {
        panic!("Cell coordinate j out of bounds: {}", i);
    }

    return (j * CELL_SUB + i) as usize;
}
