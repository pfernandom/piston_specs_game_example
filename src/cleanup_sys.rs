use std::sync::{Arc, Mutex};
use nalgebra::Vector4;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, Write, WriteStorage};
use crate::{ActionLock, BlobMarker, Color, GridCoords, GridDimensions, InputEvent, PlayerMarker, Position};
use crate::input_sys::ActionFired;

pub struct CleanupSys;

impl<'a> System<'a> for CleanupSys {
    type SystemData = (
        Entities<'a>,
        Write<'a, InputEvent>,
        WriteStorage<'a, ActionFired>,
        WriteStorage<'a, ActionLock>,
        ReadStorage<'a, PlayerMarker>,);

    fn run(&mut self, (entities, mut input_ev, mut actions, mut locks, player): Self::SystemData) {
        input_ev.0 = None;

        let clean_actions = (&entities, &actions, &player).join()
            .filter(|data|data.1.is_expired())
                .map(|(e,_,_)|e)
            .collect::<Vec<_>>();

        for entity in clean_actions {
            actions.remove(entity);
        }

        let locks_to_clean = (&entities, &locks).join().filter_map(|a|if a.1.is_expired() {
            Some(a.0)
        } else {
            None
        }).collect::<Vec<_>>();
        for lock in locks_to_clean {
            locks.remove(lock);
        }

        // actions.
    }
}