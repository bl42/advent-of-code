import gleam/bool
import gleam/int
import gleam/list
import gleam/string
import simplifile.{read}

pub fn run() {
  let assert Ok(input) = read("./inputs/day02.dat")
  #(part1(input), part2(input))
}

fn parse(input) {
  use line <- list.map(string.split(string.trim_end(input), "\n"))

  line
  |> string.split(" ")
  |> list.map(fn(x) {
    let assert Ok(a) = int.parse(x)
    a
  })
}

pub fn part1(input) {
  parse(input)
  |> list.count(is_safe)
}

pub fn part2(input) {
  parse(input)
  |> list.count(is_really_safe)
}

pub type Direction {
  ASC(Int)
  DESC(Int)
  Unknown
}

pub fn to_changes(values: List(Int)) {
  to_changes_acc(values, [])
}

fn to_changes_acc(values: List(Int), acc: List(Direction)) {
  case values {
    [f, s, ..rest] if f < s -> to_changes_acc([s, ..rest], [ASC(s - f), ..acc])
    [f, s, ..rest] if f > s -> to_changes_acc([s, ..rest], [DESC(f - s), ..acc])
    [_, s, ..rest] -> to_changes_acc([s, ..rest], [Unknown, ..acc])
    [_, ..rest] -> to_changes_acc(rest, acc)
    [] -> list.reverse(acc)
  }
}

pub fn is_safe(record: List(Int)) {
  record
  |> to_changes
  |> acceptable
}

fn acceptable(changes: List(Direction)) {
  case changes {
    [Unknown, ..] -> False
    [DESC(_), ASC(_), ..] -> False
    [ASC(_), DESC(_), ..] -> False
    [DESC(x), ..] if x > 3 -> False
    [ASC(x), ..] if x > 3 -> False
    [_, ..rest] -> acceptable(rest)
    [] -> True
  }
}

pub fn is_really_safe(record: List(Int)) {
  // If it satisfies the regular safety check, we are fine.
  use <- bool.guard(is_safe(record), True)

  // If the regular check does not pass, we try removing each level
  // to see if removing one will cause it to pass.
  record
  |> list.combinations(list.length(record) - 1)
  |> list.any(fn(subrecord) {
    subrecord
    |> to_changes
    |> acceptable
  })
}
