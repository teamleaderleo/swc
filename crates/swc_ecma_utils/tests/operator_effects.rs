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

fn extracted_effects(source: &str) -> Vec<Box<Expr>> {
    let mut effects = Vec::new();
    expr_ctx().extract_side_effects_to(&mut effects, *parse_expr(source));
    effects
}

#[test]
fn instanceof_operator_is_effectful() {
    assert!(parse_expr("value instanceof Constructor").may_have_side_effects(expr_ctx()));
    assert!(parse_expr("1 instanceof 2").may_have_side_effects(expr_ctx()));
}

#[test]
fn extracting_effects_preserves_instanceof_operator() {
    for source in ["value instanceof Constructor", "1 instanceof 2"] {
        let effects = extracted_effects(source);
        assert_eq!(
            effects.len(),
            1,
            "expected one retained whole-expression effect for {source}: {effects:#?}"
        );
        assert!(
            matches!(effects.first().map(|expr| &**expr), Some(Expr::Bin(_))),
            "expected instanceof itself to be retained for {source}: {effects:#?}"
        );
    }
}

#[test]
fn primitive_controls_remain_pure() {
    assert!(!parse_expr("1 + 2").may_have_side_effects(expr_ctx()));
    assert!(!parse_expr("1 === 2").may_have_side_effects(expr_ctx()));
    assert!(extracted_effects("1 + 2").is_empty());
    assert!(extracted_effects("1 === 2").is_empty());
}
