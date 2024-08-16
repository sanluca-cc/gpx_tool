use crate::GPXRoute;
use gpx::{write, Gpx, GpxVersion, Track, TrackSegment};
use std::{error::Error, fs::File, io::BufWriter, path::Path};

pub fn write_gpx<P: AsRef<Path>>(out_path: P, gpx_route: GPXRoute) -> Result<(), Box<dyn Error>> {
    let track_segment = TrackSegment {
        points: gpx_route
            .waypoints
            .iter()
            .map(|wpt| wpt.waypoint().clone())
            .collect(),
    };

    let track = Track {
        name: gpx_route.name,
        comment: None,
        description: None,
        source: None,
        links: vec![],
        type_: None,
        number: None,
        segments: vec![track_segment],
    };

    let gpx = Gpx {
        version: GpxVersion::Gpx11,
        creator: Some("sanluca.cc gpx tool".to_string()),
        metadata: None,
        waypoints: vec![],
        tracks: vec![track],
        routes: vec![],
    };

    let gpx_file = File::create(out_path)?;
    let buf = BufWriter::new(gpx_file);

    write(&gpx, buf)?;

    Ok(())
}
