use clap::Parser;
use demos::DemoList;

mod args_parser;
mod demos;

fn main() {
    let args = args_parser::Args::parse();
    match args.demo {
        DemoList::Light => demos::light::start_demo(),
        DemoList::Actions => demos::actions::start_demo(),
        DemoList::SolarSystem => demos::solar_system::start_demo(),
    }
}
