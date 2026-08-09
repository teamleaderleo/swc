use swc_common::Mark;
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_optimization::simplify::dce::{dce, Config};
use swc_ecma_transforms_testing::test_transform;

fn shake(src: &str, expected: &str) {
    test_transform(
        swc_ecma_parser::Syntax::default(),
        None,
        |_| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            (
                resolver(unresolved_mark, top_level_mark, false),
                dce(
                    Config {
                        top_level: false,
                        ..Default::default()
                    },
                    unresolved_mark,
                ),
            )
        },
        src,
        expected,
    )
}

#[test]
fn unused_initializer_preserves_instanceof_callback() {
    shake(
        "function test(value, Constructor) { const unused = value instanceof Constructor; }",
        "function test(value, Constructor) { value instanceof Constructor; }",
    );
}

#[test]
fn unused_initializer_preserves_instanceof_exception() {
    shake(
        "function test() { const unused = 1 instanceof 2; }",
        "function test() { 1 instanceof 2; }",
    );
}

#[test]
fn unused_pure_initializer_is_removed() {
    shake(
        "function test() { const unused = 1 === 2; }",
        "function test() {}",
    );
}
