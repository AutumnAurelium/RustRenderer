#![allow(unused_must_use)]

extern crate sdl2;

mod data;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use std::vec::Vec;
use std::boxed::Box;
use std::f64::consts::PI;

use data::Point3D;
use data::Camera;
use data::Rendered;
use data::Sphere;
use data::Light;
use data::RaycastResult;

fn set_pixel(canvas: &mut WindowCanvas, x: u32, y: u32, c: Color) {
    canvas.set_draw_color(c);
    canvas.fill_rect(Rect::new(x as i32, y as i32, 1, 1,));
}

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const RAYLIMIT: u32 = 25;
const COLLIDE_DISTANCE: f64 = 0.0001;
const RAY_START_DIST: f64 = 0.01;

const REFLECTIONS: u32 = 2;

const MOVE_STEP: f64 = 0.2;
const ROTATE_STEP: f64 = 7.5; // in degrees

fn raycast_hit(origin: Point3D, pitch: f64, yaw: f64, rendered: &[Box<dyn Rendered>]) -> bool {
    let mut ray_pos: Point3D = origin + Camera::ray_step(pitch, yaw, RAY_START_DIST);
    for _i in 0..RAYLIMIT {
        let mut min_dist: f64 = rendered[0].distance(ray_pos);
        for k in 1..rendered.len() {
            let dist = rendered[k].distance(ray_pos);
            if dist < min_dist {
                min_dist = dist;
            }
        }

        if min_dist <= COLLIDE_DISTANCE { // we've hit a surface! cool!
            return true;
        }

        ray_pos = ray_pos + Camera::ray_step(pitch, yaw, min_dist);
    }

    return false;
}

fn can_see(origin: Point3D, pos: Point3D, rendered: &[Box<dyn Rendered>]) -> bool {
    let xy_vector_length: f64 = origin.distance(Point3D::new(pos.x, pos.y, origin.z));
    let x_length: f64 = pos.x - origin.x;
    let length = origin.distance(pos);

    let pitch = (xy_vector_length / length).acos();
    let yaw = (x_length / xy_vector_length).acos();

    return !raycast_hit(origin, pitch, yaw, rendered);
}

fn mix(color1: Color, color2: Color, weight: f64) -> Color {
    let inverse = 1.0 - weight;
    let r: u8 = ((color1.r as f64 * weight) + (color2.r as f64 * inverse)) as u8; 
    let g: u8 = ((color1.g as f64 * weight) + (color2.g as f64 * inverse)) as u8; 
    let b: u8 = ((color1.b as f64 * weight) + (color2.b as f64 * inverse)) as u8; 
    return Color::RGB(r, g, b);
}

fn raycast(start_pos: Point3D, pitch: f64, yaw: f64, rendered: &[Box<dyn Rendered>], light: Light, reflections: u32) -> RaycastResult {
    let mut result: RaycastResult = RaycastResult::new(Point3D::new(0.0, 0.0, 0.0), Color::RGB(50, 50, 50), 0.0); // default color  

    let mut ray_pos: Point3D = start_pos + Camera::ray_step(pitch, yaw, RAY_START_DIST);
    for _i in 0..RAYLIMIT {
        let mut min_dist: f64 = rendered[0].distance(ray_pos);
        let mut min_object: usize = 0;
        for k in 1..rendered.len() {
            let dist = rendered[k].distance(ray_pos);
            if dist < min_dist {
                min_dist = dist;
                min_object = k;
            }
        }

        if min_dist <= COLLIDE_DISTANCE { // we've hit a surface! cool!
            if can_see(ray_pos, light.pos, rendered)  {
                result.color = rendered[min_object].color();
                result.reflectivity = rendered[min_object].reflectivity();
                result.hit_pos = ray_pos;
            } else {
                result.color = Color::RGB(0, 0, 0);
            }

            if reflections > 0 {
                let reflection_result = raycast(ray_pos, PI - pitch, PI - yaw, rendered, light, reflections - 1);
                result.color = mix(reflection_result.color, result.color, result.reflectivity);
            }
            
            break;
        }

        ray_pos = ray_pos + Camera::ray_step(pitch, yaw, min_dist);
    }

    return result;
}

