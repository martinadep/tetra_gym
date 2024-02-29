use std::thread::scope;
use robotics_lib::world::world_generator::{Generator, World};
use tetra::{Context, State, TetraError};
use tetra::graphics::DrawParams;
use tetra::math::Vec2;
use tyrannousarust_rex_world_generator::WorldGenerator;
use crate::textures::get_texture;

const SEED : u32 = 210;
const WORLD_SIZE : usize = 40;
pub(crate) const PIXEL : f32 = 64.0;
pub(crate) const SCALE : f32 = 0.2;
const MAP_OFFSET: Vec2<f32> = Vec2::new(0.0, 0.0);

pub(crate) struct WorldViewer{
    world : World,
}
impl WorldViewer{
    pub(crate) fn new(ctx : &mut Context) -> tetra::Result<WorldViewer>{
        Ok(Self::new_with_params(SEED, WORLD_SIZE))
    }
    pub(crate) fn new_with_params (seed : u32, size : usize) -> Self{
        Self{
            world : WorldGenerator::new()
                .set_seed(seed)
                .set_size(size)
                .gen(),
        }
    }
    pub(crate) fn draw_map (&self, ctx : &mut Context){
        for (x, row) in self.world.0.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                //draw tile
                get_texture(Box::new(tile.tile_type), ctx)
                    .draw(ctx, DrawParams::new()
                        .position(Vec2::new(MAP_OFFSET.x + x as f32 * PIXEL * SCALE,MAP_OFFSET.y + y as f32 * PIXEL * SCALE))
                        .scale(Vec2::new(SCALE, SCALE))
                    );
                //draw content
                get_texture(Box::new(tile.content.clone()), ctx)
                    .draw(ctx, DrawParams::new()
                        .position(Vec2::new(MAP_OFFSET.x + x as f32 * PIXEL * SCALE,MAP_OFFSET.y + y as f32 * PIXEL * SCALE))
                        .scale(Vec2::new(SCALE, SCALE))
                    );
            }
        }
    }
}

impl State for WorldViewer{
    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        self.draw_map(ctx);
        Ok(())
    }
}