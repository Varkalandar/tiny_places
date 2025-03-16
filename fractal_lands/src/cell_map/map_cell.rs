// the item grid size of a map cell. I.e. a cell can hold SUB*SUB items 
pub const CELL_SUB: u32 = 11;

const K_DECO: u8 = 1;
const K_ITEM: u8 = 2;
const K_CURRENCY: u8 = 4;


pub struct MapCell {
    value: [u16; (CELL_SUB * CELL_SUB) as usize],
    kind:  [u8; (CELL_SUB * CELL_SUB) as usize],
}

impl MapCell {

    pub fn new() -> MapCell {
        MapCell {
            value: [0u16; (CELL_SUB * CELL_SUB) as usize],
            kind:  [0u8; (CELL_SUB * CELL_SUB) as usize],
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
