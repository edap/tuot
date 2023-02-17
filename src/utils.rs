use glam::Vec3A;
use rand::Rng;

pub fn random_vec3(min: f32, max: f32) -> Vec3A {
    let mut rng = rand::thread_rng();
    Vec3A::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_in_unit_sphere() -> Vec3A {
    loop {
        let p = random_vec3(-1.0, 1.0);
        // maybe the rng var can be passed as &rng? how much does it cost to create it new every time?
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn near_zero(vec: &Vec3A) -> bool {
    vec.x.abs() < f32::EPSILON && vec.y.abs() < f32::EPSILON && vec.z.abs() < f32::EPSILON
}

pub fn u_v_from_sphere_hit_point(hit_point_on_sphere: Vec3A) -> (f32, f32) {
    let n = hit_point_on_sphere.normalize();
    let x = n.x;
    let y = n.y;
    let z = n.z;
    let u = (x.atan2(z) / (2.0 * std::f32::consts::PI)) + 0.5;
    let v = y * 0.5 + 0.5;
    (u, v)
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    // schlick
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub fn refract(v: Vec3A, n: Vec3A, ni_over_nt: f32) -> Option<Vec3A> {
    let uv = v.normalize();

    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n)
    } else {
        None
    }
}

pub fn reflect(v: &Vec3A, n: &Vec3A) -> Vec3A {
    *v - *n * (2.0 * v.dot(*n))
}
