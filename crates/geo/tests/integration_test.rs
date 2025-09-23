mod common;

// Run this with cargo test --test integration_test
#[test]
fn error_tests() {
    common::setup();
    geo::error_test();
}

#[test]
fn shape_tests() {
    geo::shape_test();
}

#[test]
fn trait_tests() {
    geo::trait_test();
}

#[test]
fn pyramid_test() {
    geo::pyramid(5);
    geo::pyramid(4);
}
