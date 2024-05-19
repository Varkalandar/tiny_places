use std::collections::HashMap;
use crate::item::Item;
use crate::ui::UiArea;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Slot {
    BAG = 0,
    STASH = 1,
    NOSE = 2,
    BODY = 3,
    LWING = 4,
    RWING = 5,
    ENGINE = 6,
}

#[derive(Debug)]
pub struct Entry {
    pub item_id: usize,
    pub slot: Slot,
    pub location_x: i32,
    pub location_y: i32,    
}

#[derive(Debug)]
pub struct Inventory {

    // Keeps all the items (owns all the items)
    pub bag: HashMap <usize, Item>,

    // describes details about each of the owned items
    pub entries: Vec<Entry>,
}


impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            bag: HashMap::new(),
            entries: Vec::new(),
        }
    }

    pub fn put_item(&mut self, item: Item, slot: Slot) {

        let spot = 
            if slot == Slot::BAG 
                {self.find_free_location(&item)}
            else
                {[0, 0]};

        let entry = Entry {
            item_id: item.id,
            slot,
            location_x: spot[0],
            location_y: spot[1],
        };

        self.bag.insert(item.id, item);
        self.entries.push(entry);

    }

    fn find_free_location(&self, item: &Item) -> [i32; 2] {

        // look for free space
        for grid_y in 0..8 
        {
            for grid_x in 0..32 
            {
                let mut free = true;

                for entry in &self.entries {
                    let bag_item = self.bag.get(&entry.item_id).unwrap();

                    let area = UiArea {
                        x: entry.location_x,                        
                        y: entry.location_y,
                        w: bag_item.inventory_w,
                        h: bag_item.inventory_h,
                    };

                    for x in 0..item.inventory_w {
                        for y in 0..item.inventory_h {
                            if area.contains(grid_x + x, grid_y + y) {
                                free = false;
                                break;
                            }
                        }
                    }
                }

                if free {
                    return [grid_x, grid_y];
                }
            }
        }

        return [-1, -1];
    }
}