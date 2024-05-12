use std::collections::HashMap;
use crate::item::Item;

#[derive(PartialEq, Eq, Debug, Hash)]
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

    pub fn put_item(&mut self, item: Item) {

        let entry = Entry {
            item_id: item.id,
            slot: Slot::BAG,
            location_x: 3,
            location_y: 0,
        };

        self.bag.insert(item.id, item);
        self.entries.push(entry);

    }
}