use swc_common::Mark;
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_optimization::simplify::dead_branch_remover;
use swc_ecma_transforms_testing::test_transform;

fn fold(src: &str, expected: &str) {
    test_transform(
        swc_ecma_parser::Syntax::default(),
        None,
        |_| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            (
                resolver(unresolved_mark, top_level_mark, false),
                dead_branch_remover(unresolved_mark),
            )
        },
        src,
        expected,
    )
}

#[test]
fn empty_if_preserves_instanceof_callback() {
    fold(
        "function test(value, Constructor) { if (value instanceof Constructor) {} }",
        "function test(value, Constructor) { value instanceof Constructor; }",
    );
}

#[test]
fn empty_if_preserves_instanceof_exception() {
    fold(
        "function test() { if (1 instanceof 2) {} }",
        "function test() { 1 instanceof 2; }",
    );
}

#[test]
fn empty_if_drops_pure_comparison_control() {
    fold(
        "function test(value) { if (value === 2) {} }",
        "function test(value) {}",
    );
}
