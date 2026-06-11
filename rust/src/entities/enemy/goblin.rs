use godot::classes::{CharacterBody3D, CollisionShape3D, ICharacterBody3D, MeshInstance3D};
use godot::prelude::*;

use crate::enemy::{EnemyClass, EnemyData};
use crate::player::Player;
use crate::utils::{assets, load_scene_as};

#[derive(GodotClass)]
#[class(base=CharacterBody3D, no_init)]
pub struct Goblin {
    base: Base<CharacterBody3D>,
    mesh: Gd<MeshInstance3D>,
    shape: Gd<CollisionShape3D>,
    pub player: Gd<Player>,
    pub class: EnemyClass,
    speed: f32,
    pub is_alive: bool,
}

#[godot_api]
impl ICharacterBody3D for Goblin {
    fn ready(&mut self) {
        let mesh = self.mesh.clone();
        let shape = self.shape.clone();

        self.base_mut().add_child(&mesh);
        self.base_mut().add_child(&shape);
    }

    fn physics_process(&mut self, _delta: f32) {
        let player_pos = self.player.get_position();
        let new_pos = self.base().get_global_position();

        let direction = (player_pos - new_pos).normalized();
        let velocity = direction * self.speed;

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Goblin {
    pub fn new(player: Gd<Player>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            mesh: load_scene_as::<MeshInstance3D>(assets::GOBLIN_MESH),
            shape: load_scene_as::<CollisionShape3D>(assets::GOBLIN_SHAPE),
            player,
            class: EnemyClass::Goblin,
            speed: 3.0,
            is_alive: true,
        })
    }

    pub fn to_data(&self) -> EnemyData {
        EnemyData {
            class: EnemyClass::Goblin,
            position: self.base().get_position(),
            player: self.player.clone(),
            is_alive: self.is_alive,
        }
    }
}
