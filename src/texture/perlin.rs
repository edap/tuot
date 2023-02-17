use glam::Vec3A;
use rand::prelude::*;

const SIZE: usize = 256;

#[derive(Clone, Debug)]
pub struct Perlin {
    pub ranvec: Vec<Vec3A>,
    pub perm_x: [usize; SIZE],
    pub perm_y: [usize; SIZE],
    pub perm_z: [usize; SIZE],
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ranvec: Perlin::generate(),
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, point: &Vec3A) -> f32 {
        let u = point[0] - point[0].floor();
        let v = point[1] - point[1].floor();
        let w = point[2] - point[2].floor();

        let i = point[0].floor() as usize;
        let j = point[1].floor() as usize;
        let k = point[2].floor() as usize;

        let mut c = Vec::with_capacity(2 * 2 * 2);
        for _ in 0..c.capacity() {
            c.push(Vec3A::new(0.0, 0.0, 0.0));
        }
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let v = self.ranvec[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]]
                        .clone();
                    c[index(di, dj, dk)] = v;
                }
            }
        }
        Perlin::trilinear_interpolation(&c, u, v, w)
    }

    fn generate() -> Vec<Vec3A> {
        let mut rng = rand::thread_rng();
        let mut perlin = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            // use random unit vectors (instead of just floats) on lattice points, use dot product
            // to move min and max of the lattice
            let v = Vec3A::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )
            .normalize();
            perlin.push(v);
        }
        perlin
    }

    fn generate_perm() -> [usize; SIZE] {
        let mut perm = [0; SIZE];
        for i in 0..SIZE {
            perm[i] = i as usize;
        }
        Perlin::permute(&mut perm);
        perm
    }

    fn permute(perm: &mut [usize; SIZE]) {
        let mut rng = rand::thread_rng();
        for i in (0..perm.len()).rev() {
            let target = (rng.gen::<f32>() * (i as f32 + 1.0)) as usize;
            perm.swap(i, target);
        }
    }

    fn trilinear_interpolation(c: &[Vec3A], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut acc = 0.0;
        for i in 0..2 {
            let iuu = i as f32 * uu;
            for j in 0..2 {
                let jvv = j as f32 * vv;
                for k in 0..2 {
                    let kww = k as f32 * ww;
                    let weight_v = Vec3A::new(u - i as f32, v - j as f32, w - k as f32);

                    acc += (iuu + ((1 - i) as f32 * (1.0 - uu)))
                        * (jvv + ((1 - j) as f32 * (1.0 - vv)))
                        * (kww + ((1 - k) as f32 * (1.0 - ww)))
                        * c[index(i, j, k)].dot(weight_v);
                }
            }
        }
        acc
    }
}

fn index(i: usize, j: usize, k: usize) -> usize {
    i * 2 * 2 + j * 2 + k
    //(k * 2 * 2) + (j * 2) + i
}
