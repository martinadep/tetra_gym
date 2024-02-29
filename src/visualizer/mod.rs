use tetra::graphics::{DrawParams, Texture};
use tetra::{Context, State, TetraError};
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;

pub(crate) struct Visualizer{
    pub(crate) robot_texture : Texture,
    pub (crate) action_made_text : Text,
}
impl Visualizer {
    pub(crate) fn new(ctx : &mut Context) -> tetra::Result<Visualizer>{
        Ok(Self{
            robot_texture : Texture::new(ctx, "src/textures/utils/player.png")
                .expect("couldn't upload robot texture"),
            action_made_text: Text::new(
                format!("Robot has been created"),
                Font::vector(ctx, "src/fonts/roboto.ttf", 17.0)?
            ),
        })
    }
}

impl Clone for Visualizer {
    fn clone(&self) -> Self {
        Self{
            robot_texture: Texture::from(self.robot_texture.clone()),
            action_made_text: Text::from(self.action_made_text.clone()),
        }
    }
}

impl State for Visualizer {
    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        self.robot_texture.draw(ctx, DrawParams::new()
            .position(Vec2::new(100.0,100.0))
            .scale(Vec2::new(0.5, 0.5))
        );
        self.action_made_text.draw(ctx, DrawParams::new()
            .position(Vec2::new(10.0,10.0))
            .scale(Vec2::new(1.0, 1.0))
        );
        Ok(())
    }
}