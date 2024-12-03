import days/day02
import gleeunit
import gleeunit/should

pub fn main() {
  gleeunit.main()
}

const test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"

pub fn part1_test() {
  day02.part1(test_input)
  |> should.equal(2)
}

pub fn part2_test() {
  day02.part2(test_input)
  |> should.equal(4)
}
