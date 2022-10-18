type t =
  { mutable ip: int
  ; mem: Memory.t
  }

let init mem =
  { ip = 0; mem}

let op fn step ({ ip; mem } as cpu) =
  let open Memory in
  let p1 = mem.$(ip + 1) in
  let p2 = mem.$(ip + 2) in
  let p3 = mem.$(ip + 3) in
  let a = mem.$(p1) in
  let b = mem.$(p2) in
  mem.$(p3) <- fn a b;
  cpu.ip <- cpu.ip + step

let add = op ( + ) 4
let mul = op ( * ) 4

let rec run ({ip; mem} as cpu) =
  let open Memory in
  try
    match mem.$(ip) with
    | 1 ->
      add cpu;
      run cpu
    | 2 ->
      mul cpu;
      run cpu
    | 99 -> Ok ()
    | n -> Error (Printf.sprintf "unknown opcode: %d" n)
  with _ -> Error "intcode runtime error"
