extern crate image;
use image::ImageBuffer;
use image::Rgb;

extern crate num;
use num::Complex;

use std::f64::consts::PI;

fn normalize(pos: u32, max: u32) -> f64 {
    pos as f64 * 2.0 / max as f64 - 1.0
}

fn mandel_escape(c: Complex<f64>, max_iters: u64, pow: f64) -> Vec<Complex<f64>> {
    let mut z = Complex::new(0.0, 0.0);
    let mut out = vec![];
    for _ in 0..max_iters {
        if z.norm_sqr() >= 9.0 {
            break;
        }
        z = z.powf(pow) + c;
        out.push(z);
    }
    out
}

fn make_mandel(h: u32, w: u32, size: u32, samples: u64, max_iters: u64) -> Rgb<u8> {
    let im = normalize(h, size);
    let re = normalize(w, size) - 0.4;
    let c = Complex::new(re, im);
    let distance: f64 = 1.5;
    let samples: i64 = samples as i64;
    let mut overall_color = (0.0, 0.0, 0.0);
    for i in -samples..=samples {
        let pow = 2.0 * distance.powf(i as f64 / samples as f64);
        let points = mandel_escape(c, max_iters, pow);
        let norm_arg = {
            let value = points[0] - points[points.len() - 1];
            let arg = value.arg() + PI;
            arg / (PI * 2.0 / 3.0)
        };
        assert!(-1e-5 < norm_arg && 3.0 + 1e-5 > norm_arg);
        let color = if norm_arg < 1.0 {
            (1.0 - norm_arg, norm_arg, 0.0)
        } else if norm_arg < 2.0 {
            (0.0, 2.0 - norm_arg, norm_arg - 1.0)
        } else {
            (norm_arg - 2.0, 0.0, 3.0 - norm_arg)
        };
        let intensity = points.len() as f64 / max_iters as f64;
        let color_sum = color.0.powi(3) + color.1.powi(3) + color.2.powi(3);
        overall_color.0 += color.0 * intensity / color_sum;
        overall_color.1 += color.1 * intensity / color_sum;
        overall_color.2 += color.2 * intensity / color_sum;
    }
    overall_color.0 /= (2 * samples + 1) as f64;
    overall_color.1 /= (2 * samples + 1) as f64;
    overall_color.2 /= (2 * samples + 1) as f64;

    while overall_color.0 > 1.0 ||overall_color.1 > 1.0 ||overall_color.2 > 1.0 {
        overall_color.0 /= 2.0;
        overall_color.1 /= 2.0;
        overall_color.2 /= 2.0;
    }

    Rgb([
        ((overall_color.0 * 255.0) as u64).min(255) as u8,
        ((overall_color.1 * 255.0) as u64).min(255) as u8,
        ((overall_color.2 * 255.0) as u64).min(255) as u8,
    ])
}
fn main() {
    let max_iters = 100;
    assert!(std::env::args().count() >= 3);
    let size = std::env::args().nth(1).unwrap().parse().unwrap();
    let samples = std::env::args().nth(2).unwrap().parse().unwrap();
    let make_mandel = |h, w| make_mandel(h, w, size, samples, max_iters);
    let buf = ImageBuffer::from_fn(size, size, &make_mandel);
    buf.save(format!("mandel-{}-{}.png", size, samples)).expect("Saved successfully");
}
