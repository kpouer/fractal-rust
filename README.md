# fractal-rust

A small project rendering https://en.wikipedia.org/wiki/Mandelbrot_set in Rust using speedy2d.
It uses speedy2d for display management and rayon to use CPU parallelism.

## Usage

Just run the app

```
Press 'w' to reduce the number of iterations
Press 'x' to increase the number of iterations
Press 'c' to zoom out
Press 'v' to zoom in 
```

## Licence

The project is released under MIT licence

## Dependencies

This project depends on
- speedy2d (https://crates.io/crates/speedy2d) for the display rendering
- hsv (https://crates.io/crates/hsv) for the color conversion
- rayon (https://crates.io/crates/rayon) for parallelism