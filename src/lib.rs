#[allow(unused_imports, unused_attributes)] // Used for an optional feature
#[cfg(feature = "serde")]
#[macro_use]
use serde::{Deserialize, Serialize};

pub mod traits;
// pub mod projections;
/// Structures for computing points in 3D space.
pub mod three_dimensional;
/// Structures for computing points on a 2D plane, e.g. cartesian plane.
pub mod two_dimensional;

pub mod prelude;
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
