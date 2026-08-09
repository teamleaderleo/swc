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
fn primitive_lhs_keeps_instanceof_operator_semantics() {
    // The operator can still throw for an invalid RHS.
    fold("1 instanceof 2;", "1 instanceof 2;");

    // An unknown constructor can define Symbol.hasInstance and accept primitives.
    fold("1 instanceof Constructor;", "1 instanceof Constructor;");
}

#[test]
fn object_lhs_keeps_global_object_has_instance_semantics() {
    // Object can have an own Symbol.hasInstance installed at runtime.
    fold("({}) instanceof Object;", "({}) instanceof Object;");
}

#[test]
fn unrelated_constant_comparison_still_folds() {
    fold("1 === 2;", "false;");
}
