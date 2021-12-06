defmodule Day5Test do
  use ExUnit.Case
  doctest Aoc2021

  test "given case, both part answers should be correct" do
    str = elem(File.read("lib/day5/test1.txt"), 1)

    assert Day5.part1(str) == 5
    assert Day5.part2(str) == 12
  end
end
