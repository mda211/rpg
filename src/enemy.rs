use crate::{inventory::Inventory, player::Player};

#[derive(Debug)]
pub(crate) enum Type {
    Goblin,
    Evil,
}

#[derive(Debug)]
pub(crate) struct Enemy {
    pub name: String,
    pub enemy_type: Type,
    pub hp: i32,
    pub damage: i32,

    pub inventory: Inventory,
}

impl Enemy {
    pub(crate) fn new(enemy_type: Type) -> Self {
        match enemy_type {
            Type::Goblin => Self {
                name: String::from("Goblin"),
                enemy_type,
                hp: 35,
                damage: 8,

                inventory: Inventory::new(),
            },

            Type::Evil => Self {
                name: String::from("evil"),
                enemy_type,
                hp: 1000000,
                damage: 1000000,

                inventory: Inventory::new(),
            },
        }
    }

    pub(crate) fn attack(&self, player: &mut Player) {
        player.take_damage(self.damage);
    }

    pub(crate) fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
    }

    pub(crate) fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub(crate) fn killed(&self) {
        println!("{} died! Dropped: {:?}", self.name, self.inventory.peek())
    }
}
