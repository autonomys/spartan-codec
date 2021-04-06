<div align="center">
  <h1><code>spartan-codec</code></h1>
  <strong>A proof-of-concept encoder for the <a href="https://subspace.network/">Subspace Network Blockchain</a> based on the <a href="https://eprint.iacr.org/2015/366">SLOTH permutation</a></strong>
</div>

[![CI](https://github.com/subspace/spartan-codec/actions/workflows/ci.yaml/badge.svg)](https://github.com/subspace/spartan-codec/actions/workflows/ci.yaml)
[![Crates.io](https://img.shields.io/crates/v/spartan-codec?style=flat-square)](https://crates.io/crates/spartan-codec)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/spartan-codec)
[![License](https://img.shields.io/github/license/subspace/spartan-codec?style=flat-square)](https://github.com/subspace/spartan-codec)

## Overview

**Notes:** The code is un-audited and not production ready, use it at your own risk.

Subspace is a proof-of-storage blockchain that resolves the farmer's dilemma, to learn more read our <a href="https://drive.google.com/file/d/1v847u_XeVf0SBz7Y7LEMXi72QfqirstL/view">whitepaper</a>. 

This is an adaptation of [SLOTH](https://eprint.iacr.org/2015/366) (slow-timed hash function) into a time-asymmetric permutation using a standard CBC block cipher. This code is largely based on the C implementation used in [PySloth](https://github.com/randomchain/pysloth/blob/master/sloth.c) which is the same as used in the paper.

### Install
This crate requires Rust 1.51 or newer to compile.

If you have not previously installed the `gmp_mpfr_sys` crate, follow these [instructions](https://docs.rs/gmp-mpfr-sys/1.3.0/gmp_mpfr_sys/index.html#building-on-gnulinux).

```
git clone https://github.com/subspace/spartan-codec.git
cd spartan-codec
cargo build
```

### Run Tests

`cargo test`

### Run Benches

TODO

Benches single block encode/decode time and full piece encode/decode time for each prime size.
