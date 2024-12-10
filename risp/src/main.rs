// Basic expressions, no environments yet
// `cargo run`
use std::num::ParseFloatError;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum RispExp {
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
}

#[derive(Debug)]
enum RispErr {
    Reason(String),
}

// storing variables, built-in funcs, etc
#[derive(Clone)]
struct RispEnv {
    data: HashMap<String, RispExp>,
}

fn main() {
    let input = "(+ 1 2 (3 4))".to_string();
    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens);

    match parse(&tokens) {
        Ok((exp, _)) => println!("Parsed Expression: {:?}", exp),
        Err(err) => println!("Error: {:?}", err),
    }
}

fn tokenize(expr: String) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn parse<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(RispErr::Reason("could not get token".to_string()))?;

    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(RispErr::Reason("unexpected ')'".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

// read until we hit a ')', sequentially adding to a list of expressions
fn read_seq<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
  let mut res: Vec<RispExp> = vec![];
  let mut xs = tokens;

  loop {
    let (next_token, rest) = xs
        .split_first()
        .ok_or(RispErr::Reason("could not find closing `)`".to_string()))
        ?;
    
    if next_token == ")" {
      return Ok((RispExp::List(res), rest));
    }

    let (exp, new_xs) = parse(&xs)?;
    res.push(exp);
    xs = new_xs;
  }
}

fn parse_atom(token: &str) -> RispExp {
  let potential_float: Result<f64, ParseFloatError> = token.parse();

  match potential_float {
    Ok(v) => RispExp::Number(v),
    Err(_) => RispExp::Symbol(token.to_string().clone())
  }
}