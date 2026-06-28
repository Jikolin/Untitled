use godot::classes::{Area3D, MeshInstance3D};
use godot::prelude::*;

use crate::Player;
use crate::utils::{assets, load_scene_as};

#[derive(Clone)]
pub struct ItemData {
    pub slot_cost: i32,
    pub kind: ItemKind,
}

impl ItemData {
    pub fn to_node(&self, position: Vector3) -> Gd<Item> {
        Item::new(self.kind.clone(), position)
    }
}

#[derive(Clone)]
pub enum ItemKind {
    Potion,
    Weapon,
}

#[derive(GodotClass)]
#[class(base=Node3D, no_init)]
pub struct Item {
    base: Base<Node3D>,
    mesh: Gd<Node3D>,
    interact_box: Gd<Area3D>,
    data: ItemData,
    elapsed: f32,
    base_position: Vector3,
}

#[godot_api]
impl INode3D for Item {
    fn physics_process(&mut self, delta: f32) {
        self.elapsed += delta;

        let bob = self.elapsed.sin() * 0.15;
        let mut rot = self.base().get_rotation();
        rot.y += 1.5 * delta;
        self.base_mut().set_rotation(rot);

        let base_position = self.base_position;
        self.base_mut()
            .set_position(base_position + Vector3::new(0.0, bob, 0.0));
    }
}

impl Item {
    pub fn new(kind: ItemKind, mut position: Vector3) -> Gd<Self> {
        let mesh = load_scene_as::<Node3D>(assets::POTION_MESH);
        let interact_box = load_scene_as::<Area3D>(assets::ITEM_INTERACT_BOX);
        let data = ItemData { slot_cost: 1, kind };

        let mut item = Gd::from_init_fn(|base| Self {
            base,
            mesh: mesh.clone(),
            interact_box: interact_box.clone(),
            elapsed: 0.0,
            data,
            base_position: position,
        });

        position.y += 0.1;
        item.set_position(position);
        item.add_child(&mesh);
        item.add_child(&interact_box);

        item
    }
}
