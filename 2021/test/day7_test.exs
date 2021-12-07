defmodule Day7Test do
  use ExUnit.Case
  doctest Aoc2021

  test "basic cases for normal fuel cost" do
    assert Day7.fuelCost([1],1) == 0
    assert Day7.fuelCost([1],0) == 1
    assert Day7.fuelCost([1,3],2) == 2
  end

  test "basic cases for advanced fuel cost" do
    assert Day7.fuelCostAlternate([1],1) == 0
    assert Day7.fuelCostAlternate([1],0) == 1
    assert Day7.fuelCostAlternate([1,3],2) == 2
    assert Day7.fuelCostAlternate([1,3],0) == 7
  end

  test "given case, both part answers should be correct" do
    str = elem(File.read("lib/day7/test1.txt"), 1)

    assert Day7.part1(str) == 37
    assert Day7.part2(str) == 168
  end
end
