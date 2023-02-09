use std::ffi::c_int;
use std::ops::{Add, Sub};
use std::process::exit;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::sys::rand;
use crate::rendering::{render, SDLWindow};
use crate::simulation::tick;
use crate::util::hsv_to_rgb;
use crate::world::{Particle, World};

mod rendering;
mod world;
mod simulation;
mod util;

fn main() {
    let mut world = World::new(200, 200);
    let mut window = SDLWindow::new(200, 200);
    run(world, window);
}


fn run(mut world: World, mut window: SDLWindow) {
    let mut time = std::time::Instant::now();
    let start = std::time::Instant::now();
    let mut elapsed = 1;
    let mut paused = false;
    let mut frame_count = 0usize;
    loop {
        if let Some(event) = window.event_pump.poll_event() {
            match event {
                Event::Quit { .. } => exit(0),
                Event::KeyDown { keycode, keymod, .. } => {
                    if keymod.is_empty() || true {
                        if let Some(key) = keycode {
                            match key {
                                Keycode::Escape => exit(0),
                                Keycode::Space => paused = !paused,
                                _ => ()
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if !paused {
            let mouse = MouseState::new(&window.event_pump);
            if mouse.left() {
                for x in mouse.x().sub(2).max(0)..mouse.x().add(2).min(world.width as i32 - 1) {
                    for y in mouse.y().sub(2).max(0)..mouse.y().add(2).min(world.height as i32 - 1) {
                        world.add_particle(Particle::new(
                            x as f32 + unsafe { rand() } as f32 / c_int::MAX as f32,
                            y as f32 + unsafe { rand() } as f32 / c_int::MAX as f32,
                            0.0,
                            0.0,
                            hsv_to_rgb(start.elapsed().as_millis() as f32 / 50.0 % 360.0, 1.0, 1.0)
                        ))
                    }
                }
            }
            if elapsed < 10_000 {
                tick(&mut world, elapsed as f32);
            }
        }
        render(&world, &mut window, paused, elapsed as f32);
        elapsed = time.elapsed().as_micros();
        frame_count = frame_count.wrapping_add(1);
        time = std::time::Instant::now();
    }
}