#![feature(test)]

use plotters::prelude::*;
use serde::Deserialize;
use std::{cmp::{Ordering, min}, io::Write, ops::Range, sync::OnceLock};
use crate::point_gen::*;

const SEED_A: u64 = 4857;
const SEED_B: u64 = 41;
const SEED_C: u64 = 412;
const SEED_D: u64 = 241;
const EPSILON: f64 = 1e-10;
const LABEL_MUL: f64 = 1.2;

static ANIM: OnceLock<bool> = OnceLock::new();

fn main() {
    create_dirs();

    let conf: Config = config::Config::builder().add_source(config::File::with_name("config.toml")).build().unwrap().try_deserialize().unwrap();
    ANIM.set(conf.anim).unwrap();
    if conf.base {
        let sets = [
            ("setA", set_a(-100.0..100.0, 100, SEED_A), -120.0..120.0),
            ("setB", set_b((0.0, 0.0), 10.0, 100, SEED_B), -LABEL_MUL*20.0..LABEL_MUL*20.0),
            ("setC", set_c((-10.0, -10.0), (10.0, 10.0), 100, SEED_C), -15.0..15.0),
            ("setD", set_d((0.0, 0.0), 10.0, 25, 20, SEED_D), -2.0..12.0),
        ];

        for (name, v, label) in sets {
            let label_ranges = (label.clone(), label);
            let root_graham = BitMapBackend::gif(format!("gifs/graham/{}_g_anim.gif", name), (512,512), 0).unwrap().into_drawing_area();
            let root_jarvis = BitMapBackend::gif(format!("gifs/jarvis/{}_j_anim.gif", name), (512,512), 0).unwrap().into_drawing_area();
            let _ = draw_set(&format!("sets/{}.png", name), v.clone(), label_ranges.clone());
            let _ = draw_set_convex(&format!("graham/{}_g_convex_hull.txt", name), &format!("graham/{}_g.png", name), v.clone(), graham(v.clone(), root_graham, label_ranges.clone()), label_ranges.clone());
            let _ = draw_set_convex(&format!("jarvis/{}_j_convex_hull.txt", name), &format!("jarvis/{}_j.png", name), v.clone(), jarvis(v, root_jarvis, label_ranges.clone()), label_ranges.clone());
            println!("{name} done!");
        }
    }

    let label_a = (conf.set_a.range.0 - conf.set_a.range.1).abs() * (LABEL_MUL - 1.0);
    let label_b_x = (conf.set_b.origin.0-LABEL_MUL*conf.set_b.radius)..(conf.set_b.origin.0+LABEL_MUL*conf.set_b.radius);
    let label_b_y = (conf.set_b.origin.1-LABEL_MUL*conf.set_b.radius)..(conf.set_b.origin.1+LABEL_MUL*conf.set_b.radius);
    let label_c_x = (conf.set_c.p2.0 - conf.set_c.p1.0).abs() * (LABEL_MUL - 1.0);
    let label_c_y = (conf.set_c.p2.1 - conf.set_c.p1.1).abs() * (LABEL_MUL - 1.0);
    let sets_config = [
        ("setA", set_a(conf.set_a.range.0..conf.set_a.range.1, conf.set_a.n, SEED_A), -label_a+conf.set_a.range.0..conf.set_a.range.1+label_a, -label_a+conf.set_a.range.0..conf.set_a.range.1+label_a),
        ("setB", set_b(conf.set_b.origin, conf.set_b.radius, conf.set_b.n, SEED_B), label_b_x, label_b_y),
        ("setC", set_c(conf.set_c.p1, conf.set_c.p2, conf.set_c.n, SEED_C), conf.set_c.p1.0-label_c_x..conf.set_c.p2.0+label_c_x, conf.set_c.p1.1-label_c_y..conf.set_c.p2.1+label_c_y),
        ("setD", set_d(conf.set_d.p1, conf.set_d.len, conf.set_d.n_side, conf.set_d.n_diag, SEED_D), (conf.set_d.p1.0 - (LABEL_MUL - 1.0) * conf.set_d.len)..(conf.set_d.p1.0 + LABEL_MUL * conf.set_d.len), (conf.set_d.p1.1 - (LABEL_MUL - 1.0) * conf.set_d.len)..(conf.set_d.p1.1 + LABEL_MUL * conf.set_d.len)),
    ];

    for (name, v, label_x, label_y) in sets_config {
        let label_ranges = (label_x, label_y);
        let root_graham = BitMapBackend::gif(format!("custom/gifs/graham/{}_g_anim.gif", name), (512,512), 0).unwrap().into_drawing_area();
        let root_jarvis = BitMapBackend::gif(format!("custom/gifs/jarvis/{}_j_anim.gif", name), (512,512), 0).unwrap().into_drawing_area();
        let _ = draw_set(&format!("custom/sets/{}.png", name), v.clone(), label_ranges.clone());
        let _ = draw_set_convex(&format!("custom/graham/{}_g_convex_hull.txt", name), &format!("custom/graham/{}_g.png", name), v.clone(), graham(v.clone(), root_graham, label_ranges.clone()), label_ranges.clone());
        let _ = draw_set_convex(&format!("custom/jarvis/{}_j_convex_hull.txt", name), &format!("custom/jarvis/{}_j.png", name), v.clone(), jarvis(v, root_jarvis, label_ranges.clone()), label_ranges.clone());
        println!("{name} custom done!");
    }
}

