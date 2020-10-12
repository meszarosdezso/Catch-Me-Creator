mod catchme;

use anyhow::{Context, Result};
use catchme::gtfs_to_catchme_data;
use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Path to zip archive or directory
    path: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let data = gtfs_to_catchme_data(&opt.path)?;

    let file: File = File::create("ui/public/export.json").context("Failed to create file.")?;

    serde_json::to_writer_pretty(file, &data).context("Failed to export data")?;

    Ok(())
}
