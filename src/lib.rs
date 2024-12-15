macro_rules! wrapper {
    ($($old:ident => $new:ident $(,)?)*) => {$(
        pub mod $new {
            #[must_use]
            #[inline]
            pub fn part1(input: &str) -> impl std::fmt::Display {
                let solution = aoc::year2024::$old::new(
                    input.trim_ascii_end(),
                    aoc::utils::input::InputType::Real
                ).unwrap();
                solution.part1()
            }

            #[must_use]
            #[inline]
            pub fn part2(input: &str) -> impl std::fmt::Display {
                let solution = aoc::year2024::$old::new(
                    input.trim_ascii_end(),
                    aoc::utils::input::InputType::Real
                ).unwrap();
                solution.part2()
            }
        }
    )*};
}

wrapper!(
    Day01 => day1,
    Day02 => day2,
    Day03 => day3,
    Day04 => day4,
    Day05 => day5,
    Day06 => day6,
    Day07 => day7,
    Day08 => day8,
    Day09 => day9,
    Day10 => day10,
    Day11 => day11,
    Day12 => day12,
    Day13 => day13,
    Day14 => day14,
    Day15 => day15,
);