fn graham(mut points: Vec<(f64, f64)>, root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>, label_ranges: (Range<f64>, Range<f64>)) -> Vec<(f64, f64)> {
    let original_pts = points.clone();

    let pivot = points
        .iter()
        .min_by(|a, b| {
            if a.1 == b.1 {
                a.0.partial_cmp(&b.0).unwrap()
            } else {
                a.1.partial_cmp(&b.1).unwrap()
            }
        })
        .unwrap()
        .to_owned();

    points.retain(|p| *p != pivot);

    points.sort_unstable_by(|a, b| {
        let d = det_3x3(pivot, *a, *b);
        if eq_float(d, 0.0, EPSILON) {
            let da2 = (a.0 - pivot.0).hypot(a.1 - pivot.1);
            let db2 = (b.0 - pivot.0).hypot(b.1 - pivot.1);
            da2.partial_cmp(&db2).unwrap()
        } else if d > 0.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut filtered: Vec<(f64, f64)> = Vec::with_capacity(points.len());
    for p in points.into_iter() {
        if let Some(last) = filtered.last() {
            if eq_float(det_3x3(pivot, *last, p), 0.0, EPSILON) {
                filtered.pop();
                filtered.push(p);
                continue;
            }
        }
        filtered.push(p);
    }

    let mut stack: Vec<(f64, f64)> = vec![pivot, filtered[0], filtered[1]];
    for p in filtered.iter().skip(2) {
        if *ANIM.get().unwrap() {
            root.fill(&WHITE).unwrap();
            let mut ctx = draw_labels(&root, label_ranges.clone());
            ctx.draw_series(
                original_pts
                    .iter()
                    .map(|(x, y)| Circle::new((*x, *y), 2, BLACK.filled())),
            ).unwrap();
            ctx.draw_series(LineSeries::new(stack.iter().cloned(), RED)).unwrap();
            ctx.draw_series(stack.iter().cloned().map(|(x,y)| Circle::new((x,y), 2, RED.filled()))).unwrap();
            root.present().unwrap();
        }

        while stack.len() >= 2 {
            let s = stack.len();
            let d = det_3x3(stack[s - 2], stack[s - 1], *p);
            if d > EPSILON {
                break; 
            } else {
                stack.pop();
            }
        }
        stack.push(*p);
    }

    if *ANIM.get().unwrap() {
        root.fill(&WHITE).unwrap();
        let mut ctx = draw_labels(&root, label_ranges.clone());
        ctx.draw_series(
            original_pts
                .iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, BLACK.filled())),
        ).unwrap();
        ctx.draw_series(LineSeries::new(stack.iter().cloned().chain(std::iter::once(stack[0])) , RED)).unwrap();
        ctx.draw_series(stack.iter().cloned().map(|(x,y)| Circle::new((x,y), 2, RED.filled()))).unwrap();
        for _ in 0..40 {
            root.present().unwrap();
        }
    }

    stack
}

