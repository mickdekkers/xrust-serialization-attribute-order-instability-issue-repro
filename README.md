Minimal reproducible example for a serialization issue encountered with the [xrust](https://github.com/ballsteve/xrust) crate.

## Issue

Calling `xrust::parser::xml::parse` with `Rc<xrust::trees::smite::Node>` as the first argument, followed by `xrust::trees::smite::Rc::to_xml` to serialize the parsed XML tree back to a string, does not produce a stable output. In particular, when performing this sequence of operations multiple times on the same source XML file, the _order of attributes_ in the serialized XML output is different between calls.

## Steps to reproduce

1. Clone this repo.
2. Run `cargo run` in the root directory of the repo.
3. Observe the assertion failure.

## Expected behavior

Ideally, the output of multiple different `parse` + `to_xml` calls on the same source XML file should be equivalent. This allows downstream consumers to rely on the output of `to_xml` for testing and debugging purposes, such as [snapshot testing](https://github.com/mitsuhiko/insta).

## Notes

The issue occurs with the released `xrust` crate version [`1.0.0`](https://crates.io/crates/xrust/1.0.0), as well as the latest commits on the `main` branch ([313aafd](https://github.com/ballsteve/xrust/commit/313aafd523b1f9fbfdfc4b961e6d7fe6b61c5db3)) and `dev` branch ([2449e4d](https://github.com/ballsteve/xrust/commit/2449e4dfe85d613d8d096fed21f7c1e2f768e0f9)) at the time of writing this README.
