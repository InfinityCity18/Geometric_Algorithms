use plotters::prelude::*;
use std::{cmp::Ordering, ops::Range};

use crate::point_gen::*;

const SEED_A: u64 = 4857;
const SEED_B: u64 = 41;
const SEED_C: u64 = 412;
const SEED_D: u64 = 241;
const EPSILON: f64 = 1e-16;

fn main() {
    draw_set("setA.png", set_a(-100.0..100.0, 100, SEED_A), (-120.0..120.0, -120.0..120.0)).unwrap();
    draw_set("setB.png", set_b((0.0,0.0), 10.0, 100, SEED_B), (-20.0..20.0, -20.0..20.0)).unwrap();
    draw_set("setB_g.png", graham(set_b((0.0,0.0), 10.0, 100, SEED_B)), (-20.0..20.0, -20.0..20.0)).unwrap();
    draw_set("setC.png", set_c((-10.0, -10.0), (10.0,10.0), 100, SEED_C), (-15.0..15.0, -15.0..15.0)).unwrap();
    draw_set("setC_g.png", graham(set_c((-10.0, -10.0), (10.0,10.0), 100, SEED_C)), (-15.0..15.0, -15.0..15.0)).unwrap();
    draw_set("setD.png", set_d((0.0, 0.0), 10.0, 25, 20, SEED_D), (-2.0..12.0, -2.0..12.0)).unwrap();
    draw_set("setD_g.png", graham(set_d((0.0, 0.0), 10.0, 25, 20, SEED_D)), (-2.0..12.0, -2.0..12.0)).unwrap();
    draw_set("setA_g.png", graham(set_a(-100.0..100.0, 100, SEED_A)), (-120.0..120.0, -120.0..120.0)).unwrap();
}

fn graham(mut points: Vec<(f64,f64)>) -> Vec<(f64, f64)> {
    let start = points.iter().min_by(|a, b|
        if a.1 == b.1 {
            a.0.partial_cmp(&b.0).unwrap()
        } else {
            a.1.partial_cmp(&b.1).unwrap()
        }
    ).unwrap().to_owned();
    points.remove(points.iter().position(|x| *x == start).unwrap());
    //points.sort_unstable_by(|x,y| orient(start, *x, *y));
    points = mergesort(points, start);
    let mut stack = vec![start, points[0], points[1]];
    let mut t = 2;
    let mut i = 3;
    while i < points.len() {
        if det_3x3(stack[t-1], stack[t], points[i]) > 0.0 {
            stack.push(points[i]);
            t += 1;
            i += 1;
        } else {
            stack.pop();
            t -= 1;
        }
    }
    return stack;
}

fn det_3x3(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    a.0 * b.1 + b.0 * c.1 + c.0 * a.1 - c.0 * b.1 - a.1 * b.0 - a.0 * c.1
}

fn eq_float(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() <= epsilon
}

