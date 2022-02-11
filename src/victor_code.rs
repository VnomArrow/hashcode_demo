// Add parser

#[allow(unused_macros)]
macro_rules! get_input {
    ($inputoutput:expr) => {
        std::str::from_utf8(&std::fs::read($filename).unwrap())
        .unwrap()
        .lines()
    };
}

pub fn parser() {
    for line in get_input!("input.in") {
        /*if line == "" {
            break;
        }*/
        let mut line_split = line.split_whitespace();

        /*let val1 = line_split.next().unwrap().parse::<i32>().unwrap();
        let operator = line_split.next().unwrap().clone();
        let val2 = line_split.next().unwrap().parse::<i32>().unwrap();
        if operator == "+" {
            let val = add(val1, val2);
            input_output.print(val.to_string() + "\n");
        }
        else if operator == "*" {
            let val = multiply(val1, val2);
            input_output.print(val.to_string() + "\n");
        }
        else if operator == "^" {
            let val = pow(val1, val2);
            input_output.print(val.to_string() + "\n");
        }*/
    }
}