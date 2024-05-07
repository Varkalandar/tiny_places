use std::collections::HashMap;
use crate::item::Item;

#[derive(PartialEq, Eq)]
pub enum Slot {
    BAG = 0,
    STASH = 1,
    NOSE = 2,
    BODY = 3,
    LWING = 4,
    RWING = 5,
    ENGINE = 6,
}

struct Entry {
    item_id: usize,
    slot: usize,    
}


pub struct Inventory {

    // Keeps all the items (owns all the items)
    item_bag: HashMap <usize, Item>,

    // describes details about each of the owned items
    entries: Vec<Entry>,
}


impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            item_bag: HashMap::new(),
            entries: Vec::new(),
        }
    }
}