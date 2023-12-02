defmodule AdventOfCode.D02.RGB do
  alias AdventOfCode.D02.RGB

  @type t() :: %RGB{}

  # if a key isn't specified it's ok to default to 0 in this case
  defstruct red: 0,
            green: 0,
            blue: 0

  @spec from_line!(binary()) :: t()
  def from_line!(line) do
    parse_part = fn part ->
      case part |> String.trim() |> String.split(" ") do
        [n, "red"] -> %RGB{red: String.to_integer(n)}
        [n, "green"] -> %RGB{green: String.to_integer(n)}
        [n, "blue"] -> %RGB{blue: String.to_integer(n)}
      end
    end

    line
    |> String.trim()
    |> String.split(",", trim: true)
    |> Enum.map(parse_part)
    |> RGB.add()
  end

  @spec add(list(t())) :: t()
  def add(list) do
    Enum.reduce(
      list,
      %RGB{},
      fn element, acc ->
        %RGB{
          red: element.red + acc.red,
          green: element.green + acc.green,
          blue: element.blue + acc.blue
        }
      end
    )
  end

  @spec max(list(t())) :: t()
  def max([]), do: %RGB{}

  @spec max(list(t())) :: t()
  def max(list) do
    first = hd(list)

    Enum.reduce(
      list,
      first,
      fn element, acc ->
        %RGB{
          red: max(element.red, acc.red),
          green: max(element.green, acc.green),
          blue: max(element.blue, acc.blue)
        }
      end
    )
  end
end

defmodule AdventOfCode.D02 do
  alias AdventOfCode.D02.RGB

  @spec part1(binary()) :: integer()
  def part1(input) do
    # "configuration" for the given input
    max_allowed = %RGB{
      red: 12,
      green: 13,
      blue: 14
    }

    decode =
      fn line ->
        [game, draws] = String.split(line, ":")

        # the integer game id
        [_, id] = String.split(game, " ")
        id = String.to_integer(id)

        # the highest R/G/B value seen across all draws in this game
        max_seen =
          draws
          |> String.split(";", trim: true)
          |> Enum.map(&RGB.from_line!/1)
          |> RGB.max()

        # as a simplification "trick", return 0 for any discarded games
        # and return the game id otherwise
        if max_seen.red <= max_allowed.red and
             max_seen.green <= max_allowed.green and
             max_seen.blue <= max_allowed.blue do
          id
        else
          0
        end
      end

    input
    |> String.split("\n", trim: true)
    |> Enum.map(decode)
    |> Enum.sum()
  end
end
