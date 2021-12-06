defmodule Day6Test do
  use ExUnit.Case
  doctest Aoc2021

  test "given case, both part answers should be correct" do
    str = elem(File.read("lib/day6/test1.txt"), 1)

    assert Day6.part1(str) == 5934
    assert Day6.part2(str) == 26984457539
  end
end
