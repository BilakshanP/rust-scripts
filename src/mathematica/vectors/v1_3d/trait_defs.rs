use super::Vector3D;

impl std::fmt::Display for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", super::internals::pretty_print(self.x, self.y, self.z))
    }
}

impl std::fmt::Debug for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} {} {} )", self.x, self.y, self.z)
    }
}

impl std::ops::Add for Vector3D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl std::ops::Sub for Vector3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl std::ops::Mul for Vector3D {
    type Output = f64;

    fn mul(self, other: Self) -> Self::Output {
        self.dot(other)
    }
}

impl std::ops::Div for Vector3D {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.cross(other)
    }
}

impl std::ops::Mul<f64> for Vector3D {
    type Output = Self;

    fn mul(self, num: f64) -> Self::Output {
        Self { x: self.x * num, y: self.y * num, z: self.z * num }
    }
}

impl std::ops::Mul<Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, vector: Vector3D) -> Self::Output {
        vector * self
    }
}

impl std::ops::Div<f64> for Vector3D {
    type Output = Self;

    fn div(self, num: f64) -> Self::Output {
        Self { x: self.x / num, y: self.y / num, z: self.z / num }
    }
}

impl std::ops::Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl std::cmp::PartialEq for Vector3D {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

