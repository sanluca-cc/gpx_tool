use common::Wpt;
use geo::HaversineDistance;
use log::debug;

/// Function to smoothen out the elevation spikes in the route.
///
/// This function only fixes spikes of a single point and with a grade of more than 25% on both sides.
pub fn fix_spikes(route: Vec<Wpt>) -> Vec<Wpt> {
    let mut new_route = route;

    for i in 1..new_route.len() - 1 {
        let last = new_route[i - 1].clone();
        let current = new_route[i].clone();
        let next = new_route[i + 1].clone();

        let dist_to = last.point().haversine_distance(&current.point());
        let dist_from = current.point().haversine_distance(&next.point());

        let grade_to = (current.ele() - last.ele()) / dist_to * 100.;
        let grade_from = (next.ele() - current.ele()) / dist_from * 100.;

        if grade_to.abs() > 25.
            && grade_from.abs() > 25.
            && grade_from.signum() != grade_to.signum()
        {
            debug!(
                "Fixing spike at km {:.1}",
                route_utils::route_length_along(&new_route, 0, i)
            );

            let dist_full = dist_to + dist_from;
            let d_ele = next.ele() - last.ele();

            new_route[i].set_ele(last.ele() + (d_ele / dist_full) * dist_to);
        }
    }

    new_route
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpx_utils::read_gpx;

    #[test]
    fn test_fix_ele_zero_short() {
        let route = read_gpx("tests/data/fix_spikes_short.gpx")
            .unwrap()
            .waypoints;
        assert!(route[2].ele() > route[3].ele());

        let new_route = fix_spikes(route);
        assert!(new_route[2].ele() < new_route[3].ele());
    }

    #[test]
    fn test_fix_spikes_full() {
        let route = read_gpx("tests/data/fix_spikes_full.gpx")
            .unwrap()
            .waypoints;
        assert!(route[336].ele() > route[337].ele());

        let new_route = fix_spikes(route);
        assert!(new_route[336].ele() < new_route[337].ele());
    }
}
