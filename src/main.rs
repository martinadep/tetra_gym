use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use rand::Rng;
use robotics_lib::event::events::Event::{DayChanged, Moved, Terminated};
use robotics_lib::runner::{Runnable, Runner};
use robotics_lib::world::tile::{Content, Tile, TileType};
use tetra::ContextBuilder;
use tyrannousarust_rex_world_generator::WorldGenerator;
use crate::visualizer_robot::VisualizerRobot;
use crate::world_viewer::WorldViewer;

mod visualizer;
mod world_viewer;
mod textures;
mod visualizer_robot;

pub(crate) const WINDOW_WIDTH : i32 = 500;
pub(crate) const WINDOW_HEIGHT : i32 = 500;


fn main() {
    ContextBuilder::new("tyr", WINDOW_WIDTH, WINDOW_HEIGHT)
        .quit_on_escape(true)
        .resizable(true)
        .build().expect("failed to build the context")
        .run(WorldViewer::new).expect("failed to run context");

    /*
    let mut vis_robot = VisualizerRobot::new();
    let run = Runner::new(Box::new(vis_robot), &mut WorldGenerator::new()).expect("failed to build runner");
    println!("energy level: {:?}",  run.get_robot().get_energy());*/
}
