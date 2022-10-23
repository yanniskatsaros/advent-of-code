type t =
  { mutable memory: int array }
  [@@deriving show]

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


let of_string str =
  str
    |> String.trim
    |> String.split_on_char ','
    |> List.map (
      fun s -> s |> Float.of_string |> Int.of_float
    )
    |> of_list

let of_file file =
  let contents = In_channel.with_open_bin file In_channel.input_all in
  of_string contents
