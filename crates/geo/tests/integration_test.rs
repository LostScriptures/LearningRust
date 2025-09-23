// Run this with cargo test --test integration_test
#[test]
fn error_tests() {
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
