open Core

let convert s =
  String.split ~on:' ' s
  |> List.filter_map ~f:(fun s ->
    if String.is_empty s then None else Some (Int.of_string s))
;;

let find_common_items list1 list2 =
  List.filter ~f:(fun item -> Stdlib.List.mem item list2) list1
;;

let score list =
  match List.length list with
  | 0 -> 0
  | 1 -> 1
  | x -> Int.of_float (2.0 ** (Float.of_int x -. 1.0))
;;

let parse_line line =
  let (_ :: numbers :: winning_numbers :: _) =
    String.split_on_chars ~on:[ ':'; '|' ] line
  in
  find_common_items (convert numbers) (convert winning_numbers) |> score
[@@warning "-8"]
;;

let () =
  Utils.read_lines "./inputs/day04.dat"
  |> List.map ~f:parse_line
  |> List.fold ~f:(fun acc n -> acc + n) ~init:0
  |> Fmt.pr "@.Day 04 Part1: %d@."
;;

(* let score_cards list = *)
(*   Fmt.pr "%d" (List.hd_exn list); *)
(*   30 *)
(* ;; *)

(* let () = *)
(*   Utils.read_lines "./inputs/day04.example.dat" *)
(*   |> List.map ~f:parse_line *)
(*   |> score_cards *)
(*   |> Fmt.pr "@.Day 04 Part1: %d@." *)
(* ;; *)
