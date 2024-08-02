use common::Wpt;

pub struct GPXRoute {
    pub name: Option<String>,
    pub waypoints: Vec<Wpt>,
}
