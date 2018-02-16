use vector::*;
use scene::*;
use characteristics::*;
use geometry::*;
use std::f64::*;

pub trait Field {
    fn ray_distance_sampler(&self, Vector, Vector) -> f64;
    fn distance_sampler(&self, pos: Vector) -> f64 {
        let (_, chars) = self.characteristic_sampler(pos);
        self.ray_distance_sampler(pos, chars.normal)
    }
    fn characteristic_sampler(&self, Vector) -> (Vector, Characteristics);
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
    fn ray_distance_sampler(&self, pos: Vector, dir: Vector) -> f64 {
        match sphere_intersection(self.position, self.radius, pos, dir) {
            Some(intersect) => (intersect - pos).length(),
            None => INFINITY
        }
    }

    fn distance_sampler(&self, pos: Vector) -> f64 {
        (pos - self.position).length() - self.radius
    }

    fn characteristic_sampler(&self, pos: Vector) -> (Vector, Characteristics) {
        (pos, Characteristics {
            normal: (pos - self.position).normalize(),
            .. self.characteristics
        })
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
                characteristics: Characteristics {
                    normal: normal,
                    ..chars
                }
            }
        }
    }
}

impl Field for Plane {
    fn ray_distance_sampler(&self, pos: Vector, dir: Vector) -> f64 {
        match plane_intersection(self.normal, self.point, pos, dir) {
            Some(intersect) => (intersect - pos).length(),
            None => INFINITY
        }
    }

    fn characteristic_sampler(&self, pos: Vector) -> (Vector, Characteristics) {
        (pos, self.characteristics)
    }
}

pub struct Negate<T: Field> {
    pub field: T
}

impl<T: Field> Field for Negate<T> {
    fn ray_distance_sampler(&self, pos: Vector, dir: Vector) -> f64 {
        -self.field.ray_distance_sampler(pos, dir)
    }

    fn characteristic_sampler(&self, pos: Vector) -> (Vector, Characteristics) {
        self.field.characteristic_sampler(pos)
    }
}

pub struct Union<T1: Field, T2: Field> {
    pub field1: T1,
    pub field2: T2
}

impl<T1: Field, T2: Field> Field for Union<T1, T2> {
    fn ray_distance_sampler(&self, pos: Vector, dir: Vector) -> f64 {
        let dist1 = self.field1.ray_distance_sampler(pos, dir);
        let dist2 = self.field2.ray_distance_sampler(pos, dir);

        if dist1 < dist2 {
            dist1
        } else {
            dist2
        }
    }

    fn characteristic_sampler(&self, pos: Vector) -> (Vector, Characteristics) {
        let dist1 = self.field1.distance_sampler(pos);
        let dist2 = self.field2.distance_sampler(pos);

        if dist1 < dist2 {
            self.field1.characteristic_sampler(pos)
        } else {
            self.field2.characteristic_sampler(pos)
        }
    }
}

pub struct Intersection<T1: Field, T2: Field> {
    pub field1: T1,
    pub field2: T2
}

impl<T1: Field, T2: Field> Field for Intersection<T1, T2> {
    fn ray_distance_sampler(&self, pos: Vector, dir: Vector) -> f64 {
        let dist1 = self.field1.ray_distance_sampler(pos, dir);
        let dist2 = self.field2.ray_distance_sampler(pos, dir);

        if dist1 > dist2 {
            dist1
        } else {
            dist2
        }
    }

    fn characteristic_sampler(&self, pos: Vector) -> (Vector, Characteristics) {
        let dist1 = self.field1.distance_sampler(pos);
        let dist2 = self.field2.distance_sampler(pos);

        if dist1 > dist2 {
            self.field1.characteristic_sampler(pos)
        } else {
            self.field2.characteristic_sampler(pos)
        }
    }
}
