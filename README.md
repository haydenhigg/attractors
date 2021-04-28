# attractors

Attractors in Rust. See [Paul Bourke's site on Peter de Jong attractors](http://paulbourke.net/fractals/peterdejong/) for an explanation. All code is commented. This implementation is tested on \*nix only.

## compilation

```zsh
rustc -C opt-level=3 main.rs
```

## color functions

Three color functions are provided:

- `color::amber`: monochromatic shades of amber on a white background
- `color::sparks`: monochromatic shades of orange on a black background
- `color::heat`: heat-map-inspired; black->blue->green->yellow->orange

The fourth argument to `make_image_data` in the `file` module takes a color function. Any function that fits fn(u8) -> [u8; 3] works (takes a density from 0-255 and returns [r, g, b]).

## examples

*a = 2.01, b = 2.53, c = 1.61, d = -0.33*

<img src="https://raw.githubusercontent.com/haydenhigg/attractors/main/examples/1.jpg" alt="1" width="50%"/>

*a = -2.0, b = -2.0, c = -1.2, d = 2.0*

<img src="https://raw.githubusercontent.com/haydenhigg/attractors/main/examples/2.jpg" alt="2" width="50%"/>

*a = 0.56, b = -5.6, c = -1.9, d = 2.0*

<img src="https://raw.githubusercontent.com/haydenhigg/attractors/main/examples/3.jpg" alt="3" width="50%"/>

## to-do

- [ ] allow for writing to more than just PPM
