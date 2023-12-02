defmodule Mix.Tasks.D1.P1 do
  use Mix.Task

  import AdventOfCode.D01

  @shortdoc "Day 1 Part 1"
  @spec run(list(bitstring())) :: nil
  def run(args) do
    filepath = hd(args)
    {:ok, input} = File.read(filepath)

    if Enum.member?(args, "-b") do
      Benchee.run(%{part_1: fn -> input |> part1() end})
    else
      input
      |> part1()
      |> IO.inspect(label: "Part 1")
    end
  end
end
