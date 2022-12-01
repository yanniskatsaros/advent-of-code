open Aoc.Utils

let parse_input str =
  let int_of_string s =
    s |> Float.of_string |> Int.of_float
  in

  let grouper s (items, acc) = match s with
    | "" -> (acc::items, [])
    | _ -> (items, (int_of_string s) :: acc)
  in

  let items, last =
    str
    |> String.split_on_char '\n'
    |> fun lst -> List.fold_right grouper lst ([], [])
  in
  last :: items


let part1 str =
  let sum = List.fold_left ( + ) 0 in

  parse_input str
  |> List.map sum
  |> fun lst -> List.fold_right max lst 0


let () =
  let p1 = part1 (read_file Sys.argv.(1)) in
  Format.printf "Part 1: %d\n" p1
