defmodule Day3Test do
  use ExUnit.Case
  doctest Aoc2021

  test "should parse strings into binary" do

  end

  test "given case, should parse input correctly" do
    str = ["00100","11110","10110","10111","10101","01111","00111","11100","10000","11001","00010","01010"]
    |> Enum.join("\n")

    assert Day3.parseInput(str) == [["0", "0", "1", "0", "0"], ["1", "1", "1", "1", "0"], ["1", "0", "1", "1", "0"], ["1", "0", "1", "1", "1"], ["1", "0", "1", "0", "1"], ["0", "1", "1", "1", "1"], ["0", "0", "1", "1", "1"], ["1", "1", "1", "0", "0"], ["1", "0", "0", "0", "0"], ["1", "1", "0", "0", "1"], ["0", "0", "0", "1", "0"], ["0", "1", "0", "1", "0"]]
  end

  test "given case, both part answers should be correct" do
    str = ["00100","11110","10110","10111","10101","01111","00111","11100","10000","11001","00010","01010"]
    |> Enum.join("\n")

    assert Day3.part1(str) == 198
    assert Day3.part2(str) == 230
  end
end