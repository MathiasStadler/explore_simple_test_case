# explore_simple_test_case
<!-- keep the format -->
>[TIP!]
>Listing all tests in a Cargo project without running them [![alt text][1]](https://stackoverflow.com/questions/64908864/is-there-a-way-of-listing-all-tests-in-a-cargo-project-without-running-them)
><!-- keep the format -->
>```bash <!-- markdownlint-disable-line code-block-style -->
>$ cargo test -- --list
><!-- keep the format -->
>Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
>     Running unittests src/lib.rs (target/debug/deps/
>   simple_test_case-879a93406e9c4cca)
>
>test_case::tests::parse_test_case: test
>util::tests::leading_num: test
>util::tests::punctuation: test
>util::tests::space_delimited: test
>
>0 tests, 1 benchmark
><SKIP OUTPUT>
<!-- keep the format -->
## How to run a specific unit test in Rust? **-- --exact**  [![alt text][1]](https://stackoverflow.com/questions/54585804/how-to-run-a-specific-unit-test-in-rust)
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
# cargo test <test_fn_name> -- --exact
cargo test
```
<!-- keep the format -->
>[!NOTE]
>Symbol to mark web external links [![alt text][1]](./README.md)
<!-- -->
<!-- mkdir folder for link sign and download the link sign -->
<!-- mkdir -p img && curl --create-dirs --output-dir img -O  "https://raw.githubusercontent.com/MathiasStadler/link_symbol_svg/refs/heads/main/link_symbol.svg"-->
<!-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email -->
[1]: ./img/link_symbol.svg
<!-- keep the format -->