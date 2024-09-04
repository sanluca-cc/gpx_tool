use common::Wpt;
use geo::HaversineBearing;
use log::debug;

/// Function to fix route data whenever the route has a point moving out and then going straight back to the route.
///
/// This function removes points after which there is a 180 degree turn, and then going back to a point at the same location as the previous.
pub fn fix_pointspike(route: Vec<Wpt>) -> Vec<Wpt> {
    let mut new_route = route;

    let mut orignal_index = 2;
    let mut i = 2;

    while i < new_route.len() - 2 {
        let before = new_route[i - 2].clone();
        let last = new_route[i - 1].clone();
        let current = new_route[i].clone();
        let next = new_route[i + 1].clone();
        let after = new_route[i + 2].clone();

        let bearing_before = before.point().haversine_bearing(last.point());
        let bearing_after = next.point().haversine_bearing(after.point());

        let bearing_to = last.point().haversine_bearing(current.point());
        let bearing_from = current.point().haversine_bearing(next.point());

        if (bearing_to - bearing_from).abs() > 175.
            && (bearing_to - bearing_from).abs() < 185.
            && (bearing_before - bearing_after).abs() < 10.
        {
            debug!("Removing pointspike at {orignal_index}");
            new_route.remove(i + 1);
            new_route.remove(i);
        } else {
            i += 1;
        }

        orignal_index += 1;
    }

    new_route
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpx_utils::read_gpx;

    #[test]
    fn test_fix_pointspike_short() {
        let route = read_gpx("tests/data/fix_pointspike_short.gpx")
            .unwrap()
            .waypoints;
        assert_eq!(route[1].point(), route[3].point());
        assert_eq!(route.len(), 5);

        let new_route = fix_pointspike(route);
        assert_eq!(new_route.len(), 3);
    }

    #[test]
    fn test_fix_pointspike_full() {
        let route = read_gpx("tests/data/fix_pointspike_full.gpx")
            .unwrap()
            .waypoints;
        let route_len = route.len();

        let new_route = fix_pointspike(route);
        assert!(new_route.len() < route_len);
    }
}
