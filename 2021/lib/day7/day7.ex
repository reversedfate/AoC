defmodule Day7 do
  # [x, y, ...]
  def parseInput(inputString) do
    inputString # only one line
    |> String.split(",")
    |> Enum.map(fn e -> elem(Integer.parse(e), 0) end)
    |> Enum.sort()
  end

  def fuelCost(parsedInput, position) do
    parsedInput
    |> Enum.map(fn e -> abs(position - e) end)
    |> Enum.reduce(0, fn e, acc -> acc + e end)
  end

  def fuelCostAlternate(parsedInput, position) do
    parsedInput
    |> Enum.map(fn e -> 0..abs(position - e) |> Enum.sum end)
    |> Enum.reduce(0, fn e, acc -> acc + e end)
  end

  def findBestPosition(parsedInput) do
    minPosition = parsedInput |> List.first()
    maxPosition = parsedInput |> List.last()

    minPosition..maxPosition
    |> Enum.to_list()
    |> Enum.map(fn e -> fuelCost(parsedInput, e) end)
    |> Enum.min()
  end

  def findBestPositionAlternateFuel(parsedInput) do
    minPosition = parsedInput |> List.first()
    maxPosition = parsedInput |> List.last()

    minPosition..maxPosition
    |> Enum.to_list()
    |> Enum.map(fn e -> fuelCostAlternate(parsedInput, e) end)
    |> Enum.min()
  end

  def part1(inputString) do
    parsedInput = parseInput(inputString)
    findBestPosition(parsedInput)
  end

  def part2(inputString) do
    parsedInput = parseInput(inputString)
    findBestPositionAlternateFuel(parsedInput)
  end
end
