use std::ops::*;
use vector::*;
use rand::*;

#[derive(Copy, Clone)]
pub struct Characteristics {
    pub normal: Vector,
    pub color: Vector,
    pub roughness: f64,
    pub reflectance: f64,
    pub absorbance: f64
}

impl Characteristics {
    pub fn mirror() -> Characteristics {
        Characteristics {
            normal: Vector::zero(),
            color: Vector::one(),
            roughness: 0.0,
            reflectance: 1.0,
            absorbance: 0.1
        }
    }

    pub fn matte() -> Characteristics {
        Characteristics {
            normal: Vector::zero(),
            color: Vector::one(),
            roughness: 1.0,
            reflectance: 0.0,
            absorbance: 0.1
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

pub struct Scene<T: Field> {
    field: T
}

const MINIMUM_THRESHOLD: f64 = 0.01;
impl<T: Field> Scene<T> {
    pub fn distance_sampler(&self, pos: Vector) -> f64 {
        self.field.distance_sampler(pos)
    }

    pub fn characteristic_sampler(&self, pos: Vector) -> (Vector, Characteristics) {
        self.field.characteristic_sampler(pos)
    }

    pub fn march(&self, position: Vector, direction: Vector, max_distance: f64) -> (Vector, Characteristics) {
        let distance = self.distance_sampler(position);

        if distance < MINIMUM_THRESHOLD || max_distance < 0.0 {
            self.characteristic_sampler(position)
        } else {
            let new_position = position + (direction * distance);
            let jump_distance = (new_position - position).length();
            self.march(new_position, direction, max_distance - jump_distance)
        }
    }

    pub fn trace(&self, transfered_color: Vector, position: Vector, direction: Vector, max_distance: f64) -> Vector {
        let (pos, characteristics) = self.march(position, direction, max_distance);
        let march_distance = (pos - position).length();
        let remaining_distance = max_distance - march_distance;
        if remaining_distance < 0.0 {
            transfered_color
        } else {
            let new_pos = pos + characteristics.normal * MINIMUM_THRESHOLD;
            let mut new_dir = characteristics.normal + Vector::random();

            if thread_rng().gen_range(0.0, 1.0) < characteristics.reflectance {
                let reflection_target = direction - 2.0 * characteristics.normal * characteristics.normal.dot(direction);
                new_dir = Vector::interpolate(reflection_target, new_dir, characteristics.roughness);
            }

            new_dir = new_dir.normalize();

            let material_color = Vector::interpolate(characteristics.color, Vector::one(), characteristics.reflectance) * (1.0 - characteristics.absorbance);

            self.trace(material_color * transfered_color, new_pos, new_dir, remaining_distance)
        }
    }
}

impl<T1: Field, T2: Field> Add<Scene<T2>> for Scene<T1> {
    type Output = Scene<Union<T1, T2>>;

    fn add(self, rhs: Scene<T2>) -> Scene<Union<T1, T2>> {
        Scene {
            field: Union {
                field1: self.field,
                field2: rhs.field
            }
        }
    }
}
