open Aoc.Utils

module CharSet = Set.Make(Char)

(** splits the string [s] into two equal parts, assuming an even length *)
let split_two s =
  let n = String.length s in
  let h = n / 2 in

  (* returns a substring starting from position [i] of length [n] *)
  let a = String.sub s 0 h in
  let b = String.sub s h h in

  a, b

(** returns the first common [char] between the two strings, if one exists *)
let common_char (a, b) =
  let set_a = CharSet.of_seq (String.to_seq a) in
  let set_b = CharSet.of_seq (String.to_seq b) in

  (* the set intersection of A and B *)
  let common = CharSet.inter set_a set_b in

  (* but we only expect a single common item *)
  match Seq.uncons (CharSet.to_seq common) with
    | None -> None
    | Some (c, _) -> Some c

let priority c = match c with
  | 'a' .. 'z' -> (* priority 1 - 26 *)
    (* lowercase a has ASCII code 97 *)
    (Char.code c) - 96
  | 'A' .. 'Z' -> (* priority 27 - 52 *)
    (* uppercase A has ASCII code 65 *)
    (Char.code c) - 65 + 27
  (* assume all other (unknown) chars have 0 priority *)
  | _ -> 0

let parse str =
  match str |> split_two |> common_char with
    | None -> 0
    | Some c -> priority c

let part1 str =
  str
    |> String.trim
    |> String.split_on_char '\n'
    |> List.map parse
    |> sum


let () =
  let input = read_file Sys.argv.(1) in

  let p1 = part1 input in
  Format.printf "Part 1: %d\n" p1
