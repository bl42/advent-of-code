[@@@warning "-8"]

open Core

(* 12 red cubes, 13 green cubes, and 14 blue *)
let is_valid_cube x =
  match String.split ~on:' ' @@ String.chop_prefix_exn ~prefix:" " x with
  | n :: "red" :: _ when Int.of_string n > 12 -> None
  | n :: "blue" :: _ when Int.of_string n > 14 -> None
  | n :: "green" :: _ when Int.of_string n > 13 -> None
  | _ -> Some x
;;

let is_valid_roll roll =
  let check_rolls = String.split ~on:',' roll |> List.map ~f:is_valid_cube in
  if Option.is_some (List.find ~f:Option.is_none check_rolls) then false else true
;;

let valid_game game =
  let (game :: cubes :: _) = String.split ~on:':' game in
  let rolls = String.split ~on:';' cubes in
  let valid_rolls = List.filter ~f:is_valid_roll rolls in
  if List.length valid_rolls = List.length rolls then Some game else None
;;

let () =
  Utils.read_lines "./inputs/day02.dat"
  |> List.filter_map ~f:valid_game
  |> List.fold ~init:0 ~f:(fun acc game ->
    acc + (Int.of_string @@ String.chop_prefix_exn ~prefix:"Game " game))
  |> Fmt.pr "@.\tDay 2 Part 1: %d"
;;

(* Day 2 *)

let f s =
  let (_ :: n :: color :: _) = String.split ~on:' ' s in
  Int.of_string n, color
;;

let line_in_cube_list line =
  String.substr_replace_all ~pattern:";" ~with_:"," line
  |> String.split ~on:':'
  |> List.last_exn
  |> String.split ~on:','
  |> List.map ~f
;;

type most_cube =
  { mutable red : int
  ; mutable green : int
  ; mutable blue : int
  }

let parse_line line =
  let most = { red = 0; blue = 0; green = 0 } in
  line_in_cube_list line
  |> List.iter ~f:(fun i ->
    match i with
    | n, "red" when most.red < n -> most.red <- n
    | n, "blue" when most.blue < n -> most.blue <- n
    | n, "green" when most.green < n -> most.green <- n
    | _ -> ());
  most.red * most.blue * most.green
;;

let () =
  Utils.read_lines "./inputs/day02.dat"
  |> List.fold ~init:0 ~f:(fun acc s -> acc + parse_line s)
  |> Fmt.pr "@.\tDay 2 Part 2: %d"
;;
