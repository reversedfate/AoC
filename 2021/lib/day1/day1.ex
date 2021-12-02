defmodule Day1 do
  def parseInput(inputString) do
    inputString
    |> String.replace("\r\n","\n")
    |> String.split("\n")
    |> Enum.map(&String.to_integer/1)
  end

  def countPositiveIncreases(input) do
    input
    |> Enum.zip([0] ++ input)
    |> tl()
    |> Enum.map(fn e -> elem(e, 0) - elem(e, 1) end)
    |> Enum.count(fn e -> e > 0 end)
  end

  def group3Inputs(input) do
    [input, [0] ++ input, [0, 0] ++ input]
    |> Enum.zip_with(fn [x, y, z] -> x + y + z end)
    |> tl() |> tl()
  end

  def part1(inputString) do
    countPositiveIncreases(parseInput(inputString))
  end

  def part2(inputString) do
    countPositiveIncreases(group3Inputs(parseInput(inputString)))
  end
end
