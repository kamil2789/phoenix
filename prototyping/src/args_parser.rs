use clap::Parser;

use crate::demos::DemoList;

/// This program is designed for dynamic testing of the environment using camera movement.
/// Dedicated demos are prepared to showcase the capabilities of the graphics engine.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // Run demo
    #[arg(value_enum, short, long)]
    pub demo: DemoList,
}
