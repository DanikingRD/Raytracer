pub mod math {

    pub type Point3D = Vec3D;
    pub type Color = Vec3D;

    /// Represents a 3D vector.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Vec3D {
        /// X component
        pub x: f32,
        /// Y component
        pub y: f32,
        /// Z component
        pub z: f32,
    }

    impl std::ops::Add for Vec3D {
        type Output = Self;

        /// Handles vector addition
        /// Example:
        /// ```
        /// use raytracer::math::Vec3D;
        /// let v1 = Vec3D::new(1.0, 1.0, 1.0);
        /// let v2 = Vec3D::new(1.0, 1.0, 1.0);
        /// assert_eq!(v1 + v2, Vec3D::new(2.0, 2.0, 2.0));
        /// ```
        fn add(self, other: Self) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    /// Handles vector addition
    /// Example:
    /// ```
    /// use raytracer::math::Vec3D;
    /// let v1 = Vec3D::new(1.0, 1.0, 1.0);
    /// let v2 = Vec3D::new(1.0, 1.0, 1.0);
    /// assert_eq!(v1 - v2, Vec3D::new(0.0, 0.0, 0.0));
    /// ```
    impl std::ops::Sub for Vec3D {
        type Output = Self;
        fn sub(self, other: Self) -> Self::Output {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl Vec3D {
        /// A vector that has zero magnitude.
        pub const ORIGIN: Vec3D = Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        pub fn new(x: f32, y: f32, z: f32) -> Vec3D {
            Self { x, y, z }
        }

        pub fn x(&self) -> f32 {
            self.x
        }

        pub fn y(&self) -> f32 {
            self.y
        }

        pub fn z(&self) -> f32 {
            self.z
        }

        /// Computes the length squared of the vector.
        pub fn length_squared(&self) -> f32 {
            return (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
        }

        /// Computes the magnitue of the vector.
        /// Following the pythagoras theorem.
        pub fn length(&self) -> f32 {
            return self.length_squared().sqrt();
        }
    }
}
