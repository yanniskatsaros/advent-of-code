(** Parse an array of strings from a file *)
let array_of_file file parser =
  let contents = In_channel.with_open_bin file In_channel.input_all in
  let lines = String.split_on_char '\n' (String.trim contents) in
  lines
    |> List.map parser
    |> Array.of_list

(** Parse an array of integers from a file *)
let int_array_of_file file =
  let parser s =
    Int.of_float (Float.of_string s)
  in
  array_of_file file parser
