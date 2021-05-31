<div align="center">
  <h1><code>spartan-sloth</code></h1>
  <strong>A proof-of-concept encoder/decoder for the <a href="https://subspace.network/">Subspace Network Blockchain</a> based on the <a href="https://eprint.iacr.org/2015/366">SLOTH permutation</a></strong>
</div>

[![CI](https://github.com/subspace/spartan-codec/actions/workflows/ci.yaml/badge.svg)](https://github.com/subspace/spartan-codec/actions/workflows/ci.yaml)
[![Crates.io](https://img.shields.io/crates/v/spartan-sloth?style=flat-square)](https://crates.io/crates/spartan-sloth)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/spartan-sloth)
[![License](https://img.shields.io/github/license/subspace/spartan-codec?style=flat-square)](https://github.com/subspace/spartan-codec)

## Overview

This is an adaptation of [SLOTH](https://eprint.iacr.org/2015/366) (slow-timed hash function) into a time-asymmetric permutation using a standard CBC block cipher. This code is largely based on the C implementation used in [PySloth](https://github.com/randomchain/pysloth/blob/master/sloth.c) which is the same as used in the paper.

### Install
This crate requires Rust 1.51 or newer to compile.

Add following to `Cargo.toml`:
```
spartan-sloth = "0.1.0"
```

NOTE: Software implementation uses `rug` (`GMP`) library, if you want to use it (`software` feature, enabled by default), follow these [instructions](https://docs.rs/gmp-mpfr-sys/1.3.0/gmp_mpfr_sys/index.html#building-on-gnulinux) that are necessary for `gmp_mpfr_sys` crate used internally.

### Run Tests

```
cargo test
```

### Run Benches

```
cargo bench
```

To skip software-based SLOTH implementation (hard to setup on Windows) and just test x86-64:
```
cd spartan-sloth
cargo bench --no-default-features --bench x86_64
```

Benches single block encoding, parallel encoding (will depend on number of cores) and decoding time for a prime size of 256 bits.

### Software benchmark results

#### AMD 5900x CPU / 3600MHz CL16 RAM
System config:
* CPU: AMD 5900x with XFR enabled (12 cores, 24 threads)
* RAM: 128G (4x32) Crucial Ballistix BL2K32G36C16U4B (3600MHz CL16)

Results:
* Software/Encode-single: 1.4605 ms
* Software/Encode-parallel: 104.68 us
* Software/Decode: 38.711 us
