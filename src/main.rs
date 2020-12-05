extern crate image;
extern crate rand;

mod math;
mod object;
mod material;
mod physics;
mod scene;
mod film;

use {math::*, rand::prelude::*, object::*, material::*, physics::*, scene::*, film::*};

fn main()
{
	println!("As ray shoots out of camera, photons emit from light sources, they reflects, scatters and decays, and so does the information it carries exchanges all-around.");
	println!("Eventually, geometries, objects and scenes are brought to life in this virtual spacetime, in this machine's memory. ");
	println!(" ");
	println!("PROJECT DECAY, a simple renderer. ");
	println!("WIP \n");

	let nx: u16 = 1920;
	let ny: u16 = 1080;
	let pixel: u64 = (nx as u64)*(ny as u64);
	let sample_ray_step: u8 = 6;
	let sample_per_scatter: u16 = 25; // Only on first scatter!
	let sample_ssaa_level: u8 = 6;

	let mut frame_buf = image::ImageBuffer::new(nx as u32, ny as u32);
	// let cam: Camera = Camera::default();

	// Fill Background color
	for (_x, _y, pixel) in frame_buf.enumerate_pixels_mut()
	{
		*pixel = image::Rgb([0u8, 0u8, 0u8]);
	}

	let mut hitables: Vec<Sphere> = Vec::new();

	// hitables.push(Sphere::new(Pos3::new(0.0, -1.0, 0.0), 0.5, Material::DebugNormalShading { normal: Vec3::default(), mode: 0 })); // Debug Normal
	// hitables.push(Sphere::new(Pos3::new(0.0, -1.0, 0.0), 0.5, Material::Emissive { emit: Color::new( 4.0, 4.0, 4.0 ) })); // Emissive
	// hitables.push(Sphere::new(Pos3::new(0.0, -1.0, 0.0), 0.5, Material::Metal{ albedo: Color::new(0.95, 0.95, 0.95), fuzz: 0.0, emit: Color::rgb_empty() })); // Metal Mirror
	// hitables.push(Sphere::new(Pos3::new(0.0, -1.0, 0.0), 0.5, Material::Dielectric{ albedo: Color::new(0.10, 0.10, 0.10), refraction: Color::new(0.90, 0.90, 0.90), eta: 1.5, fuzz: 0.0, emit: Color::rgb_empty() })); // Metal Mirror

	// hitables.push(Sphere::new(Pos3::new(-0.16, 1.0, -0.175), 0.10, Material::Metal{ albedo: Color::new(0.32, 0.32, 0.96), fuzz: 0.0, emit: Color::rgb_empty() }));
	// hitables.push(Sphere::new(Pos3::new(4.0, -12.0, 3.6), 4.2, Material::Metal{ albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.0025, emit: Color::rgb_empty() }));

	hitables.push(Sphere::new(Pos3::new(0.0, -1.0, -100.5), 100.0, Material::Lambertian{ albedo: Color::new(0.64, 0.64, 0.64), emit: Color::rgb_empty() }));

	//
	hitables.push(Sphere::new(Pos3::new(1.0, -1.2, 0.25), 0.6, Material::Dielectric{ albedo: Color::new(0.10, 0.10, 0.10), refraction: Color::new(0.90, 0.90, 0.90), eta: 1.5, fuzz: 0.0, emit: Color::rgb_empty() })); // Metal Mirror
	// hitables.push(Sphere::new(Pos3::new(1.0, -1.2, 0.25), 0.6, Material::Metal{ albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.01, emit: Color::rgb_empty() }));
	hitables.push(Sphere::new(Pos3::new(0.0, -1.0, 0.0), 0.5, Material::Dielectric{ albedo: Color::new(0.10, 0.10, 0.10), refraction: Color::new(0.90, 0.90, 0.90), eta: 1.5, fuzz: 0.0, emit: Color::rgb_empty() })); // Metal Mirror
	// hitables.push(Sphere::new(Pos3::new(0.0, -1.0, 0.0), -0.45, Material::Dielectric{ albedo: Color::new(0.10, 0.10, 0.10), refraction: Color::new(0.90, 0.90, 0.90), eta: 1.5, fuzz: 0.0, emit: Color::rgb_empty() })); // Metal Mirror
	hitables.push(Sphere::new(Pos3::new(-1.0, -1.2, 0.25), 0.6, Material::Dielectric{ albedo: Color::new(0.10, 0.10, 0.10), refraction: Color::new(0.90, 0.90, 0.90), eta: 1.5, fuzz: 0.0, emit: Color::rgb_empty() })); // Metal Mirror
	// hitables.push(Sphere::new(Pos3::new(-1.0, -1.0, 0.0), 0.5, Material::Metal{ albedo: Color::new(0.95, 0.95, 0.95), fuzz: 0.0, emit: Color::rgb_empty() })); // Metal Mirror
	// hitables.push(Sphere::new(Pos3::new(0.0, -9.0, 5.0), 6.0, Material::Metal{ albedo: Color::new(0.10, 0.20, 0.90), fuzz: 0.0, emit: Color::rgb_empty() })); // Metal Mirror
	// hitables.push(Sphere::new(Pos3::new(0.0, -9.0, 5.0), 6.0, Material::Dielectric{ albedo: Color::new(0.10, 0.20, 0.90), refraction: Color::new(0.000, 0.000, 0.000), eta: 1.5, fuzz: 0.0, emit: Color::rgb_empty() })); // Metal Mirror
	// hitables.push(Sphere::new(Pos3::new(0.0, -9.0, 5.0), 6.0, Material::DebugNormalShading { normal: Vec3::default(), mode: 0 })); // Debug Normal

	hitables.push(Sphere::new(Pos3::new(0.0, -9.0, 5.0), 6.0, Material::DebugNormalShading { normal: Vec3::default(), mode: 0 })); // Debug Normal
	//

	let delta: f64 = 1f64 / ((2* sample_ssaa_level) as f64);
	let multiplier: f64 = 1f64 / ((sample_ssaa_level * sample_ssaa_level) as f64);
	let mut rng: ThreadRng = thread_rng();

	// let origin: Pos3 = Pos3::new(0.0, 1.8, 0.0);
	// let origin: Pos3 = Pos3::new(0.0, 1.25, 0.18);
	let origin: Pos3 = Pos3::new(2.0, 0.8, 1.8);

	let mut count: u64 = 0;

	let camera: Camera = Camera::new(origin,  Pos3::new(0.0, -1.0, 0.0) - origin, 12f64 * std::f64::consts::PI / 6f64, 0.6f64, Screen::new(nx, ny));
	for x in 0..nx
	{
		for y in 0..ny
		{
			// println!("On pixel({}, {}), {:.4}%", x + 1, y + 1, ((count as f64) / (pixel as f64)) * 100.0);
			let pixel = frame_buf.get_pixel_mut(x as u32, y as u32);
			let mut color: Color = Color::default();

			let x_pixel_left_upper: f64 = (x as f64) - 0.5;
			let y_pixel_left_upper: f64 = (y as f64) - 0.5;
			for i in 0..sample_ssaa_level
			{
				for j in 0..sample_ssaa_level
				{
					let sx: f64 = x_pixel_left_upper + delta*((2*i + 1) as f64);
					let sy: f64 = y_pixel_left_upper + delta*((2*j + 1) as f64);
					// let sx = 400f64; let sy = 450f64;

					// let direction_ray: Vec3 = pos_upper_left_corner + horizontal*u - vertical*v - origin;
					let direction_ray = camera.get_direction(Pos2::new(sx, sy));
					let mut ray: Ray = Ray::new(camera.pos, direction_ray.unit(), Color::new(0.0, 0.0, 0.0));
					pathtracing(&mut ray, &hitables, sample_per_scatter, sample_ray_step, false);
					color += ray.color*multiplier;
				}
			}

			let rgb: RGB8 = color.gamma_correction(2.2).into();
			// rgb = rgb.gamma_correction();    // Table based is not accurate enough! 
			*pixel = image::Rgb([rgb.r, rgb.g, rgb.b]);
			count+=1;
		}
	}

	println!("End of render, now saving frame buffer. ");

	frame_buf.save("./o.png").unwrap();

	println!("\nDone. Annihilating. ");
}

