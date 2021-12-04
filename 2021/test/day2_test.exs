defmodule Day2Test do
  use ExUnit.Case
  doctest Aoc2021

  test "given case, should parse input correctly" do
    str = ["forward 5","down 5","forward 8","up 3","down 8","forward 2"]
    |> Enum.join("\n")

    assert Day2.parseInput(str) == [
      {"forward", 5},
      {"down", 5},
      {"forward", 8},
      {"up", 3},
      {"down", 8},
      {"forward", 2}
    ]
  end

  test "given case, should calculate forward displacement correctly" do
    str = ["forward 5","down 5","forward 8","up 3","down 8","forward 2"]
    |> Enum.join("\n")
    input = Day2.parseInput(str)

    assert Day2.finalHorizontalPosition(input) == 15
  end

  test "given case, should calculate depth displacement correctly" do
    str = ["forward 5","down 5","forward 8","up 3","down 8","forward 2"]
    |> Enum.join("\n")
    input = Day2.parseInput(str)

    assert Day2.finalDepth(input) == 10
  end

  test "given case, both part answers should be correct" do
    str = ["forward 5","down 5","forward 8","up 3","down 8","forward 2"]
    |> Enum.join("\n")

    assert Day2.part1(str) == 150
    assert Day2.part2(str) == 900
  end
end
