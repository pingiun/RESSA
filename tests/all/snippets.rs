use resast::prelude::*;
use ressa::*;
use std::borrow::Cow;
#[test]
fn doc1() {
    let js = "function helloWorld() { alert('Hello world'); }";
    let p = Parser::new(&js).unwrap();
    let f = ProgramPart::decl(Decl::Func(Func {
        id: Some(Ident::from("helloWorld")),
        params: vec![],
        body: FuncBody(vec![ProgramPart::Stmt(Stmt::Expr(Expr::Call(CallExpr {
            callee: Box::new(Expr::ident_from("alert")),
            arguments: vec![Expr::Lit(Lit::single_string_from("Hello world"))],
        })))]),
        generator: false,
        is_async: false,
    }));
    for part in p {
        assert_eq!(part.unwrap(), f);
    }
}

#[test]
fn readme_iter_example() {
    let js = "function helloWorld() { alert('Hello world'); }";
    let p = Parser::new(&js).unwrap();
    let f = ProgramPart::decl(Decl::Func(Func {
        id: Some(Ident::from("helloWorld")),
        params: vec![],
        body: FuncBody(vec![ProgramPart::Stmt(Stmt::Expr(Expr::Call(CallExpr {
            callee: Box::new(Expr::ident_from("alert")),
            arguments: vec![Expr::Lit(Lit::String(StringLit::Single(Cow::Owned(
                "Hello world".to_string(),
            ))))],
        })))]),
        generator: false,
        is_async: false,
    }));
    for part in p {
        assert_eq!(part.unwrap(), f);
    }
}

#[test]
fn arrow_func_args() {
    let js = "(a, b = 0, [c,, d = 0, ...e], {f, g: h, i = 0, i: j = 0}, ...k) => {;};";
    let mut parser = Parser::new(&js).unwrap();
    let _parsed = parser.parse().unwrap();
}

#[test]
fn destructuring_default() {
    let _ = env_logger::try_init();
    let js = "[a = {y: 2}, a.x = 1] = [];";
    let mut parser = Parser::new(js).expect("failed to create parser");
    parser.parse().expect("failed to parser js");
}
#[test]
fn destructuring_obj() {
    let _ = env_logger::try_init();
    let js = "0, [...{} [throwlhs()]] = iterable;";
    let mut parser = Parser::new(js).expect("failed to create parser");
    parser.parse().expect("failed to parser js");
}

#[test]
fn strict_global_yield() {
    let _ = env_logger::try_init();
    let js = "'use strict'
yield;
";
    let mut parser = Parser::new(js).expect("failed to create parser");
    let expect = parser.parse();
    if let Err(ressa::Error::NonStrictFeatureInStrictContext(_, _)) = expect {
        ()
    } else {
        panic!("Incorrectly parsed reserved word as identifier");
    }
}

#[test]
fn new_line_in_fat_arrow() {
    let js = "var af = x
=> x;";
    let mut parser = Parser::new(js).expect("failed to create parser");
    let expect = parser.parse();
    if let Err(ressa::Error::NewLineAfterFatArrow(_)) = expect {
        ()
    } else {
        panic!(
            "Incorrectly parsed arrow function with new line after =>\n{:?}",
            expect
        );
    }
}

#[test]
fn arguments_as_param_arrow() {
    let _ = env_logger::try_init();
    let js = "'use strict';
var x = arguments => arguments;";
    let mut parser = Parser::new(js).expect("failed to create parser");
    let expect = parser.parse();
    if let Err(ressa::Error::StrictModeArgumentsOrEval(_)) = expect {
        ()
    } else {
        panic!("Incorrectly parsed arguments as param in strict mode");
    }
}

#[test]
fn duplicate_proto() {
    let js = "({
__proto__: Number,
'__proto__': Number,
});";
    let mut parser = Parser::new(js).expect("failed to create parser");
    let expect = parser.parse();
    if let Err(ressa::Error::Redecl(_, _)) = expect {
        ()
    } else {
        panic!("Incorrectly parsed multiple __proto__ properties");
    }
}
#[test]
#[should_panic = "expected to fail on super call in function"]
fn super_in_func() {
    let _ = env_logger::try_init();
    let js = "function() { super() }";
    let mut p = Parser::new(js).unwrap();
    p.parse().expect("expected to fail on super call in function");
}

#[test]
fn super_in_ctor() {
    let _ = env_logger::try_init();
    let js = "
class A {}
class B extends A {
    constructor() {
        super()
    }
}";
    let mut p = Parser::new(js).unwrap();
    p.parse().expect("failed to handle super call in ctor");
}

#[test]
fn super_in_method() {
    let _ = env_logger::try_init();
    let js = "
class A {}
class B extends A {
    thing() {
        return super.stuff;
    }
}";
    let mut p = Parser::new(js).unwrap();
    p.parse().expect("failed to handle super property in method");
}

#[test]
#[should_panic = "base classes can't refer to super"]
fn super_in_ctor_neg() {
    let _ = env_logger::try_init();
    let js = "
class B {
    constructor() {
        super()
    }
}";
    let mut p = Parser::new(js).unwrap();
    p.parse().expect("base classes can't refer to super");
}

#[test]
#[should_panic = "base classes can't refer to super"]
fn super_in_method_prop_neg() {
    let _ = env_logger::try_init();
    let js = "
class B {
    thing() {
        return super.stuff;
    }
}";
    let mut p = Parser::new(js).unwrap();
    p.parse().expect("base classes can't refer to super");
}
#[test]
#[should_panic = "super calls should only be allowed in ctors"]
fn super_in_method_neg() {
    let _ = env_logger::try_init();
    let js = "
class A {}
class B {
    thing() {
        super();
    }
}";
    let mut p = Parser::new(js).unwrap();
    p.parse().expect("super calls should only be allowed in ctors");
}
#[test]
#[should_panic = "super is invalid in an arrow"]
fn super_in_arrow_func() {
    let _ = env_logger::try_init();
    let js = "() => super();";
    let mut p = Parser::new(js).unwrap();
    p.parse().expect("super is invalid in an arrow");
}
#[test]
fn super_in_lit_getter() {
    let _ = env_logger::try_init();
    let js = "({
get a() { return super.stuff; }
});";
    let mut p = Parser::new(js).unwrap();
    p.parse().unwrap();
}
#[test]
fn line_term_comment() {
    let _ = env_logger::try_init();
    let js = "''/*
*/''";
    let mut parser = Parser::new(js).expect("failed to create parser");
    parser.parse().unwrap();
}
