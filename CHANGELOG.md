# Changelog

All notable changes to `packed_struct` and `packed_struct_codegen` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). The crate is
pre-1.0, so a minor version bump can contain breaking changes.

## [0.12.0] - Unreleased

This is the first release since 0.10.1. The `0.11.0` version number was used on `master` in the
meantime but never published to crates.io, so everything below is relative to **0.10.1**.

The release contains several **silent wire-format fixes**: some struct definitions that compiled
under 0.10.1 packed and unpacked their fields at the wrong bits. They still compile, but the bytes
they produce and accept are now different (and correct). If you store packed data or exchange it
with other systems, read [Wire-format fixes](#wire-format-fixes) before upgrading.

### Upgrade checklist

1. Search your code for exclusive byte ranges, `bytes="x..y"`. Every one of them changes its layout,
   see [below](#exclusive-byte-ranges-bytesxy-were-one-byte-short-107).
2. Check for nested structs, reserved fields or other fields of 32 bytes or more that don't start
   and end on a byte boundary.
3. Fix any new compile errors about field positions and sizes that disagree. Each one points at a
   field whose layout was previously wrong or ambiguous.
4. Upgrade the toolchain to Rust 1.85 or newer.

### Wire-format fixes

These change what `pack()` produces and what `unpack()` expects for the affected structs, with no
compile error or warning.

#### Exclusive byte ranges, `bytes="x..y"`, were one byte short ([#107])

The byte range was converted to bits as `x*8 ..= (y-1)*8 - 1` instead of `x*8 ..= y*8 - 1`, so
`bytes="0..2"` covered only byte 0, and `bytes="1..3"` only byte 1. Bit ranges (`bits="x..y"`) and
inclusive byte ranges (`bytes="x..=y"`) were not affected.

Whenever the type still fit into the shorter range, this compiled and silently truncated the field:

```rust
#[derive(PackedStruct)]
#[packed_struct(endian="msb", bit_numbering="msb0", size_bytes="4")]
pub struct Reg {
    #[packed_field(bytes="0..4")]
    value: u32,
}
```

|                                      | 0.10.1                                       | 0.12.0                     |
|--------------------------------------|----------------------------------------------|----------------------------|
| `Reg { value: 0x12345678 }.pack()`   | `[0x34, 0x56, 0x78, 0x00]`, a 24-bit field   | `[0x12, 0x34, 0x56, 0x78]` |
| `Reg::unpack(&[0x12, 0x34, 0x56, 0x78])` | `value: 0x123456`, the last byte ignored | `value: 0x12345678`        |

The same happened to a `u64` at `bytes="0..8"` (packed as 56 bits), and to array fields, whose
elements shrank to fit: `[u8; 4]` at `bytes="0..4"` was packed as four 6-bit elements into the first
3 bytes. Fields that didn't fit into the shorter range, like a `u16` at `bytes="0..2"`, failed to
compile.

**Who is affected:** every field positioned with `bytes="x..y"`.

**Migration:** no code change is needed to get the intended layout. If you need to keep reading data
packed by 0.10.1, describe the old layout explicitly: `bytes="x..y"` used to cover the bytes
`x..=y-2`, so `bytes="0..4"` on a `u32` becomes `bytes="0..=2"` on an `Integer<u32, Bits<24>>`.

#### Fields of 32 bytes or more at unaligned positions lost bits

For fields that are not byte-aligned (their start bit or width is not a multiple of 8), the derive
builds a per-byte mask from a running bit count that was cast to `u8`. For fields of 32 bytes or
more the count wrapped around, so the mask for every 32nd byte of the field was truncated and some
of its bits were silently dropped, both on `pack()` and on `unpack()`. For example, a 40-byte nested
struct of all ones at bit offset 4 packed with 4 bits of its 32nd byte cleared, and didn't
round-trip.

**Who is affected:** fields that are at least 256 bits wide and don't start and end on a byte
boundary. In practice these are nested `PackedStruct` fields, large `ReservedZero`/`ReservedOne`
fields and similar wide types placed after a bit field. Integer fields are at most 64 bits and are
not affected.

**Migration:** none. Data that went through the old code lost those bits, and there is nothing to
restore.

### Breaking changes

- **Field positions and sizes must agree.** A field whose position covers a different number of
  bits than its declared size is now a compile error, instead of one of them silently winning:

  ```rust
  #[packed_field(bits="0..=7", size_bits="4")]  // error: The field's position covers 8 bits, but its size is 4 bits.
  a: Integer<u8, packed_bits::Bits::<4>>,
  ```

- **Array fields must split evenly into their elements.** An array whose bit width is not a
  multiple of its length is now a compile error. Previously the leftover bits were silently ignored
  (e.g. `[u8; 3]` in 8 bits became three 2-bit elements plus 2 unused bits), and an array with more
  elements than bits made the derive panic with a division by zero.

  ```rust
  #[packed_field(bits="0..=7")]  // error: The array's 8 bits can't be evenly split into 3 elements.
  a: [u8; 3],
  ```

- **Minimum supported Rust version is now 1.85** (was 1.51). All crates use edition 2024 and
  resolver 3 ([#116]).
- **Dependencies:** `syn` 3.0 (via 2.0, [#101]), `quote` 1.0.47, `proc-macro2` 1.0.107,
  `serde` 1.0.229. The `serde_derive` dependency was replaced by serde's `derive` feature.
- **The `std` feature no longer enables the `serde` dependency.** It used to pull in `serde/std`
  unconditionally; it is now `serde?/std` and only applies together with `use_serde`.
- **`bitvec` is no longer a dependency** ([#114], [#117]). It pulled in `radium`, which doesn't build
  on targets without `AtomicU64` (e.g. ESP32-S3, `xtensa-esp32s3-none-elf`). The two bit shifts it
  was used for in `LsbInteger` are now implemented in the crate; the packed output is unchanged and
  is verified against the previous implementation.

### Added

- **Integers narrower than their native type** in every native type that can hold them.
  `Integer<T, Bits<N>>` was only implemented when `N` needed exactly as many bytes as `T`, so a
  `u32` in 12 bits, a `u16` in 4 bits, or `[u32; 4]` with `element_size_bits="12"` didn't compile.
  All widths up to the size of the native type are now supported, including sign extension for the
  signed types.
- `PackingError` implements `core::error::Error` in `no_std` builds too ([#115]).
- `fmt::Binary` for `Integer<T, B>` ([#105]).

### Fixed

- **Derived code builds in edition 2024 crates.** The derive emitted `{ &<temp> }.pack()`, which
  fails to borrow-check under edition 2024's tail-expression temporary scope rules.
- **Array fields no longer blow up compile times** ([#110], [#102], [#118]). Parsing, pack/unpack,
  the `Debug` formatter and the rustdoc table used to be unrolled per element, so large arrays took
  very long to compile, and a `[u16; 20000]` field overflowed rustc's stack. The derive now emits one
  code template per distinct bit alignment and loops over the elements, so the generated code no
  longer grows with the array length. The rustdoc table shows one row per array field.
- **`#[derive(PrimitiveEnum)]` infers a type that holds every discriminant.** Enums with large
  negative discriminants (`-3000000000` was inferred as `i32`) or with mixed signs (`-1` and `200`
  as `i8`) failed to compile. The type is now the smallest integer that holds both the smallest and
  the largest discriminant, and an enum that no integer type can hold is reported as a compile
  error. Enums that compiled before keep their type.
- **Unpacking a dynamically sized tuple from a too short slice** returns
  `PackingError::BufferSizeMismatch` instead of underflowing the length calculation (a panic in debug
  builds).
- The `alloc` feature builds on stable Rust. It used the nightly-only `#![feature(alloc)]`, and the
  `no_std` prelude was missing an import.

### Internal

- `packed_struct::__private` is a new hidden module with support code for the derive
  (`try_array_from_fn`). It is not public API and can change in any release.
- Bit-level oracle tests: a reference implementation writes each field bit by bit, and the generated
  code is checked against it for every integer width at many bit offsets, MSB and LSB, signed and
  unsigned, for arrays of unaligned integers, and for large unaligned fields.
- Shared package metadata and dependency versions live in the workspace manifest.
- CI: `actions/checkout@v5`, explicit toolchains, feature-combination checks, an MSRV job, and
  `s390x-unknown-linux-gnu` replaces `mips64` (tier 3, no prebuilt `std`) as the big-endian target.

## [0.10.1] - 2022-11-16

- Updated `bitvec` to 1.0 ([#95]).
- Clippy fixes, including lints triggered by the generated code ([#84]).

## [0.10.0] - 2021-09-15

- **Breaking:** const generics. The generated `Bits1`, `Bits2`, ... and `Bytes1`, `Bytes2`, ...
  marker types were replaced by `Bits<N>` and `Bytes<N>` ([#69]). Minimum Rust version 1.51.
- LSB integers of non-full-byte widths, implemented with `bitvec`.

## [0.6.1] - 2021-09-10

- Fixed rustc warnings (unused borrows) in the generated code ([#79]).

## [0.6.0] - 2021-06-27

- Fixed the discriminants of primitive enums with implicit values ([#71], [#75]).
- Unknown attributes are reported as errors ([#74]).
- Nested struct example ([#76]).

## [0.5.0] - 2021-01-28

- The derive macros are re-exported by `packed_struct`, a separate `packed_struct_codegen` dependency
  is no longer needed ([#66]).
- Sign extension for signed integers narrower than their native type ([#65]).
- Updated to Rust 2018 ([#67]).

## [0.4.0] - 2020-12-02

- **Breaking:** reworked the packing traits. `pack()` returns a `Result`, the byte array is an
  associated type, and slice indexing is checked everywhere, so malformed input returns a
  `PackingError` instead of panicking ([#44], [#52]).
- Updated to `syn`/`quote` 1.0 ([#54]).
- serde support moved behind the `use_serde` feature ([#57]).
- `Hash` and `Eq` for the built-in type wrappers ([#58]).
- `byte_types_64` and `byte_types_256` features for types above 32 bytes, which aren't generated by
  default ([#56]).

## [0.3.1] - 2020-10-27

- `unpack_from_slice` checks that the slice is long enough ([#37]).
- Fixed packing on big-endian platforms ([#50]), and added cross-platform CI ([#49]).

## [0.3.0] - 2018-06-28

- **Breaking:** primitive enums declare their primitive type through an associated type.
- Primitive enums backed by types other than `u8` can be used in structures ([#31]).
- `EnumCatchAll` for enum fields that need to keep unknown values ([#26]).
- Bit ranges can be given in either order, for LSB0 numbering ([#32]).

## [0.2.3] - 2018-03-29

- The `alloc` feature for `no_std` + `alloc` builds ([#24]).

## [0.2.2] - 2018-03-13

- Basic `no_std` support ([#23]).

## [0.2.1] - 2018-02-13

- Reserved fields, always packed as zeroes or ones ([#19], [#21]).
- Primitive enums backed by any integer type, with implicit discriminants ([#17], [#18]).
- Friendlier `Debug`/`Display` formatting of packed structs and integers ([#8], [#20]).

## [0.2.0] - 2018-02-07

- **Breaking:** `..` in bit positions is exclusive, like in Rust. Use `:` or `..=` for inclusive
  ranges ([#12]).

## [0.1.0] - 2018-01-14

- First release.

[0.12.0]: https://github.com/hashmismatch/packed_struct.rs/compare/v0.10.1...HEAD
[0.10.1]: https://github.com/hashmismatch/packed_struct.rs/compare/0.10.0...v0.10.1
[0.10.0]: https://github.com/hashmismatch/packed_struct.rs/compare/v0.6.1...0.10.0
[0.6.1]: https://github.com/hashmismatch/packed_struct.rs/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/hashmismatch/packed_struct.rs/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/hashmismatch/packed_struct.rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/hashmismatch/packed_struct.rs/compare/0.3.1...v0.4.0
[0.3.1]: https://github.com/hashmismatch/packed_struct.rs/compare/v0.3.0...0.3.1
[0.3.0]: https://github.com/hashmismatch/packed_struct.rs/releases/tag/v0.3.0
[0.2.3]: https://crates.io/crates/packed_struct/0.2.3
[0.2.2]: https://crates.io/crates/packed_struct/0.2.2
[0.2.1]: https://crates.io/crates/packed_struct/0.2.1
[0.2.0]: https://crates.io/crates/packed_struct/0.2.0
[0.1.0]: https://github.com/hashmismatch/packed_struct.rs/releases/tag/0.1.0

[#8]: https://github.com/hashmismatch/packed_struct.rs/pull/8
[#12]: https://github.com/hashmismatch/packed_struct.rs/pull/12
[#17]: https://github.com/hashmismatch/packed_struct.rs/pull/17
[#18]: https://github.com/hashmismatch/packed_struct.rs/pull/18
[#19]: https://github.com/hashmismatch/packed_struct.rs/pull/19
[#20]: https://github.com/hashmismatch/packed_struct.rs/pull/20
[#21]: https://github.com/hashmismatch/packed_struct.rs/pull/21
[#23]: https://github.com/hashmismatch/packed_struct.rs/pull/23
[#24]: https://github.com/hashmismatch/packed_struct.rs/pull/24
[#26]: https://github.com/hashmismatch/packed_struct.rs/pull/26
[#31]: https://github.com/hashmismatch/packed_struct.rs/pull/31
[#32]: https://github.com/hashmismatch/packed_struct.rs/pull/32
[#37]: https://github.com/hashmismatch/packed_struct.rs/pull/37
[#44]: https://github.com/hashmismatch/packed_struct.rs/pull/44
[#49]: https://github.com/hashmismatch/packed_struct.rs/pull/49
[#50]: https://github.com/hashmismatch/packed_struct.rs/pull/50
[#52]: https://github.com/hashmismatch/packed_struct.rs/pull/52
[#54]: https://github.com/hashmismatch/packed_struct.rs/pull/54
[#56]: https://github.com/hashmismatch/packed_struct.rs/pull/56
[#57]: https://github.com/hashmismatch/packed_struct.rs/pull/57
[#58]: https://github.com/hashmismatch/packed_struct.rs/pull/58
[#65]: https://github.com/hashmismatch/packed_struct.rs/pull/65
[#66]: https://github.com/hashmismatch/packed_struct.rs/pull/66
[#67]: https://github.com/hashmismatch/packed_struct.rs/pull/67
[#69]: https://github.com/hashmismatch/packed_struct.rs/pull/69
[#71]: https://github.com/hashmismatch/packed_struct.rs/issues/71
[#74]: https://github.com/hashmismatch/packed_struct.rs/pull/74
[#75]: https://github.com/hashmismatch/packed_struct.rs/pull/75
[#76]: https://github.com/hashmismatch/packed_struct.rs/pull/76
[#79]: https://github.com/hashmismatch/packed_struct.rs/pull/79
[#84]: https://github.com/hashmismatch/packed_struct.rs/pull/84
[#95]: https://github.com/hashmismatch/packed_struct.rs/pull/95
[#101]: https://github.com/hashmismatch/packed_struct.rs/pull/101
[#102]: https://github.com/hashmismatch/packed_struct.rs/issues/102
[#105]: https://github.com/hashmismatch/packed_struct.rs/pull/105
[#107]: https://github.com/hashmismatch/packed_struct.rs/issues/107
[#110]: https://github.com/hashmismatch/packed_struct.rs/issues/110
[#114]: https://github.com/hashmismatch/packed_struct.rs/issues/114
[#115]: https://github.com/hashmismatch/packed_struct.rs/pull/115
[#116]: https://github.com/hashmismatch/packed_struct.rs/pull/116
[#117]: https://github.com/hashmismatch/packed_struct.rs/pull/117
[#118]: https://github.com/hashmismatch/packed_struct.rs/pull/118
