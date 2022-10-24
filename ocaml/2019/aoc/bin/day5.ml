open Aoc

let mem = Memory.of_file Sys.argv.(1)

let part1 () =
  let memory = Memory.copy mem in
  let io = Io.init () in
  Io.write_stdin io 1 ;
  let cpu = (Cpu.init memory io) in

  match Instruction.run cpu with
    | Ok () -> Printf.printf "%d\n" (Io.read_stdout io)
    | Error s -> Printf.printf "%s\n" s

let part2 () =
  let memory = Memory.copy mem in
  let io = Io.init () in
  Io.write_stdin io 5 ;
  let cpu = (Cpu.init memory io) in

  match Instruction.run cpu with
    | Ok () -> Printf.printf "%d\n" (Io.read_stdout io)
    | Error s -> Printf.printf "%s\n" s

let () =
  part1 ();
  part2 ()
