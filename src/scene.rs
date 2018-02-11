use std::ops::*;
use vector::*;

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

    pub fn trace(&self, position: Vector, direction: Vector, max_distance: f64) -> Vector {
        let (pos, characteristics) = self.march(position, direction, max_distance);
        let march_distance = (pos - position).length();
        let remaining_distance = max_distance - march_distance;
        if remaining_distance < 0.0 {
            Vector::one()
        } else {
            let new_pos = pos + characteristics.normal * MINIMUM_THRESHOLD;
            let mut new_dir = characteristics.normal + Vector::random();

            if thread_rng().gen_range(0.0, 1.0) < characteristics.reflectance {
                let reflection_target = direction - 2.0 * characteristics.normal * characteristics.normal.dot(direction);
                new_dir = Vector::interpolate(reflection_target, new_dir, characteristics.roughness);
            }

            new_dir = new_dir.normalize();

            let material_color = Vector::interpolate(characteristics.color, Vector::one(), characteristics.reflectance) * (1.0 - characteristics.absorbance);

            material_color * self.trace(new_pos, new_dir, remaining_distance)
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
