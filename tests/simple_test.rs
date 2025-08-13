fn double(n: usize) -> usize {
    n * 2
}

#[test]
fn it_works() {
    
    assert_eq!(1+1, 2);
    // Ok(())
}

// cargo test it_works -- --exact
// Running tests/simple_test.rs (target/debug/deps/simple_test-8a642eb5adee8835)

#[test]
fn test_double() {
    let n = 1;
    let expected = 2;
    assert_eq!(double(n), expected);
    // Ok(())
}

// cargo test test_double -- --exact
// Running tests/simple_test.rs (target/debug/deps/simple_test-8a642eb5adee8835)

/*
#[test]
fn itx_works(_path: &str, contents: &str) -> anyhow::Result<()> {
    let (n, expected) = parse_test_file(contents)?;

    assert_eq!(double(n), expected);
    Ok(())
}
*/