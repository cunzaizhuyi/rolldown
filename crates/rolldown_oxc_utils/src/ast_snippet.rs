use oxc::{
  allocator::{self, Allocator},
  ast::ast::{self, Statement},
  span::Atom,
};

use crate::{Dummy, IntoIn};

type PassedStr<'a> = &'a str;

// `AstBuilder` is more suitable name, but it's already used in oxc.
pub struct AstSnippet<'ast> {
  pub alloc: &'ast Allocator,
}

impl<'ast> AstSnippet<'ast> {
  pub fn new(alloc: &'ast Allocator) -> Self {
    Self { alloc }
  }

  pub fn atom(&self, value: &str) -> Atom<'ast> {
    let alloc_str = allocator::String::from_str_in(value, self.alloc).into_bump_str();
    Atom::Arena(alloc_str)
  }

  pub fn id(&self, name: PassedStr) -> ast::BindingIdentifier<'ast> {
    ast::BindingIdentifier { name: self.atom(name), ..Dummy::dummy(self.alloc) }
  }

  pub fn id_ref(&self, name: PassedStr) -> ast::IdentifierReference<'ast> {
    ast::IdentifierReference { name: self.atom(name), ..Dummy::dummy(self.alloc) }
  }

  pub fn id_name(&self, name: PassedStr) -> ast::IdentifierName<'ast> {
    ast::IdentifierName { name: self.atom(name), ..Dummy::dummy(self.alloc) }
  }

  pub fn id_ref_expr(&self, name: PassedStr) -> ast::Expression<'ast> {
    ast::Expression::Identifier(self.id_ref(name).into_in(self.alloc))
  }

  /// `[object].[property]`
  pub fn literal_prop_access_member_expr(
    &self,
    object: PassedStr,
    property: PassedStr,
  ) -> ast::MemberExpression<'ast> {
    ast::MemberExpression::StaticMemberExpression(ast::StaticMemberExpression {
      object: ast::Expression::Identifier(self.id_ref(object).into_in(self.alloc)),
      property: ast::IdentifierName { name: self.atom(property), ..Dummy::dummy(self.alloc) },
      ..Dummy::dummy(self.alloc)
    })
  }

  /// `[object].[property]`
  pub fn literal_prop_access_member_expr_expr(
    &self,
    object: PassedStr,
    property: PassedStr,
  ) -> ast::Expression<'_> {
    ast::Expression::MemberExpression(
      self.literal_prop_access_member_expr(object, property).into_in(self.alloc),
    )
  }

  /// `name()`
  pub fn call_expr(&self, name: PassedStr) -> ast::CallExpression<'ast> {
    ast::CallExpression {
      callee: ast::Expression::Identifier(self.id_ref(name).into_in(self.alloc)),
      arguments: allocator::Vec::new_in(self.alloc),
      ..Dummy::dummy(self.alloc)
    }
  }

  /// `name()`
  pub fn call_expr_expr(&self, name: PassedStr) -> ast::Expression<'ast> {
    ast::Expression::CallExpression(self.call_expr(name).into_in(self.alloc))
  }

  /// `name(arg)`
  pub fn call_expr_with_arg_expr(&self, name: PassedStr, arg: PassedStr) -> ast::Expression<'ast> {
    let arg =
      ast::Argument::Expression(ast::Expression::Identifier(self.id_ref(arg).into_in(self.alloc)));
    let mut call_expr = self.call_expr(name);
    call_expr.arguments.push(arg);
    ast::Expression::CallExpression(call_expr.into_in(self.alloc))
  }

  /// `name(arg)`
  pub fn call_expr_with_arg_expr_expr(
    &self,
    name: PassedStr,
    arg: ast::Expression<'ast>,
  ) -> ast::Expression<'ast> {
    let arg = ast::Argument::Expression(arg);
    let mut call_expr = self.call_expr(name);
    call_expr.arguments.push(arg);
    ast::Expression::CallExpression(call_expr.into_in(self.alloc))
  }

  /// `name(arg1, arg2)`
  pub fn call_expr_with_2arg_expr(
    &self,
    name: PassedStr,
    arg1: PassedStr,
    arg2: PassedStr,
  ) -> ast::Expression<'_> {
    let arg1 =
      ast::Argument::Expression(ast::Expression::Identifier(self.id_ref(arg1).into_in(self.alloc)));
    let arg2 =
      ast::Argument::Expression(ast::Expression::Identifier(self.id_ref(arg2).into_in(self.alloc)));
    let mut call_expr = self.call_expr(name);
    call_expr.arguments.push(arg1);
    call_expr.arguments.push(arg2);
    ast::Expression::CallExpression(call_expr.into_in(self.alloc))
  }

  /// `name(arg1, arg2)`
  pub fn call_expr_with_2arg_expr_expr(
    &self,
    name: PassedStr,
    arg1: ast::Expression<'ast>,
    arg2: ast::Expression<'ast>,
  ) -> ast::Expression<'ast> {
    let arg1 = ast::Argument::Expression(arg1);
    let arg2 = ast::Argument::Expression(arg2);
    let mut call_expr = self.call_expr(name);
    call_expr.arguments.push(arg1);
    call_expr.arguments.push(arg2);
    ast::Expression::CallExpression(call_expr.into_in(self.alloc))
  }

  /// `name()`
  pub fn call_expr_stmt(&self, name: PassedStr) -> ast::Statement<'_> {
    ast::Statement::ExpressionStatement(
      ast::ExpressionStatement {
        expression: self.call_expr_expr(name),
        ..Dummy::dummy(self.alloc)
      }
      .into_in(self.alloc),
    )
  }

  /// `var [name] = [init]`
  pub fn var_decl_stmt(
    &self,
    name: PassedStr,
    init: ast::Expression<'ast>,
  ) -> ast::Statement<'ast> {
    ast::Statement::Declaration(self.var_decl(name, init))
  }

  /// `var [name] = [init]`
  pub fn var_decl(&self, name: PassedStr, init: ast::Expression<'ast>) -> ast::Declaration<'ast> {
    let mut declarations = allocator::Vec::new_in(self.alloc);
    declarations.push(ast::VariableDeclarator {
      id: ast::BindingPattern {
        kind: ast::BindingPatternKind::BindingIdentifier(
          ast::BindingIdentifier { name: self.atom(name), ..Dummy::dummy(self.alloc) }
            .into_in(self.alloc),
        ),
        ..Dummy::dummy(self.alloc)
      },
      init: Some(init),
      ..Dummy::dummy(self.alloc)
    });
    ast::Declaration::VariableDeclaration(
      ast::VariableDeclaration {
        kind: ast::VariableDeclarationKind::Var,
        declarations,
        ..Dummy::dummy(self.alloc)
      }
      .into_in(self.alloc),
    )
  }

  /// ```js
  ///  var require_foo = __commonJS((exports, module) => {
  ///    ...
  ///  });
  /// ```
  pub fn commonjs_wrapper_stmt(
    &'ast self,
    binding_name: PassedStr,
    commonjs_name: PassedStr,
    body: allocator::Vec<'ast, Statement<'ast>>,
  ) -> ast::Statement<'ast> {
    // (exports, module) => {}
    let mut arrow_expr = ast::ArrowFunctionExpression {
      body: ast::FunctionBody { statements: body, ..Dummy::dummy(self.alloc) }.into_in(self.alloc),
      ..Dummy::dummy(self.alloc)
    };
    arrow_expr.params.items.push(ast::FormalParameter {
      pattern: ast::BindingPattern {
        kind: ast::BindingPatternKind::BindingIdentifier(self.id("exports").into_in(self.alloc)),
        ..Dummy::dummy(self.alloc)
      },
      ..Dummy::dummy(self.alloc)
    });
    arrow_expr.params.items.push(ast::FormalParameter {
      pattern: ast::BindingPattern {
        kind: ast::BindingPatternKind::BindingIdentifier(self.id("module").into_in(self.alloc)),
        ..Dummy::dummy(self.alloc)
      },
      ..Dummy::dummy(self.alloc)
    });

    //  __commonJS(...)
    let mut commonjs_call_expr = self.call_expr(commonjs_name);
    commonjs_call_expr.arguments.push(ast::Argument::Expression(
      ast::Expression::ArrowFunctionExpression(arrow_expr.into_in(self.alloc)),
    ));

    // var require_foo = ...

    let var_decl_stmt = self.var_decl_stmt(
      binding_name,
      ast::Expression::CallExpression(commonjs_call_expr.into_in(self.alloc)),
    );

    var_decl_stmt
  }

  /// ```js
  /// var init_foo = __esm(() => { ... });
  /// ```
  pub fn esm_wrapper_stmt(
    &'ast self,
    binding_name: PassedStr,
    esm_fn_name: PassedStr,
    body: allocator::Vec<'ast, Statement<'ast>>,
  ) -> ast::Statement<'ast> {
    // () => { ... }
    let arrow_expr: ast::ArrowFunctionExpression<'_> = ast::ArrowFunctionExpression {
      body: ast::FunctionBody { statements: body, ..Dummy::dummy(self.alloc) }.into_in(self.alloc),
      ..Dummy::dummy(self.alloc)
    };

    //  __esm(...)
    let mut commonjs_call_expr = self.call_expr(esm_fn_name);
    commonjs_call_expr.arguments.push(ast::Argument::Expression(
      ast::Expression::ArrowFunctionExpression(arrow_expr.into_in(self.alloc)),
    ));

    // var init_foo = ...

    self.var_decl_stmt(
      binding_name,
      ast::Expression::CallExpression(commonjs_call_expr.into_in(self.alloc)),
    )
  }

  /// ```js
  /// (a, b)
  /// ```
  pub fn seq2_in_paren_expr(
    &self,
    a: ast::Expression<'ast>,
    b: ast::Expression<'ast>,
  ) -> ast::Expression<'ast> {
    let mut expressions = allocator::Vec::new_in(self.alloc);
    expressions.push(a);
    expressions.push(b);
    let seq_expr = ast::Expression::SequenceExpression(
      ast::SequenceExpression { expressions, ..Dummy::dummy(self.alloc) }.into_in(self.alloc),
    );
    ast::Expression::ParenthesizedExpression(
      ast::ParenthesizedExpression { expression: seq_expr, ..Dummy::dummy(self.alloc) }
        .into_in(self.alloc),
    )
  }

  /// ```js
  /// 42
  /// ```
  pub fn number_expr(&self, value: f64) -> ast::Expression<'ast> {
    ast::Expression::NumericLiteral(
      ast::NumericLiteral {
        span: Dummy::dummy(self.alloc),
        value,
        raw: self.alloc.alloc(value.to_string()),
        base: oxc::syntax::NumberBase::Decimal,
      }
      .into_in(self.alloc),
    )
  }

  /// ```js
  ///  id = ...
  /// ￣￣ AssignmentTarget
  /// ```
  pub fn simple_id_assignment_target(&self, id: PassedStr) -> ast::AssignmentTarget<'ast> {
    ast::AssignmentTarget::SimpleAssignmentTarget(
      ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(self.id_ref(id).into_in(self.alloc)),
    )
  }

  // `() => xx`
  pub fn only_return_arrow_expr(&self, expr: ast::Expression<'ast>) -> ast::Expression<'ast> {
    let mut statements = allocator::Vec::new_in(self.alloc);
    statements.reserve_exact(1);
    statements.push(ast::Statement::ExpressionStatement(
      ast::ExpressionStatement { expression: expr, ..Dummy::dummy(self.alloc) }.into_in(self.alloc),
    ));
    ast::Expression::ArrowFunctionExpression(
      ast::ArrowFunctionExpression {
        expression: true,
        body: ast::FunctionBody { statements, ..Dummy::dummy(self.alloc) }.into_in(self.alloc),
        ..Dummy::dummy(self.alloc)
      }
      .into_in(self.alloc),
    )
  }
}
