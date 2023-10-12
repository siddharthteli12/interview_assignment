pub fn arithmetic_parser(value_str: String) -> u64 {
    let value_chars: Vec<char> = value_str.chars().collect();
    let mut operator_stack = vec![];
    let mut operand_stack: Vec<f64> = vec![];
    let mut index = 0;
    while index < value_str.len() {
        if value_chars[index].is_alphabetic() {
            match value_chars[index] {
                'f' => loop {
                    let operator = operator_stack.pop().unwrap();
                    if operator == 'e' {
                        break;
                    }
                    let num1 = operand_stack.pop().unwrap();
                    let num2 = operand_stack.pop().unwrap();
                    let result = perform_operation(num2, num1, operator);
                    operand_stack.push(result);
                },
                value => {
                    if value == 'e'
                        || operator_stack.is_empty()
                        || *operator_stack.last().unwrap() == 'e'
                    {
                        operator_stack.push(value);
                    } else {
                        let operator = operator_stack.pop().unwrap();
                        let num1 = operand_stack.pop().unwrap();
                        let num2 = operand_stack.pop().unwrap();
                        let result = perform_operation(num2, num1, operator);
                        operand_stack.push(result);
                        operator_stack.push(value);
                    }
                }
            };
            index += 1;
        } else {
            let mut temp_index = index + 1;
            while temp_index < value_chars.len() && value_chars[temp_index].is_numeric() {
                temp_index += 1;
            }
            operand_stack.push(value_str[index..temp_index].parse::<f64>().unwrap());
            index = temp_index;
        }
    }
    if let Some(operator) = operator_stack.pop() {
        let num1 = operand_stack.pop().unwrap();
        let num2 = operand_stack.pop().unwrap();
        let result = perform_operation(num2, num1, operator);
        operand_stack.push(result);
    }

    operand_stack
        .pop()
        .map(|value| value as u64)
        .unwrap_or_default()
}

fn perform_operation(num1: f64, num2: f64, opeartor: char) -> f64 {
    match opeartor {
        'a' => num1 + num2,
        'b' => num1 - num2,
        'c' => num1 * num2,
        'd' => num1 / num2,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(arithmetic_parser(String::new()), 0);
    }

    #[test]
    fn test_simple_input_1() {
        let input = "3a2c4";
        assert_eq!(arithmetic_parser(input.to_string()), 20);
    }

    #[test]
    fn test_simple_input_3() {
        let input = "500a10b66c32";
        assert_eq!(arithmetic_parser(input.to_string()), 14208);
    }

    #[test]
    fn test_simple_input_4() {
        let input = "3ae4c66fb32";
        assert_eq!(arithmetic_parser(input.to_string()), 235);
    }

    #[test]
    fn test_simple_input_5() {
        let input = "3c4d2aee2a4c41fc4f";
        assert_eq!(arithmetic_parser(input.to_string()), 990);
    }
}
