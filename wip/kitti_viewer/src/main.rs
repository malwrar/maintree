extern crate clap;
extern crate kdtree;
extern crate malicious;

use std::fs::File;
use std::time::Instant;

use clap::{AppSettings, Clap};
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;

use malicious::{
    parsing::kitti::VelodyneBinParser,
};

#[derive(Clap)]
#[clap(version = "1.0", author = "malwrar <malwrar@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    //dataset_path: String,

    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
enum SubCommand {
    Lidar(Lidar),
}

#[derive(Clap)]
struct Lidar {
    #[clap(short)]
    debug: bool,
}

fn main() {
    let opts = Opts::parse();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Info and warnings."),
        2 => println!("Full debug logs."),
        _ => println!("Don't be ridiculous"),
    }

    env_logger::init();
    log::info!("Starting logging!");
    log::warn!("Starting logging!");
    log::debug!("Starting logging!");
    log::error!("Starting logging!");
}
