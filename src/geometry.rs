use vector::*;

pub fn sphere_intersection(center: Vector, radius: f64, position: Vector, direction: Vector) -> Option<Vector> {
    let position = position - center;
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

    return Some(position + center + t * direction);
}

pub fn plane_intersection(normal: Vector, point: Vector, position: Vector, direction: Vector) -> Option<Vector> {
    let denom = normal.dot(direction);
    if denom.abs() > 1.0e-6 {
        let difference = point - position;
        let t = difference.dot(normal) / denom;
        if t > 0.0001 {
            return Some(position + direction * t);
        }
    }
    return None;
}
