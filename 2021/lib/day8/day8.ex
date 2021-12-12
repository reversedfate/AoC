defmodule Day8 do
  # [
  #   [[<different signals>],[<list of actual out put numbers>]],
  # ...
  # ]
  def parseInput(inputString) do
    lines = inputString
    |> String.replace("\r\n","\n")
    |> String.split("\n")

    lines # each line
    |> Enum.map(fn e -> String.split(e, " | ") # right now the line looks like ["asd, asd, asd", "asd, asd"]
      |> Enum.map(fn e -> String.split(e, " ") end)
    end)
  end

  # maps signals as %{3 => ["asd", "dsa"], 4 => ["1234"]}
  def mapSignalsByLength(randomSignals) do
    randomSignals
    |> Enum.reduce(%{}, fn e, acc ->
      key = String.length(e)
      Map.put(acc, key, Map.get(acc, key, []) ++ [e])
    end)
  end

  # figures out signals for 1, 4, 7, 8, because they have a unique length
  def figureOutBasicSignals(signalLengthMap) do
    signalLengthMap
    |> Enum.reduce(%{}, fn {k, v}, acc ->
      Map.merge(
        acc,
        Enum.reduce(v, %{}, fn e, acc ->
          cond do
            k == 2 -> Map.put(acc, e, 1)
            k == 4 -> Map.put(acc, e, 4)
            k == 3 -> Map.put(acc, e, 7)
            k == 7 -> Map.put(acc, e, 8)
            true -> acc
          end
        end)
      )
    end)
  end

  # returns a map as
  # %{signal1 => number1, signal2, number2, ...}
  # where key = digit
  def figureOutSignalMap(randomSignals) do
    signalLengthMap = mapSignalsByLength(randomSignals)
    simpleSignalMap = figureOutBasicSignals(signalLengthMap)

    simpleSignalMap
  end

  defp part1ProcessLine(line) do
    randomSignals = Enum.at(line, 0)
    numberSignals = Enum.at(line, 1)

    signalMap = figureOutSignalMap(numberSignals) # part 1 specifically want to look at the number part of line

    numberSignals
    |> Enum.filter(fn e -> Map.has_key?(signalMap, e) end)
    |> Enum.map(fn e -> Map.get(signalMap, e) end)
    |> Enum.filter(fn e -> e == 1 or e == 4 or e == 7 or e == 8 end)
    |> Enum.count()
  end

  def sortString(str) do
    str |> String.split("") |> Enum.filter(fn e -> e != "" end) |> Enum.sort() |> Enum.join("")
  end


  #  000
  # 1   2
  # 1   2
  #  333
  # 4   5
  # 4   5
  #  666
  def constructNumberKeys(codeKey) do
    [
      sortString(Enum.at(codeKey,0) <> Enum.at(codeKey,1) <> Enum.at(codeKey,2) <> Enum.at(codeKey,4) <> Enum.at(codeKey,5) <> Enum.at(codeKey,6)                      ),
      sortString(                                            Enum.at(codeKey,2) <>                                             Enum.at(codeKey,5)                      ),
      sortString(Enum.at(codeKey,0) <>                       Enum.at(codeKey,2) <> Enum.at(codeKey,3) <> Enum.at(codeKey,4) <>                       Enum.at(codeKey,6)),
      sortString(Enum.at(codeKey,0) <>                       Enum.at(codeKey,2) <> Enum.at(codeKey,3) <>                       Enum.at(codeKey,5) <> Enum.at(codeKey,6)),
      sortString(                      Enum.at(codeKey,1) <> Enum.at(codeKey,2) <> Enum.at(codeKey,3) <>                       Enum.at(codeKey,5)                      ),
      sortString(Enum.at(codeKey,0) <> Enum.at(codeKey,1) <>                       Enum.at(codeKey,3) <>                       Enum.at(codeKey,5) <> Enum.at(codeKey,6)),
      sortString(Enum.at(codeKey,0) <> Enum.at(codeKey,1) <>                       Enum.at(codeKey,3) <> Enum.at(codeKey,4) <> Enum.at(codeKey,5) <> Enum.at(codeKey,6)),
      sortString(Enum.at(codeKey,0) <>                       Enum.at(codeKey,2) <>                                             Enum.at(codeKey,5)                      ),
      sortString(Enum.at(codeKey,0) <> Enum.at(codeKey,1) <> Enum.at(codeKey,2) <> Enum.at(codeKey,3) <> Enum.at(codeKey,4) <> Enum.at(codeKey,5) <> Enum.at(codeKey,6)),
      sortString(Enum.at(codeKey,0) <> Enum.at(codeKey,1) <> Enum.at(codeKey,2) <> Enum.at(codeKey,3) <>                       Enum.at(codeKey,5) <> Enum.at(codeKey,6)),
    ]
  end

  def doesKeyConstructAllNumbers(codeKey, randomSignals) do
    numberCodes = constructNumberKeys(codeKey)

    #then this is only a matter of checking of all random signals(alphabetically sorted) are also in numberCodes
    containedSignals = randomSignals
    |> Enum.filter(fn e -> Enum.member?(numberCodes, sortString(e)) end)

    # IO.inspect({length(randomSignals), length(containedSignals)})
    length(randomSignals) == length(containedSignals)
  end

  # permutation is correct only if using the initial 10 random signals, it generates 10 different digit keys
  def findCorrectPermutation(symbols, codePermutations, randomSignals) do
    codePermutations
    |> Enum.map(fn e ->
      Enum.map(e, fn number -> Enum.at(symbols, number) end) # each permutation now contains the symbols from input
    end)
    |> Enum.reduce([], fn e, acc ->
      if doesKeyConstructAllNumbers(e, randomSignals) do
        [e | acc]
      else
        acc
      end
    end)
    |> Enum.at(0)
  end

  defp part2ProcessLine(line, codePermutations) do
    randomSignals = Enum.at(line, 0)
    numberSignals = Enum.at(line, 1)

    allSymbols = figureOutSignalMap(randomSignals)
    |> Enum.filter(fn {_,v} ->  v == 8 end)
    |> Enum.map(fn {k,_} ->  k end)
    |> Enum.at(0)
    |> String.split("")
    |> Enum.filter(fn e -> e != "" end)

    correctPermutation = findCorrectPermutation(allSymbols, codePermutations, randomSignals)

    numberKeys = constructNumberKeys(correctPermutation)
    numberSignals
    |> Enum.map(fn e -> sortString(e) end)
    |> Enum.map(fn e ->
      cond do
        e == Enum.at(numberKeys, 0) -> 0
        e == Enum.at(numberKeys, 1) -> 1
        e == Enum.at(numberKeys, 2) -> 2
        e == Enum.at(numberKeys, 3) -> 3
        e == Enum.at(numberKeys, 4) -> 4
        e == Enum.at(numberKeys, 5) -> 5
        e == Enum.at(numberKeys, 6) -> 6
        e == Enum.at(numberKeys, 7) -> 7
        e == Enum.at(numberKeys, 8) -> 8
        e == Enum.at(numberKeys, 9) -> 9
        true -> nil
      end
    end)
    |> Enum.reduce(0, fn e, acc ->
      acc * 10 + e
    end)
  end

  def part1(inputString) do
    parsedInput = parseInput(inputString)

    parsedInput
    |> Enum.map(fn e -> part1ProcessLine(e) end)
    |> Enum.sum()
  end

  def part2(inputString) do
    parsedInput = parseInput(inputString)
    codePermutations = Permutation.permutaionsWithoutRepetition([0,1,2,3,4,5,6]) # since there are only permutations of 7 elements this approach is ok, since each line will have to check up to 7! = 5040 solutions
    # TODO there should be more performan solution, the current one takes all permutations of possible random positions the signals could get mixed up
    # mixed solution which reduces some permutations instantly?

    parsedInput
    |> Enum.map(fn e -> part2ProcessLine(e, codePermutations) end)
    |> Enum.sum()
  end
end
