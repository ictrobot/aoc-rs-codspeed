macro_rules! callback {
    ($(
        $y:literal => $year:ident{$(
            $d:literal => $day:ident,
        )*}
    )*) => {$($(
        paste::paste! {
            pub mod [< day $d >] {
                #[must_use]
                #[inline]
                pub fn part1(input: &str) -> String {
                    let solution = aoc::$year::$day::new(
                        input.trim_ascii_end(),
                        aoc::utils::input::InputType::Real
                    ).unwrap();
                    solution.part1().to_string()
                }

                #[must_use]
                #[inline]
                pub fn part2(input: &str) -> String {
                    let solution = aoc::$year::$day::new(
                        input.trim_ascii_end(),
                        aoc::utils::input::InputType::Real
                    ).unwrap();
                    solution.part2().to_string()
                }
            }
        }
    )*)*};
}

aoc::year2024::puzzles!([callback]);
