use vector::*;
use distance_field::*;
use scene::*;
use std::f64::*;

const RAYLEIGH_SCALE_HEIGHT: f64 = 7994.0;
const MEI_SCALE_HEIGHT: f64 = 1200.0;
const PLANET_RADIUS: f64 = 6360e3;
const ATMOSPHERE_RADIUS: f64 = 6420e3;
const SUN_INTENSITY: f64 = 50.0;
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

pub fn calculate_sky_color(direction: Vector, sun_direction: Vector) -> Vector {
    let position = Vector {
        y: PLANET_RADIUS + 100.0,
        x: 0.0,
        z: 0.0
    };

    let view_interesect = atmosphere_intersection(position, direction);
    let mu = direction.dot(sun_direction);
    let rayleigh_phase = rayleigh_phase_function(mu);
    let mei_phase = mei_phase_function(mu);

    let color = numerical_integration(position, view_interesect, |pos| {
        let sun_atmosphere_intersect = sphere_intersection(ATMOSPHERE_RADIUS, pos, sun_direction);
        if sun_atmosphere_intersect == None {
            return Vector::zero();
        }
        let sun_atmosphere_intersect = sun_atmosphere_intersect.unwrap();
        let atmosphere_dist = (sun_atmosphere_intersect - pos).length_squared();
        let sun_planet_intersect = sphere_intersection(PLANET_RADIUS, pos, sun_direction);
        if sun_planet_intersect != None {
            let sun_planet_intersect = sun_planet_intersect.unwrap();
            let planet_dist = (sun_planet_intersect - pos).length_squared();
            if planet_dist < atmosphere_dist {
                return Vector::zero();
            }
        }

        let atmosphere_height = pos.length() - PLANET_RADIUS;

        let trans_camera_to_pos = transmittance(position, pos);
        let trans_pos_to_sky = transmittance(pos, sun_atmosphere_intersect);
        let ray_extinction = rayleigh_phase * rayleigh_extinction_coefficients(atmosphere_height);
        let mei_extinction = mei_phase * mei_extinction_coefficients(atmosphere_height);
        SUN_INTENSITY * trans_camera_to_pos * trans_pos_to_sky * (ray_extinction + mei_extinction)
    });
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
    if a == b {
        return Vector::zero();
    }
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
    coefficient * numerator / denominator
}

fn atmosphere_intersection(position: Vector, direction: Vector) -> Vector {
    let planet_intersect = sphere_intersection(PLANET_RADIUS, position, direction);
    let atmosphere_intersect = sphere_intersection(ATMOSPHERE_RADIUS, position, direction);

    match (planet_intersect, atmosphere_intersect) {
        (None, None) => position,
        (Some(p), None) => p,
        (None, Some(p)) => p,
        (Some(p1), Some(p2)) => {
            let p1_dist = (p1 - position).length_squared();
            let p2_dist = (p2 - position).length_squared();
            if p1_dist < p2_dist {
                p2
            } else {
                p1
            }
        }
    }
}

fn sphere_intersection(radius: f64, position: Vector, direction: Vector) -> Option<Vector> {
    let l = position * -1.0;
    let tca = l.dot(direction);
    let d2 = l.dot(l) - tca * tca;
    let radius2 = radius * radius;
    if d2 > radius2 {
        return None;
    }
    let thc = (radius2 - d2).sqrt();

    let t0 = tca - thc;
    let t1 = tca + thc;
    if t0 < 0.0 {
        if t1 < 0.0 {
            return None;
        } else {
            return Some(position + t1 * direction);
        }
    } else if t1 < 0.0 {
        return Some(position + t0 * direction);
    }

    let t = t0.min(t1);

    return Some(position + t * direction);
}
