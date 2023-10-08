use std::str::FromStr;

use crate::action::{Action, BreakPointAction, MemoryAction, Value};
use num_traits::PrimInt;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "cpcemu.pest"]
pub struct CpcParser;

fn get_decimal_number<N: PrimInt + FromStr>(pair: &Pair<Rule>) -> Option<N> {
    assert_eq!(pair.as_rule(), Rule::dec_number);

    match pair.as_rule() {
        Rule::dec_number => Some(pair.as_span().as_str().parse::<N>().unwrap_or(N::zero())),
        _ => None,
    }
}

fn get_address(pair: Pair<Rule>) -> u16 {
    assert_eq!(pair.as_rule(), Rule::address);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::dec_number => get_decimal_number::<u16>(&pair).unwrap(),
        _ => unreachable!(),
    }
}

fn get_value<T: PrimInt + FromStr>(pair: Pair<Rule>) -> T {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::dec_number => get_decimal_number::<T>(&pair).unwrap(),
        _ => unreachable!(),
    }
}

fn get_label(pair: Pair<Rule>) -> String {
    assert_eq!(Rule::label, pair.as_rule());
    match pair.as_rule() {
        Rule::label => pair.as_span().as_str().to_string(),
        _ => unreachable!(),
    }
}

fn get_label_or_address(pair: Pair<Rule>) -> Value<u16> {
    assert_eq!(pair.as_rule(), Rule::label_or_address);
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::address => get_address(pair).into(),
        Rule::label => Value::from_label(get_label(pair)),
        _ => unreachable!(),
    }
}

fn get_label_or_value<T: PrimInt + FromStr>(pair: Pair<Rule>) -> Value<T> {
    assert_eq!(Rule::label_or_value, pair.as_rule());

    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::value => get_value::<T>(pair).into(),
        Rule::label => Value::from_label(get_label(pair)),
        _ => unreachable!(),
    }
}

pub fn get_action(pair: Pair<Rule>) -> Action {
    assert_eq!(pair.as_rule(), Rule::command);

    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::memory => Action::Memory(get_memory_action(pair)),
        Rule::breakpoint => Action::BreakPoint(get_breakpoint_action(pair)),
        _ => unreachable!(),
    }
}

fn get_breakpoint_action(pair: Pair<Rule>) -> BreakPointAction {
    assert_eq!(pair.as_rule(), Rule::breakpoint);
    let pair = pair.into_inner().next().unwrap();
    assert_eq!(pair.as_rule(), Rule::breakpoint_action);
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::breakpoint_action_add => {
            let label_or_address = get_label_or_address(pair.into_inner().next().unwrap());
            BreakPointAction::AddAt {
                dest: label_or_address,
                cond: None,
            }
        }

        _ => unreachable!(),
    }
}

fn get_memory_action(pair: Pair<Rule>) -> MemoryAction {
    assert_eq!(pair.as_rule(), Rule::memory);
    let pair = pair.into_inner().next().unwrap();
    assert_eq!(pair.as_rule(), Rule::memory_action);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::memory_action_read => {
            let label_or_address = get_label_or_address(pair.into_inner().next().unwrap());
            MemoryAction::Read {
                address: label_or_address,
                amount: 1.into(),
            }
        }

        Rule::memory_action_write => {
            let mut inner = pair.into_inner();
            let address = inner.next().unwrap();
            let value = inner.next().unwrap();

            let address = get_label_or_address(address).into();
            let value = get_label_or_value::<u8>(value).into();
            MemoryAction::Write { address, value }
        }
        _ => unreachable!("{:?}", pair),
    }
}
