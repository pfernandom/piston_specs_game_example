use std::sync::{Arc, Mutex};
use std::time::Duration;
use specs::prelude::*;
use rand::{random, Rng};
use crate::{ActionLock, AIMarker, GridCoords, GridDimensions, NewGridCoords};




pub struct AISys;

impl <'a> System<'a> for AISys {
    type SystemData = (Entities<'a>, ReadStorage<'a, AIMarker>, ReadStorage<'a, GridCoords>, WriteStorage<'a, NewGridCoords>,
                    WriteStorage<'a, ActionLock>,
                       Read<'a, Arc<Mutex<GridDimensions>>>);

    fn run(&mut self, (entities, ai_marker, grid_coord, mut new_grid_coord, mut action_loc, grid_dims): Self::SystemData) {
        let (columns, rows)  = grid_dims.lock().map(|gd| (gd.grid_columns()-1, gd.grid_rows()-1)).unwrap();

        let mut moved_entities = Vec::new();
        for (entity, _, grid_coord, _) in (&entities, &ai_marker, &grid_coord, !&action_loc).join() {
            if rand::random::<bool>() {
                let mut options_x = Vec::new();
                options_x.push(grid_coord.x);
                if grid_coord.x < columns {
                    options_x.push(grid_coord.x+1);
                }
                if grid_coord.x > 0 {
                    options_x.push(grid_coord.x-1);
                }
                let mut r = rand::thread_rng();
                r.shuffle(&mut options_x);
                new_grid_coord.insert(entity, NewGridCoords {
                    x: options_x.get(0).unwrap().clone(),
                    y: grid_coord.y
                });
            } else {
                let mut options_y = Vec::new();
                options_y.push(grid_coord.y);
                if grid_coord.y < rows {
                    options_y.push(grid_coord.y+1);
                }
                if grid_coord.y > 0 {
                    options_y.push(grid_coord.y-1);
                }
                let mut r = rand::thread_rng();
                r.shuffle(&mut options_y);
                new_grid_coord.insert(entity, NewGridCoords {
                    x: grid_coord.x,
                    y: options_y.get(0).unwrap().clone(),
                });
            }
            moved_entities.push(entity);
        }

        for e in moved_entities {
            action_loc.insert(e, ActionLock::new(Duration::from_millis(500)));
        }
    }
}