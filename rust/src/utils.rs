use godot::prelude::*;

use std::collections::HashMap;

pub type AssetMap = HashMap<&'static str, Gd<PackedScene>>;

pub fn get<T: Inherits<Node>>(assets: &AssetMap, key: &str) -> Gd<T> {
    assets[key].clone().instantiate_as::<T>()
}

#[derive(GodotClass, Debug)]
#[class(base=Node, init)]
pub struct Assets {
    base: Base<Node>,
    #[init(val=HashMap::new())]
    scenes: HashMap<&'static str, Gd<PackedScene>>,
}

#[godot_api]
impl INode for Assets {
    fn ready(&mut self) {
        let scenes = &mut self.scenes;
        preload_assets(scenes);
    }
}

#[godot_api]
impl Assets {
    #[func]
    pub fn get_scene(&self, key: GString) -> Gd<PackedScene> {
        self.scenes[key.to_string().as_str()].clone()
    }
}

pub fn preload_assets(map: &mut HashMap<&'static str, Gd<PackedScene>>) {
    map.insert("player/mesh", load_asset("player/mesh"));
    map.insert("player/shape", load_asset("player/shape"));

    map.insert("goblin/mesh", load_asset("goblin/mesh"));
    map.insert("goblin/shape", load_asset("goblin/shape"));

    map.insert("interact_box", load_asset("interact_box"));
    map.insert("potion/health", load_asset("potion/health"));
    map.insert("weapon/spear", load_asset("weapon/spear"));

    map.insert("door/mesh", load_asset("door/mesh"));
    map.insert("door/shape", load_asset("door/shape"));
    map.insert("room", load_asset("room"));
}

pub fn load_resource<T: Inherits<Resource>>(path: &str) -> Gd<T> {
    load::<T>(path)
}

// Examples: player/mesh, goblin/shape, labyrinth/tres
// Exceptions: potion/parameter: potion/health, potion/mana
fn load_asset(key: &str) -> Gd<PackedScene> {
    let keys: Vec<&str> = key.split('/').collect();
    let path = match keys[0] {
        "player" => format!("entities/player/{}", keys[1]),
        "goblin" | "skeleton" => format!("entities/enemies/{}/{}", keys[0], keys[1]),
        "potion" => format!("entities/items/potions/{}", keys[1]),
        "weapon" => format!("entities/items/weapons/{}", keys[1]),
        "door" => format!("map/{}_{}", keys[0], keys[1]),

        // EXCEPTIONS
        "interact_box" => format!("entities/items/interact_box"),
        "room" | "labyrinth" => format!("map/{}", keys[0]),

        _ => panic!("unknown asset: {}", keys[0]),
    };

    load::<PackedScene>(&format!("res://assets/{path}.tscn"))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dir2i;
impl Dir2i {
    pub const UP: Vector2i = Vector2i::new(0, -1);
    pub const RIGHT: Vector2i = Vector2i::new(1, 0);
    pub const DOWN: Vector2i = Vector2i::new(0, 1);
    pub const LEFT: Vector2i = Vector2i::new(-1, 0);

    pub fn all() -> [Vector2i; 4] {
        [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT]
    }
}

#[derive(Clone, Copy)]
pub struct Dir3;
impl Dir3 {
    pub const UP: Vector3 = Vector3::new(0.0, 0.0, -1.0);
    pub const RIGHT: Vector3 = Vector3::new(1.0, 0.0, 0.0);
    pub const DOWN: Vector3 = Vector3::new(0.0, 0.0, 1.0);
    pub const LEFT: Vector3 = Vector3::new(-1.0, 0.0, 0.0);

    pub fn all() -> [Vector3; 4] {
        [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT]
    }
}
