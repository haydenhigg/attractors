use std::cmp;

fn log_scale(d: u8) -> f32 {
	((d as f32) + 1.0).log10() * 105.88645
}

#[allow(dead_code)]
pub fn amber(density: u8) -> [u8; 3] {
	[
		255 - cmp::min((density as u16) * 4, 255) as u8,
		255 - cmp::min((density as u16) * 8, 255) as u8,
		255 - cmp::min((density as u16) * 20, 255) as u8
	]
}

#[allow(dead_code)]
pub fn sparks(density: u8) -> [u8; 3] {
	[
		cmp::min((density as u16) * 18, 255) as u8,
		cmp::min((density as u16) * 10, 255) as u8,
		cmp::min((density as u16) * 4, 255) as u8
	]
}

#[allow(dead_code)]
pub fn heat(density: u8) -> [u8; 3] {
	let fld = log_scale(density);

	let a = (fld * 6.0)           as u8;
	let b = ((fld - 42.5) * 6.0)  as u8;
	let c = ((fld - 85.0) * 6.0)  as u8;
	let d = ((fld - 127.5) * 6.0) as u8;
	let e = ((fld - 170.0) * 6.0) as u8;
	let f = ((fld - 212.5) * 6.0) as u8;

	if fld < 42.5 {         // black->blue
		[0, 0, a]
	} else if fld < 85.0 {  // blue->cyan
		[0, b, 255]
	} else if fld < 127.5 { // cyan->green
		[0, 255, 255 - c]
	} else if fld < 170.0 { // green->yellow
		[d, 255, 0]
	} else if fld < 212.5 { // yellow->red
		[255, 255 - e, 0]
	} else {                // red->white
		[255, f, f]
	}
}

#[allow(dead_code)]
pub fn viridis(density: u8) -> [u8; 3] {
	let fld = log_scale(density);

	let r = if fld <= 25.5 {
		((fld * 0.15686).floor() + 68.0) as u8
	} else if fld < 38.25 {
		72 as u8
	} else if fld < 140.25 {
		(69.0 - (fld - 38.25) * 0.37254) as u8
	} else {
		(32.0 + (fld - 140.25) * 1.94335) as u8
	};

	let g = (fld * 0.90196 + 1.0) as u8;

	let b = if fld < 102.0 {
		((fld * 0.56863).floor() + 84.0) as u8
	} else if fld < 242.25 {
		(142.0 - (fld - 102.0) * 0.83422) as u8
	} else {
		(((fld - 242.25) * 0.94118).floor() + 25.0) as u8
	};
	
	[r, g, b]
}

#[allow(dead_code)]
pub fn blue_purple(density: u8) -> [u8; 3] {
	let fld = log_scale(density);

	if fld <= 127.5 {
		[0, (fld / 4.0) as u8, fld as u8]
	} else {
		let a = fld - 127.5;

		[(a * 1.5) as u8, (31.875 - a / 4.0) as u8, fld as u8]
	}
}
