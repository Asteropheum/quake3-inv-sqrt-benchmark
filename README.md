# quake3-inv-sqrt-benchmark (0x5F3759DF)
Benchmarking std::f32 inverse square root and Quake III Arena inverse square root.

[Quake III Arena Q_rsqrt source code](https://github.com/id-Software/Quake-III-Arena/blob/dbe4ddb10315479fc00086f08e25d968b4b43c49/code/game/q_math.c#L552)

[Wikipedia](https://en.wikipedia.org/wiki/Fast_inverse_square_root)

## Environment

CPU: Intel i9-9880H (16) @ 2.30GHz

rustc 1.53.0 (53cb7b09b 2021-06-17)


## Run
```console
cargo bench
```

## Results
Benchmarked on loop from 0 to LIMIT.
```
const LIMIT: i32 = 1000000;
```

The numbers are expressed in US notation.

| Name               | Median            | Deviation        |
| ------------------ | ----------------- | ---------------- |
| tests::bench_quake | 936,146 ns/iter   | 93,941 ns/iter   |
| tests::bench_std   | 1,352,039 ns/iter | 127,779 ns/iter  |