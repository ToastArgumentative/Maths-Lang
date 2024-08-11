use std::collections::HashMap;

use super::keyphrases::{handle_if, handle_let_assignment, handle_return, handle_throw};
use super::math_expr::evaluate_expression;
use super::values::expressions::Expression;
use super::values::variables::Variables;



pub fn parse_expression_file(input: &str) -> Variables {
    let mut variables = Variables {
        string_vars: HashMap::new(),
        expr_vars: HashMap::new(),
    };

    let lines = input
        .lines()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    for line in &lines {
        handle_line(line, &mut variables, &lines);
    }

    Variables {
        expr_vars: variables.expr_vars,
        string_vars: variables.string_vars,
    }
}

pub fn handle_line(line: &str, variables: &mut Variables, lines: &Vec<&str>) {
    let last_line = lines.last().unwrap_or(&"").to_owned();
    if line.starts_with("let") {
        handle_let_assignment(line, variables);
    } else if line.starts_with(";") {
        handle_return(line, variables, lines.last().unwrap_or(&""));
    } else if line.starts_with("throw") {
        handle_throw(line, variables);
    } else if line.starts_with("???") {
        handle_if(line, variables, &lines)
    }
}



pub fn calculate_sequence(sequence: &mut Variables) -> Box<Expression> {
    if let Some(expr) = sequence.expr_vars.get("result") {
        let variables = Variables {
            expr_vars: sequence.expr_vars.clone(),
            string_vars: sequence.string_vars.clone(),
        };
        return evaluate_expression(expr, &variables)
            .expect("Error evaluating expression");
    }

    Box::new(Expression::Number(0.0)) // Return 0 if 'result' is not defined
}
