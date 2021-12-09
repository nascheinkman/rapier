use crate::math::{Point, Real};

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
/// A joint that simulates a hookean spring attached to a pair of points on two bodies.
pub struct SpringJoint {
    /// Where the spring joint is attached on the first body, expressed in the first body local
    /// frame.
    pub local_anchor1: Point<Real>,

    /// Where the spring joint is attached on the second body, expressed in the second body local
    /// frame.
    pub local_anchor2: Point<Real>,

    /// The rest length of this joint
    pub rest_length: Real,

    /// The linear stiffness in units of force / unit length
    pub stiffness: Real,

    /// The linear damping in units of force / unit velocity
    pub damping: Real,

    /// The impulse applied by the spring force.
    pub impulse: Real,

    /// Are the limits enabled for this joint?
    pub limits_enabled: bool,

    /// The minimum length allowed between the two anchor points
    pub limits_min_length: Real,

    /// The maximum length allowed between the two anchor points
    pub limits_max_length: Real,

    /// The impulse applied by the lower limit
    pub limits_lower_impulse: Real,

    /// The impulse applied by the upper limit
    pub limits_upper_impulse: Real,
}

impl SpringJoint {
    /// Creates a new Spring joint from two anchors given on the local spaces of the respective
    /// bodies, and a given rest length, stiffness, and damping
    pub fn new(
        local_anchor1: Point<Real>,
        local_anchor2: Point<Real>,
        rest_length: Real,
        stiffness: Real,
        damping: Real,
    ) -> Self {
        Self::with_impulse(
            local_anchor1,
            local_anchor2,
            rest_length,
            stiffness,
            damping,
            0.0,
        )
    }

    pub(crate) fn with_impulse(
        local_anchor1: Point<Real>,
        local_anchor2: Point<Real>,
        rest_length: Real,
        stiffness: Real,
        damping: Real,
        impulse: Real,
    ) -> Self {
        Self {
            local_anchor1,
            local_anchor2,
            rest_length,
            stiffness,
            damping,
            impulse,
            limits_enabled: false,
            limits_min_length: 0.0,
            limits_max_length: Real::MAX,
            limits_lower_impulse: 0.0,
            limits_upper_impulse: 0.0,
        }
    }

    /// Can a SIMD constraint be used for resolving this joint?
    pub fn supports_simd_constraints(&self) -> bool {
        // SIMD spring constraints don't support limits right now.
        true
    }
}
