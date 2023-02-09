use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use crate::util::mutate;
use crate::world::{Particle, World};

pub(crate) fn tick(world: &mut World, delta_micro: f32) {
    let sub_steps = 10;
    let delta = delta_micro.max(1.0) / 100_000.0 / sub_steps as f32;
    let delta_sq = delta * delta;

    for _ in 0..sub_steps {
        // apply gravity
        let gx = 0.0;
        let gy = 9.81 * 5.0;
        (0..world.cells.len()).into_par_iter().for_each(|x| {
            for y in 0..world.cells[x].len() {
                for p in &world.cells[x][y] {
                    let p = unsafe { mutate(p) };
                    p.ax += gx;
                    p.ay += gy;
                }
            }
        });

        let response_coef = 0.75;
        let min_dist_sq = 1.2f32; // r = 0.5
        // collisions on odd columns
        (1..world.cells.len()-1).into_par_iter().step_by(2).for_each(|x| {
            for y in 1..(world.cells[x].len()-1) {
                if world.cells[x][y].len() > 0 {
                    let near = [
                        &world.cells[x - 1][y - 1], &world.cells[x][y - 1], &world.cells[x + 1][y - 1],
                        &world.cells[x - 1][y], &world.cells[x][y], &world.cells[x + 1][y],
                        &world.cells[x - 1][y + 1], &world.cells[x][y + 1], &world.cells[x + 1][y + 1]
                    ].into_iter().flatten().collect::<Vec<&Particle>>();
                    if near.len() > 1 {
                        // (i, k) collision pairs
                        for i in 0..near.len() {
                            let p1 = unsafe { mutate(*near.get_unchecked(i)) };
                            for k in (i + 1)..near.len() {
                                let p2 = unsafe { mutate(*near.get_unchecked(k)) };
                                let dx = p1.x - p2.x;
                                let dy = p1.y - p2.y;
                                let dsq = dx * dx + dy * dy;
                                if dsq < 1.0 {
                                    let d = dsq.sqrt();
                                    let nx = dx / d;
                                    let ny = dy / d;
                                    let dm = 0.5 * response_coef * (d - min_dist_sq.sqrt());
                                    p1.x -= nx * 0.5 * dm;
                                    p1.y -= ny * 0.5 * dm;
                                    p2.x += nx * 0.5 * dm;
                                    p2.y += ny * 0.5 * dm;
                                }
                            }
                        }
                    }
                }
            }
        });
        // collisions on even columns
        (1..world.cells.len()-1).into_par_iter().skip(1).step_by(2).for_each(|x| {
            for y in 1..(world.cells[x].len()-1) {
                if world.cells[x][y].len() > 0 {
                    let near = [
                        &world.cells[x - 1][y - 1], &world.cells[x][y - 1], &world.cells[x + 1][y - 1],
                        &world.cells[x - 1][y], &world.cells[x][y], &world.cells[x + 1][y],
                        &world.cells[x - 1][y + 1], &world.cells[x][y + 1], &world.cells[x + 1][y + 1]
                    ].into_iter().flatten().collect::<Vec<&Particle>>();
                    if near.len() > 1 {
                        // (i, k) collision pairs
                        for i in 0..near.len() {
                            let p1 = unsafe { mutate(*near.get_unchecked(i)) };
                            for k in (i + 1)..near.len() {
                                let p2 = unsafe { mutate(*near.get_unchecked(k)) };
                                let dx = p1.x - p2.x;
                                let dy = p1.y - p2.y;
                                let dsq = dx * dx + dy * dy;
                                if dsq < min_dist_sq {
                                    let d = dsq.sqrt();
                                    let nx = dx / d;
                                    let ny = dy / d;
                                    let dm = 0.5 * response_coef * (d - min_dist_sq.sqrt());
                                    p1.x -= nx * 0.5 * dm;
                                    p1.y -= ny * 0.5 * dm;
                                    p2.x += nx * 0.5 * dm;
                                    p2.y += ny * 0.5 * dm;
                                }
                            }
                        }
                    }
                }
            }
        });
        // borders
        (0..world.cells.len()).into_par_iter().for_each(|x| {
            for y in 0..world.cells[x].len() {
                for p in &world.cells[x][y] {
                    let p = unsafe { mutate(p) };
                    if p.x < 0.0 { p.x = 0.0 }
                    if p.x > world.width as f32 - 1.0 { p.x = world.width as f32 - 1.0 }
                    if p.y < 0.0 { p.y = 0.0 }
                    if p.y > world.height as f32 - 1.0 { p.y = world.height as f32 - 1.0 }
                }
            }
        });

        // update pos + cell
        let removes = Arc::new(Mutex::new(vec![]));
        (0..world.cells.len()).into_par_iter().for_each(|x| {
            for y in 0..world.cells[x].len() {
                unsafe { mutate(world) }.cells[x][y].retain_mut(|p| {
                    let dx = p.x - p.px;
                    let dy = p.y - p.py;
                    p.px = p.x;
                    p.py = p.y;
                    p.x += dx + p.ax * delta_sq;
                    p.y += dy + p.ay * delta_sq;
                    p.ax = 0.0;
                    p.ay = 0.0;
                    if p.x as usize != x || p.y as usize != y {
                        removes.lock().unwrap().push(p.clone());
                        false
                    } else { true }
                });
            }
        });
        removes.lock().unwrap().to_vec().into_iter().for_each(|p| {
            world.add_particle(p)
        });
    }
}