defmodule Day1Test do
  use ExUnit.Case
  doctest Aoc2021

  test "should parse input correctly" do
    assert Day1.parseInput("333") == [333]
    assert Day1.parseInput("2\n3") == [2, 3]
    assert Day1.parseInput("2\n3\n5\n9") == [2, 3, 5, 9]
    assert Day1.parseInput("2\r\n3\r\n5\r\n9") == [2, 3, 5, 9]
  end

  test "count positive increases" do
    assert Day1.countPositiveIncreases([1, 2, 3]) == 2
    assert Day1.countPositiveIncreases([4, 1, 2, 3]) == 2
    assert Day1.countPositiveIncreases([1, 1, 1]) == 0
    assert Day1.countPositiveIncreases([1, 2, 1]) == 1
  end

  test "group inputs by 3" do
    assert Day1.group3Inputs([1, 2, 3]) == [6]
    assert Day1.group3Inputs([1, 2, 3, 4]) == [6, 9]
    assert Day1.group3Inputs([1, 2, 3, 1, 2, 3]) == [6, 6, 6, 6]
  end

  test "simple test data" do
    str = "1\n2\n3\n4"
    assert Day1.part1(str) == 3
    assert Day1.part2(str) == 1
  end

  test "given test case" do
    str = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    |> Enum.join("\n")

    assert Day1.part1(str) == 7
    assert Day1.part2(str) == 5
  end
end
