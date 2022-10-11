let array_of_file file =
  let contents = In_channel.with_open_bin file In_channel.input_all in
  let lines = String.split_on_char ',' (String.trim contents) in
  lines
    |> List.map (fun s -> Int.of_float (Float.of_string s))
    |> Array.of_list
