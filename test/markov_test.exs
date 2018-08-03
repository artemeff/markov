defmodule MarkovTest do
  use ExUnit.Case
  doctest Markov

  test "greets the world" do
    assert Markov.hello() == :world
  end
end
