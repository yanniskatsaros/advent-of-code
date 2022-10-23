type t =
  { mutable ip: int
  ; mem: Memory.t
  ; io: Io.t
  }
  [@@deriving show]

let init mem io =
  { ip = 0; mem; io }
