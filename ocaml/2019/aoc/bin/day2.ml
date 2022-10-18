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

exception Done of (int * int)

let part2 () =
  let open Memory in
  let open Cpu in

  (* loop through all 100x100 combinations,
     initializing fresh memory each time *)
  try
    for i = 1 to 100 do
      for j = 1 to 100 do
        let memory = Memory.copy mem in
        memory.$(1) <- i;
        memory.$(2) <- j;
        let cpu = (Cpu.init memory) in

        match run cpu with
        | Ok () ->
          if cpu.mem.$(0) = 19690720
            then raise (Done (i, j))
            else ()
        | Error _ -> ()
      done
    done;
  with Done (x, y) ->
    Printf.printf "%d\n" (100 * x + y)


let () =
  part1 ();
  part2 ()
