defmodule Day4 do

  # {list of numbers, list of bingo cards}
  # bingo card = map of maps, %{0 => %{0 => ..., ...}}
  def parseInput(inputString) do
    lines = inputString
    |> String.replace("\r\n","\n")
    |> String.split("\n")
    # |> IO.inspect()

    numbers = hd(lines)
    |> String.split(",")
    |> Enum.map(fn e -> elem(Integer.parse(e), 0) end)

    bingoCards = tl(lines)
    |> Enum.reject(fn e -> e == "" end) # remove all empty lines
    |> Enum.chunk_every(5)
    |> Enum.map(fn e -> # whole bingo
      Enum.map(e, fn e -> # line
        e
        |> String.replace("  ", " ")
        |> String.replace(~r/^ /, "")
        |> String.split(" ")
        |> Enum.map(fn e -> elem(Integer.parse(e), 0) end) # each line element
      end)
      |> Matrix.from_list() # actually create the map from 2d array / list
    end)

    {numbers, bingoCards}
  end

  def hasBingoWon(bingoCard, currentNumbers) do
    winningRows = 0..4
    |> Enum.to_list()
    |> Enum.map(fn y ->
      0..4
      |> Enum.to_list()
      |> Enum.map(fn x ->
        bingoCard[x][y]
      end)
    end)
    |> Enum.filter(fn e ->
      Enum.all?(e, fn e -> Enum.member?(currentNumbers, e) end)
    end)

    winningColumns = 0..4
    |> Enum.to_list()
    |> Enum.map(fn y ->
      0..4
      |> Enum.to_list()
      |> Enum.map(fn x ->
        bingoCard[y][x]
      end)
    end)
    |> Enum.filter(fn e ->
      Enum.all?(e, fn e -> Enum.member?(currentNumbers, e) end)
    end)

    length(winningRows) > 0 or length(winningColumns) > 0
  end

  def sumOfAllUnpickedNumbers(bingoCard, currentNumbers) do
    bingoCard
    |> Enum.reduce(0, fn ({_x, v}, acc) ->
      Enum.reduce(v, acc, fn ({_vx, y}, acc) ->
        if Enum.member?(currentNumbers, y) do
          acc
        else
          acc + y
        end
      end)
    end)
  end

  def findWinningBingoAndNumber(numbers, bingoCards, index) do
    currentNumbers = numbers
    |> Enum.slice(0..index)

    winningBingoCards = bingoCards
    |> Enum.filter(fn e -> hasBingoWon(e, currentNumbers) end)

    if length(winningBingoCards) == 1 do
      {Enum.at(winningBingoCards, 0), currentNumbers |> Enum.reverse |> hd, sumOfAllUnpickedNumbers(Enum.at(winningBingoCards, 0), currentNumbers)}
    else
      findWinningBingoAndNumber(numbers, bingoCards, index+1)
    end
  end

  def findLastWinningBingoAndNumber(numbers, bingoCards, index) do
    currentNumbers = numbers
    |> Enum.slice(0..index)

    winningBingoCards = bingoCards
    |> Enum.filter(fn e -> hasBingoWon(e, currentNumbers) end)

    notWinningBingoCards = bingoCards
    |> Enum.reject(fn e -> hasBingoWon(e, currentNumbers) end)

    if length(winningBingoCards) == 1 and length(notWinningBingoCards) == 0 do
      {Enum.at(winningBingoCards, 0), currentNumbers |> Enum.reverse |> hd, sumOfAllUnpickedNumbers(Enum.at(winningBingoCards, 0), currentNumbers)}
    else
      findLastWinningBingoAndNumber(numbers, notWinningBingoCards, index+1)
    end
  end

  def part1(inputString) do
    {numbers, bingoCards} = parseInput(inputString)

    {_, winningNumber, winningUnusedSum} = findWinningBingoAndNumber(numbers, bingoCards, 0)
    winningNumber * winningUnusedSum
  end

  def part2(inputString) do
    {numbers, bingoCards} = parseInput(inputString)

    {_, winningNumber, winningUnusedSum} = findLastWinningBingoAndNumber(numbers, bingoCards, 0)
    winningNumber * winningUnusedSum
  end

end
