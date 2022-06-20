// Generated from {{template_path}} template. Edit the template, not the generated file.

{% if scalar_t == "f32" %}
    {% if dim == 3 %}
        {% set self_t = "Affine3A" %}
        {% set col_t = "Vec3A" %}
        {% set mat_t = "Mat3A" %}
    {% else %}
        {% set self_t = "Affine" ~ dim %}
        {% set col_t = "Vec" ~ dim %}
        {% set mat_t = "Mat" ~ dim %}
    {% endif %}
    {% set quat_t = "Quat" %}
    {% set vec2_t = "Vec2" %}
    {% set vec3_t = "Vec3" %}
    {% set mat3_t = "Mat3" %}
    {% set mat4_t = "Mat4" %}
{% elif scalar_t == "f64" %}
    {% set self_t = "DAffine" ~ dim %}
    {% set col_t = "DVec" ~ dim %}
    {% set mat_t = "DMat" ~ dim %}
    {% set quat_t = "DQuat" %}
    {% set vec2_t = "DVec2" %}
    {% set vec3_t = "DVec3" %}
    {% set mat3_t = "DMat3" %}
    {% set mat4_t = "DMat4" %}
{% endif %}

{% if dim == 2 %}
    {% set size = 6 %}
{% elif dim == 3 %}
    {% set size = 12 %}
{% endif %}

{% set components = ["x", "y", "z", "w"] | slice(end = dim + 1) %}
{% set axes = ["x_axis", "y_axis", "z_axis", "w_axis"] | slice(end = dim + 1) %}

use crate::{
{% if self_t == "Affine2" %}
    Mat3A, Vec3A,
{% elif self_t == "Affine3A" %}
    Vec3, Mat3,
{% endif %}
{% if dim == 2 %}
    {{ mat_t }}, {{ col_t }}, {{ mat3_t }},
{% elif dim == 3 %}
    {{ mat_t }}, {{ col_t}}, {{ mat4_t }}, {{ quat_t }},
{% endif %}
};
use core::ops::{Add, Deref, DerefMut, Mul, Sub};

/// A {{ dim }}D affine transform, which can represent translation, rotation, scaling and shear.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct {{ self_t }} {
    pub matrix{{ dim }}: {{ mat_t }},
    pub translation: {{ col_t }},
}

impl {{ self_t }} {
    /// The degenerate zero transform.
    ///
    /// This transforms any finite vector and point to zero.
    /// The zero transform is non-invertible.
    pub const ZERO: Self = Self {
        matrix{{ dim }}: {{ mat_t }}::ZERO,
        translation: {{ col_t }}::ZERO,
    };

    /// The identity transform.
    ///
    /// Multiplying a vector with this returns the same vector.
    pub const IDENTITY: Self = Self {
        matrix{{ dim }}: {{ mat_t }}::IDENTITY,
        translation: {{ col_t }}::ZERO,
    };

    /// All NAN:s.
    pub const NAN: Self = Self {
        matrix{{ dim }}: {{ mat_t }}::NAN,
        translation: {{ col_t }}::NAN,
    };

    /// Creates an affine transform from three column vectors.
    #[inline(always)]
    pub const fn from_cols(
            {% for axis in axes %}
                {{ axis }}: {{ col_t }},
            {% endfor %}
        ) -> Self {
        Self {
            matrix{{ dim }}: {{ mat_t }}::from_cols(
                {% for axis in axes | slice(end = dim) %}
                    {{ axis }},
                {% endfor %}
            ),
            translation: {{ axes[dim] }},
        }
    }

    /// Creates an affine transform from a `[{{ scalar_t }}; {{ size }}]` array stored in column major order.
    #[inline]
    pub fn from_cols_array(m: &[{{ scalar_t }}; {{ size }}]) -> Self {
        Self {
            matrix{{ dim }}: {{ mat_t }}::from_cols_slice(&m[0..{{ dim * dim }}]),
            translation: {{ col_t }}::from_slice(&m[{{ dim * dim }}..{{ size }}]),
        }
    }

    /// Creates a `[{{ scalar_t }}; {{ size }}]` array storing data in column major order.
    #[inline]
    pub fn to_cols_array(&self) -> [{{ scalar_t }}; {{ size }}] {
        {% for i in range(end = dim) %}
            let {{ components[i] }} = &self.matrix{{ dim }}.{{ axes[i] }};
        {%- endfor %}
        let {{ components[dim] }} = &self.translation;
        [
            {% for i in range(end = dim + 1) %}
                {% for j in range(end = dim) %}
                    {{ components[i] }}.{{ components[j] }},
                {% endfor %}
            {% endfor %}
        ]
    }

