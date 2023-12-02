defmodule AdventOfCode.D01 do
  @spec part1(binary()) :: integer()
  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&decode_part1/1)
    |> Enum.sum()
  end

  @spec digit?(binary()) :: boolean()
  defp digit?(x) do
    case Integer.parse(x) do
      {_, ""} -> true
      _ -> false
    end
  end

  @spec decode_part1(binary()) :: integer()
  defp decode_part1(line) do
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

  @spec part2(binary()) :: integer()
  def part2(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&decode_part2/1)
    |> Enum.sum()
  end

  @spec consume(binary(), list(integer())) :: list(integer())
  def consume("", acc), do: Enum.reverse(acc)

  @spec consume(binary(), list(integer())) :: list(integer())
  def consume(line, acc) do
    # we consume the first character, keeping the remaining of the line to recurse on
    <<_::utf8, rest::binary>> = line

    case line do
      <<"zero", _::binary>> ->
        consume(rest, [0 | acc])

      <<"one", _::binary>> ->
        consume(rest, [1 | acc])

      <<"two", _::binary>> ->
        consume(rest, [2 | acc])

      <<"three", _::binary>> ->
        consume(rest, [3 | acc])

      <<"four", _::binary>> ->
        consume(rest, [4 | acc])

      <<"five", _::binary>> ->
        consume(rest, [5 | acc])

      <<"six", _::binary>> ->
        consume(rest, [6 | acc])

      <<"seven", _::binary>> ->
        consume(rest, [7 | acc])

      <<"eight", _::binary>> ->
        consume(rest, [8 | acc])

      <<"nine", _::binary>> ->
        consume(rest, [9 | acc])

      _ ->
        # check if the first character is an actual integer digit
        case line |> String.at(0) |> Integer.parse() do
          {i, ""} -> consume(rest, [i | acc])
          _ -> consume(rest, acc)
        end
    end
  end

  @spec decode_part2(binary()) :: integer()
  defp decode_part2(line) do
    digits =
      line
      |> String.trim()
      |> consume([])

    # assumes at least 1 digit is present
    first = hd(digits)
    last = List.last(digits)

    Integer.undigits([first, last])
  end
end
