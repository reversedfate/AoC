defmodule Day9Test do
  use ExUnit.Case
  doctest Aoc2021

  # findLowPoints
  test "basic cases for finding low points" do
    map1 = %{
      {0,0} => 1,
      {1,0} => 2,
      {0,1} => 3,
      {1,1} => 4
    }
    assert Day9.findLowPoints(map1) |> length == 1

    map2 = %{
      {0,0} => 1,
      {1,0} => 2,
      {0,1} => 2,
      {1,1} => 1
    }
    assert Day9.findLowPoints(map2) |> length == 2
  end

  test "given case, both part answers should be correct" do
    str = elem(File.read("lib/day9/test1.txt"), 1)

    assert Day9.part1(str) == 15
    assert Day9.part2(str) == 1134
  end
end
