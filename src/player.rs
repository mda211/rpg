use crate::inventory::Inventory;

#[derive(Debug)]
pub(crate) enum Class {
    Warrior,
    Mage,
    Rogue,
}

#[derive(Debug)]
pub(crate) struct Player {
    pub name: String,
    pub class: Class,
    pub hp: i32,
    pub damage: i32,
    pub level: u32,

    pub inventory: Inventory,
}

impl Player {
    pub(crate) fn new(name: String, class: Class) -> Self {
        Self {
            name,
            class,
            hp: 100,
            damage: 4,
            level: 0,

            inventory: Inventory::new(),
        }
    }

    pub(crate) fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
    }

    pub(crate) fn heal(&mut self, amount: i32) {
        self.hp += amount;
    }

    pub(crate) fn is_alive(&self) -> bool {
        self.hp > 0
    }
}