fn orient(p0: (f64, f64), b: (f64, f64), c: (f64, f64)) -> Ordering {
    let d = det_3x3(p0, b, c);
    if eq_float(d, 0.0, EPSILON) {
        return Ordering::Equal;
    } else if d > 0.0 {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

fn mergesort(mut points: Vec<(f64, f64)>, p0: (f64, f64)) -> Vec<(f64, f64)> {
    if points.len() <= 1 {
        return points;
    }
    let mid = points.len() / 2;
    let left = mergesort(points.drain(0..mid).collect(), p0);
    let right = mergesort(points, p0);
    return merge(left, right, p0);
}

fn merge(left: Vec<(f64, f64)>, right: Vec<(f64, f64)>, p0: (f64, f64)) -> Vec<(f64, f64)> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        if orient(p0, left[i], right[j]) == Ordering::Equal { //bierzemy tylko dalszy punkt bo sa wspolliniowe
            if left[i].0 > right[j].0 {
                merged.push(left[i]);
                i += 1;
                j += 1;
            } else {
                merged.push(right[j]);
                i += 1;
                j += 1;
            }
        } else if orient(p0, left[i], right[j]) == Ordering::Less {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }
    while i < left.len() {
        merged.push(left[i]);
        i += 1;
    }
    while j < right.len() {
        merged.push(right[j]);
        j += 1;
    }
    return merged;
}

mod point_gen {
    use rand::{
        Rng, SeedableRng, distr::{Distribution, StandardUniform, Uniform}, rngs::SmallRng, seq::IndexedRandom
    };

    pub fn set_a(
        range: std::ops::Range<f64>,
        n: usize,
        seed: u64,
    ) -> Vec<(f64, f64)> {
        let uni_dist = Uniform::new_inclusive(range.start, range.end).unwrap();
        let mut x_rand = SmallRng::seed_from_u64(seed);
        let mut y_rand = SmallRng::seed_from_u64(seed + 1);
        let x_iter = uni_dist.sample_iter(&mut x_rand);
        let y_iter = uni_dist.sample_iter(&mut y_rand);
        x_iter.zip(y_iter).take(n).collect()
    }

    pub fn set_b(origin: (f64, f64), radius: f64, n: usize, seed_theta: u64) -> Vec<(f64, f64)> {
        let uni_dist = Uniform::new_inclusive(0.0, 2.0 * std::f64::consts::PI).unwrap();
        let mut theta_rand = SmallRng::seed_from_u64(seed_theta);
        let theta_iter = uni_dist.sample_iter(&mut theta_rand);

        theta_iter
            .map(|theta| {
                (
                    radius * theta.cos() + origin.0,
                    radius * theta.sin() + origin.1,
                )
            })
            .take(n)
            .collect()
    }

    pub fn set_c(
        p1: (f64, f64),
        p2: (f64, f64),
        n: usize,
        seed: u64,
    ) -> Vec<(f64, f64)> {

        enum Side {
            LEFT,
            RIGHT,
            TOP,
            BOT
        }

        impl Distribution<Side> for StandardUniform {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Side {
                match rng.random_range(0..4) {
                    0 => Side::LEFT,
                    1 => Side::RIGHT,
                    2 => Side::TOP,
                    3 => Side::BOT,
                    _ => unreachable!()
                }
            }
        }

        // dwa punkty jednoznacznie definiuja prostokat z bokami rownoleglymi do obu osi
        let lt = (p1.0.min(p2.0), p1.1.max(p2.1));
        let lb = (p1.0.min(p2.0), p1.1.min(p2.1));
        let rt = (p1.0.max(p2.0), p1.1.max(p2.1));
        let rb = (p1.0.max(p2.0), p1.1.min(p2.1));

        let uni_dist_x = Uniform::new_inclusive(lb.0, rb.0).unwrap();
        let uni_dist_y = Uniform::new_inclusive(lb.1, lt.1).unwrap();
        let mut side_rand = SmallRng::seed_from_u64(seed);
        let mut x_rand = SmallRng::seed_from_u64(seed + 1);
        let mut y_rand = SmallRng::seed_from_u64(seed + 2);
        let mut x_iter = uni_dist_x.sample_iter(&mut x_rand);
        let mut y_iter = uni_dist_y.sample_iter(&mut y_rand);
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            let choice: Side = side_rand.random();
            v.push(
                match choice {
                    Side::BOT => (x_iter.next().unwrap(), lb.1),
                    Side::TOP => (x_iter.next().unwrap(), lt.1),
                    Side::LEFT => (lb.0, y_iter.next().unwrap()),
                    Side::RIGHT => (rb.0, y_iter.next().unwrap()),
                }
            );
        }
        return v;
    }

    pub fn set_d(
        p1: (f64, f64),
        side_len: f64,
        n_side: usize,
        n_diag: usize,
        seed: u64,
    ) -> Vec<(f64, f64)> {
        // dwa punkty jednoznacznie definiuja prostokat z bokami rownoleglymi do obu osi

        let uni_dist_x = Uniform::new_inclusive(p1.0, p1.0 + side_len).unwrap();
        let uni_dist_y = Uniform::new_inclusive(p1.1, p1.1 + side_len).unwrap();
        let mut x_rand = SmallRng::seed_from_u64(seed);
        let mut y_rand = SmallRng::seed_from_u64(seed + 1);
        let mut choice_rand = SmallRng::seed_from_u64(seed + 2);
        let mut x_iter = uni_dist_x.sample_iter(&mut x_rand);
        let mut y_iter = uni_dist_y.sample_iter(&mut y_rand);
        let mut v = Vec::with_capacity(n_side + n_diag);
        for _ in 0..n_side {
            let choice: usize = *[0,1].choose(&mut choice_rand).unwrap();
            v.push(
                if choice == 0 {
                    (x_iter.next().unwrap(), p1.1)
                } else {
                    (p1.0, y_iter.next().unwrap())
                }
            );
        }
        for _ in 0..n_diag {
            let choice: usize = *[0,1].choose(&mut choice_rand).unwrap();
            v.push(
                if choice == 0 {
                    let a = x_iter.next().unwrap();
                    (a, a)
                } else {
                    let a = y_iter.next().unwrap();
                    (a, p1.0 + side_len - a)
                }
            );
        }
        return v;
    }
}

pub fn draw_set(filename: &str, points: Vec<(f64,f64)>, label_ranges: (Range<f64>, Range<f64>)) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (512, 512)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let areas = root.split_by_breakpoints([488], [24]);

        let mut ctx = ChartBuilder::on(&areas[2])
            .x_label_area_size(80)
            .y_label_area_size(80)
            .build_cartesian_2d(label_ranges.0, label_ranges.1)?;
        ctx
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_desc("x")
            .y_desc("y")
            .x_labels(5)
            .y_labels(5)
            .label_style(("sans-serif", 22).into_font())
            .draw()?;

        ctx.draw_series(
            points.iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, BLACK.filled())),
        )?;
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        Ok(())
    }

