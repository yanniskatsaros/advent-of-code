open Cpu

type loc =
  | Mem of int
  | Imm of int
  [@@deriving show]

type t =
  | Add of (loc * loc * loc)
  | Mul of (loc * loc * loc)
  | Inp of (int * loc)
  | Out of loc
  | Hlt
  [@@deriving show]

let decode ({ ip; mem; io } as cpu) =
  let open Memory in 
  let instr = cpu.mem.$(ip) in

  (* if the opcode is 99, immediately halt *)
  if instr = 99 then Hlt else

  let (op, mode1, mode2, mode3) =
    let op = instr mod 10 in

    let mode1 = (instr / 100) mod 10 in
    let mode2 = (instr / 1000) mod 10 in
    let mode3 = (instr / 10000) mod 10 in
    (op, mode1, mode2, mode3)
  in

  let decode_param n mode =
    let value = mem.$(ip + n) in
    match mode with
      | 0 -> Mem value
      | 1 -> Imm value
      | n -> raise (Invalid_argument (Printf.sprintf "invalid parameter mode: %d" n))
  in

  let decode_add () =
    let p1 = decode_param 1 mode1 in
    let p2 = decode_param 2 mode2 in
    let dst = decode_param 3 mode3 in
    Add (p1, p2, dst)
  in
  
  let decode_mul () =
    let p1 = decode_param 1 mode1 in
    let p2 = decode_param 2 mode2 in
    let dst = decode_param 3 mode3 in
    Mul (p1, p2, dst)
  in
  
  let decode_inp () =
    (* parameters that write will never be in immediate mode *)
    let p1 = decode_param 1 mode1 in
    let value = Io.read_stdin io in
    Inp (value, p1)
  in

  let decode_out () =
    let p1 = decode_param 1 mode1 in
    Out p1
  in

  match op with
    | 1 -> decode_add ()
    | 2 -> decode_mul ()
    | 3 -> decode_inp ()
    | 4 -> decode_out ()
    | 99 -> Hlt
    | n -> raise (Invalid_argument (Printf.sprintf "invalid opcode: %d" n))

let eval cpu instr =
  let open Memory in

  let get = function
    | Mem i -> cpu.mem.$(i)
    | Imm i -> i
  in

  match instr with
    | Hlt -> ()
    | Add (p1, p2, dst) ->
      let p1 = get p1 in
      let p2 = get p2 in
      (* let dst = get dst in *)
      let dst = match dst with
        | Mem i | Imm i -> i
      in
      cpu.mem.$(dst) <- p1 + p2 ;
      cpu.ip <- cpu.ip + 4
    | Mul (p1, p2, dst) ->
      let p1 = get p1 in
      let p2 = get p2 in
      (* let dst = get dst in *)
      let dst = match dst with
        | Mem i | Imm i -> i
      in
      cpu.mem.$(dst) <- p1 * p2 ;
      cpu.ip <- cpu.ip + 4
    | Inp (i, dst) ->
      (* let dst = get dst in *)
      let dst = match dst with
        | Mem i | Imm i -> i
      in
      cpu.mem.$(dst) <- i ;
      cpu.ip <- cpu.ip + 2
    | Out i ->
      let i = get i in
      Io.write_stdout cpu.io i ;
      cpu.ip <- cpu.ip + 2

let rec run cpu =
  try
    let instr = decode cpu in
    match instr with
      | Hlt -> Ok ()
      | _ ->
        eval cpu instr;
        run cpu
  with _ -> Error "intcode runtime error"


let step_debug cpu =
  let instr = decode cpu in
  eval cpu instr;
  print_endline (Cpu.show cpu)
