use std::cmp::Ordering;
use specs::{ ReadStorage, System, WriteStorage};
use crate::{GridCoords, NewGridCoords, PlayerSprite};

pub struct SpriteMovementSys;

impl<'a> System<'a> for SpriteMovementSys {
    type SystemData = (
                       ReadStorage<'a, NewGridCoords>,
                       ReadStorage<'a, GridCoords>,
                       WriteStorage<'a, PlayerSprite>);

    fn run(&mut self, (mut new_coords, mut coords, mut sprite): Self::SystemData) {
        use specs::Join;

        for (new_coord, coord, sprite) in (&new_coords, &coords, &mut sprite).join() {
            match coord.x.cmp(&new_coord.x) {
                Ordering::Less => {
                    sprite.update_frame("right")
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    sprite.update_frame("left")
                }
            }

            match coord.y.cmp(&new_coord.y) {
                Ordering::Less => {
                    sprite.update_frame("vertical")
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    sprite.update_frame("vertical")
                }
            }
        }


    }
}
