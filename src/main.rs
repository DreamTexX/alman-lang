extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

use std::collections::HashMap;
use pest::{Parser};
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "alman.pest"]
struct AlmanLangParser;

fn main() {
    let program = AlmanLangParser::parse(Rule::Program, include_str!("../main.alman"))
        .expect("Parsing failed")
        .next()
        .expect("No program found");

    let mut stack: HashMap<&str, usize> = HashMap::new();

    for statement in program.into_inner() {
        parse_statement(statement, &mut stack)
    }
}

fn parse_statement<'a>(statement: Pair<'a, Rule>, stack: &mut HashMap<&'a str, usize>) {
    match statement.as_rule() {
        Rule::Declaration => {
            let mut declaration = statement.into_inner();
            let name = declaration.next().unwrap().as_str();
            let value = declaration.next().unwrap().as_str().parse::<usize>().unwrap();
            stack.insert(name, value);
        }
        Rule::If => {
            let mut statement = statement.into_inner();
            let mut condition = statement.next().unwrap().into_inner();
            let block = statement.next().unwrap();

            let var = condition.next().unwrap().as_str();
            let val = stack.get(var).expect("Variable does not exist");

            let op = condition.next().unwrap().as_str();
            let num = &(condition.next().unwrap().as_str().parse::<usize>().unwrap());
            let ok = match op {
                "<" => { val < num }
                ">" => { val > num }
                "==" => { val == num }
                "<=" => { val <= num }
                ">=" => { val >= num }
                "!=" => { val != num }
                _ => { false }
            };
            if ok {
                for statement in block.into_inner() {
                    parse_statement(statement, stack);
                }
            }
        }
        Rule::PrintLn => {
            let mut statement = statement.into_inner();
            let variable = statement.next().unwrap().as_str();
            match stack.get(variable) {
                Some(val) => { println!("{}", val) }
                None => { panic!("Undefined variable {}", variable) }
            }
        }
        Rule::Print => {
            let mut statement = statement.into_inner();
            let variable = statement.next().unwrap().as_str();
            match stack.get(variable) {
                Some(val) => { print!("{}", val) }
                None => { panic!("Undefined variable {}", variable) }
            }
        }
        Rule::EOI => {}
        _ => { println!("not implemented: {}", statement) }
    }
}