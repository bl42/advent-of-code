import gleam/bool
import gleam/int
import gleam/list
import gleam/string
import simplifile.{read}

pub fn run() {
  let assert Ok(input) = read("./inputs/day01.dat")
  #(part1(input), part2(input))
}

pub fn part1(input) {
  let lines = string.split(input, on: "\n")

  let #(left, right) = parse_list(lines)
  let left = list.sort(left, int.compare)
  let right = list.sort(right, int.compare)

  list.zip(left, right)
  |> list.map(fn(tuple) { tuple.0 - tuple.1 |> int.absolute_value })
  |> list.fold(0, int.add)
}

pub fn part2(input) {
  let lines = string.split(input, on: "\n")
  let #(left, right) = parse_list(lines)

  list.map(left, fn(num) { num * list.count(right, fn(r) { r == num }) })
  |> list.fold(0, int.add)
}

fn parse_list(lines) {
  list.fold(lines, #([], []), fn(left_and_right, line) {
    use <- bool.guard(when: line == "", return: left_and_right)

    let #(left, right) = left_and_right
    let assert Ok(#(a, b)) = string.split_once(line, on: "   ")
    let assert Ok(a) = int.parse(a)
    let assert Ok(b) = int.parse(b)

    #([a, ..left], [b, ..right])
  })
}
