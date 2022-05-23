pub fn calc(expression: String) -> String {
    let mut postfix_exp: Vec<&str> = Vec::new();
    let mut operator_stack: Vec<&str> = Vec::new();

    let tokens: Vec<&str> = expression.as_str().split(' ').collect();
    for value in tokens.iter() {
        match String::from(*value).parse::<f64>() {
            Ok(num) => postfix_exp.push(*value),
            _ => {
                if (*value).eq("+") || (*value).eq("-") {
                    while operator_stack.is_empty() == false {
                        postfix_exp.push(operator_stack.pop().unwrap());
                    }
                    operator_stack.push(*value);
                } else {
                    operator_stack.push(*value)
                }
            }
        }
    }
    while operator_stack.is_empty() == false {
        postfix_exp.push(operator_stack.pop().unwrap());
    }

    let mut calc_stack: Vec<f64> = Vec::new();
    for value in postfix_exp.iter() {
        match String::from(*value).parse::<f64>() {
            Ok(num) => {
                calc_stack.push(num);
            },
            _ => {
                let num2 = calc_stack.pop().unwrap();
                let num1 = calc_stack.pop().unwrap();
                if (*value).eq("+") {
                    calc_stack.push(num1 + num2);
                } else if (*value).eq("-") {
                    calc_stack.push(num1 - num2);
                } else if (*value).eq("x") {
                    calc_stack.push(num1 * num2);
                } else if (*value).eq("÷") {
                    calc_stack.push(num1 / num2);
                }
            }
        }
    }

    return String::from(calc_stack.pop().unwrap().to_string()) 
}