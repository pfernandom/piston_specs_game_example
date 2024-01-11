extern crate piston_window;

use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use gfx_device_gl::{CommandBuffer, Factory, Resources};
use nalgebra::Vector4;
use piston_window::*;
use specs::{Builder, Component, DispatcherBuilder, System, VecStorage, World, WorldExt};
use specs::prelude::*;
use specs::shred::Fetch;
use sprite::{Sprite};
use update_position_sys::UpdatePos;
use crate::ai_sys::AISys;
use crate::blob_interaction::BlobInteractionSys;
use crate::cleanup_sys::CleanupSys;
use crate::game_info::{GameInfo, GameInfoSys};
use crate::grid_changes::GridChangesSys;
use crate::health_sys::HealthSys;
use crate::input_sys::{ActionFired, InputSys};
use crate::sprite_movement_sys::SpriteMovementSys;


mod input_sys;
mod update_position_sys;
mod collisions_sys;
mod sprite_movement_sys;
mod blob_interaction;
mod grid_changes;
mod cleanup_sys;
mod health_sys;
mod game_info;
mod ai_sys;


#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct PlayerMarker;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct AIMarker;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct BlobMarker;


#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct CollisionMarker;


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ActionLock {
    duration: Duration,
    created: Instant,
}

impl Default for ActionLock {
    fn default() -> Self {
        ActionLock::new(Duration::from_millis(0))
    }
}

