extern crate image;
extern crate rand;

mod math;
mod object;
mod material;
mod physics;
mod scene;
mod film;

use {math::*, rand::prelude::*, object::*, material::*, physics::*, scene::*};

fn main()
{
    println!("As ray shoots out of camera, photons emit from light sources, they reflects, scatters and decays, and so does the information it carries exchanges all-around.");
    println!("Eventually, geometries, objects and scenes are brought to life in this spacetime, this machine's imaginary, it's dream. ");
    println!(" ");
    println!("DECAY, a simple renderer. ");
    println!("WIP \n");

    let nx: u16 = 640;
    let ny: u16 = 480;
    let pixel: u64 = (nx as u64)*(ny as u64);
    let aspectRatio: f64 = nx as f64 / ny as f64;
    let screenWidth: f64 = 2.0;
    let screenHeight: f64 = screenWidth/aspectRatio;
    let sampleRayStep: u8 = 8;
    let samplePerScatter: u16 = 256; // Only on first scatter!
    let sampleSSAA_level: u8 = 4;

    let mut frameBuf = image::ImageBuffer::new(nx as u32, ny as u32);

    // Fill Background color
    for (x, y, pixel) in frameBuf.enumerate_pixels_mut()
    {
        *pixel = image::Rgb([0u8, 0u8, 0u8]);
    }

    let mut hitables: Vec<Sphere> = Vec::new();

    hitables.push(Sphere::new(Pos3::new(0.0, -1.0, 0.0), 0.5, Material::DebugNormalShading { normal: Vec3::default(), mode: 0 })); // Debug Normal
    // hitables.push(Sphere::new(Pos3::new(0.0, -1.0, 0.0), 0.5, Material::Emissive { emit: Color::new( 4.0, 4.0, 4.0 ) })); // Emissive
    // hitables.push(Sphere::new(Pos3::new(0.0, -1.0, 0.0), 0.5, Material::Metal{ albedo: 0.95, fuzz: 0.0})); // Metal Mirror

    hitables.push(Sphere::new(Pos3::new(-0.16, 1.0, -0.175), 0.10, Material::Metal{ albedo: Color::new(0.75, 0.75, 0.75), fuzz: 0.36, emit: Color::rgbEmpty() }));
    hitables.push(Sphere::new(Pos3::new(4.0, -12.0, 3.6), 4.2, Material::Metal{ albedo: Color::new(0.85, 0.85, 0.85), fuzz: 0.0025, emit: Color::rgbEmpty() }));

    hitables.push(Sphere::new(Pos3::new(0.0, -1.0, -256.5), 256.0, Material::Lambertian{ albedo: Color::new(0.5, 0.5, 0.5), emit: Color::rgbEmpty() }));

    let delta: f64 = 1.0f64 / ((2*sampleSSAA_level) as f64);
    let multiplier: f64 = 1.0f64 / ((sampleSSAA_level*sampleSSAA_level) as f64);
    let mut rng: ThreadRng = thread_rng();

    let origin: Pos3 = Pos3::new(0.0, 1.8, 0.0);
    let horizontal: Vec3 = Vec3::new(screenWidth, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 0.0, screenHeight);
    let posUpperLeftCorner: Pos3 = origin - horizontal/2.0 + vertical/2.0 - Vec3::new(0.0, 1.0, 0.0);
    let mut count: u64 = 0;

    for x in 0..nx
    {
        for y in 0..ny
        {
            println!("On pixel({}, {}), {:.4}%", x, y, ((count as f64) / (pixel as f64)) * 100.0);
            let pixel = frameBuf.get_pixel_mut(x as u32, y as u32);
            let mut rgb: RGB8 = RGB8::new(0, 0, 0);
            let mut color: Color = Color::default();

            let x_pixelLeftUpper: f64 = (x as f64) - 0.5;
            let y_pixelLeftUpper: f64 = (y as f64) - 0.5;
            for i in 0..sampleSSAA_level
            {
                for j in 0..sampleSSAA_level
                {
                    let sx: f64 = x_pixelLeftUpper + delta*((2*i + 1) as f64);
                    let sy: f64 = y_pixelLeftUpper + delta*((2*j + 1) as f64);
                    let u: f64 = sx / (nx as f64);
                    let v: f64 = sy / (ny as f64);

                    let directionRay: Vec3 = posUpperLeftCorner + horizontal*u - vertical*v - origin;
                    let mut ray: Ray = Ray::new(origin, directionRay.unit(), Color::new(0.0, 0.0, 0.0));
                    pathtracing(&mut ray, &hitables, samplePerScatter, sampleRayStep);
                    color += ray.color*multiplier;
                }
            }

            rgb = color.into();
            rgb = rgb.gammaCorrection();
            *pixel = image::Rgb([rgb.r, rgb.g, rgb.b]);
            count+=1;

            // println!("Pixel Done. \n")
        }
    }

    println!("End of render, now saving frame buffer. ");

    frameBuf.save("./o.png").unwrap();

    println!("\nDone. Annihilating. ");
}

fn pathtracing(ray: &mut Ray, list_hitable: &Vec<Sphere>, samplePerScatter: u16, step: u8)
{
    if step > 0 && samplePerScatter > 0
    {
        let stat: BoolObjF64 = hitStat(ray, list_hitable);
        if stat.bool
        {
            // let mut material: Material = stat.obj.material.clone();
            let flagRng: bool = stat.obj.material.rngs();
            let flagScatters: bool = stat.obj.material.scatters();
            // let flag
            let flagEmits: bool = stat.obj.material.emits();

            let multiplier: f64 = (if flagRng { 1.0 / (samplePerScatter as f64) } else { 1.0 });
            for _t in 0..(if flagRng { samplePerScatter } else { 1 })
            {
                if flagScatters
                {
                    let mut nRay: Ray = Ray::new(ray.at(stat.f), scatter(ray, &stat), Color::default());
                    pathtracing(&mut nRay, list_hitable, 1, step - 1);
                    ray.color += stat.obj.material.albedo()*nRay.color*multiplier;
                }
                if flagEmits
                {
                    match stat.obj.material
                    {
                        Material::DebugNormalShading{ normal, mode } => { let normal = (ray.at( stat.f ) - stat.obj.pos).unit(); ray.color += Color::new(normal.x*0.64 + 0.36, normal.y*0.64 + 0.36, normal.z*0.64 + 0.36 );},
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