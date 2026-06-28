use godot::classes::{Area3D, CollisionShape3D, IArea3D, Input, MeshInstance3D};
use godot::prelude::*;

use crate::Player;
use crate::utils::{assets, load_scene_as};

#[derive(Debug, Clone)]
pub struct DoorData {
    pub position: Vector3,
    pub rotation: Basis,
}

#[derive(GodotClass)]
#[class(base=Area3D, no_init)]
pub struct DoorNode {
    base: Base<Area3D>,
}

#[godot_api]
impl IArea3D for DoorNode {}

// #[godot_api]
impl DoorNode {
    pub fn new(data: &DoorData) -> Gd<Self> {
        let mut door = Gd::from_init_fn(|base| Self { base });
        let mesh = load_scene_as::<MeshInstance3D>(assets::DOOR_MESH);
        let shape = load_scene_as::<CollisionShape3D>(assets::DOOR_SHAPE);
        door.add_child(&mesh);
        door.add_child(&shape);
        door.set_position(data.position);
        door.set_basis(data.rotation);

        door
    }

    pub fn player_is_colliding(&self) -> bool {
        let bodies = self.base().get_overlapping_bodies();
        for body in bodies.iter_shared() {
            if body.try_cast::<Player>().is_ok() {
                return true;
            }
        }
        false
    }
}
