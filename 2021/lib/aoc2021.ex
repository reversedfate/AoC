defmodule Aoc2021 do
  # use Application

  # def start(_type, _args) do
  #   IO.puts "Advent of Code 2021 in Elixir!"
  #   choice = IO.gets("Choose which day to run: ")

  #   cond do
  #     choice == "1\n" -> IO.inspect(day1())
  #     true -> IO.puts "That day is not yet implemented or you are doing something absurd. Better luck next time..."
  #   end
  # end

  def day1 do
    str = elem(File.read("lib/day1/input.txt"), 1)
    {Day1.part1(str), Day1.part2(str)}
  end

  def day2 do
    str = elem(File.read("lib/day2/input.txt"), 1)
    {Day2.part1(str), Day2.part2(str)}
  end

  def day3 do
    str = elem(File.read("lib/day3/input.txt"), 1)
    {Day3.part1(str), Day3.part2(str)}
  end

  def day4 do
    str = elem(File.read("lib/day4/input.txt"), 1)
    {Day4.part1(str), Day4.part2(str)}
  end

  def day5 do
    str = elem(File.read("lib/day5/input.txt"), 1)
    {Day5.part1(str), Day5.part2(str)}
  end

  def day6 do
    str = elem(File.read("lib/day6/input.txt"), 1)
    {Day6.part1(str), Day6.part2(str)}
  end

  def day7 do
    str = elem(File.read("lib/day7/input.txt"), 1)
    {Day7.part1(str), Day7.part2(str)}
  end

  def day8 do
    str = elem(File.read("lib/day8/input.txt"), 1)
    {Day8.part1(str), Day8.part2(str)}
  end

  def day9 do
    str = elem(File.read("lib/day9/input.txt"), 1)
    {Day9.part1(str), Day9.part2(str)}
  end
end
