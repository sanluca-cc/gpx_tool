use common::Wpt;
use geo::{HaversineDistance, HaversineLength, LineString};
use log::debug;

/// Function to fix elevation data where elevation is zero.
///
/// This function fixes the elevation data where the elevation drops to zero.
pub fn fix_ele_zero(route: Vec<Wpt>) -> Vec<Wpt> {
    let mut new_route = route;

    for i in 1..new_route.len() - 1 {
        let last = new_route[i - 1].clone();
        let current = new_route[i].clone();

        if current.ele() != 0. {
            continue;
        }

        let dist_to = last.point().haversine_distance(&current.point());
        let grade_to = (current.ele() - last.ele()) / dist_to * 100.;

        if grade_to.abs() < 25. {
            continue;
        }

        let mut j = 1;

        while new_route[i + j].ele() == 0. {
            j += 1;
        }

        let ls: LineString<f64> = new_route[i - 1..i + j + 1]
            .iter()
            .map(|wpt| wpt.coord())
            .collect();
        let dist_full = ls.haversine_length();
        let d_ele = new_route[i + j].ele() - last.ele();

        let mut tmp_dist = 0.;

        for k in i..i + j {
            debug!("Fixing ele_zero at {i}");
            let dist = new_route[k]
                .point()
                .haversine_distance(&new_route[k + 1].point());
            tmp_dist += dist;
            new_route[k].set_ele(last.ele() + (d_ele / dist_full) * tmp_dist);
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
        let route = read_gpx("tests/data/fix_ele_zero_short.gpx")
            .unwrap()
            .waypoints;
        assert_eq!(route[2].ele(), 0.);

        let new_route = fix_ele_zero(route);
        assert_ne!(new_route[2].ele(), 0.);
    }

    #[test]
    fn test_fix_ele_zero_full() {
        let route = read_gpx("tests/data/fix_ele_zero_full.gpx")
            .unwrap()
            .waypoints;
        assert_eq!(route[656].ele(), 0.);

        let new_route = fix_ele_zero(route);
        assert_ne!(new_route[656].ele(), 0.);
    }
}
