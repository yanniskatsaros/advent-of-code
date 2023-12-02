defmodule Mix.Tasks.D2.P1 do
  @moduledoc """
  Run the solution for Day 2 Part I
  """
  use Mix.Task

  import AdventOfCode.D02

  @spec run(list(binary())) :: nil
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
