use std::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};

use num_traits::{CheckedAdd, CheckedSub, Float, Num};

use crate::traits::{Dot, Magnitude, TrigConsts};

use super::polar::Polar;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/***************
 * BASE STRUCT *
 ***************/

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
/// Point in cartesian space on a 2D plane
///
/// # Examples
///
/// ```rust
/// # use coordinates::two_dimensional::{TwoDimensionalConsts, Vector2};
/// let origin = Vector2 {
///     x: 0.0,
///     y: 0.0,
/// };
/// # assert_eq!(origin, Vector2::ORIGIN);
/// ```
pub struct Vector2<T: Num> {
    /// Distance along the x axis
    pub x: T,
    /// Distance along the y axis
    pub y: T,
}

/*****************************
 * IMPLEMENTING LOCAL TRAITS *
 *****************************/

impl<T: Float + TrigConsts> super::TwoDimensionalConsts<T> for Vector2<T> {
    const ORIGIN: Self = Vector2 {
        x: T::ZERO,
        y: T::ZERO,
    };

    const UP: Self = Vector2 {
        x: T::ZERO,
        y: T::ONE,
    };

    const DOWN: Self = Vector2 {
        x: T::ZERO,
        y: T::NEG_ONE,
    };

    const LEFT: Self = Vector2 {
        x: T::NEG_ONE,
        y: T::ZERO,
    };

    const RIGHT: Self = Vector2 {
        x: T::ONE,
        y: T::ZERO,
    };
}

impl<T: Float> crate::traits::Positional<T> for Vector2<T> {}

impl<T: Float> crate::traits::Magnitude<T> for Vector2<T> {
    fn magnitude(&self) -> T {
        self.quick_magnitude().sqrt()
    }

    fn quick_magnitude(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

/*********************
 * ARITHMETIC TRAITS *
 *********************/

impl<T: Float> crate::traits::Dot<T> for Vector2<T> {
    fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: Neg<Output = T> + Num> Neg for Vector2<T> {
    type Output = Vector2<T>;

    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Num + Add> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Num + Sub> Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Num + Mul + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Float> std::ops::Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

/*********************
 * CHECKED ARITHMETIC TRAITS *
 *********************/

impl<T: Num + CheckedAdd> CheckedAdd for Vector2<T> {
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        Some(Vector2 {
            x: self.x.checked_add(&rhs.x)?,
            y: self.y.checked_add(&rhs.y)?,
        })
    }
}

impl<T: Num + CheckedSub> CheckedSub for Vector2<T> {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        Some(Vector2 {
            x: self.x.checked_sub(&rhs.x)?,
            y: self.y.checked_sub(&rhs.y)?,
        })
    }
}

/************************
 * FROM AND INTO TRAITS *
 ************************/

impl<T: Num> From<(T, T)> for Vector2<T> {
    fn from(tuple: (T, T)) -> Self {
        Vector2 {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T: Num> From<Vector2<T>> for (T, T) {
    fn from(v: Vector2<T>) -> Self {
        (v.x, v.y)
    }
}

impl<T: Num + Clone> From<[T; 2]> for Vector2<T> {
    fn from(a: [T; 2]) -> Self {
        Vector2 {
            x: a[0].clone(),
            y: a[1].clone(),
        }
    }
}

impl<T: Num> From<Vector2<T>> for [T; 2] {
    fn from(v: Vector2<T>) -> Self {
        [v.x, v.y]
    }
}

impl<T: Float> From<Polar<T>> for Vector2<T> {
    fn from(polar: Polar<T>) -> Self {
        (&polar).into()
    }
}

impl<T: Float> From<&Polar<T>> for Vector2<T> {
    fn from(polar: &Polar<T>) -> Self {
        let (sin, cos) = polar.theta.sin_cos();
        Vector2 {
            x: polar.radius * cos,
            y: polar.radius * sin,
        }
    }
}

/*****************
 * DISPLAY TRAIT *
 *****************/

impl<T: Num + Display> Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
