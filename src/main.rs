mod catchme;

use anyhow::{Context, Result};
use gtfs_structures::Gtfs;
use std::fs::File;
use structopt::StructOpt;

use catchme::CatchMeData;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Path to zip archive or directory
    path: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let gtfs = Gtfs::new(&opt.path).context("Failed to load GTFS data from the path.")?;

    let data = CatchMeData::from_gtfs(&gtfs);

    let file: File = File::create("ui/public/data.json").context("Failed to create file.")?;

    serde_json::to_writer_pretty(file, &data).context("Failed to export data")?;

    Ok(())
}
