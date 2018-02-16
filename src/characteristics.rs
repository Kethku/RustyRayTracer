use vector::*;

#[derive(Copy, Clone)]
pub struct Characteristics {
    pub color: Vector,
    pub roughness: f64,
    pub reflectance: f64,
    pub absorbance: f64
}

impl Characteristics {
    pub fn default() -> Characteristics {
        Characteristics {
            color: Vector::zero(),
            roughness: 0.0,
            reflectance: 0.0,
            absorbance: 0.0
        }
    }

    pub fn mirror(color: Vector) -> Characteristics {
        Characteristics {
            color: color,
            roughness: 0.0,
            reflectance: 1.0,
            absorbance: 0.1
        }
    }

    pub fn matte(color: Vector) -> Characteristics {
        Characteristics {
            color: color,
            roughness: 1.0,
            reflectance: 0.0,
            absorbance: 0.3
        }
    }
}
