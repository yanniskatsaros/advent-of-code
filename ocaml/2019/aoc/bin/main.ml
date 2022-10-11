open Aoc

module Day2 = struct

  let init memory =
    memory.(1) <- 12 ;
    memory.(2) <- 2

  let part1 memory =
    init memory ;
    let memory' = Intcode.run memory in
    Format.printf "%d\n" memory'.(0)

end

let () =
  let memory = Intcode.array_of_file Sys.argv.(1) in
  Day2.part1 memory
