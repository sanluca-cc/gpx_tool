use gpx::Waypoint;

#[derive(Debug, Clone)]
pub struct Wpt(Waypoint);

impl Wpt {
    pub fn ele(&self) -> f64 {
        self.0.elevation.expect("Elevation is missing")
    }

    pub fn point(&self) -> geo::Point<f64> {
        self.0.point()
    }

    pub fn set_ele(&mut self, ele: f64) {
        self.0.elevation = Some(ele);
    }

    pub fn waypoint(&self) -> &Waypoint {
        &self.0
    }

    pub fn coord(&self) -> geo::Coord<f64> {
        self.0.point().into()
    }
}

impl From<Waypoint> for Wpt {
    fn from(waypoint: Waypoint) -> Wpt {
        Wpt(waypoint)
    }
}

impl From<&Waypoint> for Wpt {
    fn from(waypoint: &Waypoint) -> Wpt {
        Wpt(waypoint.clone())
    }
}
