defmodule AdventOfCode.D04 do
  defp parse_line(line) do
    [card, numbers] = line |> String.split(":")

    [_, id] = card |> String.split(" ", trim: true)
    # parse to integer()
    id = String.to_integer(id |> String.trim())

    [winning, mine] = numbers |> String.split("|")
    # parse each to list(integer())
    winning = String.split(winning, " ", trim: true) |> Enum.map(&String.to_integer/1)
    mine = String.split(mine, " ", trim: true) |> Enum.map(&String.to_integer/1)

    {id, winning, mine}
  end

  defp card_value({_id, winning, mine}) do
    # find the number of items in the set intersection
    n = MapSet.intersection(MapSet.new(winning), MapSet.new(mine)) |> MapSet.size()

    # handle edge case for empty set where length = 0
    if n == 0, do: 0, else: Integer.pow(2, n - 1)
  end

  @spec part1(binary()) :: integer()
  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&(&1 |> parse_line() |> card_value()))
    |> Enum.sum()
  end
end