impl ActionLock {
    fn new(duration:Duration) -> Self {
        Self {
            duration,
            created: Instant::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created.elapsed() > self.duration
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    x: f64,
    y: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Color(Vector4<f32>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct GridCoords {
    pub x: u64,
    pub y: u64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Health(u8);

impl Health {
    fn reduce(&mut self, amount: u8) {
        if amount > self.0 {
            self.0 = 0;
        } else {
            self.0 -= amount;
        }

    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Damage(u8);


#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Death;

impl GridCoords {
    fn is_next_to(&self, other: &Self) -> bool {
        let c1x = self.x as i64;
        let c1y = self.y as i64;

        let c2x = other.x as i64;
        let c2y = other.y as i64;

        (c2x-1..=c2x+1).contains(&c1x) && (c2y-1..=c2y+1).contains(&c1y)
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct NewGridCoords {
    pub x: u64,
    pub y: u64,
}


pub struct SpriteFactory {
    assets: PathBuf,
    texture_context: TextureContext<Factory, Resources, CommandBuffer>
}

impl SpriteFactory {
    fn new(assets: PathBuf, window: &mut PistonWindow) -> Self {


        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into()
        };
        Self{
            assets,
            texture_context
        }
    }

    fn get_texture(&mut self, texture_name:&str) ->  Rc<Texture<Resources>> {
        Rc::new(Texture::from_path(
            &mut self.texture_context,
            self.assets.join(texture_name),
            Flip::None,
            &TextureSettings::new()
        ).unwrap())
    }

    fn create_sprite(&mut self, texture_name: &str) -> Sprite<Texture<Resources>> {
        Sprite::from_texture(Rc::clone(&self.get_texture(texture_name)))
    }

    fn create_sprite_from_rect(&mut self, texture_name: &str, rect: [f64; 4])-> Sprite<Texture<Resources>> {
        Sprite::from_texture_rect(Rc::clone(&self.get_texture(texture_name)), rect)
    }
}



#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct PlayerSprite {
    id: &'static str,
    texture: &'static str,
    current_frame: &'static str,
    anchor: (f64, f64),
    frames: HashMap<&'static str, [f64; 4]>
}

impl PlayerSprite {
    fn update_frame(&mut self, new_frame:&'static str) {
        self.current_frame = new_frame;
    }
}

#[derive(Default)]
pub struct GridDimensions {
    window_height:f64,
    window_width: f64,
    // width: u64,
    // height: u64,
    tile_dims: (f64, f64),
    tile_center: f64,
}

impl GridDimensions {
    fn new(window_height:f64,
           window_width: f64) -> Self {
        Self {
            window_height,
            window_width,
            tile_dims: (50.0, 50.0),
            tile_center: 0.5,
        }
    }



    fn find_grid_position(&self, x:u64, y:u64) -> (f64, f64) {
        (self.tile_center + x as f64 * self.tile_dims.0 / 2.0, self.tile_center + y as f64 * self.tile_dims.1 / 2.0)
    }
    fn find_position_for_gridx(&self, x:u64) -> (f64) {
        self.tile_center + x  as f64 * self.tile_dims.0
    }
    fn find_position_for_gridy(&self, y:u64) -> (f64) {
        self.tile_center + y  as f64 * self.tile_dims.1
    }

    fn tile_size(&self) -> (f64, f64) {
        (self.tile_dims.0, self.tile_dims.1)
    }

    fn tile_center(&self) -> f64 {
        self.tile_center
    }

    fn grid_columns(&self) -> u64 {
        (self.window_width / self.tile_dims.0) as u64
    }

    fn grid_rows(&self) -> u64 {
        ((self.window_height- self.tile_dims.1) / self.tile_dims.1) as u64
    }

}

struct WorldDimensions {
    grid: GridDimensions
}

// const GRID_WIDTH: u64 = 11;
// const GRID_HEIGHT: u64 = 6;
//
// pub const TILE_DIMS: (f64, f64) = (50.0, 50.0);
//
// const TILE_CENTER: f64 = 5.0;

#[derive(Default)]
pub struct InputEvent(Option<Event>);

// #[derive(Default)]
// pub struct PositionsMap(HashMap<GridCoords, Entity>);

fn main() {
    let opengl = OpenGL::V3_2;
    let (window_width, window_height) = (640, 480);
    let mut grid_dimensions = GridDimensions::new(window_width as f64, window_height as f64);
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [window_width, window_height])
        .exit_on_esc(true).graphics_api(opengl).build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    let mut sprite_factory = SpriteFactory::new(assets.clone(), &mut window);



    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<GridCoords>();
    world.register::<PlayerMarker>();
    world.register::<AIMarker>();
    world.register::<BlobMarker>();
    world.register::<NewGridCoords>();
    world.register::<PlayerSprite>();
    world.register::<ActionFired>();
    world.register::<Color>();
    world.register::<Health>();
    world.register::<Damage>();
    world.register::<Death>();
    world.register::<ActionLock>();
    world.insert::<InputEvent>(InputEvent(None));
    // world.insert::<PositionsMap>(PositionsMap(HashMap::new()));

    // Only the second entity will get a position update,
    // because the first one does not have a velocity.
    // world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();
    world
        .create_entity()
        .with(PlayerMarker{})
        .with(PlayerSprite{
            id: "player",
            texture: "person2.png",
            current_frame: "vertical",
            anchor: (64.0/2.0, 64.0/2.0),
            frames: [("vertical", [0.0, 0.0, 64.0, 64.0]),
                ("left", [64.0,0.0,64.0,64.0]),
                ("right", [0.0,64.0,64.0,64.0])].iter().cloned().collect()
        })
        .with(Position { x: grid_dimensions.find_position_for_gridx(0), y: grid_dimensions.find_position_for_gridy(0) })
        .with(Velocity { x: 0.0, y: 0.0 })
        .with(GridCoords {x: 0, y:0})
        .with(Health(100))
        .build();


    world
        .create_entity()
        .with(AIMarker{})
        .with(PlayerSprite{
            id: "ai1",
            texture: "person2.png",
            current_frame: "vertical",
            anchor: (64.0/2.0, 64.0/2.0),
            frames: [("vertical", [0.0, 0.0, 64.0, 64.0]),
                ("left", [64.0,0.0,64.0,64.0]),
                ("right", [0.0,64.0,64.0,64.0])].iter().cloned().collect()
        })
        .with(Position { x: grid_dimensions.find_position_for_gridx(4), y: grid_dimensions.find_position_for_gridy(4) })
        .with(Velocity { x: 0.0, y: 0.0 })
        .with(GridCoords {x: 4, y:4})
        .with(Health(100))
        .build();


    world
        .create_entity()
        .with(BlobMarker{})
        .with(Position { x: grid_dimensions.find_position_for_gridx(3), y: grid_dimensions.find_position_for_gridy(3) })
        .with(Velocity { x: 0.0, y: 0.0 })
        .with(GridCoords {x: 3, y:3})
        .with(Health(100))
        .with(Color(Vector4::new(0.0, 1.0, 0.0, 1.0)))
        .build();


    world
        .create_entity()
        .with(BlobMarker{})
        .with(Position { x: grid_dimensions.find_position_for_gridx(0), y: grid_dimensions.find_position_for_gridy(1) })
        .with(Velocity { x: 0.0, y: 0.0 })
        .with(GridCoords {x: 0, y:1})
        .with(Health(100))
        .with(Color(Vector4::new(0.0, 1.0, 0.0, 1.0)))
        .build();

    let grid_dimensions = Arc::new(Mutex::new(grid_dimensions));
    world.insert::<Arc<Mutex<GridDimensions>>>(Arc::clone(&grid_dimensions));
    // world.insert::<Instant>(Instant::now());
    world.insert::<GameInfo>(GameInfo::default());


    let mut dispatcher = DispatcherBuilder::new()
        .with(InputSys, "input", &[])
        .with(AISys, "ai_sys", &["input"])
        .with(SpriteMovementSys, "sprite_updates", &["input", "ai_sys"])
        .with(UpdatePos, "update_pos", &["input"])
        .with(BlobInteractionSys, "blob_interaction", &["update_pos"])
        .with(HealthSys, "health", &["blob_interaction"])
        .with(GridChangesSys, "grid_changes", &["input"])
        .with(CleanupSys, "cleanup", &["input", "grid_changes"])
        .with(GameInfoSys, "game_info", &["cleanup"])
        .build();

    let mut step = 0;

    let mut sprite_map = HashMap::new();

    // initialize all sprites
    {
        let sprite_components = world.read_storage::<PlayerSprite>();
        let positions = world.read_storage::<Position>();


        for (sprite, pos) in (&sprite_components, &positions).join() {
            println!("Sprite! {:?}", pos);
            let mut s = sprite_factory.create_sprite_from_rect(sprite.texture, sprite.frames.get(&sprite.current_frame).unwrap().clone());
            s.set_position(pos.x, pos.y);
            s.set_anchor(0.0, 0.0);
            sprite_map.insert(sprite.id, s);
        }
    }

    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();


    while let Some(event) = window.next() {
        world.insert::<InputEvent>(InputEvent(Some(event.clone())));

        dispatcher.dispatch(&mut world);
        world.maintain();


        // println!("time: {:?}", start.elapsed());


        window.draw_2d(&event, |context, graphics, _| {
            clear([1.0, 1.0, 1.0, 1.0], graphics);

            draw_grid(context, graphics, Arc::clone(&grid_dimensions));



            let sprite_components = world.read_storage::<PlayerSprite>();
            let positions = world.read_storage::<Position>();
            let colors = world.read_storage::<Color>();
            let players = world.read_storage::<PlayerMarker>();
            let health = world.read_storage::<Health>();
            let death_entities = world.read_storage::<Death>();

            let game_info = world.read_resource::<GameInfo>();


            let tile_dims = grid_dimensions.lock().unwrap().tile_dims.clone();



            draw_text_box(game_info, &mut glyphs, &context, graphics, Arc::clone(&grid_dimensions));

            let mut overlays = Vec::new();

            // Draw entities
            for (pos, color, _, _, h) in (&positions, &colors, !&players, !&death_entities, &health).join() {
                // println!("pos: x:{}, y:{}", pos.x, pos.y);
                ellipse([color.0[0], color.0[1], color.0[2], color.0[3]], // Red color
                        [pos.x, pos.y, tile_dims.0, tile_dims.1], // x, y, width, height
                        context.transform, graphics);

                overlays.push((format!("H: {}", h.0), pos.x, pos.y));
            }

            for (pos, sprite, h) in (&positions, &sprite_components, &health).join() {
                let mut s = sprite_map.get_mut(&sprite.id).unwrap();

                s.set_src_rect(sprite.frames.get(&sprite.current_frame).unwrap().clone());
                s.set_position(pos.x,  pos.y);
                overlays.push((format!("H: {}", h.0), pos.x, pos.y));
                s.draw(context.transform, graphics);
            }

            for (text, x, y) in overlays {
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 13).draw(
                    text.as_str(),
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(x, y),
                    graphics
                ).unwrap();
            }

        });

        glyphs.factory.encoder.flush(&mut window.device);

    }
}

fn draw_text_box(game_info: Fetch<GameInfo>, mut glyphs: &mut Glyphs, context: &Context, graphics: &mut G2d, grid_dims:Arc<Mutex<GridDimensions>>) {
    let grid_dims = grid_dims.lock().unwrap();
    let tile_size = grid_dims.tile_size();
    let tile_center = grid_dims.tile_center();
    let windows_height = grid_dims.window_height;
    let windows_width = grid_dims.window_width;

    let text_pane_height = tile_size.1 * 1.5;

    let text_transform = context.transform.trans(10.0, windows_height - text_pane_height);
    rectangle([0.0, 0.0, 0.0, 1.0], [0.0, 0.0, windows_width - tile_size.0, text_pane_height], text_transform, graphics);

    let section_width = 150.0;
    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 12).draw(
        format!("Alive blobs: {}", game_info.blobs_health.iter().filter(|h|**h > 0).count()).as_str(),
        glyphs,
        &context.draw_state,
        text_transform.trans(tile_size.0, 24.0), graphics
    ).unwrap();


    // text::Text::new_color([1.0, 1.0, 1.0, 1.0], 12).draw(
    //     "Hello world2!",
    //     glyphs,
    //     &context.draw_state,
    //     text_transform.trans(section_width, 24.0), graphics
    // ).unwrap();
}

fn draw_grid(context: Context, graphics: &mut G2d, grid_dims:Arc<Mutex<GridDimensions>>) {
    let grid_dims = grid_dims.lock().unwrap();
    let tile_size = grid_dims.tile_size();
    let tile_center = grid_dims.tile_center();
    for row in 0..grid_dims.grid_rows() + 2 {
        let i = ((row as f64) * tile_size.1) + tile_center;
        line_from_to([1.0, 0.5, 1.0, 1.0], 1.0, [tile_center, i], [tile_size.1 * (grid_dims.grid_columns() as f64 + 1.1), i],
                     context.transform, graphics);
    }


    for row in 0..grid_dims.grid_columns() + 2 {
        let i = ((row as f64) * tile_size.0) + tile_center;
        line_from_to([1.0, 0.5, 1.0, 1.0], 1.0, [i, tile_center], [i, tile_size.0 * (grid_dims.grid_rows() as f64 + 1.1)],
                     context.transform, graphics);
    }
}
