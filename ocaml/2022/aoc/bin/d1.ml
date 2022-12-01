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
  parse_input str
    |> List.map sum
    |> fun lst -> List.fold_right max lst 0


let part2 str =
  let top3 = parse_input str
    |> List.map sum
    |> List.sort (fun x y -> - (compare x y))
    |> take 3
  in
  match top3 with
    | None -> None
    | Some xs -> Some (sum xs)


let () =
  let input = read_file Sys.argv.(1) in

  Format.printf "Part 1: %d\n" (part1 input) ;

  match part2 input with
    | None -> Format.printf "something went wrong"
    | Some total ->
      Format.printf "Part 2: %d\n" total;
  ()
