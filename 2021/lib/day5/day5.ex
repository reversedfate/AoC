defmodule Day5 do
  # [
  #   [start, end],
  # ...
  # ]
  # start, end = [x, y]
  def parseInput(inputString) do
    lines = inputString
    |> String.replace("\r\n","\n")
    |> String.split("\n")

    lines
    |> Enum.map(fn e -> String.split(e, " -> ") # "x,y -> x,y"
      |> Enum.map(fn e -> String.split(e, ",") # "x,y"
        |> Enum.map(fn e -> elem(Integer.parse(e), 0) end) # ["x", "y"]
      end)
    end)
  end

  def generateMap(parsedInput) do
    parsedInput
    |> Enum.reduce(%{}, fn e, acc ->
      xPositions = [Enum.at(Enum.at(e, 0), 0),Enum.at(Enum.at(e, 1), 0)]
      |> Enum.sort()
      xRange = Enum.at(xPositions, 0)..Enum.at(xPositions, 1)
      xRangeList = xRange |> Enum.to_list()

      yPositions = [Enum.at(Enum.at(e, 0), 1),Enum.at(Enum.at(e, 1), 1)]
      |> Enum.sort()
      yRange = Enum.at(yPositions, 0)..Enum.at(yPositions, 1)
      yRangeList = yRange |> Enum.to_list()

      acc = Enum.reduce(xRangeList, acc, fn x, acc ->
        Enum.reduce(yRangeList, acc, fn y, acc ->
          key = {x,y}
          acc = Map.put(acc, key, Map.get(acc, key, 0) + 1)
        end)
      end)
    end)
  end

  def step(i) do
    if i >= 0 do
      1
    else
      -1
    end
  end

  def generateAndAppendDiagonals(generatedVentMap, parsedInput) do
    parsedInput
    |> Enum.reduce(generatedVentMap, fn e, acc ->
      xPositions = [Enum.at(Enum.at(e, 0), 0),Enum.at(Enum.at(e, 1), 0)]
      x1 = Enum.at(xPositions, 0)
      x2 = Enum.at(xPositions, 1)
      xRange = x1..x2//step(x2-x1)
      xRangeList = xRange |> Enum.to_list()

      yPositions = [Enum.at(Enum.at(e, 0), 1),Enum.at(Enum.at(e, 1), 1)]
      y1 = Enum.at(yPositions, 0)
      y2 = Enum.at(yPositions, 1)
      yRange = y1..y2//step(y2-y1)
      yRangeList = yRange |> Enum.to_list()

      positionList = Enum.zip([xRangeList, yRangeList])

      acc = Enum.reduce(positionList, acc, fn key, acc ->
          Map.put(acc, key, Map.get(acc, key, 0) + 1)
      end)
    end)
  end

  def countLargerThan(generatedVentMap, count) do
    generatedVentMap
    |> Enum.filter(fn {_, v} ->
      v >= 2
    end)
    |> Enum.reduce(0, fn _e, acc -> acc + 1 end)
  end

  def filterOnlyHorizontalOrVertical(filteredInput) do
    filteredInput
    |> Enum.filter(fn e ->
      Enum.at(Enum.at(e, 0), 0) == Enum.at(Enum.at(e, 1), 0) or
      Enum.at(Enum.at(e, 0), 1) == Enum.at(Enum.at(e, 1), 1)
    end)
  end

  def filterOnlyDiagonal(filteredInput) do
    filteredInput
    |> Enum.reject(fn e ->
      Enum.at(Enum.at(e, 0), 0) == Enum.at(Enum.at(e, 1), 0) or
      Enum.at(Enum.at(e, 0), 1) == Enum.at(Enum.at(e, 1), 1)
    end)
  end

  def part1(inputString) do
    parsedInput = parseInput(inputString)
    generateMap(parsedInput |> filterOnlyHorizontalOrVertical())
    |> countLargerThan(2)
  end

  def part2(inputString) do
    parsedInput = parseInput(inputString)

    generateMap(parsedInput |> filterOnlyHorizontalOrVertical())
    |> generateAndAppendDiagonals(parsedInput |> filterOnlyDiagonal())
    |> countLargerThan(2)
  end
end
