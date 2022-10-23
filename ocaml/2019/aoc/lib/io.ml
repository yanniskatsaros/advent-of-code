type t =
  { mutable stdin: int
  ; mutable stdout: int
  }
  [@@deriving show]

let init () =
  (* this isn't great, but a convenient way to be lazy *)
  { stdin = 0; stdout = 0}

let read_stdin io =
  io.stdin

let write_stdin io value =
  io.stdin <- value

let read_stdout io =
  io.stdout

let write_stdout io value =
  io.stdout <- value
