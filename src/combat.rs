use crate::{enemy::Enemy, player::Player};

pub(crate) fn initiate_combat(enemy: &mut Enemy, player: &mut Player) -> () {
    while enemy.is_alive() {
        enemy.attack(player);

        println!(
            "{:?} hit {:?} for {:?} damage! Health: {:?}",
            enemy.enemy_type,
            player.name,
            enemy.damage,
            { if player.hp < 0 { 0 } else { player.hp } }
        );

        if player.is_alive() {
            enemy.take_damage(player.damage);

            println!(
                "{:?} hit {:?} for {:?} damage! Health: {:?}",
                player.name,
                enemy.enemy_type,
                player.damage,
                { if enemy.hp < 0 { 0 } else { enemy.hp } }
            );
        } else {
            println!("You died!");
            break;
        }
    }

    enemy.killed();
}
