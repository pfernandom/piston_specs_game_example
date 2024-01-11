use specs::prelude::*;
use crate::{Damage, Death, Health};

pub struct HealthSys;

impl<'a> System<'a> for HealthSys {
    type SystemData = (Entities<'a>, WriteStorage<'a, Health>, WriteStorage<'a, Damage>, WriteStorage<'a, Death>);

    fn run(&mut self, (entities, mut health, mut damage, mut death): Self::SystemData) {

        let mut cleanup = Vec::new();

        for (entity, h, d) in (&entities, &mut health, &damage).join() {
            h.reduce(d.0);

            if h.0 == 0 {
                death.insert(entity, Death).unwrap();
            }

            cleanup.push(entity);
        }

        for e in cleanup {
            damage.remove(e);
        }
    }
}