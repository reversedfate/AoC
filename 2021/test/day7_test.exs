defmodule Day7Test do
  use ExUnit.Case
  doctest Aoc2021

  test "given case, both part answers should be correct" do
    str = elem(File.read("lib/day7/test1.txt"), 1)

    assert Day7.part1(str) == 37
    assert Day7.part2(str) == 168
  end
end
