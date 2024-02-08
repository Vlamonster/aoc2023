pub fn p1() -> String {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    let answer = include_str!("input")
        .lines()
        .enumerate()
        .filter_map(|(id, game)| {
            let (_, subsets) = game.split_once(": ").unwrap();

            subsets
                .split("; ")
                .all(|subset| {
                    subset.split(", ").all(|pull| {
                        let (number, color) = pull.split_once(' ').unwrap();
                        let number = number.parse::<usize>().unwrap();

                        match color {
                            "red" => number <= MAX_RED,
                            "green" => number <= MAX_GREEN,
                            "blue" => number <= MAX_BLUE,
                            _ => unreachable!("Unexpected color: {color}."),
                        }
                    })
                })
                .then_some(id + 1)
        })
        .sum::<usize>();

    answer.to_string()
}

pub fn p2() -> String {
    let answer = include_str!("input")
        .lines()
        .map(|game| {
            let (_, subsets) = game.split_once(": ").unwrap();

            let (r, g, b) = subsets.split("; ").fold((0, 0, 0), |(r, g, b), subset| {
                subset
                    .split(", ")
                    .fold((r, g, b), |(max_r, max_g, max_b), pull| {
                        let (number, color) = pull.split_once(' ').unwrap();
                        let number = number.parse::<usize>().unwrap();

                        match color {
                            "red" => (number.max(max_r), max_g, max_b),
                            "green" => (max_r, number.max(max_g), max_b),
                            "blue" => (max_r, max_g, number.max(max_b)),
                            _ => unreachable!("Unexpected color: {color}."),
                        }
                    })
            });

            r * g * b
        })
        .sum::<usize>();

    answer.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1() {
        assert_eq!(super::p1().as_str(), "2913")
    }

    #[test]
    fn p2() {
        assert_eq!(super::p2().as_str(), "55593")
    }
}
