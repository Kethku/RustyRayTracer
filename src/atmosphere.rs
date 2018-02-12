use vector::*;
use distance_field::*;
use scene::*;
use std::f64::*;

const RAYLEIGH_SCALE_HEIGHT: f64 = 7994.0;
const MEI_SCALE_HEIGHT: f64 = 1200.0;
const PLANET_RADIUS: f64 = 6360e3;
const ATMOSPHERE_RADIUS: f64 = 6420e3;
const SUN_INTENSITY: f64 = 20.0;
const SAMPLE_COUNT: i32 = 8;
const MEI_SCATTERING_COEFFICIENTS_AT_SEA_LEVEL: Vector = Vector {
    x: 21.0e-6,
    y: 21.0e-6,
    z: 21.0e-6
};
const RAYLEIGH_EXTINCTION_COEFFICIENTS_AT_SEA_LEVEL: Vector = Vector {
    x: 3.8e-6,
    y: 13.5e-6,
    z: 33.1e-6
};

pub fn calculate_sky_color(position: Vector, direction: Vector, sun_direction: Vector) -> Vector {
    let atmosphere_geometry =
        Sphere::new(Vector::zero(), PLANET_RADIUS, Characteristics::default()) +
        !Sphere::new(Vector::zero(), ATMOSPHERE_RADIUS, Characteristics::default());
    let position = Vector {
        y: position.y + PLANET_RADIUS + 100.0,
        .. position
    };

    let view_interesect = atmosphere_intersection(&atmosphere_geometry, position, direction);
    let mu = direction.dot(sun_direction);
    let rayleigh_phase = rayleigh_phase_function(mu);
    let mei_phase = mei_phase_function(mu);

    println!("view: {:?}", view_interesect);
    println!("mu: {:?}", mu);
    println!("rayleigh_phase: {:?}", rayleigh_phase);
    println!("mei_phase: {:?}", mei_phase);

    let color = numerical_integration(position, view_interesect, |pos| {
        let sun_intersect = atmosphere_intersection(&atmosphere_geometry, pos, sun_direction);
        let atmosphere_height = pos.length() - PLANET_RADIUS;

        let trans_camera_to_pos = transmittance(position, pos);
        let trans_pos_to_sky = transmittance(pos, sun_intersect);
        let ray_extinction = rayleigh_phase * rayleigh_extinction_coefficients(atmosphere_height);
        let mei_extinction = mei_phase * mei_extinction_coefficients(atmosphere_height);
        SUN_INTENSITY * trans_camera_to_pos * trans_pos_to_sky * (ray_extinction + mei_extinction)
    });

    println!("{:?}", color);

    color
}

fn transmittance(a: Vector, b: Vector) -> Vector {
    let result = numerical_integration(a, b, |pos| {
        let atmosphere_distance = pos.length() - PLANET_RADIUS;
        rayleigh_extinction_coefficients(atmosphere_distance) +
            mei_extinction_coefficients(atmosphere_distance)
    });
    Vector {
        x: (-result.x).exp(),
        y: (-result.y).exp(),
        z: (-result.z).exp()
    }
}

fn numerical_integration<F>(a: Vector, b: Vector, body: F) -> Vector
    where F: Fn(Vector) -> Vector {
    let mut current_pos = a;
    let diff = b - a;
    let dir = diff.normalize();
    let total_distance = diff.length();
    let sample_distance = total_distance / SAMPLE_COUNT as f64;
    let sample_delta = dir * sample_distance;
    current_pos = current_pos + sample_delta / 2.0;
    let mut sum = Vector::zero();

    for _ in 0..SAMPLE_COUNT {
        sum = sum + body(current_pos) * sample_distance;
        current_pos = current_pos + sample_delta;
    }
    sum
}

fn rayleigh_extinction_coefficients(height: f64) -> Vector {
    RAYLEIGH_EXTINCTION_COEFFICIENTS_AT_SEA_LEVEL * (-height / RAYLEIGH_SCALE_HEIGHT).exp()
}

fn rayleigh_phase_function(mu: f64) -> f64 {
    3.0 / (16.0 * consts::PI) * (1.0 + mu * mu)
}

fn mei_extinction_coefficients(height: f64) -> Vector {
    mei_scattering_coefficients(height) * 1.1
}

fn mei_scattering_coefficients(height: f64) -> Vector {
    MEI_SCATTERING_COEFFICIENTS_AT_SEA_LEVEL * (-height / MEI_SCALE_HEIGHT).exp()
}

fn mei_phase_function(mu: f64) -> f64 {
    let g = 0.76;
    let coefficient = 3.0 / (8.0 * consts::PI);
    let numerator = (1.0 - g * g) * (1.0 + mu * mu);
    let denominator = (2.0 + g * g) * (1.0 + g * g - 2.0 * g * mu).powf(3.0 / 2.0);
    println!("denominator: {:?}", denominator);
    coefficient * numerator / denominator
}

fn atmosphere_intersection<T: Field>(atmosphere_scene: &Scene<T>, position: Vector, direction: Vector) -> Vector {
    let (new_position, _) = atmosphere_scene.march(position, direction, ATMOSPHERE_RADIUS * 2.0, ATMOSPHERE_RADIUS / 100000.0);
    new_position
}
