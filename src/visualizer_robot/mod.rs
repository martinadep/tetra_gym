use std::fmt::format;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::World;
use tetra::{Context, ContextBuilder, State};
use crate::visualizer::Visualizer;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct VisualizerRobot{
    robot : Robot,
    discovered_tiles : Vec<Vec<(Tile, Coordinate)>>,
    //coordinates of the tiles in the map

    ctx : Context,
    visualizer : Option<Visualizer>,


}
impl VisualizerRobot{
    pub(crate) fn new() -> Self{
        let mut to_ret = Self{
            robot: Default::default(),
            discovered_tiles: vec![],
            ctx: ContextBuilder::new("hello world!", WINDOW_WIDTH, WINDOW_HEIGHT)
                .quit_on_escape(true)
                .resizable(true)
                .build()
                .expect("failed to build the context"),
            visualizer : None,
        };
        to_ret.visualizer = Some(Visualizer::new(&mut to_ret.ctx).expect("failed to build visualizer"));
        to_ret.run_ctx();
        to_ret
    }
    pub(crate) fn run_ctx(&mut self){
        self.ctx.run(|_| Ok(self.visualizer.clone().expect("failed running due to None value of visualzier"))).expect("failed ro run context")
    }
    fn add_tiles(&mut self, new_discover : Vec<Vec<Tile>>, moved_coords : (usize, usize)) {

    }
}

impl Runnable for VisualizerRobot{
    fn process_tick(&mut self, world: &mut World) {
        todo!()
    }

    fn handle_event(&mut self, event: Event) {
        //println!("handle event:")
        match event {
            Event::Ready => {}
            Event::Terminated => {}
            Event::TimeChanged(_) => {}
            Event::DayChanged(_) => {}
            Event::EnergyRecharged(_) => {}
            Event::EnergyConsumed(_) => {}
            Event::Moved(t, c) => {
                //self.add_tiles(c)
                self.visualizer.clone().unwrap().action_made_text.set_content(format!("Robot moved to {}, {}", c.0, c.1))
            }
            Event::TileContentUpdated(_, _) => {}
            Event::AddedToBackpack(_, _) => {}
            Event::RemovedFromBackpack(_, _) => {}
        }
    }

    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }

    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }

    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }

    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}