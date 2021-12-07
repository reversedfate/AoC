defmodule Day6Test do
  use ExUnit.Case
  doctest Aoc2021

  test "basic cases for simulating days" do
    assert Day6.simulateDays(%{}, 0, 0) == %{}
    assert Day6.simulateDays(%{}, 0, 100) == %{}

    # basic behaviour by stage with one fish, simulate one step
    assert Day6.simulateDays(%{8=>1}, 0, 1) == %{7=>1}
    assert Day6.simulateDays(%{7=>1}, 0, 1) == %{6=>1}
    assert Day6.simulateDays(%{6=>1}, 0, 1) == %{5=>1}
    assert Day6.simulateDays(%{5=>1}, 0, 1) == %{4=>1}
    assert Day6.simulateDays(%{4=>1}, 0, 1) == %{3=>1}
    assert Day6.simulateDays(%{3=>1}, 0, 1) == %{2=>1}
    assert Day6.simulateDays(%{2=>1}, 0, 1) == %{1=>1}
    assert Day6.simulateDays(%{1=>1}, 0, 1) == %{0=>1}
    assert Day6.simulateDays(%{0=>1}, 0, 1) == %{6=>1, 8=>1}
  end

  test "given case, both part answers should be correct" do
    str = elem(File.read("lib/day6/test1.txt"), 1)

    assert Day6.part1(str) == 5934
    assert Day6.part2(str) == 26984457539
  end
end
