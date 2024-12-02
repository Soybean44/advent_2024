enum Change {
    Inc,
    Dec,
}
fn code(input: &str, allow_problem: i32) -> i32 {
    let lines = input.split('\n');
    let mut safe = 0;
    for line in lines {
        if is_line_safe(line) {
            safe += 1;
        }
    }
    return safe;
}

fn is_line_safe(line: &str) -> bool {
    let levels: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
    let change = if (levels[1] - levels[0]).signum() == 1 {
        Change::Inc
    } else {
        Change::Dec
    };
    for level in levels.windows(2) {
        let diff = level[1] - level[0];
        if diff.abs() > 3 || diff.abs() < 1 {
            return false;
        }
        match diff.signum() {
            1 => match change {
                Change::Inc => continue,
                Change::Dec => return false,
            },
            -1 => match change {
                Change::Inc => return false,
                Change::Dec => continue,
            },
            _ => panic!("should not reach here"),
        };
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn pt1_example() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let output = code(input, 0);
        println!("{}", output);
        assert!(output == 2);
    }
    #[test]
    fn pt1_test() {
        let input = include_str!("pt1.txt").trim_end();
        let output = code(input, 0);
        println!("{}", output);
        assert!(output == 490);
    }
    #[test]
    fn pt2_example() {
        let input = r#"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;
        let output = code(input, 1);
        println!("{}", output);
        assert!(output == 4);
    }
    //#[test]
    //fn pt2_test() {
    //    let input = include_str!("pt1.txt").trim_end();
    //    let output = code(input, 1);
    //    println!("{}", output);
    //    assert!(output == 26593248);
    //}
}
