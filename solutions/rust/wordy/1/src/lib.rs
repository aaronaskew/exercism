use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{i32, multispace0},
    combinator::map,
    multi::many_till,
    sequence::preceded,
};

#[derive(Debug)]
enum Element {
    Number(i32),
    Operation(Operation),
}

#[derive(Debug)]
enum Operation {
    Add(i32),
    Subtract(i32),
    Multiply(i32),
    Divide(i32),
    Exponent(i32),
}

fn operation(input: &str) -> IResult<&str, Element> {
    alt((
        map(Parser::into((tag("plus "), i32)), |(_, n)| {
            Element::Operation(Operation::Add(n))
        }),
        map(Parser::into((tag("minus "), i32)), |(_, n)| {
            Element::Operation(Operation::Subtract(n))
        }),
        map(Parser::into((tag("multiplied by "), i32)), |(_, n)| {
            Element::Operation(Operation::Multiply(n))
        }),
        map(Parser::into((tag("divided by "), i32)), |(_, n)| {
            Element::Operation(Operation::Divide(n))
        }),
        map(
            Parser::into((tag("raised to the "), i32, take_until(" "), tag(" power"))),
            |(_, n, _, _)| Element::Operation(Operation::Exponent(n)),
        ),
    ))
    .parse(input)
}

fn number(input: &str) -> IResult<&str, Element> {
    i32(input).map(|(input, n)| (input, Element::Number(n)))
}

fn element(input: &str) -> IResult<&str, Element> {
    preceded(multispace0, alt((number, operation))).parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Element>> {
    let (input, (elements, _)) =
        preceded(tag("What is "), many_till(element, tag("?"))).parse(input)?;

    Ok((input, elements))
}

pub fn answer(command: &str) -> Option<i32> {
    if let Ok((input, elements)) = parse(command) {
        dbg!(&input, &elements);

        let mut answer;

        if let Some(Element::Number(number)) = elements.first() {
            answer = *number;
        } else {
            return None;
        }

        for element in elements.iter().skip(1) {
            if let Element::Operation(operation) = element {
                match operation {
                    Operation::Add(n) => answer += n,
                    Operation::Subtract(n) => answer -= n,
                    Operation::Multiply(n) => answer *= n,
                    Operation::Divide(n) => answer /= n,
                    Operation::Exponent(n) => answer = answer.pow(*n as u32),
                }
            } else {
                return None;
            }
        }

        Some(answer)
    } else {
        None
    }
}
