enum Change {
    None,
    Inc,
    Dec,
}
fn code(input: &str, allow_problem: i32) -> i32 {
    let lines = input.split('\n');
    let mut safe = 0;
    for line in lines {
        let levels: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();

        let mut is_safe = true;
        let mut change = Change::None;

        for level in levels.windows(2) {
            let diff = level[1] - level[0];
            if diff.abs() > 3 || diff.abs() < 1 {
                is_safe = false;
            }
            match change {
                Change::None => change = if (diff) > 0 { Change::Inc } else { Change::Dec },
                Change::Inc => {
                    if (diff) > 0 {
                    } else {
                        is_safe = false;
                    }
                }
                Change::Dec => {
                    if (diff) > 0 {
                        is_safe = false;
                    } else {
                    }
                }
            }
        }
        if is_safe {
            safe += 1;
        }
    }
    return safe;
}

fn pt2(input: &str) -> i32 {
    0
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
