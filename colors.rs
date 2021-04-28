use std::cmp;

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
	let d4 = cmp::min((density as u16) * 4, 255) as u8;
	let d12 = cmp::min((density as u16) * 12, 255) as u8;
	let d22 = cmp::min((density as u16) * 22, 255) as u8;

	if density < 7 {
		[d4, 0, d22]
	} else if density < 12 {
		[0, d12, 255 - d22]
	} else if density < 22 {
		[d12, d12, 0]
	} else if density < 190 {
		[d22, 255 - d4, 0]
	} else {
		[density, density, density]
	}
}
