use godot::classes::{Area3D, MeshInstance3D};
use godot::prelude::*;

use crate::utils::Assets;

#[derive(Clone)]
pub struct ItemData {
    pub slot_cost: i32,
    pub kind: ItemKind,
}

impl ItemData {
    pub fn to_node(&self, assets: Gd<Assets>, position: Vector3) -> Gd<Item> {
        Item::new(assets, self.kind.clone(), self.slot_cost, position)
    }
}

#[derive(GodotConvert, Clone, Copy, Debug)]
#[godot(via = GString)]
pub enum ItemKind {
    Potion,
    Weapon,
}

#[derive(GodotClass)]
#[class(base=Node3D, no_init)]
pub struct Item {
    base: Base<Node3D>,
    mesh: Gd<MeshInstance3D>,
    interact_box: Gd<Area3D>,
    elapsed: f32,
    base_position: Vector3,
    bob_speed: f32,
    rotation_speed: f32,
    bob_amplitude: f32,
    slot_cost: i32,
    kind: ItemKind,
}

#[godot_api]
impl INode3D for Item {
    fn ready(&mut self) {
        let mesh = self.mesh.clone();
        let interact_box = self.interact_box.clone();

        self.base_mut().add_child(&mesh);
        self.base_mut().add_child(&interact_box);
    }
    fn physics_process(&mut self, delta: f32) {
        self.elapsed += delta;

        let bob = self.elapsed.mul_add(self.bob_speed, 0.0).sin() * self.bob_amplitude;
        let mut rot = self.base().get_rotation();
        rot.y += self.rotation_speed * delta;
        self.base_mut().set_rotation(rot);

        let base_position = self.base_position;
        self.base_mut()
            .set_position(base_position + Vector3::new(0.0, bob, 0.0));
    }
}

impl Item {
    pub fn new(
        assets: Gd<Assets>,
        kind: ItemKind,
        slot_cost: i32,
        mut position: Vector3,
    ) -> Gd<Self> {
        // let mesh = load_scene_as::<Node3D>(assets::POTION_MESH);
        let mesh = assets
            .bind()
            .get_scene(GString::from("potion/health"))
            .instantiate_as::<MeshInstance3D>();
        let interact_box = assets
            .bind()
            .get_scene(GString::from("interact_box"))
            .instantiate_as::<Area3D>();
        let slot_cost = slot_cost.max(1);
        let cost_factor = slot_cost as f32;
        let bob_speed = 2.0 / cost_factor;
        let rotation_speed = 2.5 / cost_factor;
        let bob_amplitude = 0.12 + 0.02 * cost_factor;

        let mut item = Gd::from_init_fn(|base| Self {
            base,
            mesh: mesh.clone(),
            interact_box: interact_box.clone(),
            elapsed: 0.0,
            base_position: position,
            bob_speed,
            rotation_speed,
            bob_amplitude,
            slot_cost: 1,
            kind: ItemKind::Potion,
        });

        position.y += 1.0;
        item.bind_mut().base_position = position;
        item.set_position(position);

        item
    }

    // pub fn to_data(&self) -> ItemData {}
}
