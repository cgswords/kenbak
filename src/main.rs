use std::collections::BTreeMap;

mod input;
// mod introduce_call_conventions;
mod normalize_context;
mod shared;
mod simplify_values;

use crate::input::{Exp, Stmt};
// use crate::introduce_call_conventions::pass::Pass as icc;
use crate::normalize_context::pass::Pass as nc;
use crate::shared::ast::{Op, Program, Var};
use crate::simplify_values::pass::Pass as sv;

fn value(n: u8) -> Box<Exp> {
    Box::new(Exp::Value(shared::ast::Value::Int(n)))
}

fn var(x: &str) -> Box<Exp> {
    Box::new(Exp::Var(x.to_string()))
}

fn add(lhs: Box<Exp>, rhs: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::Binop(lhs, Op::Add, rhs))
}

fn sub(lhs: Box<Exp>, rhs: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::Binop(lhs, Op::Sub, rhs))
}

fn eq(lhs: Box<Exp>, rhs: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::Binop(lhs, Op::Eq, rhs))
}

fn call(subject: Box<Exp>, args: Vec<Exp>) -> Box<Exp> {
    Box::new(Exp::Call(subject, args))
}

fn let_(name: &str, rhs: Box<Exp>, body: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::Seq(vec![Stmt::Let(name.to_string(), rhs)], body))
}

fn if_(test: Box<Exp>, conseq: Box<Exp>, alt: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::If(test, conseq, alt))
}

/*
fn test000() {
    let e = call(
        call(var("t"), vec![]),
        vec![*add(add(value(10), value(20)), value(30))],
    );
    let funcs = BTreeMap::from([("fn".to_string(), *e)]);
    let program = Program { funcs };
    println!("\ninput program: {:#?}", program);
    let Program { funcs } = icc::run(fs::run(sv::run(program)));
    for (name, body) in funcs {
        println!("name: {name}");
        let _ = body.to_doc().render(100, &mut std::io::stdout());
    }
}
*/
fn test001() {
    let e_inner = call(
        call(var("t"), vec![]),
        vec![*add(add(value(10), value(20)), value(30))],
    );
    let e = if_(e_inner.clone(), e_inner.clone(), e_inner);

    let funcs = BTreeMap::from([("fn".to_string(), *e)]);
    let program = Program { funcs };
    println!("\ninput program: {:?}", program);
    let Program { funcs } = sv::run(nc::run(program));
    for (name, body) in funcs {
        println!("name: {name}");
        println!("body: {:#?}", body);
    }
}

fn test002() {
    let n = var("n");
    let e = if_(
        eq(n.clone(), value(0)),
        value(1),
        if_(
            eq(n.clone(), value(1)),
            value(1),
            add(
                call(var("fib"), vec![*sub(n.clone(), value(1))]),
                call(var("fib"), vec![*sub(n.clone(), value(2))]),
            ),
        ),
    );
    let funcs = BTreeMap::from([("fib".to_string(), *e)]);
    let program = Program { funcs };
    println!("\ninput program: {:?}\n", program);
    // let Program { funcs } = icc::run(fs::run(sv::run(program)));
    let Program { funcs } = sv::run(nc::run(program));
    for (name, body) in funcs {
        println!("name: {name}");
        println!("body: {:#?}", body);
    }
}

fn test003() {
    let n = var("n");
    let e = if_(
        eq(n.clone(), value(0)),
        value(1),
        if_(
            eq(n.clone(), value(1)),
            value(1),
            let_(
                "rec",
                add(
                    call(var("fib"), vec![*sub(n.clone(), value(1))]),
                    call(var("fib"), vec![*sub(n.clone(), value(2))]),
                ),
                add(
                    var("rec"),
                    call(var("fib"), vec![*sub(n.clone(), value(2))]),
                ),
            ),
        ),
    );
    let funcs = BTreeMap::from([("fib".to_string(), *e)]);
    let program = Program { funcs };
    println!("\ninput program: {:?}\n", program);
    // let Program { funcs } = icc::run(fs::run(sv::run(program)));
    let Program { funcs } = sv::run(nc::run(program));
    for (name, body) in funcs {
        println!("name: {name}");
        println!("body: {:#?}", body);
    }
}

fn main() {
    // test000();
    // test001();
    test003();
}
