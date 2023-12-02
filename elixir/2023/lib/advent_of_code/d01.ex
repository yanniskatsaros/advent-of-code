defmodule AdventOfCode.D01 do
  @spec part1(bitstring()) :: integer()
  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&decode/1)
    |> Enum.sum()
  end

  @spec digit?(bitstring()) :: boolean()
  @doc """
  Returns `true` if `x` is an integer digit 0 .. 9, `false` otherwise
  """
  def digit?(x) do
    case Integer.parse(x) do
      {_, ""} -> true
      _ -> false
    end
  end

  @spec decode(bitstring()) :: integer()
  defp decode(line) do
    digits =
      line
      |> String.trim()
      |> String.codepoints()
      |> Enum.filter(&digit?/1)
      |> Enum.map(&String.to_integer/1)

    # assumes at least 1 digit is present
    first = hd(digits)
    last = List.last(digits)

    Integer.undigits([first, last])
  end
end
