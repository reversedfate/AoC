defmodule Day4Test do
  use ExUnit.Case
  doctest Aoc2021

  test "given case, finds correct answer recursively " do
    str = elem(File.read("lib/day4/input.txt"), 1)
    {numbers, bingoCards} = Day4.parseInput(str)

    assert Day4.findWinningBingoAndNumber(numbers, bingoCards, 0) == {3, 10}
  end

  # test "given case, both part answers should be correct" do
  #   str = elem(File.read("lib/day4/input.txt"), 1)

  #   assert Day4.part1(str) == 4512
  #   # assert Day3.part2(str) == 230
  # end
end
