use crate::camera::Camera;
use crate::color::Color;
use crate::disc::Disc;
use crate::hitable::HitableStore;
use crate::material::{Dielectric, Material};
use crate::sdf::TracedSDF;
use crate::setup::BLACK;
use crate::sphere::Sphere;
use crate::rect::Rect;
use crate::texture::Texture;
use crate::utils::random_in_unit_sphere;
use glam::Vec3A;
use rand::Rng;
use sdfu::SDF;

#[derive(PartialEq)]
pub enum Worlds {
    Random,
    RandomGlass,
    CornellBox,
    ThreeSphere,
    VerticalWall,
    SdfSpheres,
    SdfWall,
    SimpleAreaLight,
}

pub fn simple_area_light() -> HitableStore {
    let white = Texture::constant_color(Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    });

    let red = Texture::constant_color(Color {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
    });
    let noise = Material::lambertian(Texture::noise(8.0));

    let area_light = Rect::new_yz((-2.0,2.0), (-2.0,2.0), 4.0, Material::diffuse_light(white));
    let sphere = Sphere {
        position: Vec3A::new(0.0, 0.0, -1.0),
        radius: 2.0,
        mat: Material::lambertian(red),
    };
    let floor = Rect::new_xz((-8.0, 8.0), (-8.0, 8.0), -2.0, noise);
    let mut hitables = HitableStore::new();
    hitables.push(area_light);
    hitables.push(sphere);
    hitables.push(floor);
    hitables 
}

pub fn world_default() -> HitableStore {
    let red = Texture::constant_color(Color {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
    });
    let green = Texture::constant_color(Color {
        red: 0.0,
        green: 1.0,
        blue: 0.0,
    });
    let ocra = Texture::constant_color(Color {
        red: 0.8,
        green: 0.6,
        blue: 0.2,
    });

    let l = Texture::constant_color(Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    });

    let sphere = Sphere {
        position: Vec3A::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat: Material::lambertian(red),
    };

    let sphere_a = Sphere {
        position: Vec3A::new(-1.0, 0.0, -1.0),
        // An interesting and easy trick with dielectric spheres is to note that if you use a negative radius,
        // the geometry is unaffected, but the surface normal points inward.
        // This can be used as a bubble to make a hollow glass sphere. Source, RTIAW
        radius: -0.4,
        // radius: 0.5,
        mat: Material::dielectric(1.5),
    };
    let sphere_b = Sphere {
        position: Vec3A::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat: Material::dielectric(1.5),
    };

    let sphere_c = Sphere {
        position: Vec3A::new(1.0, 0.0, -1.0),
        radius: 0.5,
        mat: Material::metal(ocra, 0.2),
    };

    let light = Sphere {
        position: Vec3A::new(0.0, 0.8, -1.0),
        radius: 0.3,
        mat: Material::diffuse_light(l),
    };

    let earth = Sphere {
        position: Vec3A::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat: Material::lambertian(green),
    };
    let mut hitables = HitableStore::new();
    hitables.push(sphere);
    hitables.push(sphere_a);
    hitables.push(sphere_b);
    hitables.push(sphere_c);
    hitables.push(light);
    hitables.push(earth);

    hitables
}

