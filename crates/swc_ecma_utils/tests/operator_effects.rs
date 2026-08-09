use swc_common::{BytePos, SyntaxContext};
use swc_ecma_ast::Expr;
use swc_ecma_parser::{Parser, StringInput, Syntax};
use swc_ecma_utils::{ExprCtx, ExprExt};

fn parse_expr(source: &str) -> Box<Expr> {
    let mut parser = Parser::new(
        Syntax::Es(Default::default()),
        StringInput::new(source, BytePos(0), BytePos(source.len() as u32)),
        None,
    );
    parser.parse_expr().unwrap()
}

fn expr_ctx() -> ExprCtx {
    ExprCtx {
        unresolved_ctxt: SyntaxContext::empty(),
        is_unresolved_ref_safe: true,
        in_strict: false,
        remaining_depth: 6,
    }
}

fn assert_may_have_side_effects(source: &str) {
    assert!(
        parse_expr(source).may_have_side_effects(expr_ctx()),
        "expected operator evaluation to be treated as potentially effectful: {source}"
    );
}

fn extracted_effects(source: &str) -> Vec<Box<Expr>> {
    let mut effects = Vec::new();
    expr_ctx().extract_side_effects_to(&mut effects, *parse_expr(source));
    effects
}

fn assert_preserves_binary_operator(source: &str) {
    let effects = extracted_effects(source);
    assert_eq!(
        effects.len(),
        1,
        "expected one retained whole-expression effect for {source}: {effects:#?}"
    );
    assert!(
        matches!(effects.first().map(|expr| &**expr), Some(Expr::Bin(_))),
        "expected the binary operator itself to be retained for {source}: {effects:#?}"
    );
}

#[test]
fn membership_and_instance_callbacks_are_effectful() {
    assert_may_have_side_effects("'x' in proxy");
    assert_may_have_side_effects("value instanceof Constructor");
}

#[test]
fn invalid_membership_and_instance_operands_are_effectful() {
    assert_may_have_side_effects("1 in 2");
    assert_may_have_side_effects("1 instanceof 2");
}

#[test]
fn extracting_effects_preserves_membership_and_instance_operators() {
    assert_preserves_binary_operator("'x' in proxy");
    assert_preserves_binary_operator("value instanceof Constructor");
    assert_preserves_binary_operator("1 in 2");
    assert_preserves_binary_operator("1 instanceof 2");
}

#[test]
fn safe_primitive_controls_remain_pure() {
    assert!(!parse_expr("1 + 2").may_have_side_effects(expr_ctx()));
    assert!(!parse_expr("1 === 2").may_have_side_effects(expr_ctx()));
    assert!(!parse_expr("true && false").may_have_side_effects(expr_ctx()));

    assert!(extracted_effects("1 + 2").is_empty());
    assert!(extracted_effects("1 === 2").is_empty());
}
