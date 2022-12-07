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

(** returns the first common [char] between the three strings, if one exists *)
let common_char2 (a, b, c) =
  let set_a = CharSet.of_seq (String.to_seq a) in
  let set_b = CharSet.of_seq (String.to_seq b) in
  let set_c = CharSet.of_seq (String.to_seq c) in

  (* the set intersection of A and B *)
  let ab = CharSet.inter set_a set_b in
  (* intersected with C *)
  let abc = CharSet.inter ab set_c in

  (* but we only expect a single common item *)
  match Seq.uncons (CharSet.to_seq abc) with
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

let parse_triple lst =
  let arr = Array.of_list lst in

  try
    let s1 = arr.(0) in
    let s2 = arr.(1) in
    let s3 = arr.(2) in
    match common_char2 (s1, s2, s3) with
      | None -> 0
      | Some c -> priority c
  with _ -> 0

let part1 str =
  str
    |> String.trim
    |> String.split_on_char '\n'
    |> List.map parse
    |> sum

let part2 str =
  str
    |> String.trim
    |> String.split_on_char '\n'
    |> List.to_seq
    |> chunks 3
    |> List.map parse_triple
    |> sum

let () =
  let input = read_file Sys.argv.(1) in

  let p1 = part1 input in
  Format.printf "Part 1: %d\n" p1;

  let p2 = part2 input in
  Format.printf "Part 2: %d\n" p2
