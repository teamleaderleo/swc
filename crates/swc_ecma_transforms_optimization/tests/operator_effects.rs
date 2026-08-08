use swc_common::{pass::Repeat, Mark};
use swc_ecma_transforms_base::{fixer::paren_remover, resolver};
use swc_ecma_transforms_optimization::simplify::expr_simplifier;
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
                paren_remover(None),
                Repeat::new(expr_simplifier(unresolved_mark, Default::default())),
            )
        },
        src,
        expected,
    )
}

#[test]
fn selected_array_member_preserves_operator_coercion() {
    fold(
        "[({ valueOf() { return 1; } }) + 1, 42][1];",
        "({ valueOf() { return 1; } }) + 1, 42;",
    );

    fold(
        "[({ valueOf() { return 1; } }) == 1, 42][1];",
        "({ valueOf() { return 1; } }) == 1, 42;",
    );
}

#[test]
fn selected_array_member_preserves_operator_exception() {
    fold("[1n + 1, 42][1];", "1n + 1, 42;");
}

#[test]
fn selected_array_member_can_drop_strict_equality_control() {
    fold("[1 === 2, 42][1];", "42;");
}
