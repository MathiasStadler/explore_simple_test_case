# I`m explore here the repo simple_test_case [![alt text][1]](https://github.com/sminez/simple_test_case.git)

## original repo find here [![alt text][1]](https://github.com/sminez/simple_test_case.git)

## Test helpers should be simple

You don't want to have to worry about bugs in your test suite or unexpected behaviour leading
to failing tests silently passing. With that in mind, `simple_test_case` aims to do the bare
minimum to eliminate the boilerplate of writing parameterised tests and no more.

The `test_case` attribute macro handles generating multiple test functions for you which are
parameterised by the inputs you provide. You still need to provide the `#[test]` attribute (or
an alternative such as `#[tokio::test]`) and all test cases _must_ be provided before any
additional attribute macros you wish to apply.

And that's it.

There is no additional support for custom assertions, fixtures etc. That said, if you want or
need a more complicated testing set up, additional attribute macros should play nice with
`simple_test_case` provided you follow the advice below.

## Usage

### Valid
<!-- keep the format -->
Here the `#[test]` attribute is provided after all instances of `test_case`. This will work.
<!-- keep the format -->
```rust
use simple_test_case::test_case;

fn double(n: usize) -> usize {
    n * 2
}

#[test_case(1, 2; "case 1")]
#[test_case(3, 6; "case 2")]
#[test]
fn double_test(n: usize, double: usize) {
    assert_eq!(double(n), double)
}
```

### Invalid
<!-- keep the format -->
Here the `#[test]` attribute is provided before all instances of `test_case`. This will cause
the compiler to complain about functions used as tests not being allowed to have any arguments.
<!-- keep the format -->
```rust
use simple_test_case::test_case;

fn double(n: usize) -> usize {
    n * 2
}

#[test]
#[test_case(1, 2; "case 1")]
#[test_case(3, 6; "case 2")]
fn double_test(n: usize, double: usize) {
    assert_eq!(double(n), double)
}
```

## Additional attributes

`test_case` preserves all attributes beneath it, forwarding them on to the individual generated
test functions. As an example, the standard library `should_panic` attribute works just fine as
shown below (just make sure to provide your test cases first as described above):

```rust
use simple_test_case::test_case;

#[test_case(1, 2; "case 1")]
#[test_case(3, 6; "case 2")]
#[test]
#[should_panic(expected = "this works")]
fn panic_test(n: usize, double: usize) {
    assert_eq!(double(a), b);
    panic!("this works")
}
```

### Async tests
<!-- keep the format -->
Async tests are supported in the same way that all other attributes are supported: add your
tests cases first and then apply the async testing macro of your choice beneath.
<!-- keep the format -->
```rust
use simple_test_case::test_case;

async fn async_double(n: usize) -> usize {
    n * 2
}

#[test_case(1, 2; "case 1")]
#[test_case(3, 6; "case 2")]
#[tokio::test]
async fn double_test(n: usize, double: usize) {
    assert_eq!(double(n).await, double)
}
```
<!-- keep the format -->
## How does it work?
<!-- keep the format -->
You are encouraged to read the source of the macro itself (the macro plus associated helper
functions are under 150 lines of code) but the general idea is as follows:
<!-- keep the format -->
- Collect all `test_case` (or `simple_test_case::test_case`) attributes, each of which maps a
  set of function arguments to a test case name.
- For each test case create a copy of the original test function with the function arguments
  replaced with explicit variable bindings at the top of the function body.
- Write out each of the cases as their own test inside of a new module that is named using the
  original test function name.
<!-- keep the format -->
You can use [cargo expand](https://github.com/dtolnay/cargo-expand) to see what the generated
tests look like using the example provided in the `examples` directory like so:
<!-- keep the format -->
```bash
$ cargo expand --example=expand_me --tests
  Compiling simple_test_case v0.1.0 (/home/innes/repos/personal/simple_test_case)
   Finished test [unoptimized + debuginfo] target(s) in 0.12s
```
<!-- keep the format -->
```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use simple_test_case::test_case;
mod example {
    #[allow(unused_imports)]
    use super::*;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const small_example: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("example::small_example"),
            ignore: false,
            allow_fail: false,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(small_example())),
    };
    fn small_example() {
        let a: usize = 1;
        let b: usize = 2;
        if !(a < b) {
            ::core::panicking::panic("assertion failed: a < b")
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const large_example: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("example::large_example"),
            ignore: false,
            allow_fail: false,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(large_example())),
    };
    fn large_example() {
        let a: usize = 100;
        let b: usize = 200;
        if !(a < b) {
            ::core::panicking::panic("assertion failed: a < b")
        }
    }
}
#[allow(dead_code)]
fn main() {}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&small_example, &large_example])
}
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