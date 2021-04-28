mod attractor;
mod colors;
mod file;

const ITERS: usize	= 100000000;
const WIDTH: usize	= 6000;
const HEIGHT: usize	= 6000;
const OUTFILE: &str	= "out.ppm";

fn main() {
	// 2.01, 2.53, 1.61, -0.33
	// -2.0, -2.0, -1.2, 2.0
	// 0.56, -5.6, -1.9, 2.0

	let points = attractor::generate(ITERS, -2.0, -2.0, -1.2, 2.0);
	let histogram = attractor::make_histogram(WIDTH, HEIGHT, points);
	let image_data = file::make_image_data(WIDTH, HEIGHT, histogram, colors::amber);

	if let Ok(()) = file::write_to_ppm(WIDTH, HEIGHT, image_data, OUTFILE) {
		println!("success");
	}
}