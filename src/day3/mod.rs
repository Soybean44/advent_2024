use regex::Regex;
fn pt1(input: &str) -> i32 {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();
    let mut sum: i32 = 0;
    for cap in re.captures_iter(input) {
        sum += &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap();
    }
    return sum;
}

fn pt2(input: &str) -> i32 {
    let pattern = r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)";
    let re = Regex::new(pattern).unwrap();
    let mut sum: i32 = 0;
    let mut enable = false;
    for cap in re.captures_iter(input) {
        if cap[0].contains("do()") {
            enable = true;
        } else if cap[0].contains("don't()") {
            enable = false;
        } else if enable {
            sum += &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap();
        }
    }
    return sum;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn pt1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert!(pt1(input) == 161);
    }
    #[test]
    fn pt1_test() {
        let input = include_str!("pt1.txt").trim_end();
        let output = pt1(input);
        println!("{}", output);
        assert!(output == 170778545);
    }
    #[test]
    fn pt2_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert!(pt2(input) == 48);
    }
    #[test]
    fn pt2_test() {
        let input = include_str!("pt1.txt").trim_end();
        let output = pt2(input);
        println!("{}", output);
        assert!(output == 3508942);
    }
}
