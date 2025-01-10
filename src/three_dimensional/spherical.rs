use std::{
    fmt::Display,
    ops::{Add, Neg, Sub},
};

use num_traits::Float;

use super::{cylindrical::Cylindrical, vector3::Vector3};
use crate::traits::{Magnitude, Positional, TrigConsts};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/*********************
 * STRUCT DEFINITION *
 *********************/

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
/// A point in 3D space using spherical coordinates as defined by [ISO 80000-2:2019](https://en.wikipedia.org/wiki/Spherical_coordinate_system#Definition).
///
/// This means that the coordinates are provided in the order `radius` (`r`), `polar angle` (`theta`), and finally `azimuthal angle` (`phi`)
/// ## Examples
///
/// ```
/// # use coordinates::three_dimensional::Spherical;
/// # use crate::coordinates::three_dimensional::ThreeDimensionalConsts;
/// let right = Spherical::<f64>::new(1.0, 0.0, 0.0);
/// assert_eq!(right, Spherical::<f64>::UP);
/// ```
pub struct Spherical<T: Float> {
    /// Distance from the origin
    #[cfg_attr(feature = "serde", serde(rename = "r"))]
    pub radius: T,
    /// Angle from the positive `z` direction
    #[cfg_attr(feature = "serde", serde(rename = "theta"))]
    pub polar_angle: T,
    /// angle from the positive `x` direction along the `xy` plane
    #[cfg_attr(feature = "serde", serde(rename = "phi"))]
    pub azimuthal_angle: T,
}

impl<T: Float + TrigConsts> Spherical<T> {
    /// Maps parameters into appropriate domains.
    ///
    /// # Mapping
    ///
    /// - `azimuthal angle` ∈ [0,τ)
    ///     - Doesn't have side effects
    /// - `Polar angle ∈ [0,π]`
    ///     - Mutates azimuthal angle
    /// - `radius ∈ [0,∞)`
    ///     - Mutates polar angle and azimuthal angle
    ///
    /// # Examples
    /// ## Clamping Azimuthal angle
    ///
    /// ```
    /// # use coordinates::three_dimensional::Spherical;
    /// # use coordinates::traits::Positional;
    /// let right = Spherical::<f64>::new(1.0, std::f64::consts::FRAC_PI_2, 0.0);
    /// let also_right = Spherical::<f64>::new(1.0, std::f64::consts::FRAC_PI_2, std::f64::consts::TAU);
    ///
    /// assert!(right.angle_to(&also_right) < std::f64::EPSILON);
    /// ```
    /// ## Clamping Polar Angle
    /// ```
    /// # use coordinates::three_dimensional::Spherical;
    /// # use coordinates::traits::Positional;
    /// let up = Spherical::<f64>::new(1.0, 0.0, 0.0);
    /// let also_up = Spherical::<f64>::new(1.0, std::f64::consts::TAU, 0.0);
    /// assert!(up.angle_to(&also_up) < std::f64::EPSILON);
    /// ```
    /// ## Clamping Radius
    /// ```
    /// # use coordinates::three_dimensional::Spherical;
    /// # use coordinates::traits::Positional;
    /// let left = Spherical::<f64>::new(1.0, std::f64::consts::FRAC_PI_2, std::f64::consts::PI);
    /// let also_left = Spherical::<f64>::new(-1.0, std::f64::consts::FRAC_PI_2, 0.0);
    ///
    /// assert!(left.angle_to(&also_left) < std::f64::EPSILON);
    /// ```
    //ALTERNATE NEW METHOD
    pub fn new(radius: T, polar_angle: T, azimuthal_angle: T) -> Spherical<T> {
        let mut result = Self {
            radius,
            polar_angle,
            azimuthal_angle,
        };
        result.set_azimuthal_angle(azimuthal_angle);
        result.set_polar_angle(polar_angle);
        result.set_radius(radius);

        result
    }

    /// Returns the latitude/elevation of the point.
    ///
    /// i.e. the polar angle with respect to the equator instead of the
    /// north pole
    pub fn get_elevation(&self) -> T {
        T::FRAC_PI_2 - self.polar_angle
    }

    /// Ensures the azimuth is always in the range [0,tau)
    pub fn set_azimuthal_angle(&mut self, azimuthal_angle: T) {
        if azimuthal_angle.is_sign_positive() {
            self.azimuthal_angle = azimuthal_angle % T::TAU;
        } else {
            self.azimuthal_angle = azimuthal_angle % T::TAU + T::TAU;
        }
    }

