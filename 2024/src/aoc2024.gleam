import days/day01
import days/day02
import gleam/io

pub fn main() {
  day01.run()

  day02.run()
  |> io.debug
}
