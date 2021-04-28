#[derive(Copy, Clone)]
pub struct Point {
	x: f64,
	y: f64
}

pub fn generate(iters: usize, a: f64, b: f64, c: f64, d: f64) -> Vec<Point> {
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

pub fn make_histogram(w: usize, h: usize, points: Vec<Point>) -> Vec<Vec<u8>> {
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