use plotters::prelude::*;
use rand::{
    SeedableRng,
    distr::{Distribution, Uniform},
    rngs::SmallRng,
};
use std::ops::Range;

type Float = f64;
const PI: Float = std::f64::consts::PI;
const A: (Float, Float) = (-1.0, 0.0);
const B: (Float, Float) = (1.0, 0.1);
const EPSILON: Float = 1e-16;

const SEED_X_1: u64 = 117;
const SEED_Y_1: u64 = 2139;

const SEED_X_2: u64 = 2121;
const SEED_Y_2: u64 = 1283;

const SEED_R_3: u64 = 9214;

const SEED_X_4: u64 = 1321;


fn main() {
    { // zbiór 1
        use problemset1::*;
        let range = -1000.0..1000.0;
        let points = new_points(range.clone(), 100000, SEED_X_1, SEED_Y_1);
        draw_unprocessed("plots/1_up.png", points.clone(), -1200.0..1200.0).unwrap();
        let (l, r, colin) = draw_processed("plots/1_p.png", points, -1200.0..1200.0, EPSILON, det_3x3).unwrap();
        println!("SET 1");
        println!("Points on the left: {}", l);
        println!("Points on the right: {}", r);
        println!("Collinear points: {}", colin);
    }

    { // zbiór 2
        use problemset2::*;
        let range = -1e14..1e14;
        let points = new_points(range.clone(), 100000, SEED_X_2, SEED_Y_2);
        draw_unprocessed("plots/2_up.png", points.clone(), -1e14-2e13..1e14+2e13).unwrap();
        let (l, r, colin) = draw_processed("plots/2_p.png", points, -1e14-2e13..1e14+2e13, EPSILON, det_3x3).unwrap();
        println!("SET 2");
        println!("Points on the left: {}", l);
        println!("Points on the right: {}", r);
        println!("Collinear points: {}", colin);
    }

    { // zbiór 3
        use problemset3::*;
        let r = 100.0;
        let points = new_points_circle(r, 1000, SEED_R_3);
        draw_unprocessed("plots/3_up.png", points.clone(), -120.0..120.0).unwrap();
        let (l, r, colin) = draw_processed("plots/3_p.png", points, -120.0..120.0, EPSILON, det_3x3).unwrap();
        println!("SET 3");
        println!("Points on the left: {}", l);
        println!("Points on the right: {}", r);
        println!("Collinear points: {}", colin);
    }

    { // zbiór 4
        use problemset4::*;
        let range= -1000.0..1000.0;
        let points = new_points_line(range, 1000, SEED_X_4);
        draw_unprocessed("plots/4_up.png", points.clone(), -1200.0..1200.0).unwrap();
        let (l, r, colin) = draw_processed("plots/4_p.png", points, -1200.0..1200.0, EPSILON, det_3x3).unwrap();
        println!("SET 4");
        println!("Points on the left: {}", l);
        println!("Points on the right: {}", r);
        println!("Collinear points: {}", colin);
    }
}

fn det_3x3(a: (Float, Float), b: (Float, Float), c: (Float, Float)) -> Float {
    a.0 * b.1 + b.0 * c.1 + c.0 * a.1 - c.0 * b.1 - a.1 * b.0 - a.0 * c.1
}

fn det_2x2(a: (Float, Float), b: (Float, Float), c: (Float, Float)) -> Float {
    //(a.0 - c.0) * (b.1 - c.1) - (b.0 - c.0) * (a.1 - c.1)
    let x = a.0 - c.0;
    let y = b.1 -c.1;
    let z = b.0 - c.0;
    let w = a.1 - c.1;
    let xy = x * y;
    let zw = z * w;
    return xy - zw;
}

fn eq_float(a: Float, b: Float, epsilon: Float) -> bool {
    (a - b).abs() <= epsilon
}

mod problemset1 {
    use super::*;
    pub fn draw_unprocessed(filename: &str, points: Vec<(Float, Float)>, label_range: Range<Float>) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (512, 512)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let areas = root.split_by_breakpoints([488], [24]);

        let mut ctx = ChartBuilder::on(&areas[2])
            .x_label_area_size(80)
            .y_label_area_size(80)
            .build_cartesian_2d(label_range.clone(), label_range)?;
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

    pub fn draw_processed(filename: &str, points: Vec<(Float, Float)>, label_range: Range<Float>, epsilon: Float, det: fn((Float, Float), (Float, Float), (Float, Float)) -> Float) -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (512, 512)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let areas = root.split_by_breakpoints([488], [24]);

