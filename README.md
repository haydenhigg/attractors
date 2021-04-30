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
- `color::heat`: heat-map-inspired; black->blue->cyan->green->yellow->red->white
- `colors::viridis`: accurate recreation of ColorBrewer's viridis
- `colors::blue_purple`: black->blue->purple

The fourth argument to `make_image_data` in the `file` module takes a color function. Any function that fits
```rust
fn(u8) -> [u8; 3]
```
works (takes a density from 0-255 and returns [r, g, b]).

---

To see a test image for a color function, you can use the function
```rust
file::test_color(color: fn(u8) -> [u8; 3], file_name: &'static str) -> Result<()>
```
and get an image like this (for `viridis`):

![sample for viridis](https://raw.githubusercontent.com/haydenhigg/attractors/main/examples/viridis.jpg)

## examples

<span>
<img src="https://raw.githubusercontent.com/haydenhigg/attractors/main/examples/1.jpg" width="30%"/>
<img src="https://raw.githubusercontent.com/haydenhigg/attractors/main/examples/2.jpg" width="30%"/>
<img src="https://raw.githubusercontent.com/haydenhigg/attractors/main/examples/3.jpg" width="30%"/>
</span>

## to-do

- [ ] add new attractor functions
- [ ] allow for writing to more than just PPM
