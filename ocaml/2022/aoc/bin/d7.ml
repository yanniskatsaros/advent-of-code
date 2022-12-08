module Fs = struct
  type t =
    | File of file
    | Dir of dir
    and file =
      { filename: string
      ; size: int
      }
    and dir =
      { dirname: string
      ; contents: t list
      ; parent: dir option
      }

  let root () = { dirname = "/" ; contents = [] ; parent = None}

  let cd loc {contents; _} =
    let matcher = function
      | File _ -> false
      | Dir { dirname; _} -> dirname = loc
    in
    try
      match List.find matcher contents with
        | File _ -> failwith "cd: not a directory"
        | Dir dir -> dir
    with _ -> failwith "cd: no such directory"

  let set_parent dir parent =
    { dir with parent = parent }

  let set_contents dir contents =
    { dir with contents = contents }

  let parse_listed parent str =
    let str = String.trim str in
    match String.split_on_char ' ' str with
      | ["dir"; dirname] -> Dir { dirname; contents = [] ; parent = parent}
      | [x; filename] -> begin
        match int_of_string_opt x with
          | Some size -> File { filename; size }
          | None -> failwith "invalid filesystem format"
      end
      | _ -> failwith "invalid filesystem format"

  let parse_cd str =
    match String.split_on_char ' ' str with
      | ["cd"; loc] -> Some loc
      | _ -> None

  let cmd x dir = match x with
    | ["cd"; loc] -> cd loc dir
    | "ls" :: files -> begin
      let contents = List.map (parse_listed (Some dir)) files in
      set_contents dir contents
      end
    | _ -> failwith "invalid command"
end


module Command = struct
  type t =
    | Cd of string
    | Ls

  let parse cmd =
    let cmd = String.trim cmd in
    match String.split_on_char ' ' cmd with
      | ["$"; "cd"; loc] -> Some (Cd loc)
      | ["$"; "ls"] -> Some Ls
      | _ -> None
end


let parse_commands str =
  str
    |> String.trim
    |> String.split_on_char '$'
    |> List.filter_map (function
        | "" -> None
        | s -> Some (s |> String.trim |> String.split_on_char '\n')
      )