        let mut ctx = ChartBuilder::on(&areas[2])
            .x_label_area_size(80)
            .y_label_area_size(80)
            .build_cartesian_2d(label_range.clone(), label_range)?;
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

        let mut on_left = Vec::new();
        let mut on_right = Vec::new();
        let mut collinear = Vec::new();
        for point in points {
            let determinant = det(A, B, point);
            if eq_float(determinant, 0.0, epsilon) {
                collinear.push(point);
            } else if determinant < 0.0 {
                on_right.push(point);
            } else if determinant > 0.0 {
                on_left.push(point);
            }
        }

        ctx.draw_series(
            on_left.clone().iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, GREEN.filled())),
        )?;
        ctx.draw_series(
            on_right.clone().iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, RED.filled())),
        )?;
        ctx.draw_series(
            collinear.clone().iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, BLUE.filled())),
        )?;
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        Ok((on_left.len(), on_right.len(), collinear.len()))
    }

    pub fn new_points(range: std::ops::Range<Float>, n: usize, seed_x: u64, seed_y: u64) -> Vec<(Float, Float)> {
        let uni_dist = Uniform::new_inclusive(range.start, range.end).unwrap();
        let mut x_rand = SmallRng::seed_from_u64(seed_x);
        let mut y_rand = SmallRng::seed_from_u64(seed_y);
        let x_iter = uni_dist.sample_iter(&mut x_rand);
        let y_iter = uni_dist.sample_iter(&mut y_rand);
        x_iter.zip(y_iter).take(n).collect()
    }
}

mod problemset2 {
    use super::*;
    pub fn draw_unprocessed(filename: &str, points: Vec<(Float, Float)>, label_range: Range<Float>) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (512, 512)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let areas = root.split_by_breakpoints([488], [24]);

        let mut ctx = ChartBuilder::on(&areas[2])
            .x_label_area_size(80)
            .y_label_area_size(80)
            .build_cartesian_2d(label_range.clone(), label_range)?;
        ctx
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_desc("x")
            .y_desc("y")
            .x_labels(5)
            .y_labels(5)
            .x_label_formatter(&|x| format!("{:e}", &x))
            .y_label_formatter(&|x| format!("{:e}", &x))
            .label_style(("sans-serif", 22).into_font())
            .draw()?;

        ctx.draw_series(
            points.iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, BLACK.filled())),
        )?;
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        Ok(())
    }

    pub fn draw_processed(filename: &str, points: Vec<(Float, Float)>, label_range: Range<Float>, epsilon: Float, det: fn((Float, Float), (Float, Float), (Float, Float)) -> Float) -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (512, 512)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let areas = root.split_by_breakpoints([488], [24]);

        let mut ctx = ChartBuilder::on(&areas[2])
            .x_label_area_size(80)
            .y_label_area_size(80)
            .build_cartesian_2d(label_range.clone(), label_range)?;
        ctx
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_desc("x")
            .y_desc("y")
            .x_labels(5)
            .y_labels(5)
            .x_label_formatter(&|x| format!("{:e}", &x))
            .y_label_formatter(&|x| format!("{:e}", &x))
            .label_style(("sans-serif", 22).into_font())
            .draw()?;

        let A_off = (A.0 * 1e10, A.1 * 1e10);
        let B_off = (B.0 * 1e10, B.1 * 1e10);
        let mut on_left = Vec::new();
        let mut on_right = Vec::new();
        let mut collinear = Vec::new();
        for point in points {
            let determinant = det(A_off, B_off, point);
            if eq_float(determinant, 0.0, epsilon) {
                collinear.push(point);
            } else if determinant < 0.0 {
                on_right.push(point);
            } else if determinant > 0.0 {
                on_left.push(point);
            }
        }

        ctx.draw_series(
            on_left.clone().iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, GREEN.filled())),
        )?;
        ctx.draw_series(
            on_right.clone().iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, RED.filled())),
        )?;
        ctx.draw_series(
            collinear.clone().iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, BLUE.filled())),
        )?;
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        Ok((on_left.len(), on_right.len(), collinear.len()))
    }

    pub fn new_points(range: std::ops::Range<Float>, n: usize, seed_x: u64, seed_y: u64) -> Vec<(Float, Float)> {
        let uni_dist = Uniform::new_inclusive(range.start, range.end).unwrap();
        let mut x_rand = SmallRng::seed_from_u64(seed_x);
        let mut y_rand = SmallRng::seed_from_u64(seed_y);
        let x_iter = uni_dist.sample_iter(&mut x_rand);
        let y_iter = uni_dist.sample_iter(&mut y_rand);
        x_iter.zip(y_iter).take(n).collect()
    }
}

