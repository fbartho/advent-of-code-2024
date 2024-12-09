use crate::prelude::*;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use strum_macros::Display;

pub struct Day05;

impl Puzzle for Day05 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, _input: &str) -> super::PuzzleResult {
        let doc = parse_part1(_input);
        Ok(doc
            .middle_page_of_valid_updates()
            .iter()
            .sum::<DataType>()
            .to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}
type DataType = i32;
#[derive(Debug, Display)]
enum Expr {
    // Num(DataType),
    // PageOrderRule(Vec<DataType>),
    // PageOrderRules(Vec<Vec<DataType>>),
    // PageUpdateSequence(Vec<DataType>),
    // PageUpdates(Vec<Vec<DataType>>),
    Document(Part1Doc),
}
#[derive(Clone, Debug, Default)]
struct Part1Doc {
    order_rules: Vec<(DataType, DataType)>,
    updates_section: Vec<Vec<DataType>>,
}
impl Part1Doc {
    fn new(_order_rules: Vec<Vec<DataType>>, updates_section: Vec<Vec<DataType>>) -> Self {
        let order_rules: Vec<(DataType, DataType)> = _order_rules
            .iter()
            .filter_map(|rule| rule.iter().map(|b| *b).take(2).collect_tuple())
            .collect();
        Self {
            order_rules,
            updates_section,
        }
    }
    fn active_ordering_constraints(&self, update: &Vec<DataType>) -> Vec<(DataType, DataType)> {
        self.order_rules
            .iter()
            .filter(|(a, b)| update.contains(a) && update.contains(&b))
            .map(|r| *r)
            .collect()
    }
    fn update_is_in_order(&self, update: &Vec<DataType>) -> bool {
        let constraints = self.active_ordering_constraints(update);
        Part1Doc::verify(&constraints, update)
    }
    fn verify(constraints: &Vec<(DataType, DataType)>, update: &Vec<DataType>) -> bool {
        constraints
            .iter()
            .filter_map(|(a, b)| {
                let a_idx = update.iter().position(|e| e == a)?;
                let b_idx = update.iter().position(|e| e == b)?;
                Some((a_idx, b_idx))
            })
            .all(|(a, b)| a < b)
    }
    fn middle_page_of_valid_updates(&self) -> Vec<DataType> {
        let valid_updates: Vec<&Vec<DataType>> = self
            .updates_section
            .iter()
            .filter(|u| self.update_is_in_order(u))
            .collect();
        return valid_updates.iter().map(|u| middle(u)).collect();
    }
}
fn middle<T>(src: &Vec<T>) -> T
where
    T: Copy,
{
    if let Some(r) = src.get(src.len() / 2) {
        return *r;
    }
    panic!("No middle item in vec")
}

fn parser_part1<'a>() -> impl Parser<
    'a,
    &'a str,
    Expr,
    // Simple<&'a str>>
    chumsky::extra::Err<Rich<'a, char>>,
> {
    let int = text::int(10).map(|s: &str| s.parse::<DataType>().unwrap());

    let page_order_rule = int.separated_by(just('|')).exactly(2).collect::<Vec<_>>();
    // .map(|n| Box::new(Expr::PageOrderRule(n)));
    let page_update_seq = int.separated_by(just(',')).at_least(1).collect::<Vec<_>>();

    let page_order_rules = page_order_rule.separated_by(just('\n')).collect::<Vec<_>>();
    let page_updates_section = page_update_seq.separated_by(just('\n')).collect::<Vec<_>>();

    let doc = page_order_rules
        .then_ignore(just('\n').then(just('\n')))
        .then(page_updates_section)
        .padded()
        .map(|(order_rules, update_pages)| {
            Expr::Document(Part1Doc::new(order_rules, update_pages))
        });

    return doc;
}
fn convert_ast_into_doc(ast: &Expr) -> Result<Part1Doc, &str> {
    return match ast {
        Expr::Document(doc) => Result::Ok(doc.clone()),
        // _ => Result::Err("No document"),
    };
}
fn parse_part1(input: &str) -> Part1Doc {
    return match parser_part1().parse(input).into_result() {
        Ok(ast) => match convert_ast_into_doc(&ast) {
            Ok(output) => {
                println!("{:?}", output);
                output
            }
            Err(eval_err) => {
                println!("Evaluation error: {}", eval_err);
                panic!("Invalid Document")
            }
        },
        Err(parse_errs) => {
            // parse_errs
            //     .into_iter()
            //     .for_each(|e| println!("Parse error: {}", e));
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
    };
}

#[test]
fn sample_day05_1() {
    let input = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#
    .trim();
    println!("{:?}", Day05.part_one(input).unwrap());
}