    /// Creates an affine transform from a `[[{{ scalar_t }}; {{ dim }}]; {{ dim + 1 }}]`
    /// {{ dim }}D array stored in column major order.
    /// If your data is in row major order you will need to `transpose` the returned
    /// matrix.
    #[inline]
    pub fn from_cols_array_2d(m: &[[{{ scalar_t }}; {{ dim }}]; {{ dim + 1 }}]) -> Self {
        Self {
            matrix{{ dim }}: {{ mat_t }}::from_cols(
                {% for i in range(end = dim) %}
                    m[{{ i }}].into(),
                {% endfor %}
            ),
            translation: m[{{ dim }}].into(),
        }
    }

    /// Creates a `[[{{ scalar_t }}; {{ dim }}]; {{ dim + 1 }}]` {{ dim }}D array storing data in
    /// column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    pub fn to_cols_array_2d(&self) -> [[{{ scalar_t }}; {{ dim }}]; {{ dim + 1 }}] {
        [
            {% for i in range(end = dim) %}
                self.matrix{{ dim }}.{{ axes[i] }}.into(),
            {% endfor %}
            self.translation.into(),
        ]
    }

    /// Creates an affine transform from the first {{ size }} values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than {{ size }} elements long.
    #[inline]
    pub fn from_cols_slice(slice: &[{{ scalar_t }}]) -> Self {
        Self {
            matrix{{ dim }}: {{ mat_t }}::from_cols_slice(&slice[0..{{ dim * dim }}]),
            translation: {{ col_t }}::from_slice(&slice[{{ dim * dim }}..{{ size }}]),
        }
    }

    /// Writes the columns of `self` to the first {{ size }} elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than {{ size }} elements long.
    #[inline]
    pub fn write_cols_to_slice(self, slice: &mut [{{ scalar_t }}]) {
        self.matrix{{ dim }}.write_cols_to_slice(&mut slice[0..{{ dim * dim }}]);
        self.translation.write_to_slice(&mut slice[{{ dim * dim }}..{{ size }}]);
    }

{% if dim == 2 %}
    /// Creates an affine transform that changes scale.
    /// Note that if any scale is zero the transform will be non-invertible.
    #[inline]
    pub fn from_scale(scale: {{ vec2_t }}) -> Self {
        Self {
            matrix{{ dim }}: {{ mat_t }}::from_diagonal(scale),
            translation: {{ col_t }}::ZERO,
        }
    }

    /// Creates an affine transform from the given rotation `angle`.
    #[inline]
    pub fn from_angle(angle: {{ scalar_t }}) -> Self {
        Self {
            matrix2: {{ mat_t }}::from_angle(angle),
            translation: {{ col_t }}::ZERO,
        }
    }

    /// Creates an affine transformation from the given 2D `translation`.
    #[inline]
    pub fn from_translation(translation: {{ vec2_t }}) -> Self {
        Self {
            matrix2: {{ mat_t }}::IDENTITY,
            translation,
        }
    }

    /// Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)
    #[inline]
    pub fn from_mat2(matrix2: {{ mat_t }}) -> Self {
        Self {
            matrix2,
            translation: {{ col_t }}::ZERO,
        }
    }

    /// Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
    /// translation vector.
    ///
    /// Equivalent to
    /// `{{ self_t }}::from_translation(translation) * {{ self_t }}::from_mat{{ dim }}(mat{{ dim }})`
    #[inline]
    pub fn from_mat2_translation(matrix2: {{ mat_t }}, translation: {{ vec2_t }}) -> Self {
        Self {
            matrix2,
            translation,
        }
    }

    /// Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
    /// `translation`.
    ///
    /// Equivalent to `{{ self_t }}::from_translation(translation) *
    /// {{ self_t }}::from_angle(angle) * {{ self_t }}::from_scale(scale)`
    #[inline]
    pub fn from_scale_angle_translation(
        scale: {{ vec2_t }},
        angle: {{ scalar_t }},
        translation: {{ vec2_t }},
    ) -> Self {
        let rotation = {{ mat_t }}::from_angle(angle);
        Self {
            matrix2: {{ mat_t }}::from_cols(
                         rotation.x_axis * scale.x,
                         rotation.y_axis * scale.y,
                     ),
                     translation,
        }
    }

    /// Creates an affine transform from the given 2D rotation `angle` (in radians) and
    /// `translation`.
    ///
    /// Equivalent to `{{ self_t }}::from_translation(translation) * {{ self_t }}::from_angle(angle)`
    #[inline]
    pub fn from_angle_translation(angle: {{ scalar_t }}, translation: {{ vec2_t }}) -> Self {
        Self {
            matrix2: {{ mat_t }}::from_angle(angle),
            translation,
        }
    }

    /// The given `{{ mat3_t }}` must be an affine transform,
    #[inline]
    pub fn from_mat3(m: {{ mat3_t }}) -> Self {
        use crate::swizzles::Vec3Swizzles;
        Self {
            matrix2: {{ mat_t }}::from_cols(m.x_axis.xy(), m.y_axis.xy()),
            translation: m.z_axis.xy(),
        }
    }

    /// Transforms the given 2D point, applying shear, scale, rotation and translation.
    #[inline]
    pub fn transform_point2(&self, rhs: {{ vec2_t }}) -> {{ vec2_t }} {
        self.matrix2 * rhs + self.translation
    }

    /// Transforms the given 2D vector, applying shear, scale and rotation (but NOT
    /// translation).
    ///
    /// To also apply translation, use [`Self::transform_point2`] instead.
    #[inline]
    pub fn transform_vector2(&self, rhs: {{ vec2_t }}) -> {{ vec2_t }} {
        self.matrix2 * rhs
    }

