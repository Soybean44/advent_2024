fn pt1(input: &str) -> i32 {
    let chars: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut matches = 0;
    for (i, line) in chars.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'X' {
                println!("{},{}", i, j);
                matches += check_path(4, &chars, (i as i32, j as i32));
            }
        }
    }
    return matches;
}

fn check_path(curr_letter: i32, chars: &Vec<Vec<char>>, idx: (i32, i32)) -> i32 {
    let curr_char = chars[idx.0 as usize][idx.1 as usize];
    let (h, w) = (chars.len() as i32, chars[0].len() as i32);
    let paths: Vec<(i32, i32)> = vec![
        (idx.0 - 1, idx.1 - 1),
        (idx.0 - 1, idx.1),
        (idx.0 - 1, idx.1 + 1),
        (idx.0, idx.1 + 1),
        (idx.0 + 1, idx.1 + 1),
        (idx.0 + 1, idx.1),
        (idx.0 + 1, idx.1 - 1),
        (idx.0, idx.1 - 1),
    ]
    .into_iter()
    .filter(|x| (x.0 >= 0 && x.1 >= 0 && x.0 < h && x.1 < w))
    .collect();
    if curr_letter == 4 {
        if curr_char == 'X' {
            return paths.iter().map(|x| check_path(3, chars, *x)).sum();
        }
        return 0;
    } else if curr_letter == 3 {
        if curr_char == 'M' {
            return paths.iter().map(|x| check_path(2, chars, *x)).sum();
        }
        return 0;
    } else if curr_letter == 2 {
        if curr_char == 'A' {
            return paths.iter().map(|x| check_path(1, chars, *x)).sum();
        }
        return 0;
    } else if curr_letter == 1 {
        if curr_char == 'S' {
            println!("{}: {},{}", curr_char, idx.0, idx.1);
            return 1;
        }
        return 0;
    }
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn pt1_example() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let output = pt1(input);
        println!("{}", output);
        assert!(output == 18);
    }
    #[test]
    fn pt1_test() {
        let input = include_str!("pt1.txt").trim_end();
        let output = pt1(input);
        println!("{}", output);
        assert!(output == 3508942);
    }
}
