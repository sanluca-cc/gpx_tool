use crate::GPXRoute;
use gpx::read;
use std::{error::Error, fs::File, io::BufReader, path::Path};

pub fn read_gpx<P: AsRef<Path>>(path: P) -> Result<GPXRoute, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let gpx = read(reader)?;

    let mut gpx_route = GPXRoute {
        name: None,
        waypoints: Vec::new(),
    };

    if !gpx.tracks.is_empty() {
        let track = &gpx.tracks[0];
        let segment = &track.segments[0];
        gpx_route.name.clone_from(&track.name);

        for point in segment.points.iter() {
            gpx_route.waypoints.push(point.into());
        }
    } else if !gpx.routes.is_empty() {
        let route = &gpx.routes[0];
        gpx_route.name.clone_from(&route.name);

        for point in route.points.iter() {
            gpx_route.waypoints.push(point.into());
        }
    } else {
        return Err("No track or route found in GPX file".into());
    }

    Ok(gpx_route)
}
