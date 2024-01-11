use std::sync::{Arc, Mutex};
use nalgebra::Vector4;
use piston_window::{Event, Input};
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use crate::{BlobMarker, Color, GridCoords, GridDimensions, InputEvent, PlayerMarker, Position};
use crate::input_sys::ActionFired;

pub struct GridChangesSys;

impl<'a> System<'a> for GridChangesSys {
    type SystemData = (Write<'a, Arc<Mutex<GridDimensions>>>,Read<'a, InputEvent>);

    fn run(&mut self, (grid_dims, input): Self::SystemData) {
        if let Some(ev) = input.0.clone().map(|ev|ev) {
            match ev {
                Event::Input(inp, _) => {
                    match inp {
                        Input::Resize(rargs) => {

                            // println!("rargs.draw_size:{:?}", rargs.draw_size);
                            // println!("rargs.window_size:{:?}", rargs.window_size);
                            let mut g = grid_dims.lock().unwrap();

                            g.window_width = rargs.window_size[0];
                            g.window_height = rargs.window_size[1];

                            // println!("Rows:{}, cols:{}", g.grid_rows(), g.grid_columns());
                        },
                        _ => {}
                    }
                }
                Event::Loop(_) => {}
                Event::Custom(_, _, _) => {}
            }
        }
    }
}