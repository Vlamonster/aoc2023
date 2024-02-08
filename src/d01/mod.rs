use itertools::Itertools;

pub fn p1() -> String {
    let answer = include_str!("input")
        .lines()
        .map(|line| {
            let numbers = line
                .bytes()
                .filter_map(|c| c.is_ascii_digit().then_some((c - b'0') as usize))
                .collect_vec();

            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();

            first * 10 + last
        })
        .sum::<usize>();

    answer.to_string()
}

pub fn p2() -> String {
    let answer = include_str!("input")
        .lines()
        .map(|line| {
            let numbers = line
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .bytes()
                .filter_map(|c| c.is_ascii_digit().then_some((c - b'0') as usize))
                .collect_vec();

            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();

            first * 10 + last
        })
        .sum::<usize>();

    answer.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1() {
        assert_eq!(super::p1().as_str(), "54953")
    }

    #[test]
    fn p2() {
        assert_eq!(super::p2().as_str(), "53868")
    }
}
