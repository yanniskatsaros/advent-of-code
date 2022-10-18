open Aoc

(* read the initial memory from disk once eagerly
   to avoid re-reading from disk - then copy as needed *)
let mem = Memory.of_file Sys.argv.(1)

let part1 () =
  let open Memory in
  let open Cpu in

  let memory = Memory.copy mem in
  let cpu = (Cpu.init memory) in

  (* initialize the memory as specified in Day 2 *)
  memory.$(1) <- 12 ;
  memory.$(2) <- 2;

  match run cpu with
  | Ok () ->
    Printf.printf "%d\n" cpu.mem.$(0)
  | Error s ->
    Printf.printf "%s\n" s


let () =
  part1 ()
