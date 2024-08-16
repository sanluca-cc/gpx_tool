use clap::Parser;
use gpx_utils::{read_gpx, write_gpx};
use log::{debug, error, info, LevelFilter};
use route_fixer::fix_route;
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(about = "A tool to work with GPX files", long_about = None)]
struct Args {
    /// The GPX file to process
    file: String,

    /// Output file
    #[arg(short, long, name = "FILE")]
    out: Option<String>,

    /// Fix the route
    #[arg(short, long)]
    fix: bool,

    /// Debug mode
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    env_logger::builder()
        .format_timestamp(None)
        .filter_level(if args.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .init();

    if args.debug {
        debug!("Debug mode enabled");
    }

    let in_path = Path::new(&args.file);

    if !in_path.exists() {
        error!("File {} does not exist", args.file);
        return;
    }

    let mut route = match read_gpx(&args.file) {
        Ok(route) => route,
        Err(e) => {
            error!("Error reading GPX file: {}", e);
            return;
        }
    };

    if args.fix {
        info!("Fixing route");
        route.waypoints = fix_route(route.waypoints);
    }

    let outfile = args
        .out
        .unwrap_or_else(|| format!("{}-new.gpx", in_path.file_stem().unwrap().to_str().unwrap()));

    match write_gpx(outfile.clone(), route) {
        Ok(_) => info!("Route written to {}", outfile),
        Err(e) => error!("Error writing GPX file: {}", e),
    }
}
