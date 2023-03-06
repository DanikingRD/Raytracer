
pub type Point3D = Vec3D;
pub type Color = Vec3D;

/// Represents a 3D vector. 
#[derive(Debug)]
pub struct Vec3D {
    /// X component
    pub x: f32,
    /// Y component
    pub y: f32, 
    /// Z component
    pub z: f32
}

 
impl Vec3D {

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    /// Finds out the current length squared of the vector.
    pub fn length_squared(&self) -> f32 {
        return (self.x * self.x) + (self.y * self.y) + (self.z + self.z)
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    
}