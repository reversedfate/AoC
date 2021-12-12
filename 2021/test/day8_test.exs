defmodule Day8Test do
  use ExUnit.Case
  doctest Aoc2021

  # mapSignalsByLength
  test "basic cases for signal length map" do
    assert Day8.mapSignalsByLength([]) == %{}
    assert Day8.mapSignalsByLength(["asd"]) == %{3 => ["asd"]}
    assert Day8.mapSignalsByLength(["asd", "aaa"]) == %{3 => ["asd", "aaa"]}
    assert Day8.mapSignalsByLength(["asdd"]) == %{4 => ["asdd"]}
    assert Day8.mapSignalsByLength(["asd", "asdf"]) == %{3 => ["asd"], 4 => ["asdf"]}
  end

  # figureOutBasicSignals
  test "basic cases for figuring out simple signals" do
    assert Day8.figureOutBasicSignals(%{}) == %{}
    assert Day8.figureOutBasicSignals(%{2 => ["12"]}) == %{"12" => 1}
    assert Day8.figureOutBasicSignals(%{4 => ["1234"]}) == %{"1234" => 4}
    assert Day8.figureOutBasicSignals(%{3 => ["123"]}) == %{"123" => 7}
    assert Day8.figureOutBasicSignals(%{7 => ["1234567"]}) == %{"1234567" => 8}

    assert Day8.figureOutBasicSignals(
      %{
        2 => ["12"],
        4 => ["1234"],
        3 => ["123"],
        7 => ["1234567"]
      }
    ) == %{
      "12" => 1,
      "1234" => 4,
      "123" => 7,
      "1234567" =>8
    }
  end

  # figureOutSignalMap
  test "basic cases for figuring out the whole signal map" do
    assert Day8.figureOutSignalMap(
      ["12", "1234", "123", "1234567"]
    ) == %{
      "12" => 1,
      "1234" => 4,
      "123" => 7,
      "1234567" => 8
    }
  end

  test "given case, both part answers should be correct" do
    str = elem(File.read("lib/day8/test1.txt"), 1)

    assert Day8.part1(str) == 26
    assert Day8.part2(str) == 61229
  end
end
