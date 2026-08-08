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

#[test]
fn operator_callbacks_are_effectful() {
    assert_may_have_side_effects("'x' in proxy");
    assert_may_have_side_effects("value instanceof Constructor");
}

#[test]
fn operator_coercion_is_effectful() {
    assert_may_have_side_effects("({ valueOf() { return 1; } }) + 1");
    assert_may_have_side_effects("({ valueOf() { return 1; } }) - 1");
    assert_may_have_side_effects("({ valueOf() { return 1; } }) < 2");
    assert_may_have_side_effects("({ valueOf() { return 1; } }) == 1");
}

#[test]
fn primitive_controls_remain_pure() {
    assert!(!parse_expr("1 + 2").may_have_side_effects(expr_ctx()));
    assert!(!parse_expr("1 === 2").may_have_side_effects(expr_ctx()));
    assert!(!parse_expr("true && false").may_have_side_effects(expr_ctx()));
}
