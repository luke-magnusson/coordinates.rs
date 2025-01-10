use std::{
    fmt::Display,
    ops::{Add, Neg, Sub},
};

use num_traits::Float;

use crate::traits::{Dot, Magnitude, Positional};

use super::{cylindrical::Cylindrical, spherical::Spherical};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/***************
 * BASE STRUCT *
 ***************/

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
/// A point in 3d space
pub struct Vector3<T: num_traits::Float> {
    /// Left (-)/right (+) axis
    pub x: T,
    /// In (+)/out (-) axis
    pub y: T,
    /// Up (+)/down (-) axis
    pub z: T,
}

/***************************
 * CRATE TRAIT DEFINITIONS *
 ***************************/

macro_rules! impl_3d {
    ($var: ident) => {
        impl super::ThreeDimensionalConsts<$var> for Vector3<$var> {
            const ORIGIN: Self = Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };

            const UP: Self = Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            };

            const DOWN: Self = Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            };

            const FORWARD: Self = Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            };

            const BACK: Self = Vector3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            };

            const LEFT: Self = Vector3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            };

            const RIGHT: Self = Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
        }


    };
    ($($var : ident),+) => {
        $(impl_3d!($var);)+
    }
}

impl_3d!(f32, f64);

impl<T: Float> crate::traits::Magnitude<T> for Vector3<T> {
    fn magnitude(&self) -> T {
        self.quick_magnitude().sqrt()
    }

    fn quick_magnitude(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T: Float> crate::traits::Dot<T> for Vector3<T> {
    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: Float> crate::traits::Cross3D for Vector3<T> {
    fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T: Float> Positional<T> for Vector3<T> {
    fn angle_to(&self, other: &Self) -> T {
        (self.dot(&other) / (self.magnitude() * other.magnitude())).acos()
    }
}

/********************************
 * ARITHMETIC TRAIT DEFINITIONS *
 ********************************/

impl<T: Float> Neg for Vector3<T> {
    type Output = Vector3<T>;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Float> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Float> Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Float> std::ops::Div<T> for Vector3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

/********************
 * FROM DEFINITIONS *
 ********************/

impl<T: Float> From<(T, T, T)> for Vector3<T> {
    fn from(tuple: (T, T, T)) -> Self {
        Vector3 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl<T: Float> Into<(T, T, T)> for Vector3<T> {
    fn into(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

impl<T: Float> Into<[T; 3]> for Vector3<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl<T: Float> From<[T; 3]> for Vector3<T> {
    fn from(a: [T; 3]) -> Self {
        Self {
            x: a[0],
            y: a[1],
            z: a[2],
        }
    }
}

impl<T: Float> From<Cylindrical<T>> for Vector3<T> {
    fn from(cyl: Cylindrical<T>) -> Self {
        let (sin, cos) = cyl.azimuth.sin_cos();
        Vector3 {
            x: cyl.radius * cos,
            //BUG may be off by as much as `8.742278e-8` when `azimuth` == `pi`
            // that's about 22" or 60 cm when `r=the radius of the earth` for f32
            y: cyl.radius * sin,
            z: cyl.height,
        }
    }
}

impl<T: Float> From<&Cylindrical<T>> for Vector3<T> {
    fn from(cyl: &Cylindrical<T>) -> Self {
        let (sin, cos) = cyl.azimuth.sin_cos();
        Vector3 {
            x: cyl.radius * cos,
            y: cyl.radius * sin,
            z: cyl.height,
        }
    }
}

impl<T: Float> From<Spherical<T>> for Vector3<T> {
    fn from(sph: Spherical<T>) -> Self {
        Self::from(&sph)
    }
}

impl<T: Float> From<&Spherical<T>> for Vector3<T> {
    fn from(sph: &Spherical<T>) -> Self {
        // Sin and cos for the azimuthal angle (0, 1) for straight right (positive x direction)
        let (sin_az, cos_az) = sph.azimuthal_angle.sin_cos();
        // Sin and cos relative to the polar angle (0, 1) for straight up
        let (sin_pol, cos_pol) = sph.polar_angle.sin_cos();
        Vector3 {
            // x = r \times \sin\left(\theta\right) \times \cos\left(\phi\right)
            x: sph.radius * sin_pol * cos_az,
            // x = r \times \sin\left(\theta\right) \times \sin\left(\phi\right)
            y: sph.radius * sin_pol * sin_az,
            // x = r \times \cos\left(\theta\right)
            z: sph.radius * cos_pol,
        }
    }
}

/**************************
 * DISPLAY IMPLEMENTATION *
 **************************/

impl<T: Float + Display> Display for Vector3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::three_dimensional::ThreeDimensionalConsts;
    use crate::traits::Dot;
    use crate::traits::Positional;
    use crate::traits::TrigConsts;

    use super::Vector3;

    use assert_float_eq::*;
    use std::f32::EPSILON;
    #[test]
    pub fn is_positional() {
        let up = Vector3::<f32>::UP;

        for point in [
            Vector3::<f32>::BACK,
            Vector3::<f32>::FORWARD,
            Vector3::<f32>::LEFT,
            Vector3::<f32>::RIGHT,
        ] {
            println!(
                "Angle between\n{:?} and\n{:?} is\n{}, dot is {}",
                &point,
                &up,
                up.angle_to(&point),
                up.dot(&point)
            );

            assert_float_relative_eq!(f32::FRAC_PI_2, up.angle_to(&point), EPSILON);
        }

        assert_float_relative_eq!(f32::PI, up.angle_to(&Vector3::<f32>::DOWN), EPSILON);
    }
}
