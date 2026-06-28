use godot::classes::{Camera3D, GridMap, Input};
use godot::prelude::*;

use crate::Player;
use crate::map::Map;

// Game's enter point
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct MainScene {
    base: Base<Node3D>,
    player: Gd<Player>,
    map: Gd<Map>,
    grid_map: Gd<GridMap>,
    camera: Gd<Camera3D>,
}

#[godot_api]
impl INode3D for MainScene {
    fn init(base: Base<Node3D>) -> MainScene {
        let mut map = Map::new((10, 10), 3);
        let grid_map = map.bind_mut().build_grid_map();
        let player = Player::new(map.clone());
        let camera = Camera3D::new_alloc();

        MainScene {
            base,
            player,
            map,
            grid_map,
            camera,
        }
    }

    fn ready(&mut self) {
        let player = self.player.clone();
        let grid_map = self.grid_map.clone();
        let camera = self.camera.clone();
        self.base_mut().add_child(&player);
        self.base_mut().add_child(&grid_map);
        self.base_mut().add_child(&camera);
    }

    fn physics_process(&mut self, delta: f32) {
        self.check_input();
        self.move_camera(delta);
    }
}

impl MainScene {
    fn check_input(&mut self) {
        let input = Input::singleton();
        if input.is_action_just_pressed("interact") {
            if !self.player.bind().is_in_the_room && !self.player.bind().is_moving {
                self.enter_room();
                self.player.bind_mut().enter_room();
            } else if self.player.bind().is_in_the_room && self.map.bind().try_exit_room() {
                self.exit_room();
            }
        }
    }

    fn move_camera(&mut self, delta: f32) {
        let mut camera = self.camera.clone();
        let player_pos = self.player.get_position();
        let cam_pos = camera.get_position();

        if self.player.bind().is_in_the_room {
            let target_basis = Basis::looking_at(player_pos - cam_pos);
            let new_basis = camera.get_basis().slerp(&target_basis, 5.0 * delta);
            camera.set_basis(new_basis);
        } else {
            let target = Vector3::new(player_pos.x + 1.2, 2.5, player_pos.z + 3.0);
            let new_pos = cam_pos.lerp(target, 3.5 * delta);
            camera.set_position(new_pos);
            let target_basis = Basis::looking_at(player_pos - new_pos);
            let new_basis = camera.get_basis().slerp(&target_basis, 5.0 * delta);
            camera.set_basis(new_basis);
        }
    }

    fn enter_room(&mut self) {
        let coords = self.player.bind().get_grid_position(Vector3::ZERO);
        let mut room = self.map.bind_mut().build_room(coords, self.player.clone());
        self.grid_map.set_visible(false);

        let player_pos = self.player.get_position();
        let room_pos = Vector3::new(player_pos.x, 0.5, player_pos.z);
        room.set_position(room_pos);

        let camera_pos = Vector3::new(room_pos.x, 5.0, room_pos.z + 6.0);
        let mut camera = self.camera.clone();
        camera.set_position(camera_pos);
        self.base_mut().add_child(&room);
    }

    fn exit_room(&mut self) {
        self.map.bind_mut().exit_room();
        self.player.bind_mut().exit_room();
        self.grid_map.set_visible(true);
    }
}
