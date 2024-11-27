use clap::ValueEnum;

pub mod light;
pub mod actions;
pub mod solar_system;

#[derive(ValueEnum, Debug, Clone)]
pub enum DemoList {
    Light,
    Actions,
    SolarSystem
}