open Core

let read_lines file =
  Stdio.In_channel.with_file file ~f:(fun channel ->
    let x = In_channel.input_all channel in
    String.split_lines x)
;;

let filter_decode_number list =
  match list with
  | 'o' :: 'n' :: 'e' :: _ -> Some 1
  | 't' :: 'w' :: 'o' :: _ -> Some 2
  | 't' :: 'h' :: 'r' :: 'e' :: 'e' :: _ -> Some 3
  | 'f' :: 'o' :: 'u' :: 'r' :: _ -> Some 4
  | 'f' :: 'i' :: 'v' :: 'e' :: _ -> Some 5
  | 's' :: 'i' :: 'x' :: _ -> Some 6
  | 's' :: 'e' :: 'v' :: 'e' :: 'n' :: _ -> Some 7
  | 'e' :: 'i' :: 'g' :: 'h' :: 't' :: _ -> Some 8
  | 'n' :: 'i' :: 'n' :: 'e' :: _ -> Some 9
  | ch :: _ when Char.is_digit ch -> Some (Char.to_int ch - Char.to_int '0')
  | _ -> None
;;

let prepare_calibration str =
  let rec aux list str_list =
    match filter_decode_number str_list, List.tl str_list with
    | Some x, None -> x :: list
    | None, None -> list
    | Some x, Some tl -> aux (x :: list) tl
    | None, Some tl -> if List.is_empty tl then list else aux list tl
  in
  aux [] (String.to_list str)
;;

let find_int line ~f:filterList = prepare_calibration line |> filterList |> Int.to_string

let calibration acc str =
  let first = find_int ~f:List.last_exn str in
  let last = find_int ~f:List.hd_exn str in
  acc + (Int.of_string @@ first ^ last)
;;

let () =
  let line = read_lines "./inputs/day1.dat" in
  List.fold line ~init:0 ~f:calibration |> Int.to_string |> print_endline
;;
