use std::cmp;
use std::fs::File;
use std::io::Write;

#[derive(Copy, Clone)]
struct Point {
	x: f64,
	y: f64
}

fn amber(density: u8) -> [u8; 3] {
	[
		cmp::max(255 - (density as i16) * 4, 0) as u8,
		cmp::max(255 - (density as i16) * 8, 0) as u8,
		cmp::max(255 - (density as i16) * 20, 0) as u8
	]
}

fn generate(iters: usize, a: f64, b: f64, c: f64, d: f64) -> Vec<Point> {
	let mut points = Vec::with_capacity(iters);

	points.push(Point {x: 0.0, y: 0.0});

	let mut p: Point = points[0];
	
	for i in 1..iters {
		points.push(Point {
			x: (a * p.y).sin() - (b * p.x).cos(),
			y: (c * p.x).sin() - (d * p.y).cos()
		});

		p = points[i];
	}

	points
}

fn make_histogram(w: usize, h: usize, points: Vec<Point>) -> Vec<Vec<u8>> {
	//-- find top left and bottom right --//
	let mut min = Point {x: f64::MAX, y: f64::MAX};
	let mut max = Point {x: f64::MIN, y: f64::MIN};

	for p in &points {
		if p.x < min.x {
			min.x = p.x
		} else if p.x > max.x {
			max.x = p.x
		}

		if p.y < min.y {
			min.y = p.y
		} else if p.y > max.y {
			max.y = p.y
		}
	}

	//-- so that the math is easier when scaling --//
	let width_scaling = (w as f64 - 1.0) / (max.x - min.x);
	let height_scaling = (h as f64 - 1.0) / (max.y - min.y);

	let offset_x = min.x * width_scaling;
	let offset_y = min.y * height_scaling;

	//-- initialize histogram counts to 0 --//
	let mut histogram: Vec<Vec<u8>> = Vec::with_capacity(h);

	for i in 0..h {
		histogram.push(Vec::with_capacity(w));

		for _j in 0..w {
			histogram[i].push(0u8);
		}
	}

	//-- re-adjusts all points in `attractor` to  --//
	//-- fill a box with width `w` and height `h` --//
	//-- and writes the values in that box to the --//
	//-- matrix `histogram`                       --//
	let mut x: usize;
	let mut y: usize;

	for p in &points {
		x = (p.x * width_scaling - offset_x).floor() as usize;
		y = (p.y * height_scaling - offset_y).floor() as usize;

		if histogram[y][x] < 255u8 {
			histogram[y][x] += 1;
		}
	}

	histogram
}

fn make_image_data(w: usize, h: usize, histogram: Vec<Vec<u8>>) -> Vec<u8> {
	//-- find lowest and highest densities --//
	let mut min = u8::MAX;
	let mut max = 0u8;

	for row in &histogram {
		for &point in row {
			if point < min {
				min = point;
			} else if point > max {
				max = point;
			}
		}
	}

	//-- so that the math is easier when scaling --//
	let scaling = u8::MAX as f64 / (max - min) as f64;
	let offset = min as f64 * scaling;

	//-- create 1D vector of image bytes --//
	let mut image_data: Vec<u8> = Vec::with_capacity(w * h * 3);

	for i in 0..h {
		for j in 0..w {
			image_data.extend(amber((histogram[i][j] as f64 * scaling - offset).floor() as u8).iter());
		}
	}

	image_data
}

fn write_to_ppm(w: usize, h: usize, image_data: Vec<u8>, file_name: &'static str) -> std::io::Result<()> {
	let mut file = File::create(file_name)?;

	writeln!(file, "P6 {} {} {}", w, h, u8::MAX)?;
	file.write_all(&image_data)?;

	Ok(())
}

const ITERS: usize = 50000000;
const WIDTH: usize = 4000;
const HEIGHT: usize = 4000;

fn main() {
	let points = generate(ITERS, 2.01, 2.53, 1.61, -0.33);
	let histogram = make_histogram(WIDTH, HEIGHT, points);
	let image_data = make_image_data(WIDTH, HEIGHT, histogram);

	if let Ok(()) = write_to_ppm(WIDTH, HEIGHT, image_data, "out.ppm") {
		println!("success");
	}
}