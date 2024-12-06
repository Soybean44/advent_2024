fn pt1(input: &str) -> i32 {
    let chars: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut matches = 0;
    for (i, line) in chars.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'X' {
                matches += check_letter(&chars, (i as i32, j as i32));
            }
        }
    }
    return matches;
}

fn check_letter(chars: &Vec<Vec<char>>, idx: (i32, i32)) -> i32 {
    let curr_char = chars[idx.0 as usize][idx.1 as usize];
    let (h, w) = (chars.len() as i32, chars[0].len() as i32);
    let dirs = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ]
    .into_iter()
    .filter(|x| (x.0 >= 0 && x.1 >= 0 && x.0 < h && x.1 < w));
    if curr_char == 'X' {
        return dirs
            .map(|x| check_dir(3, chars, (idx.0 + x.0, idx.1 + x.1), x))
            .sum();
    }
    return 0;
    return 0;
}

fn check_dir(curr_letter: i32, chars: &Vec<Vec<char>>, idx: (i32, i32), dir: (i32, i32)) -> i32 {
    if (idx.0 < 0 || idx.0 >= chars.len() as i32) {
        return 0;
    } else if (idx.1 < 0 || idx.1 >= chars[idx.0 as usize].len() as i32) {
        return 0;
    }
    let curr_char = chars[idx.0 as usize][idx.1 as usize];
    if curr_letter == 3 && curr_char == 'M' {
        return check_dir(2, chars, (idx.0 + dir.0, idx.1 + dir.1), dir);
    }
    if curr_letter == 2 && curr_char == 'A' {
        return check_dir(1, chars, (idx.0 + dir.0, idx.1 + dir.1), dir);
    }
    if curr_letter == 1 && curr_char == 'S' {
        return 1;
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