pub fn world_random_vertical_wall() -> HitableStore {
    let blu = Texture::constant_color(Color {
        red: 0.1,
        green: 0.0,
        blue: 0.9,
    });
    let mut spheres: Vec<Sphere> = vec![
        // Sphere {
        //     position: Vec3A::new(-17.0, 12.0, -1.0),
        //     radius: 2.0,
        //     mat: Material::lambertian(panna),
        // },
        // Sphere {
        //     position: Vec3A::new(13.0, 6.0, -2.1),
        //     radius: 1.1,
        //     mat: Material::lambertian(azul),
        // },
        // Sphere {
        //     position: Vec3A::new(-4.5, 7.8, -14.1),
        //     radius: 3.8,
        //     //mat: Material::Dielectric(Dielectric::new(1.5)),
        //     //mat: Material::lambertian(bl),
        //     mat: Material::metal(bl, 0.1),
        // },
        // Sphere {
        //     position: Vec3A::new(6.0, 4.5, 7.0),
        //     radius: 1.5,
        //     mat: Material::metal(ora, 0.2),
        // },
        // Sphere {
        //     position: Vec3A::new(14.0, 4.0, 4.1),
        //     radius: 0.7,
        //     mat: Material::lambertian(peach),
        // },
        // Sphere {
        //     position: Vec3A::new(0.0, 7.0, 2.8),
        //     radius: 3.0,
        //     mat: Material::Dielectric(Dielectric::new(1.5)),
        // },
        // sfere di vetro piccole e concentriche
        Sphere {
            position: Vec3A::new(-0.2, 1.0, -3.0),
            radius: 0.7,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(-0.2, 1.0, -3.0),
            radius: -0.6,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        //sphere2
        Sphere {
            position: Vec3A::new(-1.2, 0.5, -2.0),
            radius: 0.5,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(-1.2, 0.5, -2.0),
            radius: -0.4,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        // sphere3
        Sphere {
            position: Vec3A::new(-1.0, -1.4, -2.3),
            radius: 1.0,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(-1.0, -1.4, -2.3),
            radius: -0.9,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(100.6, -1.0, -2.4),
            radius: 0.9,
            mat: Material::metal(blu, 0.1),
        },
    ];

    fn random_material() -> Box<Material> {
        let mut rng = rand::thread_rng();
        let rand_col_a = Texture::constant_color(Color::random());
        let rand_col_b = Texture::constant_color(Color::random());

        let coin = rng.gen::<f32>();
        if coin >= 0.0 && coin < 0.5 {
            Box::new(Material::dielectric(1.5))
        } else if coin >= 0.5 && coin < 0.8 {
            if coin < 0.7 {
                Box::new(Material::metal(rand_col_b, 0.2))
            } else {
                Box::new(Material::metal(rand_col_b, 0.5 * rng.gen::<f32>()))
            }
        } else {
            Box::new(Material::lambertian(rand_col_a))
        }
    }

    let mut rng = rand::thread_rng();

    for _ in 0..500 {
        let r = rng.gen_range(0.2..0.35);
        let random_x = rng.gen_range(-7.0..7.0);
        let random_y = rng.gen_range(-7.0..7.0);
        let random_z = rng.gen_range(1.0..3.0) * -1.0;
        //let pos = 20.0 * Vec3A::new(rv.x, rv.y, random_z);
        let pos = Vec3A::new(random_x, random_y, random_z);
        if spheres
            .iter()
            .all(|s| (s.position - pos).length() >= s.radius + r)
        //.all(|s| (s.position - pos).length() >= 0.0)
        {
            spheres.push(Sphere {
                position: pos,
                radius: r,
                mat: *random_material(),
            });
        }
    }
    println!("{:?}", spheres.len());
    let mut hitables = HitableStore::new();
    for s in spheres {
        hitables.push(s);
    }

    hitables
}

pub fn world_random() -> HitableStore {
    let orange = Texture::constant_color(Color {
        red: 1.0,
        green: 0.6,
        blue: 0.5,
    });
    let panna = Texture::constant_color(Color {
        red: 0.6,
        green: 0.2,
        blue: 0.2,
    });
    let brown = Texture::constant_color(Color {
        red: 0.85,
        green: 0.9,
        blue: 0.7,
    });
    let azul = Texture::constant_color(Color {
        red: 0.12,
        green: 0.9,
        blue: 0.7,
    });
    let ora = Texture::constant_color(Color {
        red: 0.9,
        green: 0.3,
        blue: 0.15,
    });
    let bl = Texture::constant_color(Color {
        red: 1.0,
        green: 0.4,
        blue: 0.9,
    });
    let peach = Texture::constant_color(Color {
        red: 1.0,
        green: 0.75,
        blue: 0.17,
    });
    let mut spheres: Vec<Sphere> = vec![
        Sphere {
            position: Vec3A::new(0.0, -1000.0, -1.0),
            radius: 1000.0,
            mat: Material::lambertian(orange),
        },
        Sphere {
            position: Vec3A::new(-17.0, 2.0, -1.0),
            radius: 2.0,
            mat: Material::lambertian(panna),
        },
        Sphere {
            position: Vec3A::new(13.0, 1.1, -2.1),
            radius: 1.1,
            mat: Material::lambertian(azul),
        },
        Sphere {
            position: Vec3A::new(-4.5, 3.8, -14.1),
            radius: 3.8,
            //mat: Material::Dielectric(Dielectric::new(1.5)),
            //mat: Material::lambertian(bl),
            mat: Material::metal(bl, 0.1),
        },
        Sphere {
            position: Vec3A::new(6.0, 1.5, 7.0),
            radius: 1.5,
            mat: Material::metal(ora, 0.2),
        },
        Sphere {
            position: Vec3A::new(14.0, 0.7, 4.1),
            radius: 0.7,
            mat: Material::lambertian(peach),
        },
        Sphere {
            position: Vec3A::new(0.0, 3.0, 2.8),
            radius: 3.0,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        // sfere di vetro piccole e concentriche
        Sphere {
            position: Vec3A::new(13.5, 1.3, 2.3),
            radius: 1.1,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(13.5, 1.3, 2.3),
            radius: -1.0,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(8.0, 1.6, -1.3),
            radius: 1.6,
            mat: Material::metal(brown, 0.1),
        },
    ];

    fn random_material() -> Box<Material> {
        let mut rng = rand::thread_rng();
        let rand_col_a = Texture::constant_color(Color::random());
        let rand_col_b = Texture::constant_color(Color::random());

        let coin = rng.gen::<f32>();
        if coin >= 0.0 && coin < 0.5 {
            Box::new(Material::dielectric(1.5))
        } else if coin >= 0.5 && coin < 0.8 {
            if coin < 0.7 {
                Box::new(Material::metal(rand_col_b, 0.2))
            } else {
                Box::new(Material::metal(rand_col_b, 0.5 * rng.gen::<f32>()))
            }
        } else {
            Box::new(Material::lambertian(rand_col_a))
        }
    }

    for _ in 0..500 {
        let r = 0.4;
        let rv = random_in_unit_sphere();
        let pos = 20.0 * Vec3A::new(rv.x, 0.0, rv.y) + Vec3A::new(0.0, r, 0.0);
        if spheres
            .iter()
            .all(|s| (s.position - pos).length() >= s.radius + r)
        {
            spheres.push(Sphere {
                position: pos,
                radius: r,
                mat: *random_material(),
            });
        }
    }
    let mut hitables = HitableStore::new();
    for s in spheres {
        hitables.push(s);
    }

    hitables
}

pub fn world_random_glass() -> HitableStore {
    let mut spheres: Vec<Sphere> = vec![
        // sfere di vetro concentriche
        Sphere {
            position: Vec3A::new(0.0, -1000.0, -1.0),
            radius: 1000.0,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(0.0, -1000.0, -1.0),
            radius: -999.0,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(-17.0, 2.0, -1.0),
            radius: 2.0,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(-17.0, 2.0, -1.0),
            radius: -1.9,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(13.0, 1.1, -2.1),
            radius: 1.1,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(13.0, 1.1, -2.1),
            radius: -1.0,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(-4.5, 3.8, -14.1),
            radius: 3.8,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(-4.5, 3.8, -14.1),
            radius: -3.7,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(6.0, 1.5, 7.0),
            radius: 1.5,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(6.0, 1.5, 7.0),
            radius: -1.4,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(14.0, 0.7, 4.1),
            radius: 0.7,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(14.0, 0.7, 4.1),
            radius: -0.6,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(0.0, 3.0, 2.8),
            radius: 3.0,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(0.0, 3.0, 2.8),
            radius: -2.9,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(13.5, 1.3, 2.3),
            radius: 1.1,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(13.5, 1.3, 2.3),
            radius: -1.0,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            position: Vec3A::new(8.0, 1.6, -1.3),
            radius: 1.6,
            mat: Material::Dielectric(Dielectric::new(1.5)),
        },
    ];

    for _ in 0..20 {
        let r = 0.4;
        let rv = random_in_unit_sphere();
        let pos = Vec3A::new(rv.x * -5.0, 0.0, rv.y * 20.0) + Vec3A::new(0.0, r, 0.0);
        if spheres
            .iter()
            .all(|s| (s.position - pos).length() >= s.radius + r)
        {
            spheres.push(Sphere {
                position: pos,
                radius: r,
                mat: Material::Dielectric(Dielectric::new(1.5)),
            });
        }
    }
    let mut hitables = HitableStore::new();
    for s in spheres {
        hitables.push(s);
    }

    hitables
}

pub fn world_cornell_box() -> HitableStore {
    let red = Color::new(0.65, 0.05, 0.05);
    let white = Color::new(0.73, 0.73, 0.73);
    let green = Color::new(0.12, 0.45, 0.15);
    let light = Color::new(1.0, 1.0, 1.0) * 15.0;

    let area_light = Rect::new_xz(
        (-2.0,2.0), (-2.0,2.0),
        9.99,
        Material::diffuse_light(Texture::constant_color(light)));
    let mut floor = Rect::new_xz((-8.0, 8.0), (-8.0, 8.0), 0.0, Material::diffuse_light(Texture::constant_color(white)));
    floor.flip_normals();
    //let ceil = Rect::new_xz((-8.0, 8.0), (-8.0, 8.0), 0.0, Material::diffuse_light(Texture::constant_color(green)));

    let spheres: Vec<Sphere> = vec![
        // right wall
        Sphere {
            position: Vec3A::new(5006.0, 0.0, 0.0),
            radius: 5000.0,
            mat: Material::lambertian(Texture::constant_color(green)),
        },
        // left wall
        Sphere {
            position: Vec3A::new(-5006.0, 0.0, 0.0),
            radius: 5000.0,
            mat: Material::lambertian(Texture::constant_color(red)),
        },
        // ceiling
        // Sphere {
        //     position: Vec3A::new(0.0, 5010.0, 0.0),
        //     radius: 5000.0,
        //     mat: Material::lambertian(Texture::constant_color(white)),
        // },
        // floor
        // Sphere {
        //     position: Vec3A::new(0.0, -5000.0, 0.0),
        //     radius: 5000.0,
        //     mat: Material::lambertian(Texture::constant_color(white)),
        // },
        // back wall
        Sphere {
            position: Vec3A::new(0.0, 0.0, -5010.0),
            radius: 5000.0,
            mat: Material::lambertian(Texture::constant_color(white)),
        },
        Sphere {
            position: Vec3A::new(-3.5, 2.0, -3.0),
            radius: 2.0,
            mat: Material::dielectric(1.52),
        },
        Sphere {
            position: Vec3A::new(3.5, 2.0, -7.0),
            radius: 2.0,
            mat: Material::metal(Texture::constant_color(Color::new(0.05, 1.0, 0.05)), 0.25),
        },
        Sphere {
            position: Vec3A::new(5.0, 1.0, 0.0),
            radius: 1.0,
            mat: Material::metal(Texture::constant_color(Color::new(1.0, 0.05, 0.05)), 0.0),
        },
    ];

    let mut hitables = HitableStore::new();
    for s in spheres {
        hitables.push(s);
    }
    hitables.push(floor);
    //hitables.push(ceil);
    hitables.push(area_light);
    hitables

}

pub fn world_wall_sdf() -> HitableStore {
    let w = 4;
    let h = 4;
    let step = 6.2;
    let sphere_radius = 0.3;
    let min_z = 0.5;
    let max_z = 2.4;
    let mut hitables = HitableStore::new();
    let mut rng = rand::thread_rng();
    for x in 0..w {
        let mut xf = x as f32 / w as f32;
        xf -= 0.5;
        xf *= step;
        for y in 0..h {
            let mut yf = y as f32 / h as f32;
            yf -= 0.5;
            yf *= step;
            let sdf_aabb_radius: f32 = 1.0;
            let sdf_aabb_position = Vec3A::new(xf, yf, rng.gen_range(min_z..max_z) * -1.0);
            //let sdf_aabb_position = Vec3A::new(xf, yf, -1.0);
            let red = Texture::constant_color(Color {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
            });

            let sdf_sphere = TracedSDF::new(
                sdfu::Sphere::new(sphere_radius)
                    //.subtract(sdfu::Torus::new(0.4f32, 0.1f32))
                    .subtract(sdfu::Box::new(Vec3A::new(
                        sphere_radius / 2.0,
                        sphere_radius / 2.0,
                        sphere_radius * 3.0,
                    )))
                    .union(
                        sdfu::Box::new(Vec3A::new(
                            sphere_radius / 2.8,
                            sphere_radius / 2.8,
                            sphere_radius,
                        ))
                        .intersection(sdfu::Sphere::new(sphere_radius))
                        .translate(Vec3A::new(
                            0.0,
                            0.0,
                            rng.gen_range(0.1..1.0) * -0.5 + 0.7,
                        )),
                    )
                    // .union_smooth(
                    //     sdfu::Sphere::new(0.3).translate(Vec3A::new(0.3, 0.3, 0.0)),
                    //     0.1,
                    // )
                    // .union_smooth(
                    //     sdfu::Sphere::new(0.3).translate(Vec3A::new(-0.3, 0.3, 0.0)),
                    //     0.1,
                    // )
                    .translate(sdf_aabb_position),
                Material::lambertian(red),
                sdf_aabb_radius,
                sdf_aabb_position,
            );

            hitables.push(sdf_sphere);
        }
    }

    hitables
}

pub fn world_sdf() -> HitableStore {
    let red = Texture::constant_color(Color {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
    });
    let green = Texture::constant_color(Color {
        red: 0.0,
        green: 1.0,
        blue: 0.0,
    });

    let l = Texture::constant_color(Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    });

    let light = Sphere {
        position: Vec3A::new(0.0, 1.0, -1.0),
        radius: 0.3,
        mat: Material::diffuse_light(l),
    };

    let sdf_aabb_radius = 0.7;
    let sdf_aabb_position = Vec3A::new(0.0, 0.0, -1.0);

    let sdf_sphere = TracedSDF::new(
        sdfu::Sphere::new(0.45)
            .subtract(sdfu::Box::new(Vec3A::new(0.25, 0.25, 1.5)))
            .union_smooth(
                sdfu::Sphere::new(0.3).translate(Vec3A::new(0.3, 0.3, 0.0)),
                0.1,
            )
            .union_smooth(
                sdfu::Sphere::new(0.3).translate(Vec3A::new(-0.3, 0.3, 0.0)),
                0.1,
            )
            .translate(sdf_aabb_position),
        Material::lambertian(red),
        sdf_aabb_radius,
        sdf_aabb_position,
    );

    // Vec3::new(1.2, 0.0, 0.0);
    // let sdf_torus = TracedSDF::new(
    //     sdfu::Torus::new(0.45, 0.1).rotate(rot),
    //     Material::lambertian(ocra),
    //     torus_radius,
    //     torus_pos,
    // );

    // let sphere_b = Sphere {
    //     position: Vec3A::new(-0.5, 0.0, -1.0),
    //     radius: 0.45,
    //     mat: Material::lambertian(ocra),
    //     // mat: Material::metal(red, 0.2),
    // };

    let earth = Sphere {
        position: Vec3A::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat: Material::lambertian(green),
    };
    let mut hitables = HitableStore::new();
    hitables.push(earth);
    hitables.push(light);
    //hitables.push(sdf_torus);
    hitables.push(sdf_sphere);
    hitables
}

pub fn get_world_and_camera(
    w: &Worlds,
    camera_fov: f32,
    frame_width: u32,
    frame_height: u32,
    camera_aperture: f32,
) -> (HitableStore, Camera, Color) {
    match w {
        Worlds::CornellBox => {
            let look_from = Vec3A::new(0.0, 5.0, 15.0);
            let look_at = Vec3A::new(0.0, 5.0, 0.0);

            let camera = Camera::new(
                look_from,
                look_at,
                camera_fov,
                (frame_width as f32) / (frame_height as f32),
                camera_aperture,
            );
            return (world_cornell_box(), camera, Color::new(0.70, 0.80, 1.00));
        }
        Worlds::SimpleAreaLight => {
            let look_from = Vec3A::new(0.0, 0.0, 9.0);
            let look_at = Vec3A::new(0.0, 0.0, 0.0);

            let camera = Camera::new(
                look_from,
                look_at,
                camera_fov,
                (frame_width as f32) / (frame_height as f32),
                camera_aperture,
            );
            //return (simple_area_light(), camera, Color::new(0.0, 0.0, 0.0));
            return (simple_area_light(), camera, BLACK);
        }
        Worlds::Random => {
            let look_at = Vec3A::new(0.0, 0.0, -1.0);
            let look_from = Vec3A::new(20.0 * 0.47f32.cos(), 4.0 * 0.47f32.sin(), 3.0);

            let camera = Camera::new(
                look_from,
                look_at,
                camera_fov,
                (frame_width as f32) / (frame_height as f32),
                camera_aperture,
            );
            return (world_random(), camera, Color::new(0.70, 0.80, 1.00));
        }
        Worlds::RandomGlass => {
            let look_at = Vec3A::new(0.0, 0.0, -1.0);
            let look_from = Vec3A::new(20.0 * 0.47f32.cos(), 4.0 * 0.47f32.sin(), 3.0);

            let camera = Camera::new(
                look_from,
                look_at,
                camera_fov,
                (frame_width as f32) / (frame_height as f32),
                camera_aperture,
            );
            return (world_random_glass(), camera, Color::new(0.70, 0.80, 1.00));
        }
        Worlds::VerticalWall => {
            let look_at = Vec3A::new(0.0, 0.0, -1.0);
            let look_from = Vec3A::new(2.0 * 0.47f32.cos(), 2.0 * 0.47f32.sin(), 3.0);

            let camera = Camera::new(
                look_from,
                look_at,
                camera_fov,
                (frame_width as f32) / (frame_height as f32),
                camera_aperture,
            );
            return (world_random_vertical_wall(), camera, Color::new(0.70, 0.80, 1.00));
        }
        Worlds::ThreeSphere => {
            let look_at = Vec3A::new(0.0, 0.0, -1.0);
            let look_from = Vec3A::new(0.0, 0.0, 1.0);

            let camera = Camera::new(
                look_from,
                look_at,
                camera_fov,
                (frame_width as f32) / (frame_height as f32),
                camera_aperture,
            );
            return (world_default(), camera, Color::new(0.70, 0.80, 1.00));
        }

        Worlds::SdfSpheres => {
            let look_at = Vec3A::new(0.0, 0.0, -1.0);
            let look_from = Vec3A::new(0.0, 0.0, 1.0);

            let camera = Camera::new(
                look_from,
                look_at,
                camera_fov,
                (frame_width as f32) / (frame_height as f32),
                camera_aperture,
            );
            return (world_sdf(), camera, Color::new(0.70, 0.80, 1.00));
        }
        Worlds::SdfWall => {
            let look_at = Vec3A::new(0.0, 0.0, -1.0);
            let look_from = Vec3A::new(1.1, 0.9, 1.0);
            //let look_from = Vec3A::new(0.0, 0.0, 1.0);

            let camera = Camera::new(
                look_from,
                look_at,
                camera_fov,
                (frame_width as f32) / (frame_height as f32),
                camera_aperture,
            );
            return (world_wall_sdf(), camera, Color::new(0.70, 0.80, 1.00));
        }
    }
}
