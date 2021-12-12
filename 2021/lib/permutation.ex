defmodule Permutation do
  def permutaionsWithoutRepetition(%MapSet{} = set),
    do: MapSet.to_list(set) |> permutaionsWithoutRepetition

  def permutaionsWithoutRepetition([]), do: [[]]

  def permutaionsWithoutRepetition(l) do
    for h <- l, t <- permutaionsWithoutRepetition(l -- [h]),
      do: [h|t]
  end
end
