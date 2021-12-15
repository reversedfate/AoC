defmodule Day9 do
  # {map2d, width, height}
  # map2d = %{
  #   {x,y} => height,
  #  ...
  # }
  def parseInput(inputString) do
    lines = inputString
    |> String.replace("\r\n","\n")
    |> String.split("\n")

    list2d = lines # each line
    |> Enum.map(fn e -> String.split(e, "")
      |> Enum.filter(fn e -> e != "" end)
      |> Enum.map(fn e -> Integer.parse(e) |> elem(0) end)
    end)

    height = list2d |> length
    width = list2d |> hd |> length

    Enum.reduce(1..width, %{}, fn eX, acc ->
        Enum.reduce(1..height, acc, fn eY, acc ->
          Map.put(acc, {eX-1, eY-1}, list2d |> Enum.at(eY-1) |> Enum.at(eX-1))
        end)
      end)
  end

  def findLowPoints(parsedInput) do
    parsedInput
    |> Enum.filter(fn {k,v} ->
      relativeNeighbours = [
        {(k |> elem(0)) - 1,    (k |> elem(1))    },
        {(k |> elem(0)) + 1,    (k |> elem(1))    },
        {(k |> elem(0)),        (k |> elem(1)) - 1},
        {(k |> elem(0)),        (k |> elem(1)) + 1},
      ]
      |> Enum.filter(fn e -> Map.has_key?(parsedInput, e) end)

      nonLargerNeighboursHeights = relativeNeighbours
      |> Enum.map(fn e -> Map.get(parsedInput, e) end)
      |> Enum.filter(fn e -> e <= v end)

      nonLargerNeighboursHeights |> length == 0
    end)
  end

  def growBasin(basin, parsedInput) do
    basin
    |> Enum.reduce(basin, fn {k,_}, acc ->
      relativeNeighbourPositions = [
        {(k |> elem(0)) - 1,    (k |> elem(1))    },
        {(k |> elem(0)) + 1,    (k |> elem(1))    },
        {(k |> elem(0)),        (k |> elem(1)) - 1},
        {(k |> elem(0)),        (k |> elem(1)) + 1},
      ]
      |> Enum.filter(fn e -> Map.has_key?(parsedInput, e) end)
      |> Enum.filter(fn e -> Map.get(parsedInput, e) != 9 end) # 9s are never part of basin

      relativeNeighbours = parsedInput
      |> Enum.filter(fn {k,_} -> Enum.member?(relativeNeighbourPositions, k) end)
      |> Enum.reduce(%{}, fn e, acc ->
        Map.put(acc, elem(e, 0), elem(e,1))
      end)

      Map.merge(acc, relativeNeighbours)
    end)
  end

  def generateBasin(previousBasin, parsedInput) do
    # previous basin initially is %{<low point>}
    newBasin = growBasin(previousBasin, parsedInput)

    previousBasinLength = length(previousBasin |> Map.keys)
    newBasinLength = length(newBasin |> Map.keys)

    if previousBasinLength == newBasinLength do # no more growth = basin captured
      newBasin
    else
      generateBasin(newBasin, parsedInput)
    end
  end

  def part1(inputString) do
    parsedInput = parseInput(inputString)

    lowPoints = findLowPoints(parsedInput)

    lowPoints
    |> Enum.reduce(0, fn {_,v}, acc ->
      acc + (v + 1)
    end)
  end

  def part2(inputString) do
    parsedInput = parseInput(inputString)

    lowPoints = findLowPoints(parsedInput)

    # generate basins from each low point
    lowPoints
    |> Enum.map(fn {k,v} ->
      generateBasin(%{k=>v}, parsedInput)
    end)
    |> Enum.map(fn e -> length(e |> Map.keys) end)
    |> Enum.sort()
    |> Enum.reverse()
    |> Enum.slice(0..2)
    |> Enum.product()
  end
end
