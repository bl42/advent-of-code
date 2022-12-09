pub fn all() -> &'static [fn() -> String] {
    &[
        crate::day01::run,
        crate::day02::run,
        crate::day03::run,
        crate::day04::run,
        crate::day05::run,
        crate::day06::run,
        crate::day07::run,
        crate::day08::run,
        // crate::day09::run,
        // crate::day10::run,
        // crate::day11::run,
        // crate::day12::run,
    ]
}