    /// Ensures polar angle is always in the range [0,pi)
    pub fn set_polar_angle(&mut self, polar_angle: T) {
        // `Checked polar angle` ∈ [0,tau) when `polar angle` >= 0
        // `Checked polar angle` ∈ (0,tau] when `polar angle` <= -0
        let mut checked_polar_angle = polar_angle % T::TAU
            + if polar_angle.is_sign_negative() {
                // If polar angle is negative, checked polar angle ∈ (-tau, 0]
                // Add tau to move to the range (0,tau]
                T::TAU
            } else {
                T::ZERO
            };

        if checked_polar_angle > T::PI {
            // Checked polar angle is out of range, so azimuthal angle
            // will need to be adjusted as well
            self.set_azimuthal_angle(self.azimuthal_angle + T::PI);
            checked_polar_angle = T::TAU - checked_polar_angle;
        }

        self.polar_angle = checked_polar_angle;
    }

    /// Ensures radius is always in the range [0,+infinity]
    pub fn set_radius(&mut self, radius: T) {
        if radius.is_sign_negative() {
            self.set_azimuthal_angle(self.azimuthal_angle - T::PI);
            self.set_polar_angle(T::PI - self.polar_angle);
            self.radius = -radius;
        } else {
            self.radius = radius;
        }
    }
}

impl<T: Float + TrigConsts> super::ThreeDimensionalConsts<T> for Spherical<T> {
    const ORIGIN: Self = Spherical {
        azimuthal_angle: T::ZERO,
        radius: T::ZERO,
        polar_angle: T::ZERO,
    };

    const UP: Self = Spherical {
        azimuthal_angle: T::ZERO,
        radius: T::ONE,
        polar_angle: T::ZERO,
    };

    const DOWN: Self = Spherical {
        azimuthal_angle: T::ZERO,
        radius: T::ONE,
        polar_angle: T::PI,
    };

    const FORWARD: Self = Spherical {
        azimuthal_angle: T::FRAC_PI_2,
        radius: T::ONE,
        polar_angle: T::FRAC_PI_2,
    };

    const BACK: Self = Spherical {
        azimuthal_angle: T::FRAC_3PI_2,
        radius: T::ONE,
        polar_angle: T::FRAC_PI_2,
    };

    const LEFT: Self = Spherical {
        azimuthal_angle: T::PI,
        radius: T::ONE,
        polar_angle: T::FRAC_PI_2,
    };

    const RIGHT: Self = Spherical {
        azimuthal_angle: T::ZERO,
        radius: T::ONE,
        polar_angle: T::FRAC_PI_2,
    };
}

impl<T: Float> crate::traits::Magnitude<T> for Spherical<T> {
    #[inline]
    fn magnitude(&self) -> T {
        self.quick_magnitude()
    }

    #[inline]
    fn quick_magnitude(&self) -> T {
        self.radius
    }
}

impl<T: Float> crate::traits::Dot<T> for Spherical<T> {
    fn dot(&self, other: &Self) -> T {
        Into::<Vector3<T>>::into(self).dot(&other.into())
    }
}

impl<T: Float> crate::traits::Cross3D for Spherical<T> {
    fn cross(&self, other: &Self) -> Self {
        Into::<Vector3<T>>::into(self).cross(&other.into()).into()
    }
}

impl<T: Float + TrigConsts> Positional<T> for Spherical<T> {
    /// Using the spherical law of cosines
    fn angle_to(&self, other: &Self) -> T {
        let (lat_sin, lat_cos) = self.polar_angle.sin_cos();
        let (lat_sin_b, lat_cos_b) = other.polar_angle.sin_cos();

        (lat_cos * lat_cos_b
            + lat_sin * lat_sin_b * (self.azimuthal_angle - other.azimuthal_angle).cos())
        .acos()
    }
}

impl<T: Float + TrigConsts> Neg for Spherical<T> {
    type Output = Spherical<T>;

    fn neg(self) -> Self::Output {
        Spherical {
            polar_angle: T::PI - self.polar_angle,
            azimuthal_angle: (self.azimuthal_angle + T::PI) % T::TAU,
            radius: self.radius,
        }
    }
}

impl<T: Float> Add for Spherical<T> {
    type Output = Spherical<T>;

    fn add(self, rhs: Self) -> Self::Output {
        (Into::<Vector3<T>>::into(self) + Into::<Vector3<T>>::into(rhs)).into()
    }
}

impl<T: Float> Sub for Spherical<T> {
    type Output = Spherical<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        (Into::<Vector3<T>>::into(self) - Into::<Vector3<T>>::into(rhs)).into()
    }
}

impl<T: Float> std::ops::Div<T> for Spherical<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            radius: self.radius / rhs,
            azimuthal_angle: self.azimuthal_angle,
            polar_angle: self.polar_angle,
        }
    }
}

