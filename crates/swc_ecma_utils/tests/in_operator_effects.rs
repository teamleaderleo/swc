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

#[test]
fn in_operator_is_effectful_by_default() {
    assert!(parse_expr("'x' in proxy").may_have_side_effects(expr_ctx()));
    assert!(parse_expr("1 in 2").may_have_side_effects(expr_ctx()));
}

#[test]
fn extracting_effects_preserves_in_operator() {
    for source in ["'x' in proxy", "1 in 2"] {
        let mut effects = Vec::new();
        expr_ctx().extract_side_effects_to(&mut effects, *parse_expr(source));
        assert_eq!(effects.len(), 1, "expected one retained effect for {source}");
        assert!(matches!(effects.first().map(|expr| &**expr), Some(Expr::Bin(_))));
    }
}
