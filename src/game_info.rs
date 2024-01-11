use piston_window::Key::Select;
use specs::prelude::*;
use crate::{BlobMarker, Health};

pub struct GameInfoSys;

#[derive(Default)]
pub struct GameInfo {
    pub blobs_health: Vec<u8>
}



impl<'a> System<'a> for GameInfoSys {
    type SystemData = (ReadStorage<'a, BlobMarker>, ReadStorage<'a, Health>, Write<'a, GameInfo>);

    fn run(&mut self, (blobs, health,mut  game_info): Self::SystemData) {
        let mut gi = GameInfo{blobs_health: Vec::new()};
        for (_, h) in (&blobs, &health).join() {
            gi.blobs_health.push(h.0);
        }

        game_info.blobs_health = gi.blobs_health;

    }
}