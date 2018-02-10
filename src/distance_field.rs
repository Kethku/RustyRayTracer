use color::*;
use vector::*;

#[derive(Copy, Clone)]
pub struct Characteristics {
}

pub trait Field {
    fn distance_sampler(&self, Vector) -> f64;
    fn characteristic_sampler(&self, Vector) -> (Vector, Characteristics);
}

pub struct Sphere {
    pub position: Vector,
    pub radius: f64,
    pub characteristics: Characteristics
}

impl Field for Sphere {
    fn distance_sampler(&self, pos: Vector) -> f64 {
        (pos - self.position).length() - self.radius
    }

    fn characteristic_sampler(&self, pos: Vector) -> (Vector, Characteristics) {
        (pos, self.characteristics)
    }
}

pub struct Union<T1: Field, T2: Field> {
    pub field1: T1,
    pub field2: T2
}

impl<T1: Field, T2: Field> Field for Union<T1, T2> {
    fn distance_sampler(&self, pos: Vector) -> f64 {
        let dist1 = self.field1.distance_sampler(pos);
        let dist2 = self.field2.distance_sampler(pos);

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
