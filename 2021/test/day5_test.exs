defmodule Day5Test do
  use ExUnit.Case
  doctest Aoc2021

  test "basic cases for filtering horizontal and vertical lines from diagonals" do
    input = [
      [[1,10],[1,100]], # horizontal
      [[1,10],[10,10]], # vertical
      [[1,20],[20,1]], # d1
      [[1,1],[20,20]], # d2
    ]

    assert length(Day5.filterOnlyDiagonal(input)) == 2
    assert length(Day5.filterOnlyHorizontalOrVertical(input)) == 2
  end

  test "basic cases for filtering out values in map that are larger than something" do
    input = %{
      {1,0} => 0,
      {0,0} => 1,
      {0,1} => 2,
      {0,2} => 3,
      {0,3} => 4,
    }

    assert Day5.countLargerThan(input, 2) == 3
    assert Day5.countLargerThan(input, 1) == 4
    assert Day5.countLargerThan(input, 1000) == 0
  end

  test "basic cases for step function when determining if a range is incremental or decremental" do
    assert Day5.step(100) == 1
    assert Day5.step(1) == 1
    assert Day5.step(0) == 1
    assert Day5.step(-1) == -1
    assert Day5.step(-100) == -1
  end

  test "given case, both part answers should be correct" do
    str = elem(File.read("lib/day5/test1.txt"), 1)

    assert Day5.part1(str) == 5
    assert Day5.part2(str) == 12
  end
end
