use peg::{error::ParseError, str::LineCol};

use self::ast::Module;

pub mod ast;
mod stmt;

peg::parser! {
    grammar parser() for str {

    use ast::{
        Expr,
        FunctionParam,
        ImportedSymbol,
        Module,
        Stmt,
        TypeUsage,
        CallArgument
    };
    use ast::pattern::{Pattern, StructField};

    //
    // <PRIMITIVES>
    //

    rule any() = [_]
    rule numeric() = ['0'..='9']+
    rule alpha() = ['a'..='z' | 'A'..='Z']
    rule sp() =
        quiet! {[' ' | '\n' | '\t' | '\r' ]*}
        / expected!("space")
    rule escape_sequence() = "\\\\" / "\\\"" / "\\\'" / "\\n" / "\\r" / "\\t" / "\\0"

    rule alphanumeric() = (alpha() / numeric())
    rule literal_char() = escape_sequence() / (!"\"" any())



    rule colon_prefixed<T>(r: rule<T>) -> T =
        colon()
        r:r() { r }

    rule comma_separated<T>(r: rule<T>) -> Vec<T> =
        r() ** comma()

    rule spaced<T>(x: rule<T>) -> T =
        sp() r:x() sp() { r }

    rule curly_braced<T>(r: rule<T>) -> T = spaced(<"{">) r:r() spaced(<"}">) { r }
    rule angle_braced<T>(r: rule<T>) -> T = spaced(<"<">) r:r() spaced(<">">) { r }
    rule round_braced<T>(r: rule<T>) -> T = spaced(<"(">) r:r() spaced(<")">) { r }
    rule rect_braced<T>(r: rule<T>) -> T = spaced(<"[">) r:r() spaced(<"]">) { r }

    rule import_kw() = spaced(<"import">)
    rule from_kw() = spaced(<"from">)
    rule mut_kw() = spaced(<"mut">)
    rule let_kw() = spaced(<"let">)
    rule const_kw() = spaced(<"const">)
    rule fn_kw() = spaced(<"fn">)
    rule as_kw() = spaced(<"as">)
    rule assign() = spaced(<"=">)
    rule comma() = spaced(<",">)
    rule colon() = spaced(<":">)
    rule semicolon() = spaced(<";">)
    rule dot() = spaced(<".">)
    pub rule string_literal() -> &'input str =
        "\"" s:$(literal_char()*) "\"" { s }

    rule int_literal() -> i32 =
        i:$(numeric()) {?
            i.parse().or(Err("Can't parse integer"))
        }


    rule float_literal() -> f32 =
        f:$(numeric() "." numeric()) {?
            f.parse().or(Err("Can't parse float"))
        }

    pub rule ident() -> &'input str =
        $(alpha() alphanumeric()*)
    //
    // </PRIMITIVES>
    //

    //
    // <PATTERNS>
    //

    rule tuple_pattern() -> Pattern =
        "(" ps:(pattern() ** ",") ")" {
            Pattern::Tuple(
                ps.into_iter()
                    .map(|p| Box::new(p))
                    .collect()
            )
        }

    rule struct_pattern() -> Pattern =
        n:struct_name()
        b:struct_body() {
            Pattern::Struct { name: n.to_string(), fields: b }
        }

        rule struct_name() -> &'input str = ident()

        rule struct_body() -> Vec<StructField> =
            curly_braced(<struct_field_list()>)

        rule struct_field_list() -> Vec<StructField> =
            struct_field() ** comma()

        rule struct_field() -> StructField =
            n:ident()
            colon()
            p:pattern() {
                StructField {
                    name: n.to_string(),
                    pattern: p
                }
            }

    rule int_pattern() -> Pattern =
        i:int_literal() {
            Pattern::Integer(i)
        }

    rule float_pattern() -> Pattern =
        i:float_literal() {
            Pattern::Float(i)
        }
    rule string_pattern() -> Pattern =
        s:string_literal() {
            Pattern::String(s.to_string())
        }

    rule pattern() -> Pattern =
        float_pattern()
        / int_pattern()
        / tuple_pattern()
        /string_pattern()

    //
    // </PATTERNS>
    //

    //
    // <TYPE_USAGES>
    //

    rule type_usage() -> TypeUsage =
        name:spaced(<ident()>)
        params:type_param_list()? {
            TypeUsage {
                name: name.to_string(),
                params: params.unwrap_or_else(|| Vec::new()),
            }
        }

        rule type_param_list() -> Vec<TypeUsage> =
            params:angle_braced(<
                comma_separated(<
                    type_usage()
                >)
            >) { params }




    //
    // </TYPE_USAGES>
    //

    //
    // <EXPRESSIONS>
    //

    pub rule float() -> Expr =
        f:float_literal() {
            Expr::Float(f)
        }

    pub rule int() -> Expr =
        i:int_literal() {
            Expr::Integer(i)
        }

    rule string() -> Expr =
        s:string_literal() {
            Expr::String(s.to_string())
        }

    rule ident_expr() -> Expr =
        i:ident() {
            Expr::Ident(i.to_string())
        }

    rule expr_arith() -> Expr = precedence! {
        x:(@) "+" y:@ { Expr::bin_add(x, y) }
        x:(@) "-" y:@ { Expr::bin_sub(x, y) }
        --
        x:(@) "*" y:@ { Expr::bin_mul(x, y) }
        x:(@) "/" y:@ { Expr::bin_div(x, y) }
        x:(@) "%" y:@ { Expr::bin_rem(x, y) }
        --
        e:spaced(<float()>){e}
        e:spaced(<int()>){e}
        e:spaced(<string()>){e}
        e:spaced(<ident_expr()>){e}
        e:round_braced(<expr()>) {e}
    }

    rule call_arguments()-> Vec<CallArgument> =
        round_braced(<comma_separated(<call_argument()>)>)

    rule call_argument() -> CallArgument =
        name:call_argument_name()? expr:spaced(<expr()>) {
            CallArgument {
                name: name.map(str::to_string),
                expr,
            }
        }

        rule call_argument_name() -> &'input str =
            n:spaced(<ident()>) assign() { n }

    #[cache_left_rec]
    rule expr() -> Expr =
        l:expr() spaced(<".">) n:ident() {
            Expr::DotAccess { target: Box::new(l), name: n.to_string() }
        }
        / l:expr() r:rect_braced(<expr()>) {
            Expr::BracketAccess { target: Box::new(l), expr: Box::new(r) }
        }
        / l:expr() args:call_arguments() {
            Expr::Call { target: Box::new(l), arguments: args }
        }
        / l:expr()
        / expr_arith()

    //
    // </EXPRESSIONS>
    //

    //
    // <STATEMENTS>
    //

    pub rule import_stmt() -> Stmt =
        import_kw()
        symbols:curly_braced(<
            imported_sumbol_list()
        >)
        from_kw()
        path:string_literal() {
            Stmt::Import { symbols, path: path.to_string() }
        }

        rule imported_sumbol_list() -> Vec<ImportedSymbol> =
            symbols:spaced(<
                comma_separated(<
                    imported_symbol()
                >)
            >) { symbols }

        rule imported_symbol() -> ImportedSymbol =
            name:(ident())
            imported_as:imported_symbol_alias() {
                ImportedSymbol {
                    name: name.to_string(),
                    imported_as
                }
            }
        rule imported_symbol_alias() -> Option<String> =
            alias:(as_kw() n:ident() { n })? {
                alias.map(str::to_string)
            }



    // Rule for parsing any statements
    rule stmt() -> Stmt =
        import_stmt()
        / definition()
        / e:expr() { Stmt::Expr(e) }

    rule stmt_separator() =
        semicolon()?

    rule stmts() -> Vec<Stmt> = stmt() ** stmt_separator()

    //
    // </STATEMENTS>
    //

    //
    // <DEFINITIONS>
    //

    pub rule function_definition() -> Stmt =
        fn_kw()
        name:ident()
        params:function_param_list()
        ret_type:function_type()
        body:function_body() {
            Stmt::Function {
                name: name.to_string(),
                params,
                ret_type,
                body
            }
        }

        rule function_param_list() -> Vec<FunctionParam> =
            params:round_braced(<
                comma_separated(<
                    function_param()
                >)
            >) { params }

            rule function_param() -> FunctionParam =
                name:ident()
                colon()
                t:type_usage() {
                    FunctionParam::new(name, t)
                }
        rule function_type() -> TypeUsage =
            t:colon_prefixed(<
                type_usage()
            >)? {
                t.unwrap_or_else(||
                    TypeUsage::from_name("Unit")
                )
            }

        rule function_body() -> Vec<Stmt> =
            sp() s:curly_braced(<stmts()>) { s }
            / assign() s:stmt() { Vec::from([s]) }

    pub rule var_definition() -> Stmt =
        var()
        / constant()

        rule var() -> Stmt =
            let_kw()
            is_mut:optional_mut()
            name:ident()
            assign()
            e:expr() {
                Stmt::Var {
                    name: name.to_string(),
                    is_mut,
                    value: e
                }
            }
        rule constant() -> Stmt =
            const_kw()
            name:ident()
            assign()
            e:expr() {
                Stmt::Const {
                    name: name.to_string(),
                    value: e
                }
            }
        rule optional_mut() -> bool =
            m:(mut_kw() {})? { m.is_some() }

    // Rule for parsing any definitions
    rule definition() -> Stmt =
        function_definition()
        / var_definition()

    //
    // </DEFINITIONS>
    //

    // Root rule for parsing whole source
    pub rule module() -> Module =
        stmts:spaced(<stmts()>) {
            Module {
                statements: stmts
            }
        }
  }
}

