type t =
  { mutable memory: int array }

let ( .$() ) mem idx =
  mem.memory.(idx)
  (* try
    Ok mem.memory.(idx)
  with _ -> Error (MemoryOutOfBounds idx) *)

let ( .$() <- ) mem idx value =
  mem.memory.(idx) <- value
  (* try
    mem.memory.(idx) <- value;
    Ok ()
  with _ -> Error (MemoryOutOfBounds idx) *)


let copy { memory } =
  { memory = Array.copy memory }

let of_list items =
  { memory = Array.of_list items }

let of_file file =
  let contents = In_channel.with_open_bin file In_channel.input_all in
  let lines = String.split_on_char ',' (String.trim contents) in
  lines
    |> List.map (fun s -> Int.of_float (Float.of_string s))
    |> of_list