fn pathtracing(ray: &mut Ray, list_hitable: &Vec<Sphere>, sample_per_scatter: u16, step: u8, flag_scattered: bool)
{
	if step > 0 && sample_per_scatter > 0
	{
		let stat: BoolObjF64 = get_stat_hit(ray, list_hitable);
		if stat.bool
		{
			// let mut material: Material = stat.obj.material.clone();
			let flag_rngs: bool = stat.obj.material.rngs();
			let flag_scatters: bool = stat.obj.material.scatters();
			let flag_refracts: bool = stat.obj.material.refracts();
			let flag_emits: bool = stat.obj.material.emits();

			let multiplier: f64 = if flag_rngs&&!flag_scattered { 1.0 / (sample_per_scatter as f64) } else { 1.0 };
			for _t in 0..(if flag_rngs&&!flag_scattered { sample_per_scatter } else { 1 })
			{
				if flag_scatters
				{
					let mut n_ray: Ray = Ray::new(ray.at(stat.f), scatter(ray, &stat), Color::default());
					pathtracing(&mut n_ray, list_hitable, sample_per_scatter, step - 1, flag_rngs || flag_scattered);
					ray.color += stat.obj.material.albedo()*n_ray.color*multiplier;
				}
				if flag_refracts
				{
                    let r_in_dir: Vec3 = ray.direction.unit();
                    let r_rf_dir: Vec3 = refract(ray, &stat);
    				let normal: Vec3 = (ray.at( stat.f ) - stat.obj.pos).unit();
    				let cos_theta_in: f64 = -r_in_dir*normal;
                    let sin_theta_in: f64 = (1f64 - cos_theta_in*cos_theta_in).abs().sqrt();
                    let cos_theta_rf: f64 = -r_rf_dir*normal;
                    let sin_theta_rf: f64 = (1f64 - cos_theta_rf*cos_theta_rf).abs().sqrt();
					let ratio: f64 = if !(r_in_dir&normal) { 1.0 / stat.obj.material.eta() } else { stat.obj.material.eta() };
                    let fresnel: f64 = fresnel(cos_theta_in.acos(), cos_theta_rf.acos(), ratio).abs();
                    // println!("{}", fresnel);
                    // assert!(fresnel <= 1f64);

					let mut n1_ray: Ray = Ray::new(ray.at(stat.f), r_rf_dir, Color::default());
					let mut n2_ray: Ray = Ray::new(ray.at(stat.f), scatter(ray, &stat), Color::default());
					pathtracing(&mut n1_ray, list_hitable, sample_per_scatter, step - 1, flag_rngs || flag_scattered);
					pathtracing(&mut n2_ray, list_hitable, sample_per_scatter, step - 1, flag_rngs || flag_scattered);
					ray.color += stat.obj.material.refraction()*(1f64 - fresnel)*n1_ray.color*multiplier*(if sin_theta_in*ratio <= 1f64 { 1f64 } else { 0f64 });
					ray.color += stat.obj.material.refraction()*fresnel*n2_ray.color*multiplier*(if sin_theta_in*ratio <= 1f64 { 1f64 } else { 0f64 });
				}
				if flag_emits
				{
					match stat.obj.material
					{
						Material::DebugNormalShading{ normal: _normal, mode } => { let normal = (ray.at( stat.f ) - stat.obj.pos).unit(); ray.color += Color::new(normal.x*0.64 + 0.36, normal.y*0.64 + 0.36, normal.z*0.64 + 0.36 );},
						_ => { ray.color += stat.obj.material.emit()*multiplier; } // NOTICE, emit is independent with scatter or absorb!
					}
				}
			}
		}
		else
		{
			// Skylight

			// let t: f64 = (ray.direction.z + 1.0)/2.0;
			// ray.color = ((1.0 - t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.4, 0.7, 1.0))

			// ray.color = Color::new(0.01, 0.01, 0.01);
		}
	}
}