{% elif dim == 3 %}
    /// Creates an affine transform that changes scale.
    /// Note that if any scale is zero the transform will be non-invertible.
    #[inline]
    pub fn from_scale(scale: {{ vec3_t }}) -> Self {
        Self {
            matrix3: {{ mat_t }}::from_diagonal(scale),
            translation: {{ col_t }}::ZERO,
        }
    }
    /// Creates an affine transform from the given `rotation` quaternion.
    #[inline]
    pub fn from_quat(rotation: {{ quat_t }}) -> Self {
        Self {
            matrix3: {{ mat_t }}::from_quat(rotation),
            translation: {{ col_t }}::ZERO,
        }
    }

    /// Creates an affine transform containing a 3D rotation around a normalized
    /// rotation `axis` of `angle` (in radians).
    #[inline]
    pub fn from_axis_angle(axis: {{ vec3_t }}, angle: {{ scalar_t }}) -> Self {
        Self {
            matrix3: {{ mat_t }}::from_axis_angle(axis, angle),
            translation: {{ col_t }}::ZERO,
        }
    }

    /// Creates an affine transform containing a 3D rotation around the x axis of
    /// `angle` (in radians).
    #[inline]
    pub fn from_rotation_x(angle: {{ scalar_t }}) -> Self {
        Self {
            matrix3: {{ mat_t }}::from_rotation_x(angle),
            translation: {{ col_t }}::ZERO,
        }
    }

    /// Creates an affine transform containing a 3D rotation around the y axis of
    /// `angle` (in radians).
    #[inline]
    pub fn from_rotation_y(angle: {{ scalar_t }}) -> Self {
        Self {
            matrix3: {{ mat_t }}::from_rotation_y(angle),
            translation: {{ col_t }}::ZERO,
        }
    }

    /// Creates an affine transform containing a 3D rotation around the z axis of
    /// `angle` (in radians).
    #[inline]
    pub fn from_rotation_z(angle: {{ scalar_t }}) -> Self {
        Self {
            matrix3: {{ mat_t }}::from_rotation_z(angle),
            translation: {{ col_t }}::ZERO,
        }
    }

    /// Creates an affine transformation from the given 3D `translation`.
    #[inline]
    pub fn from_translation(translation: {{ vec3_t }}) -> Self {
        #[allow(clippy::useless_conversion)]
        Self {
            matrix3: {{ mat_t }}::IDENTITY,
            translation: translation.into(),
        }
    }

    /// Creates an affine transform from a 3x3 matrix (expressing scale, shear and
    /// rotation)
    #[inline]
    pub fn from_mat3(mat3: {{ mat3_t }}) -> Self {
        #[allow(clippy::useless_conversion)]
        Self {
            matrix3: mat3.into(),
            translation: {{ col_t }}::ZERO,
        }
    }

    /// Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
    /// and a translation vector.
    ///
    /// Equivalent to `{{ self_t }}::from_translation(translation) * {{ self_t }}::from_mat3(mat3)`
    #[inline]
    pub fn from_mat3_translation(mat3: {{ mat3_t }}, translation: {{ vec3_t }}) -> Self {
        #[allow(clippy::useless_conversion)]
        Self {
            matrix3: mat3.into(),
            translation: translation.into(),
        }
    }

    /// Creates an affine transform from the given 3D `scale`, `rotation` and
    /// `translation`.
    ///
    /// Equivalent to `{{ self_t }}::from_translation(translation) *
    /// {{ self_t }}::from_quat(rotation) * {{ self_t }}::from_scale(scale)`
    #[inline]
    pub fn from_scale_rotation_translation(
        scale: {{ vec3_t }},
        rotation: {{ quat_t }},
        translation: {{ vec3_t }},
    ) -> Self {
        let rotation = {{ mat_t }}::from_quat(rotation);
        #[allow(clippy::useless_conversion)]
        Self {
            matrix3: {{ mat_t }}::from_cols(
                rotation.x_axis * scale.x,
                rotation.y_axis * scale.y,
                rotation.z_axis * scale.z,
            ),
            translation: translation.into(),
        }
    }

    /// Creates an affine transform from the given 3D `rotation` and `translation`.
    ///
    /// Equivalent to `{{ self_t }}::from_translation(translation) * {{ self_t }}::from_quat(rotation)`
    #[inline]
    pub fn from_rotation_translation(rotation: {{ quat_t }}, translation: {{ vec3_t }}) -> Self {
        #[allow(clippy::useless_conversion)]
        Self {
            matrix3: {{ mat_t }}::from_quat(rotation),
            translation: translation.into(),
        }
    }

    /// The given `{{ mat4_t }}` must be an affine transform,
    /// i.e. contain no perspective transform.
    #[inline]
    pub fn from_mat4(m: {{ mat4_t }}) -> Self {
        Self {
            matrix3: {{ mat_t }}::from_cols(
                {{ col_t }}::from_vec4(m.x_axis),
                {{ col_t }}::from_vec4(m.y_axis),
                {{ col_t }}::from_vec4(m.z_axis),
            ),
            translation: {{ col_t }}::from_vec4(m.w_axis),
        }
    }

    /// Extracts `scale`, `rotation` and `translation` from `self`.
    ///
    /// The transform is expected to be non-degenerate and without shearing, or the output
    /// will be invalid.
    ///
    /// # Panics
    ///
    /// Will panic if the determinant `self.matrix3` is zero or if the resulting scale
    /// vector contains any zero elements when `glam_assert` is enabled.
    #[inline]
    pub fn to_scale_rotation_translation(&self) -> ({{ vec3_t }}, {{ quat_t }}, {{ vec3_t }}) {
        #[cfg(not(feature = "std"))]
        use num_traits::Float;

        // TODO: migrate to core module
        let det = self.matrix3.determinant();
        glam_assert!(det != 0.0);

        let scale = {{ vec3_t }}::new(
            self.matrix3.x_axis.length() * det.signum(),
            self.matrix3.y_axis.length(),
            self.matrix3.z_axis.length(),
        );

        glam_assert!(scale.cmpne({{ vec3_t }}::ZERO).all());

        let inv_scale = scale.recip();

        #[allow(clippy::useless_conversion)]
        let rotation = {{ quat_t }}::from_mat3(&{{ mat3_t }}::from_cols(
            (self.matrix3.x_axis * inv_scale.x).into(),
            (self.matrix3.y_axis * inv_scale.y).into(),
            (self.matrix3.z_axis * inv_scale.z).into(),
        ));

        #[allow(clippy::useless_conversion)]
        (scale, rotation, self.translation.into())
    }

    #[inline]
    fn look_to_lh(eye: {{ vec3_t }}, dir: {{ vec3_t }}, up: {{ vec3_t }}) -> Self {
        let f = dir.normalize();
        let s = up.cross(f).normalize();
        let u = f.cross(s);
        Self {
            matrix3: {{ mat_t }}::from_cols(
                 {{ col_t }}::new(s.x, u.x, f.x),
                 {{ col_t }}::new(s.y, u.y, f.y),
                 {{ col_t }}::new(s.z, u.z, f.z),
            ),
            translation: {{ col_t }}::new(-s.dot(eye), -u.dot(eye), -f.dot(eye)),
        }
    }

    /// Creates a left-handed view transform using a camera position, an up direction, and
    /// a focal point.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// # Panics
    ///
    /// Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn look_at_lh(eye: {{ vec3_t }}, center: {{ vec3_t }}, up: {{ vec3_t }}) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, center - eye, up)
    }

    /// Creates a right-handed view transform using a camera position, an up direction, and
    /// a focal point.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// # Panics
    ///
    /// Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn look_at_rh(eye: {{ vec3_t }}, center: {{ vec3_t }}, up: {{ vec3_t }}) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, eye - center, up)
    }

    /// Transforms the given 3D points, applying shear, scale, rotation and translation.
    #[inline]
    pub fn transform_point3(&self, rhs: {{ vec3_t }}) -> {{ vec3_t }} {
        #[allow(clippy::useless_conversion)]
        ((self.matrix3.x_axis * rhs.x)
            + (self.matrix3.y_axis * rhs.y)
            + (self.matrix3.z_axis * rhs.z)
            + self.translation)
            .into()
    }

    /// Transforms the given 3D vector, applying shear, scale and rotation (but NOT
    /// translation).
    ///
    /// To also apply translation, use [`Self::transform_point3`] instead.
    #[inline]
    pub fn transform_vector3(&self, rhs: {{ vec3_t }}) -> {{ vec3_t }} {
        #[allow(clippy::useless_conversion)]
        ((self.matrix3.x_axis * rhs.x)
            + (self.matrix3.y_axis * rhs.y)
            + (self.matrix3.z_axis * rhs.z))
            .into()
    }
{% endif %}

