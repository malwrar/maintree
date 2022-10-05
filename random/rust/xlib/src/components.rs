use glm;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub vec: glm::Vec3,
}

impl Position {
    pub fn origin() -> Self { Self { vec: glm::vec3(0.0, 0.0, 0.0) } }
    pub fn x(&self) -> f32 { self.vec.x }
    pub fn y(&self) -> f32 { self.vec.y }
    pub fn z(&self) -> f32 { self.vec.z }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Orientation {
    pub angles: glm::Vec3,
}

impl Orientation {
    pub fn forward() -> Self { Self { angles: glm::vec3(0.0, 0.0, 0.0) } }
    pub fn pitch(&self) -> f32 { self.angles.x }
    pub fn yaw(&self) -> f32   { self.angles.y }
    pub fn roll(&self) -> f32  { self.angles.z }
}

