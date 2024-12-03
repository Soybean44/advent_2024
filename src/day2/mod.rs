enum Change {
    Inc,
    Dec,
}
fn pt1(input: &str) -> i32 {
    let lines = input.split('\n');
    let mut safe = 0;
    for line in lines {
        let levels: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        if is_line_safe(levels) {
            safe += 1;
        }
    }
    return safe;
}

fn pt2(input: &str) -> i32 {
    let lines = input.split('\n');
    let mut safe = 0;
    for line in lines {
        let levels: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        if is_line_less_safe(levels) {
            safe += 1;
        }
    }
    return safe;
}

fn pair_safe(l1: i32, l2: i32, change: &Change) -> bool {
    let diff = l2 - l1;
    if diff.abs() > 3 || diff.abs() < 1 {
        return false;
    }
    match diff.signum() {
        1 => match change {
            Change::Inc => return true,
            Change::Dec => return false,
        },
        -1 => match change {
            Change::Inc => return false,
            Change::Dec => return true,
        },
        _ => panic!("should not reach here"),
    };
}

fn is_line_safe(levels: Vec<i32>) -> bool {
    let change = if (levels[1] - levels[0]).signum() == 1 {
        Change::Inc
    } else {
        Change::Dec
    };
    for i in 0..levels.len() - 1 {
        if !pair_safe(levels[i], levels[i + 1], &change) {
            return false;
        }
    }
    return true;
}

fn is_line_less_safe(levels: Vec<i32>) -> bool {
    let change = if (levels[1] - levels[0]).signum() == 1 {
        Change::Inc
    } else {
        Change::Dec
    };
    for i in 0..levels.len() - 1 {
        if !pair_safe(levels[i], levels[i + 1], &change) {
            let mut l1 = levels.clone();
            l1.remove(i);
            let mut l2 = levels.clone();
            l2.remove(i + 1);
            if !is_line_safe(l1) && !is_line_safe(l2) {
                return false;
            }
        }
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
        let output = pt1(input);
        println!("{}", output);
        assert!(output == 2);
    }
    #[test]
    fn pt1_test() {
        let input = include_str!("pt1.txt").trim_end();
        let output = pt1(input);
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
        let output = pt2(input);
        println!("{}", output);
        assert!(output == 4);
    }
    #[test]
    fn pt2_test() {
        let input = include_str!("pt1.txt").trim_end();
        let output = pt2(input);
        println!("{}", output);
        assert!(output == 529); // this is wrong
    }
}
