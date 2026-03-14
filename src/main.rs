#![allow(unused)]

mod combat;
mod encounter;
mod enemy;
mod inventory;
mod item;
mod player;

use crate::{
    combat::initiate_combat,
    encounter::rand_encounter,
    enemy::{Enemy, Type},
    item::Item,
    player::{Class, Player},
};

fn main() {
    let name = String::from("player");
    let class = Class::Warrior;
    let mut player = Player::new(name, class);

    let sword = Item::new(String::from("sword"));
    let bow = Item::new(String::from("bow"));
    let items = vec![sword, bow];
    player.inventory.add_many(items);

    println!("{:?}", player.inventory.peek());

    rand_encounter(&mut player);
}
