// Math tools
// Author: Bryce "BtheDestroyer" Dixon
// Created: 9/24/2018
// Modified: 9/24/2018

// 2D vector location
struct Vec2 {
    x: u32,
    y: u32
}

// 3D vector location
struct Vec3 {
    x: u32,
    y: u32,
    z: u32
}

// 4D vector location
struct Vec4 {
    x: u32,
    y: u32,
    z: u32,
    w: u32
}

// Vec2 + Vec2/3/4 = Vec2/3/4
impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: _rhs.z
        }
    }

    fn add(self, _rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: _rhs.z,
            w: _rhs.w
        }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: _rhs.z
        }
    }

    fn add(self, _rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: _rhs.z,
            w: _rhs.w
        }
    }
}


// Vec3 + Vec2/3/4 = Vec3/3/4
impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec2) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z
        }
    }

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }

    fn add(self, _rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: _rhs.w
        }
    }
}

// Vec4 + Vec2/3/4 = Vec4/4/4
impl std::ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, _rhs: Vec2) -> Vec4 {
        Vec4 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z,
            w: self.w
        }
    }

    fn add(self, _rhs: Vec3) -> Vec4 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w
        }
    }

    fn add(self, _rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w
        }
    }
}
