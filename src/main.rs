#![allow(unused)]

mod enemy;
mod player;

use crate::{
    enemy::{Enemy, Type},
    player::{Class, Player},
};

fn main() {
    let name = String::from("player");
    let class = Class::Warrior;

    let mut player = Player::new(name, class);
    let mut goblin = Enemy::new(Type::Goblin);

    while goblin.is_alive() {
        goblin.attack(&mut player);

        println!(
            "{:?} hit {:?} for {:?} damage! Health: {:?}",
            goblin.enemy_type, player.name, goblin.damage, player.hp
        );

        goblin.take_damage(10);

        println!(
            "{:?} hit {:?} for {:?} damage! Health: {:?}",
            player.name, goblin.enemy_type, 10, goblin.hp
        );
    }
}