fn jarvis(points: Vec<(f64, f64)>, root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>, label_ranges: (Range<f64>, Range<f64>)) -> Vec<(f64, f64)> {

    let mut pts = points.clone();
    // pts.sort_by(|a, b| {
    //     if eq_float(a.0, b.0, EPSILON) {
    //         a.1.partial_cmp(&b.1).unwrap()
    //     } else {
    //         a.0.partial_cmp(&b.0).unwrap()
    //     }
    // });
    // pts.dedup_by(|a, b| eq_float(a.0, b.0, EPSILON) && eq_float(a.1, b.1, EPSILON));

    let start = pts
        .iter()
        .min_by(|a, b| {
            if a.1 == b.1 {
                a.0.partial_cmp(&b.0).unwrap()
            } else {
                a.1.partial_cmp(&b.1).unwrap()
            }
        })
        .unwrap()
        .to_owned();

    let mut hull: Vec<(f64, f64)> = vec![start];
    let mut current = start;

    loop {
        let mut candidate: Option<(f64, f64)> = None;
        for (anim_counter, &p) in pts.iter().enumerate() {
            if p == current {
                continue;
            }
            if candidate.is_none() {
                candidate = Some(p);
                continue;
            }
            
            let cand = candidate.unwrap();

            if *ANIM.get().unwrap() && anim_counter % 5 == 0 {
                root.fill(&WHITE).unwrap();
                let mut ctx = draw_labels(&root, label_ranges.clone());
                ctx.draw_series(
                    pts
                        .iter()
                        .map(|(x, y)| Circle::new((*x, *y), 2, BLACK.filled())),
                ).unwrap();
                ctx.draw_series(LineSeries::new(hull.iter().cloned(), RED)).unwrap();
                ctx.draw_series(LineSeries::new(vec![*hull.last().unwrap(),p], BLUE)).unwrap();
                ctx.draw_series(hull.iter().cloned().map(|(x,y)| Circle::new((x,y), 2, RED.filled()))).unwrap();
                root.present().unwrap();
            }

            let d = det_3x3(current, cand, p);
            if d > EPSILON {
                candidate = Some(p);
            } else if eq_float(d, 0.0, EPSILON) {
                let dist_p = (p.0 - current.0).hypot(p.1 - current.1);
                let dist_c = (cand.0 - current.0).hypot(cand.1 - current.1);
                if dist_p > dist_c {
                    candidate = Some(p);
                }
            }
        }

        let next = match candidate {
            Some(pt) => pt,
            None => break,
        };

        if next == hull[0] {
            break;
        }

        hull.push(next);
        current = next;
    }

    if *ANIM.get().unwrap() {
        root.fill(&WHITE).unwrap();
        let mut ctx = draw_labels(&root, label_ranges.clone());
        ctx.draw_series(
            pts
                .iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, BLACK.filled())),
        ).unwrap();
        ctx.draw_series(LineSeries::new(hull.iter().cloned().chain(std::iter::once(hull[0])) , RED)).unwrap();
        ctx.draw_series(hull.iter().cloned().map(|(x,y)| Circle::new((x,y), 2, RED.filled()))).unwrap();
        for _ in 0..20 {
            root.present().unwrap();
        }
    }
    hull
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
        // if (b.0 - p0.0).powi(2) + (b.1 - p0.1).powi(2) < (c.0 - p0.0).powi(2) + (c.1 - p0.1).powi(2) {
        //     return Ordering::Less;
        // } else {
        //     return Ordering::Greater;
        // }
    } else if d > 0.0 {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

mod point_gen {
    use rand::{
        Rng, SeedableRng,
        distr::{Distribution, StandardUniform, Uniform},
        rngs::SmallRng,
    };

    pub fn set_a(range: std::ops::Range<f64>, n: usize, seed: u64) -> Vec<(f64, f64)> {
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

    pub fn set_c(p1: (f64, f64), p2: (f64, f64), n: usize, seed: u64) -> Vec<(f64, f64)> {
        enum Side {
            LEFT,
            RIGHT,
            TOP,
            BOT,
        }

        impl Distribution<Side> for StandardUniform {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Side {
                match rng.random_range(0..4) {
                    0 => Side::LEFT,
                    1 => Side::RIGHT,
                    2 => Side::TOP,
                    3 => Side::BOT,
                    _ => unreachable!(),
                }
            }
        }

        let lt = (p1.0.min(p2.0), p1.1.max(p2.1));
        let lb = (p1.0.min(p2.0), p1.1.min(p2.1));
        let _rt = (p1.0.max(p2.0), p1.1.max(p2.1));
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
            v.push(match choice {
                Side::BOT => (x_iter.next().unwrap(), lb.1),
                Side::TOP => (x_iter.next().unwrap(), lt.1),
                Side::LEFT => (lb.0, y_iter.next().unwrap()),
                Side::RIGHT => (rb.0, y_iter.next().unwrap()),
            });
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

        let uni_dist_x = Uniform::new_inclusive(p1.0, p1.0 + side_len).unwrap();
        let uni_dist_y = Uniform::new_inclusive(p1.1, p1.1 + side_len).unwrap();
        let mut x_rand = SmallRng::seed_from_u64(seed);
        let mut y_rand = SmallRng::seed_from_u64(seed + 1);
        let mut x_iter = uni_dist_x.sample_iter(&mut x_rand);
        let mut y_iter = uni_dist_y.sample_iter(&mut y_rand);
        let mut v = vec![p1, (p1.0 + side_len, p1.1), (p1.0, p1.1 + side_len), (p1.0 + side_len, p1.1 + side_len)]; 
        for _ in 0..n_side {
            v.push(
                (x_iter.next().unwrap(), p1.1)
            );
            v.push(
                (p1.0, y_iter.next().unwrap())
            );
        }
        for _ in 0..n_diag {
            let a = x_iter.next().unwrap();
            v.push((a, p1.1 + (a - p1.0)));
            let a = x_iter.next().unwrap();
            v.push((a, p1.1 + side_len - (a - p1.0)));
        }
        return v;
    }
}

pub fn draw_set(
    filename: &str,
    points: Vec<(f64, f64)>,
    label_ranges: (Range<f64>, Range<f64>),
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (512, 512)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let areas = root.split_by_breakpoints([488], [24]);

    let mut ctx = draw_labels(&areas[2], label_ranges);

    ctx.draw_series(
        points
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 2, BLACK.filled())),
    )?;
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    Ok(())
}

