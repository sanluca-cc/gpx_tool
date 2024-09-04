use crate::steps;
use common::Wpt;

pub fn fix_route(route: Vec<Wpt>) -> Vec<Wpt> {
    let mut new_route = route;

    new_route = steps::fix_duplicate(new_route);
    new_route = steps::fix_pointspike(new_route);
    new_route = steps::fix_spikes(new_route);
    new_route = steps::fix_ele_zero(new_route);

    new_route
}