{% if self_t == "Affine3A" %}
    /// Transforms the given `Vec3A`, applying shear, scale, rotation and translation.
    #[inline]
    pub fn transform_point3a(&self, rhs: Vec3A) -> Vec3A {
        self.matrix3 * rhs + self.translation
    }

    /// Transforms the given `Vec3A`, applying shear, scale and rotation (but NOT
    /// translation).
    ///
    /// To also apply translation, use [`Self::transform_point3`] instead.
    #[inline]
    pub fn transform_vector3a(&self, rhs: Vec3A) -> Vec3A {
        self.matrix3 * rhs
    }
{% endif %}

    /// Returns `true` if, and only if, all elements are finite.
    ///
    /// If any element is either `NaN`, positive or negative infinity, this will return
    /// `false`.
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.matrix{{ dim }}.is_finite() && self.translation.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.matrix{{ dim }}.is_nan() || self.translation.is_nan()
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two 3x4 matrices contain similar elements. It works
    /// best when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    pub fn abs_diff_eq(&self, rhs: Self, max_abs_diff: {{ scalar_t }}) -> bool {
        self.matrix{{ dim }}.abs_diff_eq(rhs.matrix{{ dim }}, max_abs_diff)
            && self
            .translation
            .abs_diff_eq(rhs.translation, max_abs_diff)
    }

    /// Return the inverse of this transform.
    ///
    /// Note that if the transform is not invertible the result will be invalid.
    #[must_use]
    #[inline]
    pub fn inverse(&self) -> Self {
        let matrix{{ dim }} = self.matrix{{ dim }}.inverse();
        // transform negative translation by the matrix inverse:
        let translation = -(matrix{{ dim }} * self.translation);

        Self {
            matrix{{ dim }},
            translation,
        }
    }
}

