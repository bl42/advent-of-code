[@@@warning "-27-32-34-69"]

open Core

type position =
  { x : int
  ; y : int
  }

type part_number =
  { start : position
  ; length : int
  }

let decode_line line x =
  let rec aux line parts symbols y =
    match line with
    | [] -> parts, symbols
    | ch :: line when Char.equal ch '.' ->
      Fmt.pr "@.found!: %c -- y:%d" ch y;
      aux line parts symbols (y + 1)
    | ch :: line when Char.is_digit ch ->
      Fmt.pr "@.found digit!: %c -- y:%d" ch y;
      aux line ({ start = { x; y }; length = 1 } :: parts) symbols (y + 1)
    | ch :: line ->
      Fmt.pr "@.found symbol!: %c -- y:%d" ch y;
      aux line parts ({ x; y } :: symbols) (y + 1)
  in
  aux (String.to_list line) [] [] 0
;;

let () =
  let parts, symbols = decode_line "...34..3..*..$.%..#" 0 in
  Fmt.pr "@.parts: %d symbols: %d" (List.length parts) (List.length symbols);
  ()
;;

let parse_lines lines =
  let rec aux lines parts symbols =
    match lines with
    | [] -> parts, symbols
    | line :: lines -> aux lines parts symbols
  in
  aux lines [] []
;;

(* { start = { x = 1; y = 1 }; length = 3 }, 1 *)

let () =
  Utils.read_lines "./inputs/day03.example.dat"
  |> List.hd_exn
  |> Fmt.pr "@.@.\tDay 3 Part 1 : %s"
;;
