let array_of_file file =
  let contents = In_channel.with_open_bin file In_channel.input_all in
  let lines = String.split_on_char ',' (String.trim contents) in
  lines
    |> List.map (fun s -> Int.of_float (Float.of_string s))
    |> Array.of_list

type error =
  | InvalidOpcode of int
  | MemoryOutOfBounds of int

type state =
  | Pos of int
  | Halt

let op memory pos =
  try match memory.(pos) with
  | 1 -> begin
    let pos1 = memory.(pos + 1) in
    let pos2 = memory.(pos + 2) in
    let target = memory.(pos + 3) in
    let a = memory.(pos1) in
    let b = memory.(pos2) in
    memory.(target) <- a + b ;
    Ok (Pos (pos + 4))
  end
  | 2 -> begin
    let pos1 = memory.(pos + 1) in
    let pos2 = memory.(pos + 2) in
    let target = memory.(pos + 3) in
    let a = memory.(pos1) in
    let b = memory.(pos2) in
    memory.(target) <- a * b ;
    Ok (Pos (pos + 4))
  end
  | 99 -> Ok Halt
  | opcode -> Error (InvalidOpcode opcode)
  with Invalid_argument _ -> Error (MemoryOutOfBounds pos)


let run memory =
  let rec go memory pos =
    match op memory pos with
    | Ok (Pos pos') -> go memory pos'
    | Ok Halt -> memory
    | Error (MemoryOutOfBounds i )->
      Format.printf "memory out of bounds: %d" i;
      memory
    | Error (InvalidOpcode code) ->
      Format.printf "invalid opcode: %d" code ;
      memory
  in
  go memory 0