impl Default for {{ self_t }} {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Deref for {{ self_t }} {
    type Target = crate::deref::Cols{{ dim + 1 }}<{{ col_t }}>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl DerefMut for {{ self_t }} {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

impl PartialEq for {{ self_t }} {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.matrix{{ dim }}.eq(&rhs.matrix{{ dim }}) && self.translation.eq(&rhs.translation)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl core::fmt::Debug for {{ self_t }} {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct(stringify!({{ self_t }}))
            .field("matrix{{ dim }}", &self.matrix{{ dim }})
            .field("translation", &self.translation)
            .finish()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl core::fmt::Display for {{ self_t }} {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        {% if dim == 2 %}
            write!(f, "[{}, {}, {}]", self.matrix2.x_axis, self.matrix2.y_axis, self.translation)
        {% elif dim == 3 %}
            write!(
                f,
                "[{}, {}, {}, {}]",
                self.matrix3.x_axis, self.matrix3.y_axis, self.matrix3.z_axis, self.translation
            )
        {% endif %}
    }
}

impl<'a> core::iter::Product<&'a Self> for {{ self_t }} {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| a * b)
    }
}

impl Mul for {{ self_t }} {
    type Output = {{ self_t }};

    #[inline]
    fn mul(self, rhs: {{ self_t }}) -> Self::Output {
        Self {
            matrix{{ dim }}: self.matrix{{ dim }} * rhs.matrix{{ dim }},
            translation: self.matrix{{ dim }} * rhs.translation + self.translation,
        }
    }
}

impl Mul<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline]
    fn mul(self, rhs: {{ self_t }}) -> Self::Output {
        {{ self_t }} {
            matrix{{ dim }}: self * rhs.matrix{{ dim }},
            translation: self * rhs.translation,
        }
    }
}

