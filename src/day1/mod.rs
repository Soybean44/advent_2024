fn pt1(input: &str) -> i32 {
    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = input
        .split('\n')
        .map(|x| x.split("   ").map(|y| y.parse().unwrap()).collect())
        .map(|v: Vec<i32>| (v[0], v[1]))
        .unzip();
    list1.sort();
    list2.sort();
    let lists = list1.iter().zip(list2.iter());
    return lists.map(|(x, y)| (x - y).abs()).sum::<i32>();
}

fn pt2(input: &str) -> usize {
    let (list1, list2): (Vec<i32>, Vec<i32>) = input
        .split('\n')
        .map(|x| x.split("   ").map(|y| y.parse().unwrap()).collect())
        .map(|v: Vec<i32>| (v[0], v[1]))
        .unzip();
    return list1
        .iter()
        .map(|x| {
            list2
                .iter()
                .filter(|y| **y == *x)
                .collect::<Vec<&i32>>()
                .len()
                * (*x as usize)
        })
        .sum::<usize>();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn pt1_example() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert!(pt1(input) == 11);
    }
    #[test]
    fn pt1_test() {
        let input = include_str!("pt1.txt").trim_end();
        let output = pt1(input);
        println!("{}", output);
        assert!(output == 3508942);
    }
    #[test]
    fn pt2_example() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let output = pt2(input);
        assert!(output == 31);
    }
    #[test]
    fn pt2_test() {
        let input = include_str!("pt1.txt").trim_end();
        let output = pt2(input);
        println!("{}", output);
        assert!(output == 26593248);
    }
}
