use crate::item::Item;

#[derive(Debug)]
pub(crate) struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub(crate) fn peek(&self) -> Vec<Item> {
        self.items.iter().cloned().collect()
    }

    pub(crate) fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    pub(crate) fn add_many(&mut self, items: Vec<Item>) {
        for item in items {
            self.items.push(item)
        }
    }

    pub(crate) fn remove(&mut self, item: &Item) -> bool {
        if let Some(index) = self.items.iter().position(|entry| entry == item) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }
}
