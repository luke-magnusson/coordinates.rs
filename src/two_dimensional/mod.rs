use std::ops::{Add, Neg, Sub};

use num_traits::Float;

use crate::traits::Positional;

mod polar;
mod vector2;

pub use polar::*;
pub use vector2::*;

/// Auto-trait for structs that implement consts and operators.
pub trait FullTwoDimensional<U> {}

impl<T, U: Float> FullTwoDimensional<U> for T where
    T: Positional<U> + TwoDimensionalConsts<U> + Add + Sub + Neg + Sized
{
}

/// Trait holding constants for unit vectors and the origin
pub trait TwoDimensionalConsts<T: Float> {
    /// Center of the coordinate space
    const ORIGIN: Self;
    /// Unit vector pointing in the positive y direction
    const UP: Self;
    /// Unit vector pointing in the negative y direction
    const DOWN: Self;
    /// Unit vector pointing in the positive x direction
    const LEFT: Self;
    /// Unit vector pointing in the negative x direction
    const RIGHT: Self;
}
