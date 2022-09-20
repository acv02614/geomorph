use std::fmt;

use crate::{coord::Coord, mgrs::Mgrs, utm::Utm};

/// Coordinates in DD/MM/SS.(S) format
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DMSBasic
{
    /// Degrees: is not contained, all checks are done in DMS struct
    pub dd: i32,
    /// Minutes: Must be contained in the interval [0..60.0]
    pub mm: u32,
    /// Minutes: Must be contained in the interval [0..60.0]
    pub ss: f64,
}

impl DMSBasic {
    pub fn new(mut dd: i32, mut mm: u32, mut ss: f64) -> Self {
        // Seconds are modular of 60, any excess wil be converted to minutes
        if ss > 60.0 {
            mm += (ss / 60.0).trunc() as u32;
            ss %= 60.0;
        }
        // Minutes are modular of 60, any excess wil be converted to degrees
        if mm > 60 {
            dd += (mm / 60) as i32;
            mm %= 60;
        }

        Self { dd, mm, ss }
    }
}

impl fmt::Display for DMSBasic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}°{}'{}\"", self.dd.abs(), self.mm, self.ss)
    }
}

/// Holds a coordinate pair in DD/MM/SS.(S) format
#[derive(Debug, Clone, Copy)]
pub struct DMS {
    /// Latitude: Must be contained in the interval [-90/00/00..90/00/00]
    pub lat: DMSBasic,
    /// Longitude: Must be contained in the interval [-180/00/00..180/00/00]
    pub lon: DMSBasic,
}

impl DMS {
    /// Return a new DMS instance.
    ///
    /// Latitude will be modular 90.0
    /// Longitude will be mobular 180.0
    pub fn new(mut lat: DMSBasic, mut lon: DMSBasic) -> Self {
        if lat.dd < -90 || lat.dd > 90 {
            lat.dd %= 90;
        }

        if lon.dd < -180 || lon.dd > 180 {
            lon.dd %= 180;
        }

        Self { lat, lon }
    }
}

impl fmt::Display for DMS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.lat.dd >= 0 {
            write!(f, "{}N, ", self.lat)?;
        } else {
            write!(f, "{}S, ", self.lat)?;
        }
        if self.lon.dd >= 0 {
            write!(f, "{}E", self.lon)
        } else {
            write!(f, "{}W", self.lon)
        }
    }
}

impl From<Coord> for DMS {
    fn from(coord: Coord) -> Self {
        let lat = DMSBasic::new(0, 0, coord.lat * 3600.0);
        let lon = DMSBasic::new(0, 0, coord.lon * 3600.0);
        DMS::new(lat, lon)
    }
}

impl From<Mgrs> for DMS {
    fn from(mgrs: Mgrs) -> Self {
        let coord: Coord = mgrs.into();
        coord.into()
    }
}

impl From<Utm> for DMS {
    fn from(utm: Utm) -> Self {
        let coord: Coord = utm.into();
        coord.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_dmsbasic() {
        let lat = DMSBasic::new(49, 36, 27.40);
        assert_eq!(lat.dd, 49);
        assert_eq!(lat.mm, 36);
        assert_eq!(lat.ss, 27.40);

        let lat = DMSBasic::new(-1, 31, 57.30);
        assert_eq!(lat.dd, -1);
        assert_eq!(lat.mm, 31);
        assert_eq!(lat.ss, 57.30);
    }

    #[test]
    fn instance_dms() {
        let lat = DMSBasic::new(49, 36, 27.40);
        let lon = DMSBasic::new(37, 19, 50.14);
        let point = DMS::new(lat, lon);
        assert_eq!(point.lat, lat);
        assert_eq!(point.lon, lon);
    }

    #[test]
    fn from_coord() {
        let lat = DMSBasic::new(48, 35, 11.03);
        let lon = DMSBasic::new(36, 31, 44.91);
        let base_point = Coord::new(48.5863964, 36.5291404);
        let point: DMS = base_point.into();
        assert!((point.lat.ss - lat.ss).abs() <= 0.01);
        assert!((point.lon.ss - lon.ss).abs() <= 0.01);
    }
    
    #[test]
    fn to_coord() {
        let lat = DMSBasic::new(-2, 23, 24.46);
        let lon = DMSBasic::new(18, 32, 59.56);
        let base_point = DMS::new(lat, lon);
        let point: Coord = base_point.into();
        assert!((point.lat - -2.3901266).abs() <= 0.01);
        assert!((point.lon - 18.5498764).abs() <= 0.01);
    }
}