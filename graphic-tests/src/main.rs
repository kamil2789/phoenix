use clap::Parser;
use tests::run;

mod image;
mod tests;
mod workspace;
mod args_parser;


fn main() {
    let args = args_parser::Args::parse();

    print!("{:?}", args.name);

    println!("Start graphic tests");
    run();
}