pub fn parse(source: &str) -> Result<Module, ParseError<LineCol>> {
    parser::module(source)
}

#[cfg(test)]
mod tests {
    use crate::parser::ast::{Expr, FunctionParam, ImportedSymbol, Stmt, TypeUsage};

    use super::parser;

    #[test]
    fn parse_float() {
        assert_eq!(parser::float("3.14"), Ok(Expr::Float(3.14)))
    }

    #[test]
    fn parse_int() {
        assert_eq!(parser::int("2854"), Ok(Expr::Integer(2854)))
    }

    #[test]
    fn read_ident() {
        assert_eq!(parser::ident("input12345"), Ok("input12345"));
        assert_eq!(parser::ident("input"), Ok("input"));
    }

    #[test]
    fn string_literal() {
        assert_eq!(
            parser::string_literal(r#""icyh\"nln\" ""#),
            Ok("icyh\\\"nln\\\" ")
        )
    }
    #[test]
    fn import_stmt() {
        assert_eq!(
            parser::import_stmt(r#"import { a as b, c} from "./path/to/file.sk""#),
            Ok(Stmt::Import {
                symbols: vec![
                    ImportedSymbol {
                        name: "a".to_string(),
                        imported_as: Some("b".to_string())
                    },
                    ImportedSymbol {
                        name: "c".to_string(),
                        imported_as: None
                    }
                ],
                path: "./path/to/file.sk".to_string()
            })
        )
    }

    #[test]
    fn function_def_test() {
        assert_eq!(
            parser::function_definition("fn foo(bar: Baz<Foo>) {}"),
            Ok(Stmt::Function {
                name: "foo".to_string(),
                params: vec![FunctionParam::new(
                    "bar",
                    TypeUsage {
                        name: "Baz".to_string(),
                        params: vec![TypeUsage::from_name("Foo")]
                    }
                )],
                ret_type: TypeUsage::from_name("Unit"),
                body: Vec::new()
            })
        )
    }

    #[test]
    fn var_definition_test() {
        assert_eq!(
            parser::var_definition("let a = 1"),
            Ok(Stmt::Var {
                name: "a".to_string(),
                is_mut: false,
                value: Expr::Integer(1)
            })
        );
        assert_eq!(
            parser::var_definition("let mut a = 1"),
            Ok(Stmt::Var {
                name: "a".to_string(),
                is_mut: true,
                value: Expr::Integer(1)
            })
        );
        assert_eq!(
            parser::var_definition("const a = 1"),
            Ok(Stmt::Const {
                name: "a".to_string(),
                value: Expr::Integer(1)
            })
        );
    }
}