pub fn draw_set_convex(
    convex_filename: &str,
    filename: &str,
    points: Vec<(f64, f64)>,
    convex_hull: Vec<(f64, f64)>,
    label_ranges: (Range<f64>, Range<f64>),
) -> Result<(), Box<dyn std::error::Error>> {

    let f = std::fs::File::create(convex_filename).unwrap();
    for (x, y) in convex_hull.iter() {
        writeln!(&f, "({}, {})", x, y).unwrap();
    }

    let root = BitMapBackend::new(filename, (512, 512)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let areas = root.split_by_breakpoints([488], [24]);

    let mut ctx = draw_labels(&areas[2], label_ranges);

    ctx.draw_series(
        points
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 2, BLACK.filled())),
    )?;
    ctx.draw_series(
        convex_hull
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 2, RED.filled())),
    )?;
    let first = convex_hull[0];
    ctx.draw_series(LineSeries::new(convex_hull.into_iter().chain(std::iter::once(first)), RED)).unwrap();
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    Ok(())
}

fn draw_labels<DB: DrawingBackend>(root: &DrawingArea<DB, plotters::coord::Shift>, label_ranges: (Range<f64>, Range<f64>)) -> ChartContext<'_, DB, Cartesian2d<plotters::coord::types::RangedCoordf64, plotters::coord::types::RangedCoordf64>> {
    let mut ctx: ChartContext<'_, DB, Cartesian2d<plotters::coord::types::RangedCoordf64, plotters::coord::types::RangedCoordf64>> = ChartBuilder::on(root)
        .x_label_area_size(80)
        .y_label_area_size(80)
        .build_cartesian_2d(label_ranges.0, label_ranges.1).unwrap();
    ctx.configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc("x")
        .y_desc("y")
        .x_labels(5)
        .y_labels(5)
        .label_style(("sans-serif", 22).into_font())
        .draw().unwrap();
    return ctx;
}

#[derive(Debug, Deserialize)]
struct Config {
    base: bool,
    anim: bool,
    set_a: SetA,
    set_b: SetB,
    set_c: SetC,
    set_d: SetD,
}

#[derive(Debug, Deserialize)]
struct SetA {
    range: (f64, f64),
    n: usize,
}

#[derive(Debug, Deserialize)]
struct SetB {
    origin: (f64, f64),
    radius: f64,
    n: usize,
}

#[derive(Debug, Deserialize)]
struct SetC {
    p1: (f64, f64),
    p2: (f64, f64),
    n: usize,
}

#[derive(Debug, Deserialize)]
struct SetD {
    p1: (f64, f64),
    len: f64,
    n_side: usize,
    n_diag: usize,
}

