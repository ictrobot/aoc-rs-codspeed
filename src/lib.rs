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
                pub fn part1(input: &str) -> impl std::fmt::Display {
                    let solution = aoc::$year::$day::new(
                        input,
                        aoc::utils::input::InputType::Real
                    ).unwrap();
                    solution.part1()
                }

                #[must_use]
                #[inline]
                pub fn part2(input: &str) -> impl std::fmt::Display {
                    let solution = aoc::$year::$day::new(
                        input,
                        aoc::utils::input::InputType::Real
                    ).unwrap();
                    solution.part2()
                }
            }
        }
    )*)*};
}

aoc::year2024::puzzles!([callback]);