mod problemset3 {
    use super::*;
    pub fn draw_unprocessed(filename: &str, points: Vec<(Float, Float)>, label_range: Range<Float>) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (512, 512)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let areas = root.split_by_breakpoints([488], [24]);

        let mut ctx = ChartBuilder::on(&areas[2])
            .x_label_area_size(80)
            .y_label_area_size(80)
            .build_cartesian_2d(label_range.clone(), label_range)?;
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

    pub fn draw_processed(filename: &str, points: Vec<(Float, Float)>, label_range: Range<Float>, epsilon: Float, det: fn((Float, Float), (Float, Float), (Float, Float)) -> Float) -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (512, 512)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let areas = root.split_by_breakpoints([488], [24]);

        let mut ctx = ChartBuilder::on(&areas[2])
            .x_label_area_size(80)
            .y_label_area_size(80)
            .build_cartesian_2d(label_range.clone(), label_range)?;
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

        let mut on_left = Vec::new();
        let mut on_right = Vec::new();
        let mut collinear = Vec::new();
        for point in points {
            let determinant = det(A, B, point);
            if eq_float(determinant, 0.0, epsilon) {
                collinear.push(point);
            } else if determinant < 0.0 {
                on_right.push(point);
            } else if determinant > 0.0 {
                on_left.push(point);
            }
        }

        ctx.draw_series(
            on_left.clone().iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, GREEN.filled())),
        )?;
        ctx.draw_series(
            on_right.clone().iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, RED.filled())),
        )?;
        ctx.draw_series(
            collinear.clone().iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, BLUE.filled())),
        )?;
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        Ok((on_left.len(), on_right.len(), collinear.len()))
    }

    pub fn new_points_circle(r: Float, n: usize, seed_r: u64) -> Vec<(Float, Float)> {
        let uni_dist = Uniform::new_inclusive(0.0, 2.0 * PI).unwrap();
        let mut r_rand = SmallRng::seed_from_u64(seed_r);
        let r_iter = uni_dist.sample_iter(&mut r_rand);
        r_iter.map(|theta| (r * theta.cos(), r * theta.sin())).take(n).collect()
    }
}

mod problemset4 {
    use super::*;
    pub fn draw_unprocessed(filename: &str, points: Vec<(Float, Float)>, label_range: Range<Float>) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (512, 512)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let areas = root.split_by_breakpoints([488], [24]);
        let y_spec : Range<Float> = -70.0..70.0;

        let mut ctx = ChartBuilder::on(&areas[2])
            .x_label_area_size(80)
            .y_label_area_size(80)
            .build_cartesian_2d(label_range.clone(), y_spec)?;
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

    pub fn draw_processed(filename: &str, points: Vec<(Float, Float)>, label_range: Range<Float>, epsilon: Float, det: fn((Float, Float), (Float, Float), (Float, Float)) -> Float) -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (512, 512)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let areas = root.split_by_breakpoints([488], [24]);
        let y_spec : Range<Float> = -70.0..70.0;

        let mut ctx = ChartBuilder::on(&areas[2])
            .x_label_area_size(80)
            .y_label_area_size(80)
            .build_cartesian_2d(label_range.clone(), y_spec)?;
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

        let mut on_left = Vec::new();
        let mut on_right = Vec::new();
        let mut collinear = Vec::new();
        for point in points {
            let determinant = det(A, B, point);
            if eq_float(determinant, 0.0, epsilon) {
                collinear.push(point);
            } else if determinant < 0.0 {
                on_right.push(point);
            } else if determinant > 0.0 {
                on_left.push(point);
            }
        }
        
        ctx.draw_series(
            on_left.clone().iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, GREEN.filled())),
        )?;
        ctx.draw_series(
            on_right.clone().iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, RED.filled())),
        )?;
        ctx.draw_series(
            collinear.clone().iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, BLUE.mix(0.5).filled())),
        )?;
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        Ok((on_left.len(), on_right.len(), collinear.len()))
    }

    pub fn new_points_line(range: Range<Float>, n: usize, seed_x: u64) -> Vec<(Float, Float)> {
        let uni_dist = Uniform::new_inclusive(range.start, range.end).unwrap();
        let mut r_rand = SmallRng::seed_from_u64(seed_x);
        let r_iter = uni_dist.sample_iter(&mut r_rand);
        r_iter.map(|x| (x, line(x))).take(n).collect()
    }

    fn line(x: Float) -> Float {
        0.05 * x + 0.05
    }
}


