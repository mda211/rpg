use crate::{
    combat::initiate_combat,
    enemy::{self, Enemy, Type},
    item::Item,
    player::Player,
};

pub(crate) fn rand_encounter(player: &mut Player) -> () {
    let encounter = rand::random::<u8>() % 3;
    match encounter {
        0 => {
            println!("A Goblin appears!");

            let mut goblin = Enemy::new(Type::Goblin);
            let mut gold = Item::new(String::from("gold"));
            goblin.inventory.add(gold);

            initiate_combat(&mut goblin, player);
        }
        1 => println!("You stumble upon a hidden chest."),
        2 => println!("A traveling merchant greets you."),
        _ => {}
    }
}
