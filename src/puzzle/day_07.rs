#[allow(dead_code)]
use crate::prelude::*;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use std::borrow::Borrow;
use std::borrow::Cow;

pub struct Day07;

impl Puzzle for Day07 {
	fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
		Box::new(Self)
	}

	fn part_one(&self, _input: &str) -> super::PuzzleResult {
		let equation_list = parse(_input);
		println!(
			"{}",
			equation_list.iter().map(|eq| eq.to_string()).join("\n")
		);
		let solvable_equations: Vec<&TestEquation> = equation_list
			.iter()
			.filter(|eq| eq.is_solvable_recursive(false))
			.collect();
		let _ = dbg!(solvable_equations.len());
		println!(
			"{}",
			solvable_equations
				.iter()
				.map(|eq| eq.to_string())
				.join("\n")
		);
		return Ok(solvable_equations
			.iter()
			.map(|eq| eq.result)
			.sum::<DataType>()
			.to_string());
	}

	fn part_two(&self, _input: &str) -> super::PuzzleResult {
		let equation_list = parse(_input);
		println!(
			"{}",
			equation_list.iter().map(|eq| eq.to_string()).join("\n")
		);
		let solvable_equations: Vec<&TestEquation> = equation_list
			.iter()
			.filter(|eq| eq.is_solvable_recursive(true))
			.collect();
		let _ = dbg!(solvable_equations.len());
		println!(
			"{}",
			solvable_equations
				.iter()
				.map(|eq| eq.to_string())
				.join("\n")
		);
		return Ok(solvable_equations
			.iter()
			.map(|eq| eq.result)
			.sum::<DataType>()
			.to_string());
	}
}
type DataType = u64;
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, Display, Hash)]
enum Operator {
	#[strum(serialize = "+")]
	Add,
	#[strum(serialize = "*")]
	Mul,
	#[strum(serialize = "/")]
	Divide,
	#[strum(serialize = "||")]
	Concat,
}
impl Operator {
	fn apply(&self, a: DataType, b: &DataType) -> DataType {
		match self {
			Operator::Add => a + b,
			Operator::Mul => a * b,
			Operator::Divide => a / b,
			Operator::Concat => concat_numeric_op(a, *b),
		}
	}
}
fn concat_numeric_op(a: DataType, b: DataType) -> DataType {
	(a.to_string() + &b.to_string())
		.parse::<DataType>()
		.unwrap()
}
struct TestEquation<'a> {
	/// Expected Result
	result: DataType,
	numerals: Cow<'a, Vec<DataType>>,
	operators: Cow<'a, Vec<Operator>>,
}
#[allow(dead_code)]
impl<'a> TestEquation<'a> {
	fn new(result: DataType, numerals: Vec<DataType>) -> Self {
		Self {
			result,
			numerals: Cow::Owned(numerals),
			operators: Cow::Owned(Vec::new()),
		}
	}
	fn clone_with_operators(&'a self, operators: &'a Vec<Operator>) -> Self {
		Self {
			result: self.result,
			numerals: Cow::Borrowed(self.numerals.borrow()),
			operators: Cow::Borrowed(operators),
		}
	}
	fn to_string(&self) -> String {
		format!(
			"{}{}{}",
			self.result.to_string(),
			String::from(": "),
			itertools::join(
				self.numerals
					.iter()
					.map(|n| n.to_string())
					.interleave(self.operators.iter().map(|op| op.to_string())),
				" ",
			)
		)
	}
	fn calculate(&self) -> DataType {
		let mut accum: DataType = 1;
		let mut op = &Operator::Mul;
		for (idx, current) in self.numerals.iter().enumerate() {
			accum = op.apply(accum, current);
			if let Some(_op) = self.operators.get(idx) {
				op = _op
			} else {
				if self.operators.len() != idx {
					panic!(
						"Insufficient Operands {:?} vs {:?}",
						self.numerals, self.operators
					)
				}
			}
		}
		return accum;
	}
	fn is_valid(&self) -> bool {
		println!("Testing {}", self.to_string());
		let mut accum: DataType = 1;
		let mut op = &Operator::Mul;
		for (idx, current) in self.numerals.iter().enumerate() {
			accum = op.apply(accum, current);
			if accum > self.result {
				// Early exit when it can't be reached from here
				return false;
			}
			if let Some(_op) = self.operators.get(idx) {
				op = _op
			} else {
				if self.operators.len() != idx {
					panic!(
						"Insufficient Operands {:?} vs {:?}",
						self.numerals, self.operators
					)
				}
			}
		}
		return self.result == accum;
	}
	fn is_solvable_recursive(&self, support_concat: bool) -> bool {
		let (first, remainder) = self
			.numerals
			.split_first()
			.expect("Don't know how to test empty equation");
		if support_concat {
			return num_reachable_via_add_mul_concat(self.result, *first, remainder);
		} else {
			return num_reachable_via_add_mul(self.result, *first, remainder);
		}
	}
}
fn num_reachable_via_add_mul(target: DataType, accum: DataType, remainder: &[DataType]) -> bool {
	if target == accum && remainder.is_empty() {
		return true;
	}
	if remainder.is_empty() || accum > target {
		return false;
	}
	let (next_num, next_remainder) = remainder.split_first().expect("Ran out of numbers??");
	return num_reachable_via_add_mul(target, accum * next_num, next_remainder)
		|| num_reachable_via_add_mul(target, accum + next_num, next_remainder);
}

fn num_reachable_via_add_mul_concat(
	target: DataType,
	accum: DataType,
	remainder: &[DataType],
) -> bool {
	if target == accum && remainder.is_empty() {
		return true;
	}
	if remainder.is_empty() || accum > target {
		return false;
	}
	let (next_num, next_remainder) = remainder.split_first().expect("Ran out of numbers??");
	return num_reachable_via_add_mul_concat(target, accum * next_num, next_remainder)
		|| num_reachable_via_add_mul_concat(target, accum + next_num, next_remainder)
		|| num_reachable_via_add_mul_concat(
			target,
			concat_numeric_op(accum, *next_num),
			next_remainder,
		);
}

fn parser<'a>(
) -> impl Parser<'a, &'a str, Vec<TestEquation<'a>>, chumsky::extra::Err<Rich<'a, char>>> {
	let int = text::int(10).map(|s: &str| s.parse::<DataType>().unwrap());

	let numerals = int.separated_by(just(' ')).at_least(1).collect::<Vec<_>>();
	let equation = int
		.then_ignore(just(':').padded())
		.then(numerals)
		.map(|(result, numerals)| TestEquation::new(result, numerals.clone()));

	let problem_list = equation.separated_by(just('\n')).collect::<Vec<_>>();
	return problem_list;
}
fn parse(input: &str) -> Vec<TestEquation<'_>> {
	match parser().parse(input.trim()).into_result() {
		Ok(equation_list) => equation_list,
		Err(parse_errs) => {
			parse_errs.into_iter().for_each(|e| {
				Report::build(ReportKind::Error, (), e.span().start)
					.with_message(e.to_string())
					.with_label(
						Label::new(e.span().into_range())
							.with_message(e.reason().to_string())
							.with_color(Color::Red),
					)
					.finish()
					.print(Source::from(&input))
					.unwrap()
			});
			panic!("Couldn't parse ast, cannot proceed")
		}
	}
}
#[test]
fn sample_day07_1() {
	let input = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
	println!("{:?}", Day07.part_one(input).unwrap());
}
