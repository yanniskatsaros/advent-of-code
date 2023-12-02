defmodule Mix.Tasks.D1.P2 do
  @moduledoc """
  Run the solution for Day 1 Part II
  """
  use Mix.Task

  import AdventOfCode.D01

  @spec run(list(binary())) :: nil
  def run(args) do
    filepath = hd(args)
    {:ok, input} = File.read(filepath)

    if Enum.member?(args, "-b") do
      Benchee.run(%{part_2: fn -> input |> part1() end})
    else
      input
      |> part2()
      |> IO.inspect(label: "Part 2")
    end
  end
end
