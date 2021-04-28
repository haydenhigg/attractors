use std::fs::File;
use std::io::{Result, Write};

pub fn make_image_data(w: usize, h: usize, histogram: Vec<Vec<u8>>, color: fn(u8) -> [u8; 3]) -> Vec<u8> {
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
	let scaling = u8::MAX as f64 / (max - min) as f64; // ! up to u8::MAX, not 1 !
	let offset = min as f64 * scaling;

	//-- create 1D vector of image bytes --//
	let mut image_data: Vec<u8> = Vec::with_capacity(w * h * 3);

	for i in 0..h {
		for j in 0..w {
			image_data.extend_from_slice(
				&color(
					(histogram[i][j] as f64 * scaling - offset).floor() as u8
				)
			);
		}
	}

	image_data
}

pub fn write_to_ppm(w: usize, h: usize, image_data: Vec<u8>, file_name: &'static str) -> Result<()> {
	let mut file = File::create(file_name)?;

	writeln!(file, "P6 {} {} {}", w, h, u8::MAX)?; // header
	file.write_all(&image_data)?;                  // data   | 1 call is seemingly the most efficient way

	Ok(())
}