impl<T: Float + TrigConsts> PartialEq for Spherical<T> {
    fn eq(&self, other: &Self) -> bool {
        self.angle_to(other) < T::epsilon()
    }
}

impl<T: Float + TrigConsts> Eq for Spherical<T> {}
impl<T: Float + TrigConsts> PartialOrd for Spherical<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.radius.partial_cmp(&other.radius) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        self.polar_angle.partial_cmp(&other.polar_angle)
    }
}

impl<T: Float> From<Vector3<T>> for Spherical<T> {
    fn from(cart: Vector3<T>) -> Self {
        From::from(&cart)
    }
}

impl<T: Float> From<&Vector3<T>> for Spherical<T> {
    fn from(cart: &Vector3<T>) -> Self {
        let radius = cart.magnitude();
        Spherical {
            polar_angle: (cart.z / radius).acos(),
            azimuthal_angle: cart.y.atan2(cart.x),
            radius,
        }
    }
}

impl<T: Float> From<Cylindrical<T>> for Spherical<T> {
    fn from(cyl: Cylindrical<T>) -> Self {
        From::<&Cylindrical<T>>::from(&cyl)
    }
}

impl<T: Float> From<&Cylindrical<T>> for Spherical<T> {
    fn from(cyl: &Cylindrical<T>) -> Self {
        Spherical {
            radius: cyl.magnitude(),
            polar_angle: cyl.radius.atan2(cyl.height),
            azimuthal_angle: cyl.azimuth,
        }
    }
}

impl<T: Float> From<(T, T, T)> for Spherical<T> {
    fn from(tuple: (T, T, T)) -> Self {
        Spherical {
            radius: tuple.0,
            polar_angle: tuple.1,
            azimuthal_angle: tuple.2,
        }
    }
}

impl<T: Float> From<Spherical<T>> for (T, T, T) {
    fn from(sph: Spherical<T>) -> Self {
        (sph.radius, sph.polar_angle, sph.azimuthal_angle)
    }
}

