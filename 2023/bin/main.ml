open Core

let read_lines file =
  Stdio.In_channel.with_file file ~f:(fun channel ->
    let x = In_channel.input_all channel in
    String.split_lines x)
;;

let find_int line ~f:filterList =
  let num =
    String.to_list line |> List.filter ~f:Char.is_digit |> filterList |> Char.to_int
  in
  num - Char.to_int '0' |> Int.to_string
;;

let calibration acc str =
  let first = find_int ~f:List.hd_exn str in
  let last = find_int ~f:List.last_exn str in
  acc + (Int.of_string @@ first ^ last)
;;

let () =
  let line = read_lines "./inputs/day1.dat" in
  List.fold line ~init:0 ~f:calibration |> Int.to_string |> print_endline
;;