fn render(canvas: &mut WindowCanvas, camera: Camera, rendered: &[Box<dyn Rendered>]) {
    println!("Starting frame render.");

    let mut pitch: f64 = camera.pitch + (camera.vfov(WIDTH, HEIGHT) / 2.0);
    let mut yaw: f64 = camera.yaw - (camera.hfov / 2.0);

    let pitch_step: f64 = camera.vfov(WIDTH, HEIGHT) / (HEIGHT as f64);
    let yaw_step: f64 = camera.hfov / (WIDTH as f64);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let result = raycast(camera.pos, pitch, yaw, rendered, Light::new(Point3D::new(2.0, 2.0, 5.0)), REFLECTIONS);

            set_pixel(canvas, x, y, result.color);
            yaw += yaw_step;
        }
        yaw = camera.yaw - (camera.hfov / 2.0);
        pitch -= pitch_step;
    }

    println!("Finished frame render.");
}

pub fn main() -> Result<(), String> {
    let mut camera: Camera = Camera::new(Point3D::new(0.0, 0.0, 0.0), 0f64.to_radians(), 0f64.to_radians(), 90f64.to_radians());

    let mut rendered = Vec::<Box<dyn Rendered>>::new();

    rendered.push(Box::new(Sphere::new(Point3D::new(5.0, 0.0, 0.0), 2.0, Color::RGB(255, 0, 0), 1.0)));
    rendered.push(Box::new(Sphere::new(Point3D::new(0.0, -5.0, 0.0), 2.0, Color::RGB(0, 255, 0), 1.0)));
    rendered.push(Box::new(Sphere::new(Point3D::new(5.0, 5.0, 0.0), 2.0, Color::RGB(255, 0, 0), 0.9)));
    rendered.push(Box::new(Sphere::new(Point3D::new(5.0, 10.0, 0.0), 2.0, Color::RGB(255, 0, 0), 0.25)));
    rendered.push(Box::new(Sphere::new(Point3D::new(11.0, 0.0, 3.0), 2.0, Color::RGB(255, 0, 0), 0.1)));
    rendered.push(Box::new(Sphere::new(Point3D::new(8.0, -3.0, -3.5), 3.0, Color::RGB(0, 0, 255), 0.5)));

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo: Video", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    render(&mut canvas, camera, &rendered);
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },

                Event::KeyDown { keycode: Some(Keycode::W), ..} => {
                    camera.pos = camera.pos + Camera::ray_step(camera.pitch, camera.yaw, MOVE_STEP);
                    render(&mut canvas, camera, &rendered);
                },
                Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                    camera.pos = camera.pos - Camera::ray_step(camera.pitch, camera.yaw, MOVE_STEP);
                    render(&mut canvas, camera, &rendered);
                },

                Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                    camera.pos = camera.pos - Camera::ray_step(camera.pitch, camera.yaw + 90f64.to_radians(), MOVE_STEP);
                    render(&mut canvas, camera, &rendered);
                },
                Event::KeyDown { keycode: Some(Keycode::D), ..} => {
                    camera.pos = camera.pos + Camera::ray_step(camera.pitch, camera.yaw + 90f64.to_radians(), MOVE_STEP);
                    render(&mut canvas, camera, &rendered);
                },

                Event::KeyDown { keycode: Some(Keycode::Q), ..} => {
                    camera.pos = camera.pos + Camera::ray_step(camera.pitch + 90f64.to_radians(), camera.yaw, MOVE_STEP);
                    render(&mut canvas, camera, &rendered);
                },
                Event::KeyDown { keycode: Some(Keycode::E), ..} => {
                    camera.pos = camera.pos - Camera::ray_step(camera.pitch + 90f64.to_radians(), camera.yaw, MOVE_STEP);
                    render(&mut canvas, camera, &rendered);
                },

                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    camera.pitch -= ROTATE_STEP.to_radians();
                    render(&mut canvas, camera, &rendered);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    camera.pitch += ROTATE_STEP.to_radians();
                    render(&mut canvas, camera, &rendered);
                },

                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    camera.yaw -= ROTATE_STEP.to_radians();
                    render(&mut canvas, camera, &rendered);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    camera.yaw += ROTATE_STEP.to_radians();
                    render(&mut canvas, camera, &rendered);
                },

                _ => {}
            }
        }

        //canvas.set_draw_color(Color::RGB(0, 0, 0));
        //canvas.clear();
    
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}