impl<T: Display + Float> Display for Spherical<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{},{})",
            self.radius, self.polar_angle, self.azimuthal_angle
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::three_dimensional::ThreeDimensionalConsts;
    use crate::traits::Positional;
    use crate::traits::TrigConsts;

    use super::Spherical;
    use super::Vector3;

    use assert_float_eq::*;
    use std::f32::EPSILON;

    const ARC_SECOND: f32 = 4.848e-6;

    #[test]
    pub fn convert_to_cartesian() {
        let sphericals = [
            Spherical::<f32>::UP,
            Spherical::<f32>::DOWN,
            Spherical::<f32>::BACK,
            Spherical::<f32>::FORWARD,
            Spherical::<f32>::LEFT,
            Spherical::<f32>::RIGHT,
        ];
        let cartesians = [
            Vector3::<f32>::UP,
            Vector3::<f32>::DOWN,
            Vector3::<f32>::BACK,
            Vector3::<f32>::FORWARD,
            Vector3::<f32>::LEFT,
            Vector3::<f32>::RIGHT,
        ];

        for (s, c) in sphericals.into_iter().zip(cartesians.into_iter()) {
            println!("{:?}", s);
            let s_star: Vector3<f32> = (&s).into();
            println!("{:?}", s_star);

            assert_float_absolute_eq!(
                c.x,
                s_star.x,
                EPSILON * s.azimuthal_angle.abs().log(2.0).max(1.0)
            );
            assert_float_absolute_eq!(
                c.y,
                s_star.y,
                EPSILON * s.azimuthal_angle.abs().log(2.0).max(1.0)
            );
            assert_float_absolute_eq!(
                c.z,
                s_star.z,
                EPSILON * s.polar_angle.abs().log(2.0).max(1.0)
            );
        }
    }

    #[test]
    pub fn is_positional() {
        let up = Spherical::<f32>::UP;

        for point in [
            Spherical::<f32>::BACK,
            Spherical::<f32>::FORWARD,
            Spherical::<f32>::LEFT,
            Spherical::<f32>::RIGHT,
        ] {
            assert_float_relative_eq!(f32::FRAC_PI_2, up.angle_to(&point), EPSILON);
        }

        assert_float_relative_eq!(f32::PI, up.angle_to(&Spherical::<f32>::DOWN), EPSILON);
    }

    #[test]
    pub fn is_positional_over_small_distances() {
        let roots = [
            Spherical::<f32>::BACK,
            Spherical::<f32>::FORWARD,
            Spherical::<f32>::LEFT,
            Spherical::<f32>::RIGHT,
        ];

        for root in roots {
            small_distance_loop(root);
        }
    }

    fn small_distance_loop(root: Spherical<f32>) {
        for delta in (-128..128).map(|x| x as f32 / 128.0) {
            let azi_altered = Spherical::new(1.0, root.polar_angle, root.azimuthal_angle + delta);
            let polar_altered = Spherical::new(1.0, root.polar_angle + delta, root.azimuthal_angle);

            let delta_azimuth = azi_altered.angle_to(&root);
            let delta_polar = polar_altered.angle_to(&root);

            println!("Delta={}", delta);
            println!("dist({}, {}) = {}", root, azi_altered, delta_azimuth);
            assert_float_absolute_eq!(delta_azimuth, delta.abs(), ARC_SECOND / 4.0);
            println!("dist({}, {}) = {}", root, polar_altered, delta_polar);
            assert_float_absolute_eq!(delta_polar, delta.abs(), ARC_SECOND / 4.0);
            println!();
        }
    }

    #[test]
    pub fn constructor_tests() {
        type Base = Spherical<f32>;
        // Test positive rotation around the z axis
        let mut expected = [Base::RIGHT, Base::FORWARD, Base::LEFT, Base::BACK];
        println!("Subtest 1");
        test_constructor(
            &mut (0..100).map(|i| Base::new(1.0, f32::FRAC_PI_2, i as f32 * f32::FRAC_PI_2)),
            &expected,
        );

        // Test negative rotation around the z axis
        expected.reverse();
        expected.rotate_right(1);
        println!("Subtest 2");
        test_constructor(
            &mut (0..100).map(|i| Base::new(1.0, f32::FRAC_PI_2, i as f32 * -f32::FRAC_PI_2)),
            &expected,
        );

        // Test positive rotation around the y axis for points starting at (1. 0, pi/2)
        expected = [Base::UP, Base::RIGHT, Base::DOWN, Base::LEFT];
        println!("Subtest 3");
        test_constructor(
            &mut (0..100).map(|i| Base::new(1.0, i as f32 * f32::FRAC_PI_2, 0.0)),
            &expected,
        );

        // Test negative rotation around the y axis for points starting at (1. 0, pi/2)
        expected.reverse();
        expected.rotate_right(1);
        println!("Subtest 4");
        test_constructor(
            &mut (0..100).map(|i| Base::new(1.0, i as f32 * -f32::FRAC_PI_2, 0.0)),
            &expected,
        );

        // Test positive rotation around the x axis for points starting at (1. 0, pi/2)
        expected = [Base::UP, Base::FORWARD, Base::DOWN, Base::BACK];
        println!("Subtest 5");
        test_constructor(
            &mut (0..100).map(|i| Base::new(1.0, i as f32 * f32::FRAC_PI_2, f32::FRAC_PI_2)),
            &expected,
        );

        // Test negative rotation around the x axis for points starting at (1. 0, pi/2)
        expected.reverse();
        expected.rotate_right(1);
        println!("Subtest 6");
        test_constructor(
            &mut (0..100).map(|i| Base::new(1.0, i as f32 * -f32::FRAC_PI_2, f32::FRAC_PI_2)),
            &expected,
        );
    }

    fn test_constructor(
        iterator: &mut dyn Iterator<Item = Spherical<f32>>,
        expected: &[Spherical<f32>],
    ) {
        let mut total_i = 0;
        for entry in iterator {
            // Set i between 0 and expected.len() - 1
            let i = total_i % expected.len();

            // assert_float_absolute_eq!(
            //     entry.azimuthal_angle,
            //     expected[i].azimuthal_angle,
            //     f32::EPSILON * (total_i as f32 * f32::FRAC_PI_2).log(2.0).max(1.0)
            // );
            // assert_float_absolute_eq!(
            //     entry.polar_angle,
            //     expected[i].polar_angle,
            //     f32::EPSILON * (total_i as f32 * f32::FRAC_PI_2).log(2.0).max(1.0)
            // );
            let deviation = entry.angle_to(&expected[i]);

            print!("Winding of {:5.2}τ rad ", (total_i as f32) / 4.0);
            // assert_float_relative_eq!(deviation, 0.0, f32::EPSILON);
            // Maximum acceptable inaccuracy is 0.25" (seconds of an arc)
            if deviation.abs() > ARC_SECOND {
                println!("failed (deviation {})", deviation.abs());
                println!("Expected {}, got {}", expected[i], entry);
            }
            assert_float_relative_eq!(deviation, 0.0, ARC_SECOND / 4.0);

            println!("worked (deviations: {})", deviation);
            total_i += 1;
        }
        println!("\x1b[32mSubtest Passed\x1b[0m");
        println!();
    }
}
