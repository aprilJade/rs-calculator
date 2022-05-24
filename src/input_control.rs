pub fn check_exp_validation(exp: String) -> bool {
    let buf: Vec<&str> = exp.as_str().split(" ").collect();
    let mut op_cnt = 0;
    let mut num_cnt = 0;
    for val in buf.iter() {
        match *val {
            "+" | "-" | "x" | "÷"=> op_cnt += 1,
            _ => {
                let str_num = String::from(*val).parse::<f64>();
                match str_num {
                    Ok(_num) => num_cnt += 1,
                    Err(_e) => eprintln!("is not number, operator")
                }
            }
        }
    }
    op_cnt + 1 == num_cnt
} 

fn is_number(value: &str) -> bool {
    let test = value.parse::<f64>();
    match test {
        Ok(_num) => true,
        Err(_err) => false
    }
}

pub fn can_input_float_point(value: &str) -> bool {
    let mut data: Vec<&str> = value.split(" ").collect();
    let last_word = data.pop().unwrap();
    if is_number(last_word) == false {
        return false
    }
    let last_char: Vec<&str> = last_word.split("").collect();
    for c in last_char.iter() {
        if (*c).eq(".") {
            return false
        }
    }
    true
}