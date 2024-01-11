use std::cmp::Ordering;
use std::sync::{Arc, Mutex};
use specs::{Entities, Read, ReadStorage, System, WriteStorage};
use crate::{GridCoords, GridDimensions, NewGridCoords, Position};

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (Entities<'a>,
                        Read<'a, Arc<Mutex<GridDimensions>>>,
                        WriteStorage<'a, NewGridCoords>,
                       WriteStorage<'a, GridCoords>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, (entities, grid_dims, mut new_coords, mut coords, mut pos): Self::SystemData) {
        use specs::Join;

        // let x_step = grid_dims.tile_dims.0;
        // let y_step = grid_dims.tile_dims.1;

        let mut updated_entities = Vec::new();

        for (entity, new_ccord, coord, pos) in (&entities, &new_coords, &mut coords, &mut pos).join() {
            // let mut x_diff = (coord.x.abs_diff(new_ccord.x) as f64);
            // let mut y_diff = (coord.y.abs_diff(new_ccord.y) as f64);
            //
            // match coord.x.cmp(&new_ccord.x) {
            //     Ordering::Less => {
            //         // sprite
            //     }
            //     Ordering::Equal => {}
            //     Ordering::Greater => {
            //         x_diff *= -1.0;
            //     }
            // }
            //
            // match coord.y.cmp(&new_ccord.y) {
            //     Ordering::Less => {}
            //     Ordering::Equal => {}
            //     Ordering::Greater => {
            //         y_diff *= -1.0;
            //     }
            // }

            let grid_dims = grid_dims.lock().unwrap();

            pos.x  = grid_dims.find_position_for_gridx(new_ccord.x);
            pos.y = grid_dims.find_position_for_gridx(new_ccord.y);
            coord.x = new_ccord.x;
            coord.y = new_ccord.y;
            updated_entities.push(entity);
        }

        for entity in updated_entities {
            new_coords.remove(entity);
        }

    }
}
