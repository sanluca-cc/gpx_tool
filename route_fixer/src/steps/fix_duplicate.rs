use common::Wpt;
use log::debug;

/// Function to remove duplicate points.
///
/// This function removed duplicate points by checking if the next point is the same as the current, and removed the next if it is.
pub fn fix_duplicate(route: Vec<Wpt>) -> Vec<Wpt> {
    let mut new_route = route;

    let mut i = 0;

    while i < new_route.len() - 1 {
        let current = new_route[i].clone();
        let next = new_route[i + 1].clone();

        if next.point() == current.point() {
            debug!(
                "Removing duplicate at km {:.1}",
                route_utils::route_length_along(&new_route, 0, i)
            );

            new_route.remove(i + 1);
        } else {
            i += 1;
        }
    }

    new_route
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpx_utils::read_gpx;

    #[test]
    fn test_fix_duplicate_short() {
        let route = read_gpx("tests/data/fix_duplicate_short.gpx")
            .unwrap()
            .waypoints;
        assert_eq!(route[1].point(), route[2].point());
        assert_eq!(route.len(), 4);

        let new_route = fix_duplicate(route);
        assert_eq!(new_route.len(), 3);
    }

    #[test]
    fn test_fix_duplicate_full() {
        let route = read_gpx("tests/data/fix_duplicate_full.gpx")
            .unwrap()
            .waypoints;
        let route_len = route.len();

        let new_route = fix_duplicate(route);
        assert!(new_route.len() < route_len);
    }
}