fn create_dirs() {
    let dirs = [
        "gifs/graham",
        "gifs/jarvis",
        "sets",
        "graham",
        "jarvis",
        "custom/gifs/graham",
        "custom/gifs/jarvis",
        "custom/sets",
        "custom/graham",
        "custom/jarvis",
    ];
    for dir in dirs.iter() {
        let _ = std::fs::create_dir_all(dir);
    }
}
mod bench {
    use crate::*;
    pub fn graham_bench(mut points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
        let pivot = points
            .iter()
            .min_by(|a, b| {
                if a.1 == b.1 {
                    a.0.partial_cmp(&b.0).unwrap()
                } else {
                    a.1.partial_cmp(&b.1).unwrap()
                }
            })
            .unwrap()
            .to_owned();

        points.retain(|p| *p != pivot);

        points.sort_unstable_by(|a, b| {
            let d = det_3x3(pivot, *a, *b);
            if eq_float(d, 0.0, EPSILON) {
                let da2 = (a.0 - pivot.0).hypot(a.1 - pivot.1);
                let db2 = (b.0 - pivot.0).hypot(b.1 - pivot.1);
                da2.partial_cmp(&db2).unwrap()
            } else if d > 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let mut filtered: Vec<(f64, f64)> = Vec::with_capacity(points.len());
        for p in points.into_iter() {
            if let Some(last) = filtered.last() {
                if eq_float(det_3x3(pivot, *last, p), 0.0, EPSILON) {
                    filtered.pop();
                    filtered.push(p);
                    continue;
                }
            }
            filtered.push(p);
        }

        let mut stack: Vec<(f64, f64)> = vec![pivot, filtered[0], filtered[1]];
        for p in filtered.iter().skip(2) {
            while stack.len() >= 2 {
                let s = stack.len();
                let d = det_3x3(stack[s - 2], stack[s - 1], *p);
                if d > EPSILON {
                    break; 
                } else {
                    stack.pop();
                }
            }
            stack.push(*p);
        }

        stack
    }

    pub fn jarvis_bench(pts: Vec<(f64, f64)>) -> Vec<(f64, f64)> {

        let start = pts
            .iter()
            .min_by(|a, b| {
                if a.1 == b.1 {
                    a.0.partial_cmp(&b.0).unwrap()
                } else {
                    a.1.partial_cmp(&b.1).unwrap()
                }
            })
            .unwrap()
            .to_owned();

        let mut hull: Vec<(f64, f64)> = vec![start];
        let mut current = start;

        loop {
            let mut candidate: Option<(f64, f64)> = None;
            for &p in pts.iter() {
                if p == current {
                    continue;
                }
                if candidate.is_none() {
                    candidate = Some(p);
                    continue;
                }
                
                let cand = candidate.unwrap();

                let d = det_3x3(current, cand, p);
                if d > EPSILON {
                    candidate = Some(p);
                } else if eq_float(d, 0.0, EPSILON) {
                    let dist_p = (p.0 - current.0).hypot(p.1 - current.1);
                    let dist_c = (cand.0 - current.0).hypot(cand.1 - current.1);
                    if dist_p > dist_c {
                        candidate = Some(p);
                    }
                }
            }

            let next = match candidate {
                Some(pt) => pt,
                None => break,
            };

            if next == hull[0] {
                break;
            }

            hull.push(next);
            current = next;
        }

        hull
    }
}

#[cfg(test)]
mod benches {
    use std::hint::black_box;

    use crate::bench::*;
    use crate::*;
    extern crate test;
    
    #[bench]
    fn graham_benchmark_a_100(b: &mut test::Bencher) {
        let n = 100;
        let set = point_gen::set_a(-100.0..100.0, n, SEED_A);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_a_1000(b: &mut test::Bencher) {
        let n = 1000;
        let set = point_gen::set_a(-100.0..100.0, n, SEED_A);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_a_10000(b: &mut test::Bencher) {
        let n = 10000;
        let set = point_gen::set_a(-100.0..100.0, n, SEED_A);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_a_100000(b: &mut test::Bencher) {
        let n = 100000;
        let set = point_gen::set_a(-100.0..100.0, n, SEED_A);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_b_100(b: &mut test::Bencher) {
        let n = 100;
        let set = point_gen::set_b((0.0, 0.0), 10.0, n, SEED_B);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_b_1000(b: &mut test::Bencher) {
        let n = 1000;
        let set = point_gen::set_b((0.0, 0.0), 10.0, n, SEED_B);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_b_10000(b: &mut test::Bencher) {
        let n = 10000;
        let set = point_gen::set_b((0.0, 0.0), 10.0, n, SEED_B);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_b_100000(b: &mut test::Bencher) {
        let n = 100000;
        let set = point_gen::set_b((0.0, 0.0), 10.0, n, SEED_B);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_c_100(b: &mut test::Bencher) {
        let n = 100;
        let set = point_gen::set_c((-10.0, -10.0), (10.0, 10.0), n, SEED_C);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_c_1000(b: &mut test::Bencher) {
        let n = 1000;
        let set = point_gen::set_c((-10.0, -10.0), (10.0, 10.0), n, SEED_C);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_c_10000(b: &mut test::Bencher) {
        let n = 10000;
        let set = point_gen::set_c((-10.0, -10.0), (10.0, 10.0), n, SEED_C);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_c_100000(b: &mut test::Bencher) {
        let n = 100000;
        let set = point_gen::set_c((-10.0, -10.0), (10.0, 10.0), n, SEED_C);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_d_25(b: &mut test::Bencher) {
        let set = point_gen::set_d((0.0, 0.0), 10.0, 25, 20, SEED_D);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_d_250(b: &mut test::Bencher) {
        let set = point_gen::set_d((0.0, 0.0), 10.0, 250, 200, SEED_D);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_d_2500(b: &mut test::Bencher) {
        let set = point_gen::set_d((0.0, 0.0), 10.0, 2500, 2000, SEED_D);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn graham_benchmark_d_25000(b: &mut test::Bencher) {
        let set = point_gen::set_d((0.0, 0.0), 10.0, 25000, 20000, SEED_D);
        b.iter(|| {
            black_box(graham_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_a_100(b: &mut test::Bencher) {
        let n = 100;
        let set = point_gen::set_a(-100.0..100.0, n, SEED_A);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_a_1000(b: &mut test::Bencher) {
        let n = 1000;
        let set = point_gen::set_a(-100.0..100.0, n, SEED_A);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_a_10000(b: &mut test::Bencher) {
        let n = 10000;
        let set = point_gen::set_a(-100.0..100.0, n, SEED_A);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_a_100000(b: &mut test::Bencher) {
        let n = 100000;
        let set = point_gen::set_a(-100.0..100.0, n, SEED_A);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_b_100(b: &mut test::Bencher) {
        let n = 100;
        let set = point_gen::set_b((0.0, 0.0), 10.0, n, SEED_B);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_b_1000(b: &mut test::Bencher) {
        let n = 1000;
        let set = point_gen::set_b((0.0, 0.0), 10.0, n, SEED_B);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_b_10000(b: &mut test::Bencher) {
        let n = 10000;
        let set = point_gen::set_b((0.0, 0.0), 10.0, n, SEED_B);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_c_100(b: &mut test::Bencher) {
        let n = 100;
        let set = point_gen::set_c((-10.0, -10.0), (10.0, 10.0), n, SEED_C);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_c_1000(b: &mut test::Bencher) {
        let n = 1000;
        let set = point_gen::set_c((-10.0, -10.0), (10.0, 10.0), n, SEED_C);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_c_10000(b: &mut test::Bencher) {
        let n = 10000;
        let set = point_gen::set_c((-10.0, -10.0), (10.0, 10.0), n, SEED_C);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_c_100000(b: &mut test::Bencher) {
        let n = 100000;
        let set = point_gen::set_c((-10.0, -10.0), (10.0, 10.0), n, SEED_C);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_d_25(b: &mut test::Bencher) {
        let set = point_gen::set_d((0.0, 0.0), 10.0, 25, 20, SEED_D);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_d_250(b: &mut test::Bencher) {
        let set = point_gen::set_d((0.0, 0.0), 10.0, 250, 200, SEED_D);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_d_2500(b: &mut test::Bencher) {
        let set = point_gen::set_d((0.0, 0.0), 10.0, 2500, 2000, SEED_D);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }

    #[bench]
    fn jarvis_benchmark_d_25000(b: &mut test::Bencher) {
        let set = point_gen::set_d((0.0, 0.0), 10.0, 25000, 20000, SEED_D);
        b.iter(|| {
            black_box(jarvis_bench(set.clone()));
        });
    }
}