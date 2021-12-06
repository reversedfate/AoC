defmodule Day6 do
  # %{0=>n0, ...}
  def parseInput(inputString) do
    inputString # only one line
    |> String.split(",")
    |> Enum.map(fn e -> elem(Integer.parse(e), 0) end)
    |> Enum.reduce(%{}, fn e, acc ->
      acc = Map.put(acc, e, Map.get(acc, e, 0) + 1)
    end)
  end


  def simulateDays(currentFish, currentDay, maxDay) do
    if currentDay == maxDay do
      currentFish
    else
      newFish = currentFish
      |> Enum.reduce(%{}, fn {k,v}, acc ->
        if k > 0 do
          Map.put(acc, k-1, Map.get(acc, k-1, 0) + v)
        else
          acc = Map.put(acc, 6, Map.get(acc, 6, 0) + v)
          acc = Map.put(acc, 8, Map.get(acc, 8, 0) + v)
          acc
        end
      end)
      simulateDays(newFish, currentDay + 1, maxDay)
    end
  end

  def part1(inputString) do
    parsedInput = parseInput(inputString)

    simulateDays(parsedInput, 0, 80)
    |> Enum.reduce(0, fn {_k, v}, acc ->
      acc + v
    end)
  end

  def part2(inputString) do
    parsedInput = parseInput(inputString)

    simulateDays(parsedInput, 0, 256)
    |> Enum.reduce(0, fn {_k, v}, acc ->
      acc + v
    end)
  end
end
