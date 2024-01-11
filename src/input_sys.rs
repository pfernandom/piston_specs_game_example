use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};
use specs::{Component, Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System, Write, WriteStorage};
use piston_window::{Button, ButtonState, Event, Input, Key};
use specs::prelude::*;
use crate::{GridCoords, GridDimensions, InputEvent, NewGridCoords, PlayerMarker, Position, Velocity};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ActionFired{
    duration: Duration,
    created: Instant,
    handled: bool
}

impl Default for ActionFired {
    fn default() -> Self {
        Self {
            duration:Duration::default(),
            created: Instant::now(),
            handled:false,
        }
    }
}

impl ActionFired {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            created: Instant::now(),
            handled:false,
        }
    }
    pub fn is_expired(&self) -> bool {
        self.created.elapsed() > self.duration
    }

    pub fn is_handled(&self) -> bool {
        self.handled
    }

    pub fn mark_as_handled(&mut self) {
        self.handled  = true;
    }
}


pub struct InputSys;
impl<'a> System<'a> for InputSys {
    type SystemData = (Entities<'a>, Read<'a, InputEvent>, ReadStorage<'a, Position>, WriteStorage<'a, Velocity>,
        WriteStorage<'a, GridCoords>, ReadStorage<'a, PlayerMarker>, Read<'a, Arc<Mutex<GridDimensions>>>, Read<'a, LazyUpdate>,);

    fn run(&mut self, (mut entities, mut inp,pos, mut vs, mut grid_coords, player, grid_dims, updater): Self::SystemData) {

        let mut coords_to_update = Vec::new();
        if let Some((btn, btn_state)) =  inp.0.clone().map(|event| match event.clone() {
            Event::Input(inp, _) => {
                if let Input::Button(btn) = inp {
                    Some((btn.button, btn.state))
                } else {
                    None
                }

            }
            Event::Loop(_) => {None}
            Event::Custom(_, _, _) => {None}
        }).flatten() {




            for (entity, _, v, grid_coord, _) in (&entities, &pos, &mut vs, &mut grid_coords, &player).join() {
                if btn_state == ButtonState::Press {
                match btn {
                    Button::Keyboard(k) => {
                        Self::add_location_update(&mut coords_to_update, entity, grid_coord, k.clone(), Arc::clone(&grid_dims));

                        if let Key::D = k {
                            updater.insert(entity, ActionFired::new(Duration::from_millis(100)))
                        }
                    }
                    _ =>{}
                }
                } else {
                    // v.x = 0.0;
                    // v.y = 0.0;
                }
            }
        }

        for (entity, new_coord) in coords_to_update {
            // let stone = entities.create();
            updater.insert(entity, new_coord);
        }
    }
}

impl InputSys {
    fn add_location_update(coords_to_update: &mut Vec<(Entity, NewGridCoords)>, entity: Entity, grid_coord: &GridCoords, k: Key, grid_dims: Arc<Mutex<GridDimensions>>) {
        let (columns, rows)  = grid_dims.lock().map(|gd| (gd.grid_columns()-1, gd.grid_rows()-1)).unwrap();

        match k {
            Key::Right => {
                if grid_coord.x + 1 <= columns {
                    coords_to_update.push((entity, NewGridCoords {
                        x: grid_coord.x + 1,
                        y: grid_coord.y
                    }))
                }
            }
            Key::Left => {
                if grid_coord.x > 0 {
                    coords_to_update.push((entity, NewGridCoords {
                        x: grid_coord.x - 1,
                        y: grid_coord.y
                    }))
                }
            }
            Key::Down => {
                if grid_coord.y + 1 <= rows {
                    coords_to_update.push((entity, NewGridCoords {
                        x: grid_coord.x,
                        y: grid_coord.y + 1
                    }))
                }
            }
            Key::Up => {
                if grid_coord.y > 0 {
                    coords_to_update.push((entity, NewGridCoords {
                        x: grid_coord.x,
                        y: grid_coord.y - 1
                    }))
                }
            }
            _ => {}
        }
    }
}
