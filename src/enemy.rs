use crate::player::Player;

#[derive(Debug)]
pub(crate) enum Type {
    Goblin,
}

#[derive(Debug)]
pub(crate) struct Enemy {
    pub name: String,
    pub enemy_type: Type,
    pub hp: i32,
    pub damage: i32,
}

impl Enemy {
    pub(crate) fn new(enemy_type: Type) -> Self {
        match enemy_type {
            Type::Goblin => Self {
                name: String::from("Goblin"),
                enemy_type,
                hp: 35,
                damage: 8,
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
}
