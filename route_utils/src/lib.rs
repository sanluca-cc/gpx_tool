use common::Wpt;
use geo::{HaversineLength, LineString};

/// Returns the length of a route in kilometers, rounded to 1 decimal.
pub fn route_length(route: &[Wpt]) -> f64 {
    let linestring =
        LineString::<f64>::from(route.iter().map(|wpt| wpt.coord()).collect::<Vec<_>>());

    linestring.haversine_length() / 1000.
}

/// Returns the length of a route from a given start and end index in kilometers.
pub fn route_length_along(route: &[Wpt], start: usize, end: usize) -> f64 {
    route_length(&route[start..=end])
}
