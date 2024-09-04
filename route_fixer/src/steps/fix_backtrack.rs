use common::Wpt;
use geo::HaversineBearing;
use log::debug;

/// Function to fix route data whenever the route backtracks.
///
/// This function removes backtrack segments which is segments where the route is followed normally, then going the exact same way back and then returning to follow the route normally.
pub fn fix_backtrack(route: Vec<Wpt>) -> Vec<Wpt> {
    let mut new_route = route;

    let mut orignal_index = 1;
    let mut i = 1;

    // First, fix backtrack where we get starting point via the last and next being the same
    while i < new_route.len() - 1 {
        let last = new_route[i - 1].clone();
        let next = new_route[i + 1].clone();

        if last.point() == next.point() {
            debug!("Backtrack found at {orignal_index}");
            let mut j = 1;

            while new_route[i + j].point() == new_route[i - j].point() {
                j += 1;
            }

            if j == 1 {
                i += 1;
                continue;
            }

            // Remove the backtrack, but not the last point
            new_route.drain(i - j + 1..i + j - 1);
        } else {
            i += 1;
        }

        orignal_index += 1;
    }

    orignal_index = 1;
    i = 1;

    // Second, fix single backtrack where we get starting point via the bearing
    while i < new_route.len() - 1 {
        let last = new_route[i - 1].clone();
        let current = new_route[i].clone();
        let next = new_route[i + 1].clone();

        let bearing_to = last.point().haversine_bearing(current.point());
        let bearing_from = current.point().haversine_bearing(next.point());

        if (bearing_to - bearing_from).abs() > 179.99 && (bearing_to - bearing_from).abs() < 180.01
        {
            debug!("Backtrack found at {orignal_index}");
            let mut j = 1;

            if new_route[i + j].point() != new_route[i - j].point() {
                new_route.remove(i);
            }

            while new_route[i + j].point() == new_route[i - j].point() {
                j += 1;
            }

            // Remove only the backtrack points
            new_route.drain(i - j + 1..i + j);
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
    fn test_fix_backtrack_short() {
        let route = read_gpx("tests/data/fix_backtrack_short.gpx")
            .unwrap()
            .waypoints;
        assert_eq!(route[5].point(), route[7].point());
        assert_eq!(route[4].point(), route[8].point());
        assert_eq!(route[3].point(), route[9].point());
        assert_eq!(route.len(), 17);

        let new_route = fix_backtrack(route);
        assert_eq!(new_route.len(), 11);
    }

    #[test]
    fn test_fix_backtrack_full() {
        let route = read_gpx("tests/data/fix_backtrack_full.gpx")
            .unwrap()
            .waypoints;
        let route_len = route.len();

        let new_route = fix_backtrack(route);
        assert!(new_route.len() < route_len);
    }

    #[test]
    fn test_fix_backtrack2_short() {
        let route = read_gpx("tests/data/fix_backtrack2_short.gpx")
            .unwrap()
            .waypoints;
        assert_eq!(route[1].point(), route[3].point());
        assert_eq!(route.len(), 5);

        let new_route = fix_backtrack(route);
        assert_eq!(new_route.len(), 3);
    }

    #[test]
    fn test_fix_backtrack2_full() {
        let route = read_gpx("tests/data/fix_backtrack2_full.gpx")
            .unwrap()
            .waypoints;
        let route_len = route.len();

        let new_route = fix_backtrack(route);
        assert!(new_route.len() < route_len);
    }
}
