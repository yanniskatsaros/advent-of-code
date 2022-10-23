type t =
  { mutable ip: int
  ; mem: Memory.t
  ; io: Io.t
  }
  [@@deriving show]

let init mem io =
  { ip = 0; mem; io }

let of_string str =
  let memory = Memory.of_string str in
  let io = Io.init () in
  init memory io

let equal c1 c2 =
  ( c1.mem = c2.mem )
    && ( c1.io = c2.io )
    && ( c1.ip = c2.ip )
