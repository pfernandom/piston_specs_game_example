// use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};
// use piston_window::{Button, ButtonState, Key};
// use crate::{GRID_HEIGHT, GRID_WIDTH, GridCoords, InputEvent, PlayerMarker, Position, Velocity};
//
// pub struct CollisionsSys;
// impl<'a> System<'a> for CollisionsSys {
//     type SystemData = (Entities<'a>, ReadStorage<'a, Position>, ReadStorage<'a, Velocity>,
//                        ReadStorage<'a, GridCoords>);
//
//     fn run(&mut self, (entities, positions, velocity, coords): Self::SystemData) {
//         // println!("Print events:");
//         let x_step = 50.0;
//         let y_step = 50.0;
//
//
//         if let Some((btn, btn_state)) =  inp.0.pop() {
//             for (_, v, grid_coord, _) in (&pos, &mut vs, &mut grid_coords, &player).join() {
//                 if btn_state == ButtonState::Press {
//                     match btn {
//                         Button::Keyboard(k) => {
//                             match k {
//                                 Key::Right => {
//                                     if grid_coord.x + 1 <= GRID_WIDTH {
//                                         v.x = x_step;
//                                         grid_coord.x += 1;
//                                     }
//                                 }
//                                 Key::Left => {
//                                     if grid_coord.x > 0 {
//                                         v.x = -x_step;
//                                         grid_coord.x -= 1;
//                                     }
//                                 }
//                                 Key::Down => {
//                                     if grid_coord.y + 1 <= GRID_HEIGHT {
//                                         v.y = y_step;
//                                         grid_coord.y += 1;
//                                     }
//                                 }
//                                 Key::Up => {
//                                     if grid_coord.y > 0 {
//                                         v.y = -y_step;
//                                         grid_coord.y -= 1;
//                                     }
//                                 }
//                                 _ => {}
//                             }
//                         }
//                         _ =>{}
//                     }
//                 } else {
//                     v.x = 0.0;
//                     v.y = 0.0;
//                 }
//             }
//         } else {
//             for (_, v) in (&pos, &mut vs).join() {
//                 v.x = 0.0;
//                 v.y = 0.0;
//             }
//         }
//     }
// }
