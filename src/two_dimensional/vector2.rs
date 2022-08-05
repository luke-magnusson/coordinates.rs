use std::{
    fmt::Display,
    ops::{Add, Neg, Sub},
};

use num_traits::{Float, Num};

use crate::traits::TrigConsts;

use super::polar::Polar;

#[cfg(serde)]
use serde::{Deserialize, Serialize};
/***************
 * BASE STRUCT *
 ***************/

#[cfg_attr(serde, derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct Vector2<T: Num> {
    pub x: T,
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

impl<T: Float> crate::traits::CrossMagnitude<T> for Vector2<T> {
    fn cross_magnitude(&self, rhs: &Self) -> T {
        self.x * rhs.y - rhs.x * self.y
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

impl<T: Float> std::ops::Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
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
