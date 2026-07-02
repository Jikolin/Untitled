use godot::classes::{CharacterBody3D, CollisionShape3D, ICharacterBody3D, MeshInstance3D};
use godot::prelude::*;

use crate::Player;
use crate::utils::{AssetMap, get};

#[derive(Clone, Copy)]
pub enum EnemyClass {
    Goblin,
}

#[derive(Clone)]
pub struct EnemyData {
    pub e_class: EnemyClass,
    pub position: Vector3,
    pub player: Gd<Player>,
    pub is_alive: bool,
}

impl EnemyData {
    pub fn new(e_class: EnemyClass, position: Vector3, player: Gd<Player>) -> EnemyData {
        EnemyData {
            e_class,
            position,
            player,
            is_alive: true,
        }
    }

    pub fn turn_to_life(&self, assets: &AssetMap) -> Gd<Enemy> {
        match self.e_class {
            EnemyClass::Goblin => {
                let mut enemie = Enemy::new(assets, self.player.clone(), EnemyClass::Goblin);
                enemie.set_position(self.position);
                enemie.bind_mut().is_alive = self.is_alive;
                enemie
            }
        }
    }
}

#[derive(GodotClass)]
#[class(base = CharacterBody3D, no_init)]
pub struct Enemy {
    base: Base<CharacterBody3D>,
    mesh: Gd<MeshInstance3D>,
    shape: Gd<CollisionShape3D>,
    pub e_class: EnemyClass,
    // pub name: String,
    player: Gd<Player>,

    pub is_alive: bool,
    speed: f32,
}

#[godot_api]
impl ICharacterBody3D for Enemy {
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

impl Enemy {
    pub fn new(assets: &AssetMap, player: Gd<Player>, e_class: EnemyClass) -> Gd<Enemy> {
        Gd::from_init_fn(|base| Self {
            base,
            mesh: get(&assets, "goblin/mesh"),
            shape: get(&assets, "goblin/shape"),
            e_class: e_class,
            player,

            is_alive: true,
            speed: 3.0,
        })
    }

    // pub fn new_ex

    pub fn to_data(&self) -> EnemyData {
        EnemyData {
            e_class: self.e_class,
            position: self.base().get_position(),
            player: self.player.clone(),
            is_alive: self.is_alive,
        }
    }
}
