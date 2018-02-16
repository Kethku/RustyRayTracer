use std::ops::*;
use vector::*;
use distance_field::*;
use atmosphere::*;
use rand::*;
use characteristics::*;

pub struct Scene<T: Field> {
    pub field: T
}

const MINIMUM_THRESHOLD: f64 = 0.001;
impl<T: Field> Scene<T> {
    pub fn distance_sampler(&self, pos: Vector, dir: Vector) -> f64 {
        self.field.ray_distance_sampler(pos, dir)
    }

    pub fn characteristic_sampler(&self, pos: Vector) -> (Vector, Characteristics) {
        self.field.characteristic_sampler(pos)
    }

    pub fn trace(&self, position: Vector, direction: Vector, sun_dir: Vector, max_distance: f64) -> Vector {
        let mut accumulated_color = Vector::one();
        let mut remaining_distance = max_distance;
        let mut current_pos = position;
        let mut current_direction = direction;
        loop {
            let distance = self.distance_sampler(current_pos, current_direction);
            let pos = position + direction * distance;
            let (pos, characteristics) = self.characteristic_sampler(pos);
            remaining_distance = remaining_distance - distance;
            if remaining_distance < 0.0 {
                return accumulated_color * calculate_sky_color(current_direction, sun_dir);
            } else {
                let material_color = Vector::interpolate(characteristics.color, Vector::one(), characteristics.reflectance) * (1.0 - characteristics.absorbance);
                let new_pos = pos + characteristics.normal * MINIMUM_THRESHOLD;
                let mut new_dir = characteristics.normal + Vector::random();

                if thread_rng().gen_range(0.0, 1.0) < characteristics.reflectance {
                    let reflection_target = current_direction - 2.0 * characteristics.normal * characteristics.normal.dot(current_direction);
                    new_dir = Vector::interpolate(reflection_target, new_dir, characteristics.roughness);
                }

                new_dir = new_dir.normalize();

                accumulated_color = accumulated_color * material_color;
                current_pos = new_pos;
                current_direction = new_dir;
            }
        }
    }
}

impl<T: Field> Not for Scene<T> {
    type Output = Scene<Negate<T>>;

    fn not(self) -> Scene<Negate<T>> {
        Scene {
            field: Negate {
                field: self.field
            }
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

impl<T1: Field, T2: Field> Mul<Scene<T2>> for Scene<T1> {
    type Output = Scene<Intersection<T1, T2>>;

    fn mul(self, rhs: Scene<T2>) -> Scene<Intersection<T1, T2>> {
        Scene {
            field: Intersection {
                field1: self.field,
                field2: rhs.field
            }
        }
    }
}
