use vector::*;
use scene::*;
use characteristics::*;
use geometry::*;
use std::f64::*;

pub trait Field {
    fn ray_cast(&self, Vector, Vector) -> Option<Vector>;
    fn distance(&self, Vector) -> f64;
    fn normal(&self, Vector) -> Vector;
    fn characteristics(&self, Vector) -> Characteristics;
}

pub struct Sphere {
    pub position: Vector,
    pub radius: f64,
    pub characteristics: Characteristics
}

impl Sphere {
    pub fn new(pos: Vector, r: f64, chars: Characteristics) -> Scene<Sphere> {
        Scene {
            field: Sphere {
                position: pos,
                radius: r,
                characteristics: chars
            }
        }
    }
}

impl Field for Sphere {
    fn ray_cast(&self, pos: Vector, dir: Vector) -> Option<Vector> {
        if (pos - self.position).length() < self.radius {
            return Some(pos);
        }
        sphere_intersection(self.position, self.radius, pos, dir)
    }

    fn distance(&self, pos: Vector) -> f64 {
        (pos - self.position).length() - self.radius
    }

    fn normal(&self, pos: Vector) -> Vector {
        (pos - self.position).normalize()
    }

    fn characteristics(&self, pos: Vector) -> Characteristics {
        self.characteristics
    }
}

pub struct Plane {
    pub normal: Vector,
    pub point: Vector,
    pub characteristics: Characteristics
}

impl Plane {
    pub fn new(normal: Vector, point: Vector, chars: Characteristics) -> Scene<Plane> {
        Scene {
            field: Plane {
                normal: normal,
                point: point,
                characteristics: chars
            }
        }
    }
}

impl Field for Plane {
    fn ray_cast(&self, pos: Vector, dir: Vector) -> Option<Vector> {
        let dist = self.distance(pos);
        if dist < 0.0 {
            Some(pos)
        } else {
            plane_intersection(self.normal, self.point, pos, dir)
        }
    }

    fn distance(&self, pos: Vector) -> f64 {
        (pos - self.point).dot(self.normal)
    }

    fn normal(&self, pos: Vector) -> Vector {
        self.normal
    }

    fn characteristics(&self, pos: Vector) -> Characteristics {
        self.characteristics
    }
}

pub struct Negate<T: Field> {
    pub field: T
}

impl<T: Field> Field for Negate<T> {
    fn ray_cast(&self, pos: Vector, dir: Vector) -> Option<Vector> {
        self.field.ray_cast(pos, dir)
    }

    fn distance(&self, pos: Vector) -> f64 {
        -self.field.distance(pos)
    }

    fn normal(&self, pos: Vector) -> Vector {
        -self.field.normal(pos)
    }

    fn characteristics(&self, pos: Vector) -> Characteristics {
        self.field.characteristics(pos)
    }
}

pub struct Union<T1: Field, T2: Field> {
    pub field1: T1,
    pub field2: T2
}

impl<T1: Field, T2: Field> Field for Union<T1, T2> {
    fn ray_cast(&self, pos: Vector, dir: Vector) -> Option<Vector> {
        let p1 = self.field1.ray_cast(pos, dir);
        let p2 = self.field2.ray_cast(pos, dir);
        let dist1 = p1.map_or(INFINITY, |p| (p - pos).length_squared());
        let dist2 = p2.map_or(INFINITY, |p| (p - pos).length_squared());

        if dist1 < dist2 {
            p1
        } else {
            p2
        }
    }

    fn distance(&self, pos: Vector) -> f64 {
        self.field1.distance(pos).min(self.field2.distance(pos))
    }

    fn normal(&self, pos: Vector) -> Vector {
        let dist1 = self.field1.distance(pos);
        let dist2 = self.field2.distance(pos);

        if dist1 < dist2 {
            self.field1.normal(pos)
        } else {
            self.field2.normal(pos)
        }
    }

    fn characteristics(&self, pos: Vector) -> Characteristics {
        let dist1 = self.field1.distance(pos);
        let dist2 = self.field2.distance(pos);

        if dist1 < dist2 {
            self.field1.characteristics(pos)
        } else {
            self.field2.characteristics(pos)
        }
    }
}

pub struct Intersection<T1: Field, T2: Field> {
    pub field1: T1,
    pub field2: T2
}

impl<T1: Field, T2: Field> Field for Intersection<T1, T2> {
    fn ray_cast(&self, pos: Vector, dir: Vector) -> Option<Vector> {
        let p1 = self.field1.ray_cast(pos, dir);
        let p2 = self.field2.ray_cast(pos, dir);
        let dist1 = p1.map_or(INFINITY, |p| (p - pos).length_squared());
        let dist2 = p2.map_or(INFINITY, |p| (p - pos).length_squared());

        if dist1 > dist2 {
            p1
        } else {
            p2
        }
    }

    fn distance(&self, pos: Vector) -> f64 {
        self.field1.distance(pos).max(self.field2.distance(pos))
    }

    fn normal(&self, pos: Vector) -> Vector {
        let dist1 = self.field1.distance(pos);
        let dist2 = self.field2.distance(pos);

        if dist1 > dist2 {
            self.field1.normal(pos)
        } else {
            self.field2.normal(pos)
        }
    }

    fn characteristics(&self, pos: Vector) -> Characteristics {
        let dist1 = self.field1.distance(pos);
        let dist2 = self.field2.distance(pos);

        if dist1 > dist2 {
            self.field1.characteristics(pos)
        } else {
            self.field2.characteristics(pos)
        }
    }
}