impl Mul<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: {{ scalar_t }}) -> Self::Output {
        Self {
            matrix{{ dim }}: self.matrix{{ dim }} * rhs,
            translation: self.translation * rhs,
        }
    }
}

impl Add<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            matrix{{ dim }}: self.matrix{{ dim }} + rhs.matrix{{ dim }},
            translation: self.translation + rhs.translation,
        }
    }
}

impl Sub<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            matrix{{ dim }}: self.matrix{{ dim }} - rhs.matrix{{ dim }},
            translation: self.translation - rhs.translation,
        }
    }
}

{% if dim == 2 %}
impl From<{{ self_t }}> for {{ mat3_t }} {
    #[inline]
    fn from(m: {{ self_t }}) -> {{ mat3_t }} {
        Self::from_cols(
            m.matrix2.x_axis.extend(0.0),
            m.matrix2.y_axis.extend(0.0),
            m.translation.extend(1.0),
        )
    }
}

impl Mul<{{ mat3_t }}> for {{ self_t }} {
    type Output = {{ mat3_t }};

    #[inline]
    fn mul(self, rhs: {{ mat3_t }}) -> Self::Output {
        {{ mat3_t }}::from(self) * rhs
    }
}

impl Mul<{{ self_t }}> for {{ mat3_t }} {
    type Output = {{ mat3_t }};

    #[inline]
    fn mul(self, rhs: {{ self_t }}) -> Self::Output {
        self * {{ mat3_t }}::from(rhs)
    }
}
{% elif dim == 3 %}
impl From<{{ self_t }}> for {{ mat4_t }} {
    #[inline]
    fn from(m: {{ self_t }}) -> {{ mat4_t }} {
        {{ mat4_t }}::from_cols(
            m.matrix3.x_axis.extend(0.0),
            m.matrix3.y_axis.extend(0.0),
            m.matrix3.z_axis.extend(0.0),
            m.translation.extend(1.0),
        )
    }
}

impl Mul<{{ mat4_t }}> for {{ self_t }} {
    type Output = {{ mat4_t }};

    #[inline]
    fn mul(self, rhs: {{ mat4_t }}) -> Self::Output {
        {{ mat4_t }}::from(self) * rhs
    }
}

impl Mul<{{ self_t }}> for {{ mat4_t }} {
    type Output = {{ mat4_t }};

    #[inline]
    fn mul(self, rhs: {{ self_t }}) -> Self::Output {
        self * {{ mat4_t }}::from(rhs)
    }
}
{% endif %}

{% if self_t == "Affine2" %}
impl From<Affine2> for Mat3A {
    #[inline]
    fn from(m: Affine2) -> Mat3A {
        Self::from_cols(
            Vec3A::from((m.matrix2.x_axis, 0.0)),
            Vec3A::from((m.matrix2.y_axis, 0.0)),
            Vec3A::from((m.translation, 1.0)),
        )
    }
}

impl Mul<Mat3A> for Affine2 {
    type Output = Mat3A;

    #[inline]
    fn mul(self, rhs: Mat3A) -> Self::Output {
        Mat3A::from(self) * rhs
    }
}

impl Mul<Affine2> for Mat3A {
    type Output = Mat3A;

    #[inline]
    fn mul(self, rhs: Affine2) -> Self::Output {
        self * Mat3A::from(rhs)
    }
}
{% endif %}
