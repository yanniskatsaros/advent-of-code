open Aoc.Utils

type rpc =
  | Roc
  | Pap
  | Sci

let decode s =
  if (String.length s) <> 3
    then failwith (Format.sprintf "invalid instruction: %s" s)
    else

  let decoder = function
    | 'A' | 'X' -> Roc
    | 'B' | 'Y' -> Pap
    | 'C' | 'Z' -> Sci
    | other -> failwith (Format.sprintf "unknown char: %c" other)
  in

  (* we assume that a combo cannot mix ABC or XYZ in both players *)
  let opp = s.[0] in
  let self = s.[2] in
  (decoder opp, decoder self)

let score (opp, self) =
  (* points awarded for the shape selected *)
  let score = match self with
    | Roc -> 1
    | Pap -> 2
    | Sci -> 3
  in

  (* from the point of view of self *)
  let outcome = match (opp, self) with
    | Roc, Pap
    | Sci, Roc
    | Pap, Sci -> 6  (* win *)
    | Pap, Roc
    | Roc, Sci
    | Sci, Pap -> 0  (* lose *)
    | _, _ -> 3      (* draw *)
  in
  score + outcome

let part1 str =
  str
    |> String.trim
    |> String.split_on_char '\n'
    |> List.map decode
    |> List.map score
    |> sum

let () =
  let input = read_file Sys.argv.(1) in

  Format.printf "Part 1: %d\n" (part1 input)
