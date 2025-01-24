use crate::{parse_gpx, GPXRoute};
use std::{error::Error, fs::File, io::BufReader, path::Path};

pub fn read_gpx<P: AsRef<Path>>(path: P) -> Result<GPXRoute, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    parse_gpx(reader)
}
