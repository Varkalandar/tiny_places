use std::collections::HashMap;

mod map_cell;
use map_cell::MapCell;
use map_cell::CELL_SUB;

const MAX_MAP_WIDTH_IN_CELLS:u32 = 256;


pub struct CellMap {

    cells: HashMap<u32, MapCell>,
}


impl CellMap {

    fn new() -> CellMap {
        CellMap {
            cells: HashMap::new(),
        }
    }


    /**
     * Allocate a new map cell to store items at the given coordinate.
     * If there was a cell allocated for this location previously it will be lost.
     
     * @param i Coordinate in item coordinates
     * @param j Coordinate in item coordinates
     */
    pub fn allocate_cell(&mut self, i: u32, j: u32) {
        let cell_key = cell(i, j);

        let cell = MapCell::new();

        self.cells.insert(cell_key, cell);
    }


    pub fn get_item(&self, i: u32, j: u32) -> Option<u32> {
        let cell_key = cell(i, j);

        let cell_option = self.cells.get(&cell_key);

        match cell_option {
            Some(cell) => {
                return cell.get_item(i % CELL_SUB, j % CELL_SUB);
            }
            None => {
                return None;
            }
        }
    }


    pub fn put_item(&mut self, i: u32, j: u32, item: u32) {
        let cell_key = cell(i, j);

        let cell_option = self.cells.get_mut(&cell_key);

        match cell_option {
            Some(cell) => {
                return cell.put_item(i % CELL_SUB, j % CELL_SUB, item);
            }
            None => {
                panic!("Cell for coordinate {}, {} is not allocated", i, j);
            }
        }
    }

}


fn cell(i: u32, j: u32) -> u32 {
    return (j / CELL_SUB) * MAX_MAP_WIDTH_IN_CELLS + (i / CELL_SUB);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_cell_allocation() -> Result<(), String> {
        let mut map = CellMap::new();
        
        let item_option = map.get_item(0, 0);        
        assert_eq!(item_option == Option::None, true);
        
        let i = 2 * CELL_SUB;
        let j = CELL_SUB;        

        map.allocate_cell(i, j);

        let item_option = map.get_item(i, j);        
        assert_eq!(item_option == Option::None, true);

        map.put_item(i, j, 15);

        let item_option = map.get_item(i, j);        
        assert_eq!(item_option.unwrap(), 15);

        Ok(())
    }
}