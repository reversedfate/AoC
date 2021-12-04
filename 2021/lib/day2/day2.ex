defmodule Day2 do
  def parseInput(inputString) do
    inputString
    |> String.replace("\r\n","\n")
    |> String.split("\n")
    |> Enum.map(fn e -> List.to_tuple(String.split(e, " ")) end)
    |> Enum.map(fn e -> {elem(e, 0), elem(Integer.parse(elem(e, 1)), 0)} end)
  end

  def finalHorizontalPosition(input) do
    input
    |> Enum.filter(fn el -> elem(el, 0) == "forward" end)
    |> Enum.map(fn el -> elem(el, 1) end)
    |> Enum.reduce(fn (el, acc) -> acc + el end)
  end

  def finalDepth(input) do
    upMovement = input
    |> Enum.filter(fn el -> elem(el, 0) == "up" end)
    |> Enum.map(fn el -> elem(el, 1) end)
    |> Enum.reduce(fn (el, acc) -> acc + el end)

    downMovement = input
    |> Enum.filter(fn el -> elem(el, 0) == "down" end)
    |> Enum.map(fn el -> elem(el, 1) end)
    |> Enum.reduce(fn (el, acc) -> acc + el end)

    downMovement - upMovement
  end

  def part1(inputString) do
    parsedInput = parseInput(inputString)

    fHorizontalPosition = finalHorizontalPosition(parsedInput)
    fDepth = finalDepth(parsedInput)

    fHorizontalPosition * fDepth
  end

  def part2(inputString) do
    parsedInput = parseInput(inputString)

    {finalDepth, finalHorizontalPosition, finalAngle} = Enum.reduce(parsedInput, {0, 0, 0},
    fn command, acc ->
      cond do
        elem(command, 0) == "forward" ->
          {elem(acc, 0) + elem(acc, 2) * elem(command, 1), elem(acc, 1) + elem(command, 1), elem(acc, 2)} # add depth equal to angle, add horizontal equal to forward
        elem(command, 0) == "up" ->
          {elem(acc, 0), elem(acc, 1), elem(acc, 2) - elem(command, 1)} # just change angle
        elem(command, 0) == "down" ->
          {elem(acc, 0), elem(acc, 1), elem(acc, 2) + elem(command, 1)} # just change angle
      end
    end)

    finalDepth * finalHorizontalPosition
  end
end
