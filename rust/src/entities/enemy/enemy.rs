use godot::classes::{CharacterBody3D, CollisionShape3D, ICharacterBody3D, MeshInstance3D};
use godot::prelude::*;

use crate::Player;
use crate::utils::Assets;

#[derive(GodotConvert, Clone, Copy, Debug)]
#[godot(via = GString)]
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

    pub fn turn_to_life(&self, assets: Gd<Assets>) -> Gd<Enemy> {
        match self.e_class {
            EnemyClass::Goblin => {
                let mut enemie = Enemy::create(assets, self.player.clone(), EnemyClass::Goblin);
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
    mesh: Gd<PackedScene>,
    shape: Gd<PackedScene>,
    pub e_class: EnemyClass,
    // pub name: String,
    player: Gd<Player>,

    pub is_alive: bool,
    speed: f32,
}

#[godot_api]
impl ICharacterBody3D for Enemy {
    fn ready(&mut self) {
        let mesh = self.mesh.clone().instantiate_as::<MeshInstance3D>();
        let shape = self.shape.clone().instantiate_as::<CollisionShape3D>();

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
impl Enemy {
    #[func]
    pub fn create(assets: Gd<Assets>, player: Gd<Player>, e_class: EnemyClass) -> Gd<Enemy> {
        Gd::from_init_fn(|base| Self {
            base,
            mesh: assets.bind().get_scene(GString::from("goblin/mesh")),
            shape: assets.bind().get_scene(GString::from("goblin/shape")),
            e_class: e_class,
            player,

            is_alive: true,
            speed: 3.0,
        })
    }

    pub fn to_data(&self) -> EnemyData {
        EnemyData {
            e_class: self.e_class,
            position: self.base().get_position(),
            player: self.player.clone(),
            is_alive: self.is_alive,
        }
    }
}
