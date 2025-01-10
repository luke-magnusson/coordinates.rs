use num_traits::Float;

use crate::{
    traits::{Dot, Magnitude, Positional, TrigConsts},
    two_dimensional::vector2::Vector2,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]

/// Coordinate in the format (r, theta)
///
/// Radius is the distance from the origin.
///
/// Theta is the angle between the vector pointing to this coordinate and the
/// unit vector `[1, 0]` in the clockwise direction. (in radians)
///
/// > i.e. the angle `∠POX` where `P` is the coordinate, `O` is the origin `[0, 0]`
/// and `X` is a point on the positive region of the x axis, e.g. `[1, 0]`
pub struct Polar<T: Float> {
    /// Distance from the origin.
    #[cfg_attr(feature = "serde", serde(rename = "r"))]
    pub radius: T,
    /// Angle from the `x` axis (in radians).
    pub theta: T,
}

impl<T: Float + TrigConsts> super::TwoDimensionalConsts<T> for Polar<T> {
    const ORIGIN: Self = Polar {
        radius: T::ZERO,
        theta: T::ZERO,
    };

    const UP: Self = Polar {
        radius: T::ONE,
        theta: T::ZERO,
    };

    const DOWN: Self = Polar {
        radius: T::ONE,
        theta: T::FRAC_PI_2,
    };

    const LEFT: Self = Polar {
        radius: T::ONE,
        theta: T::FRAC_3PI_2,
    };

    const RIGHT: Self = Polar {
        radius: T::ONE,
        theta: T::FRAC_PI_2,
    };
}

impl<T: Float> Magnitude<T> for Polar<T> {
    fn magnitude(&self) -> T {
        self.radius
    }

    fn quick_magnitude(&self) -> T {
        self.radius
    }
}

impl<T: Float> Dot<T> for Polar<T> {
    fn dot(&self, rhs: &Self) -> T {
        Into::<Vector2<T>>::into(self).dot(&Into::<Vector2<T>>::into(rhs))
    }
}

impl<T: Float> Positional<T> for Polar<T> {
    fn angle_to(&self, other: &Self) -> T {
        if self.radius.is_zero() || other.radius.is_zero() {
            // If one or both of the thetas are undefined, return 0
            T::zero()
        } else {
            (self.theta - other.theta).abs()
        }
    }
}

impl<T: Float> std::ops::Div<T> for Polar<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            radius: self.radius / rhs,
            theta: self.theta,
        }
    }
}

impl<T: Float> From<Vector2<T>> for Polar<T> {
    fn from(cart: Vector2<T>) -> Self {
        (&cart).into()
    }
}

impl<T: Float> From<&Vector2<T>> for Polar<T> {
    fn from(cart: &Vector2<T>) -> Self {
        Polar {
            radius: cart.magnitude(),
            theta: cart.y.atan2(cart.x),
        }
    }
}
impl<T: Float> From<(T, T)> for Polar<T> {
    fn from(tuple: (T, T)) -> Self {
        Polar {
            radius: tuple.0,
            theta: tuple.1,
        }
    }
}
impl<T: Float> From<Polar<T>> for (T, T) {
    fn from(polar: Polar<T>) -> Self {
        (polar.radius, polar.theta)
    }
}
