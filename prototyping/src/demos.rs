use clap::ValueEnum;

pub mod actions;
pub mod light;
pub mod solar_system;

#[derive(ValueEnum, Debug, Clone)]
pub enum DemoList {
    Light,
    Actions,
    SolarSystem,
}
