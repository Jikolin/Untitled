use crate::Player;
use crate::enemy::Goblin;
use godot::prelude::*;

#[derive(Clone)]
pub enum EnemyClass {
    Goblin,
}

#[derive(Clone)]
pub struct EnemyData {
    pub class: EnemyClass,
    pub position: Vector3,
    pub player: Gd<Player>,
    pub is_alive: bool,
}

impl EnemyData {
    pub fn turn_to_life(&self) -> Gd<Node3D> {
        match self.class {
            EnemyClass::Goblin => {
                let mut enemie = Goblin::new(self.player.clone());
                enemie.set_position(self.position);
                enemie.bind_mut().is_alive = self.is_alive;
                enemie.upcast::<Node3D>()
            }
        }
    }
}

// pub trait Enemy {
//     pub fn turn_to_data(&self) -> EnemyData {
//         EnemyData {
//             class: self.bind().class,
//             position: self.get_position(),
//             player: self.player,
//         }
//     }
// }
