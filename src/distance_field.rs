use vector::*;
use scene::*;

#[derive(Copy, Clone)]
pub struct Characteristics {
    pub normal: Vector,
    pub color: Vector,
    pub roughness: f64,
    pub reflectance: f64,
    pub absorbance: f64
}

impl Characteristics {
    pub fn default() -> Characteristics {
        Characteristics {
            normal: Vector::zero(),
            color: Vector::zero(),
            roughness: 0.0,
            reflectance: 0.0,
            absorbance: 0.0
        }
    }

    pub fn mirror(color: Vector) -> Characteristics {
        Characteristics {
            normal: Vector::zero(),
            color: color,
            roughness: 0.0,
            reflectance: 1.0,
            absorbance: 0.2
        }
    }

    pub fn matte(color: Vector) -> Characteristics {
        Characteristics {
            normal: Vector::zero(),
            color: color,
            roughness: 1.0,
            reflectance: 0.0,
            absorbance: 0.2
        }
    }
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

pub struct Negate<T: Field> {
    pub field: T
}

impl<T: Field> Field for Negate<T> {
    fn distance_sampler(&self, pos: Vector) -> f64 {
        -self.field.distance_sampler(pos)
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

pub struct Intersection<T1: Field, T2: Field> {
    pub field1: T1,
    pub field2: T2
}

impl<T1: Field, T2: Field> Field for Intersection<T1, T2> {
    fn distance_sampler(&self, pos: Vector) -> f64 {
        let dist1 = self.field1.distance_sampler(pos);
        let dist2 = self.field2.distance_sampler(pos);

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
