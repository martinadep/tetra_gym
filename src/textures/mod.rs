use tetra::Context;
use tetra::graphics::Texture;
use robotics_lib::runner::Robot;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::tile::TileType::*;

pub (crate) trait Texturizable {
    fn get_texture(&self, ctx: &mut Context) -> Texture;
}
impl Texturizable for TileType{
    fn get_texture(&self, ctx: &mut Context) -> Texture {
        let res ;
        res = match self{
            DeepWater => Texture::new(ctx, "src/textures/utils/tiles/DeepWater.png"),
            ShallowWater => Texture::new(ctx, "src/textures/utils/tiles/ShallowWater.png"),
            Sand => Texture::new(ctx, "src/textures/utils/tiles/Sand.png"),
            Grass => Texture::new(ctx, "src/textures/utils/tiles/Grass.png"),
            Street => Texture::new(ctx, "src/textures/utils/tiles/Street.png"),
            Hill => Texture::new(ctx, "src/textures/utils/tiles/Hill.png"),
            Mountain => Texture::new(ctx, "src/textures/utils/tiles/Mountain.png"),
            Snow => Texture::new(ctx, "src/textures/utils/tiles/Snow.png"),
            Lava => Texture::new(ctx, "src/textures/utils/tiles/Lava.png"),
            Teleport(_) => Texture::new(ctx, "src/textures/utils//tiles/Teleport.png"),
            Wall => Texture::new(ctx, "src/textures/utils/tiles/Wall.png"),
        };
        res.expect("couldn't upload tile texture")
    }
}

impl Texturizable for Content{
    fn get_texture(&self, ctx: &mut Context) -> Texture {
        let res ;
        res = match self {
            Content::Rock(_) => Texture::new(ctx, "src/textures/utils/content/Rock.png"),
            Content::Tree(_) => Texture::new(ctx, "src/textures/utils/content/Tree.png"),
            Content::Garbage(_) => Texture::new(ctx, "src/textures/utils/content/Garbage.png"),
            Content::Fire => Texture::new(ctx, "src/textures/utils/content/Fire.png"),
            Content::Coin(_) => Texture::new(ctx, "src/textures/utils/content/Coin.png"),
            Content::Bin(_) => Texture::new(ctx, "src/textures/utils/content/Bin.png"),
            Content::Crate(_) => Texture::new(ctx, "src/textures/utils/content/Crate.png"),
            Content::Bank(_) => Texture::new(ctx, "src/textures/utils/content/Bank.png"),
            Content::Market(_) => Texture::new(ctx, "src/textures/utils/content/Market.png"),
            Content::Fish(_) => Texture::new(ctx, "src/textures/utils/content/Fish.png"),
            Content::Building => Texture::new(ctx, "src/textures/utils/content/Building.png"),
            Content::Bush(_) => Texture::new(ctx, "src/textures/utils/content/Bush.png"),
            Content::JollyBlock(_) => Texture::new(ctx, "src/textures/utils/content/JollyBlock.png"),
            Content::Scarecrow => Texture::new(ctx, "src/textures/utils/content/Scarecrow.png"),
            _ => Texture::new(ctx, "src/textures/utils/content/Empty.png"),
        };
        res.expect("couldn't upload content texture")
    }
}
impl Texturizable for Robot{
    fn get_texture(&self, ctx: &mut Context) -> Texture {
        Texture::new(ctx, "src/textures/utils/player.png").expect("couldn't upload robot texture")
    }
}

///It returns the texture of the texturizable object
pub(crate) fn get_texture(text_obj: Box<dyn Texturizable>, ctx: &mut Context) -> Texture {
    text_obj.get_texture(ctx)
}
