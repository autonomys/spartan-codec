<div align="center">
  <h1><code>spartan</code></h1>
  <strong>A proof-of-concept encoder for the <a href="https://subspace.network/">Subspace Network Blockchain</a> based on the <a href="https://eprint.iacr.org/2015/366">SLOTH permutation</a></strong>
</div>

## Overview

**Notes:** The code is un-audited and not production ready, use it at your own risk.

Subspace is a proof-of-storage blockchain that resolves the farmer's dilemma, to learn more read our <a href="https://drive.google.com/file/d/1v847u_XeVf0SBz7Y7LEMXi72QfqirstL/view">whitepaper</a>. 

This is an adpation of <a href="https://eprint.iacr.org/2015/366">SLOTH</a> (slow-timed hash function) into a time-asymmetric permutation using a standard CBC block cipher. This code is largely based on the C implementation used in <a href="https://github.com/randomchain/pysloth/blob/master/sloth.c">PySloth</a> which is the same as used in the paper.

### Install

If you have not previously installed the `gmp_mpfr_sys` crate, follow these [instructions](https://docs.rs/gmp-mpfr-sys/1.3.0/gmp_mpfr_sys/index.html#building-on-gnulinux).

```
git clone https://github.com/subspace/spartan.git
cd spartan
cargo build --release
```

### Run Tests

`cargo test`

### Run Benches

`cargo bench`

Benches single block encode/decode time and full piece encode/decode time for each prime size.
