use std::cmp;

#[allow(dead_code)]
pub fn amber(density: u8) -> [u8; 3] {
	[
		cmp::max(255 - (density as i16) * 4, 0) as u8,
		cmp::max(255 - (density as i16) * 8, 0) as u8,
		cmp::max(255 - (density as i16) * 20, 0) as u8
	]
}

#[allow(dead_code)]
pub fn sparks(density: u8) -> [u8; 3] {
	[
		cmp::min((density as i16) * 18, 255) as u8,
		cmp::min((density as i16) * 10, 255) as u8,
		cmp::min((density as i16) * 4, 255) as u8
	]
}

#[allow(dead_code)]
pub fn heat(density: u8) -> [u8; 3] {
	let d1 = cmp::min((density as u16) * 4, 255) as u8;
	let d2 = cmp::min((density as u16) * 10, 255) as u8;
	let d3 = cmp::min((density as u16) * 18, 255) as u8;

	if density < 7 {
		[d1, 0, d3]
	} else if density < 12 {
		[0, d2, d3]
	} else if density < 24 {
		[d1, d2, 0]
	} else {
		[d3, d1, 0]
	}
}