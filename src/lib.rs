#![feature(test)]

extern crate test;

union FI {
    f: f32,
    i: i32,
}

// Quake III Arena fast inverse square root
fn quake_rsqrt(x: f32) -> f32 {
    let mut u = FI { f: x };
    unsafe {
        u.i = 0x5f3759df - (u.i >> 1);
        u.f * (1.5 - 0.5 * x * u.f * u.f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use test::black_box;

    const LIMIT: i32 = 1000000;

    #[bench]
    fn bench_quake(b: &mut Bencher) {
        b.iter(|| {
            for x in 0..LIMIT {
                black_box(quake_rsqrt(x as f32));
            }
        });
    }

    #[bench]
    fn bench_std(b: &mut Bencher) {
        b.iter(|| {
            for x in 0..LIMIT {
                black_box((x as f32).sqrt().recip());
            }
        });
    }
}
