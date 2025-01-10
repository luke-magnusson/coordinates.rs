use num_traits::{Float, Num};

pub trait Positional<T: Float>
where
    Self: Magnitude<T> + Dot<T> + CrossMagnitude<T> + Copy,
{
    /// Angle between two points in space.
    ///
    /// # Returns
    ///
    /// The result is a positive value (in radians) aside from if
    /// `self.magnitude() == 0` or `other.magnitude() == 0` where it will return
    /// NaN (acos(infinity) is undefined)
    ///
    /// # Examples
    ///
    /// ## In 2D
    /// ```
    /// # use coordinates::prelude::*;
    /// let right =  Vector2::<f64>::RIGHT;
    /// let up = Vector2::<f64>::UP;
    ///
    /// assert!((right.angle_to(&up) - std::f64::consts::FRAC_PI_2).abs() < std::f64::EPSILON);
    /// ```
    ///
    /// ## In 3D
    /// ```
    /// # use coordinates::prelude::*;
    /// let right= Vector3::<f64>::RIGHT;
    /// let up = Vector3::<f64>::UP;
    ///
    /// assert!((right.angle_to(&up) - std::f64::consts::FRAC_PI_2).abs() < std::f64::EPSILON);
    /// ```
    fn angle_to(&self, other: &Self) -> T {
        (self.dot(&other) / (self.magnitude() * other.magnitude())).acos()
    }
}

pub trait Magnitude<T: Float>
where
    Self: Sized + std::ops::Div<T, Output = Self> + Copy,
{
    /// Returns the exact magnitude of the vector
    ///
    /// This is the same as getting the euclidean distance from the origin to the
    /// tip of the vector, i.e. `sqrt(x^2 + y^2 + z^2)` for a cartesian coordinate
    fn magnitude(&self) -> T;

    /// Returns the magnitude of a vector in as few operations as possible
    ///
    /// # Note:
    ///
    /// This operation is not appropriate for comparing vectors of different type.
    ///  e.g. the spherical coordinates `(10,0,0)` will have a smaller
    ///  `quick_magnitude()` than the cartesian coordinate `(0,5,0)` despite
    ///  having a larger real magnitude
    fn quick_magnitude(&self) -> T;

    fn normalize(self) -> Self {
        self / self.magnitude()
    }
}

pub trait Dot<T: Num> {
    fn dot(&self, rhs: &Self) -> T;
}

pub trait Cross3D {
    fn cross(&self, rhs: &Self) -> Self;
}
pub trait CrossMagnitude<T: Float> {
    fn cross_magnitude(&self, rhs: &Self) -> T;
}

impl<T, U: Float> CrossMagnitude<U> for T
where
    T: Magnitude<U> + Cross3D,
{
    fn cross_magnitude(&self, rhs: &Self) -> U {
        self.cross(rhs).magnitude()
    }
}

pub trait TrigConsts
where
    Self: Sized,
{
    const ZERO: Self;
    const ONE: Self;
    const NEG_ONE: Self;
    const TAU: Self;
    const FRAC_3PI_2: Self;
    const PI: Self;
    const FRAC_PI_2: Self;
    const FRAC_PI_3: Self;
    const FRAC_PI_4: Self;
    const FRAC_PI_6: Self;
    const FRAC_PI_8: Self;
    const FRAC_1_PI: Self;
    const FRAC_2_PI: Self;
    const FRAC_2_SQRT_PI: Self;
}

macro_rules! impl_trig_consts {
    ($($t : ident) +) => {
        // Find constants of rust primitives.
        $(impl_trig_consts!{$t, std::$t::consts})+
    };
    ($t : ident, $path : path) => {
        // Find constants as children of the path
        //  e.g. `std::f32::consts{TAU,PI,...}`
        impl_trig_consts!{$t, $path, TAU, PI, FRAC_PI_2, FRAC_PI_3,FRAC_PI_4,FRAC_PI_6,FRAC_PI_8, FRAC_1_PI, FRAC_2_PI, FRAC_2_SQRT_PI}
    };
    ($t : ident, $path : path, $($constant : ident),+)=> {
        impl TrigConsts for $t{
        // For each provided constant expand to a usable function
        // e.g. `impl_constants!(f32, std::f32, TAU)`
        //      expands to `const TAU: f32 = std::f32::consts::TAU`
        $(
            const $constant : Self = { use $path as consts; consts::$constant };
        )+
        const NEG_ONE : Self = -1.0;
        const ZERO: Self = 0.0;
        const ONE: Self = 1.0;
        const FRAC_3PI_2: Self = { use $path as consts; consts::PI + consts::FRAC_PI_2 };
    }

    };
}

impl_trig_consts!(f32 f64);
