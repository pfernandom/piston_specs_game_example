use nalgebra::Vector4;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};
use crate::{BlobMarker, Color, Damage, GridCoords, PlayerMarker};
use crate::input_sys::ActionFired;

pub struct BlobInteractionSys;

impl<'a> System<'a> for BlobInteractionSys {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, PlayerMarker>,
        ReadStorage<'a, BlobMarker>,
        ReadStorage<'a, GridCoords>,
        WriteStorage<'a, ActionFired>,
        WriteStorage<'a, Damage>,
        WriteStorage<'a, Color>
    );

    fn run(&mut self, (entities, player, blob, coords, mut actions,  mut damage, mut colors): Self::SystemData) {

        let player_actions = (&entities, &player, &actions).join().filter(|data|!data.2.is_handled()).map(|(e,_,_)| e).collect::<Vec<_>>();

        for (_, c1) in (&player, &coords).join() {
            for (blob_entity, _, c2, color) in (&entities, &blob, &coords, &mut colors).join() {
                if c1.is_next_to(c2) {
                    if player_actions.len() > 0 {
                        color.0 = Vector4::new(1.0, 0.0, 0.0, 1.0);
                        damage.insert(blob_entity, Damage(5)).expect("Damage component added");
                    } else {
                        color.0 = Vector4::new(0.0, 0.0, 1.0, 1.0)
                    }
                } else {
                    color.0 = Vector4::new(0.0, 1.0, 0.0, 1.0)
                }
            }
        }

        for e in player_actions {
            let mut action = actions.get_mut(e).unwrap();
            action.mark_as_handled();
            // actions.remove(e);
        }


    }
}