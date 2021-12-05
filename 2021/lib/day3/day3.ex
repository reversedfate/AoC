defmodule Day3 do
  def parseInput(inputString) do
    inputString
    |> String.replace("\r\n","\n")
    |> String.split("\n")
    |> Enum.map(fn e -> String.split(e, "")
      |> Enum.reject(fn e->e=="" end)
    end)
  end

  def decimalToBinary(decimal) when is_binary(decimal) do
    decimal
    |> String.to_integer()
    |> decimalToBinary()
  end

  def decimalToBinary(decimal) when is_integer(decimal) do
    decimal
    |> Integer.to_string(2)
  end

  def gamma(input) do
    input
    |> Enum.zip()
    |> Enum.map(fn e -> Tuple.to_list(e) end)
    |> Enum.map(fn e -> e |> Enum.reduce(%{}, fn x, acc -> Map.update(acc, x, 1, &(&1 + 1)) end) end) # count the number of symbols, at this point this is a map of symbols and occourences
    |> Enum.map(fn e -> e
      |> Enum.reduce({"", 0}, fn ({k, v}, acc) -> if v > elem(acc, 1) or (v == elem(acc, 1) and k == "1"), do: {k, v}, else: acc end)
    end)
    |> Enum.map(fn e -> e |> elem(0) end)
    |> Enum.join("")
    |> Integer.parse(2)
    |> elem(0)
  end

  def epsilon(input) do
    input
    |> Enum.zip()
    |> Enum.map(fn e -> Tuple.to_list(e) end)
    |> Enum.map(fn e -> e |> Enum.reduce(%{}, fn x, acc -> Map.update(acc, x, 1, &(&1 + 1)) end) end) # count the number of symbols, at this point this is a map of symbols and occourences
    |> Enum.map(fn e -> e
      |> Enum.reduce({"", 0}, fn ({k, v}, acc) -> if v < elem(acc, 1) or (v == elem(acc, 1) and k == "0") or elem(acc, 0) == "", do: {k, v}, else: acc end)
    end)
    |> Enum.map(fn e -> e |> elem(0) end)
    |> Enum.join("")
    |> Integer.parse(2)
    |> elem(0)
  end

  def oxygen(parsedInput, index) do
    minFilterKeyLength = hd(parsedInput) |> length()

    filterKey = gamma(parsedInput)
    |> decimalToBinary() # get most common bits
    |> String.pad_leading(minFilterKeyLength, "0")

    symbol = filterKey
    |> String.split("") |> Enum.reject(fn e -> e == "" end)
    |> Enum.at(index)

    reducedList = parsedInput
    |> Enum.filter(fn e -> Enum.at(e, index) == symbol end)

    if length(reducedList) == 1 do
      Enum.at(reducedList, 0)
      |> Enum.join("")
      |> Integer.parse(2)
      |> elem(0)
    else
      oxygen(reducedList, index+1)
    end
  end

  def lifeSupport(parsedInput, index) do
    # IO.inspect(parsedInput)
    minFilterKeyLength = hd(parsedInput) |> length()

    filterKey = epsilon(parsedInput)
    # |> IO.inspect()
    |> decimalToBinary() # get most common bits
    |> String.pad_leading(minFilterKeyLength, "0")

    symbol = filterKey
    |> String.split("") |> Enum.reject(fn e -> e == "" end)
    # |> IO.inspect()
    |> Enum.at(index)

    # IO.inspect({filterKey, index, symbol})

    reducedList = parsedInput
    |> Enum.filter(fn e -> Enum.at(e, index) == symbol end)
    # IO.inspect(reducedList)

    if length(reducedList) == 1 do
      Enum.at(reducedList, 0)
      |> Enum.join("")
      |> Integer.parse(2)
      |> elem(0)
    else
      lifeSupport(reducedList, index+1)
    end
  end


  def part1(inputString) do
    parsedInput = parseInput(inputString)
    ga = gamma(parsedInput)
    ep = epsilon(parsedInput)
    # IO.inspect({ga, ep})
    ga * ep
  end

  def part2(inputString) do
    parsedInput = parseInput(inputString)
    ox = oxygen(parsedInput, 0)
    ls = lifeSupport(parsedInput, 0)
    # IO.inspect({ox, ls})
    ox * ls
  end

end
