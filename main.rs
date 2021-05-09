mod attractor;
mod colors;
mod file;

const ITERS: usize	= 140_000_000;
const WIDTH: usize	= 4_000;
const HEIGHT: usize	= 4_000;
const OUTFILE: &str	= "out.ppm";

fn main() {
	// 2.01, 2.53, 1.61, -0.33             fish
	// -2.0, -2.0, -1.2, 2.0               cornucopia
	// 0.56, -5.6, -1.9, 2.0               whirlpool
	// 1.78125, -0.78125, 1.90625, 2.65625 jellyfish
	// -2.487, -1.888, 1.233, 3.862	       shell

	let points = attractor::generate(ITERS, -2.487, -1.888, 1.233, 3.862);
	let histogram = attractor::make_histogram(WIDTH, HEIGHT, points);
	let image_data = file::make_image_data(WIDTH, HEIGHT, histogram, colors::viridis);

	if let Ok(()) = file::write_to_ppm(WIDTH, HEIGHT, image_data, OUTFILE) {
		println!("success");
	}
}
