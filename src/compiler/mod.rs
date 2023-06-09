use crate::parser::ast::{BinaryOpKind, Expr, ImportedSymbol, Module, Stmt};

static SPACE: &'static str = "  ";

pub fn gen(module: Module) -> String {
    let mut buf = String::new();

    for stmt in module.statements {
        gen_stmt(&mut buf, 0, stmt);
    }
    buf
}

fn gen_stmt(buf: &mut String, deep: usize, stmt: Stmt) {
    match stmt {
        Stmt::Import { symbols, path } => gen_import(buf, deep, symbols, path),
        Stmt::Var {
            name,
            is_mut,
            value,
        } => gen_var(buf, deep, name, is_mut, value),
        Stmt::Const { name, value } => gen_var(buf, deep, name, false, value),
        Stmt::Function {
            name,
            params,
            ret_type,
            body,
        } => todo!(),
        Stmt::Expr(expr) => gen_expr(buf, deep, expr),
    }
}

fn gen_import(buf: &mut String, deep: usize, symbols: Vec<ImportedSymbol>, path: String) {
    buf.push_str("import {");
    let s = symbols.iter().map(gen_sym).collect::<Vec<String>>();
    if s.len() > 3 {
        let space = SPACE.repeat(deep + 1);
        buf.push('\n');
        buf.push_str(space.as_str());
        buf.push_str(s.join(format!("{}{}", ",\n", space).as_str()).as_str());
        buf.push('\n');
    } else {
        buf.push(' ');
        buf.push_str(s.join(", ").as_str());
        buf.push(' ');
    }
    buf.push_str("} from ");
    gen_string(buf, path);
    buf.push_str(";\n");
}

fn gen_sym(sym: &ImportedSymbol) -> String {
    match sym.imported_as.clone() {
        Some(n) => String::from_iter(vec![sym.name.clone(), " as ".to_string(), n]),
        None => sym.name.clone(),
    }
}

fn gen_var(buf: &mut String, deep: usize, name: String, is_mut: bool, expr: Expr) {
    if is_mut {
        buf.push_str("let")
    } else {
        buf.push_str("const")
    }
    buf.push(' ');
    buf.push_str(&name);
    buf.push_str(" = ");
    gen_expr(buf, deep + 1, expr);
    buf.push_str(";\n")
}

fn gen_expr(buf: &mut String, deep: usize, expr: Expr) {
    match expr {
        Expr::Integer(i) => gen_int(buf, i),
        Expr::Float(f) => gen_float(buf, f),
        Expr::String(s) => gen_string(buf, s),
        Expr::Ident(i) => buf.push_str(i.as_str()),
        Expr::BinaryOp {
            kind,
            left,
            right,
        } => gen_bin_op(buf, deep, kind, left, right),
        Expr::Call {
            target: _,
            arguments: _,
        } => todo!(),
        Expr::DotAccess { target, name } => todo!(),
        Expr::BracketAccess { target, expr } => todo!(),
    }
}

fn gen_int(buf: &mut String, i: i32) {
    buf.push_str(i.to_string().as_str())
}

fn gen_float(buf: &mut String, f: f32) {
    buf.push_str(f.to_string().as_str())
}

fn gen_string(buf: &mut String, string: String) {
    buf.push('"');
    buf.push_str(string.as_str());
    buf.push('"');
}

fn gen_bin_op(buf: &mut String, deep: usize, op: BinaryOpKind, left: Box<Expr>, right: Box<Expr>) {
    gen_expr(buf, deep, left.as_ref().clone());
    buf.push(' ');
    buf.push_str(op.to_op());
    buf.push(' ');
    gen_expr(buf, deep, right.as_ref().clone());
}
