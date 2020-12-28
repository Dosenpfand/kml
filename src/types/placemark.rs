use std::collections::HashMap;

use num_traits::Float;

use crate::types::element::Element;
use crate::types::geometry::Geometry;

/// Placemark element defined in 9.14
///
/// Placemark not inside of kml:Update (unused) requires a Geometry according to [ATC-226](https://docs.opengeospatial.org/ts/14-068r2/14-068r2.html#atc-226),
/// but Google's  reference says it's optional [Google Placemark reference](https://developers.google.com/kml/documentation/kmlreference#placemark).
///
/// Currently leaving optional.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Placemark<T: Float = f64> {
    pub name: Option<String>,
    pub description: Option<String>,
    pub geometry: Option<Geometry<T>>,
    pub attrs: HashMap<String, String>,
    pub children: Vec<Element>,
}
