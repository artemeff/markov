defmodule MarkovTest do
  use ExUnit.Case
  doctest Markov

  describe "#new" do
    test "creates Markov instance" do
      assert markov = Markov.new
      assert is_reference(markov)
    end
  end

  describe "#of_order" do
    test "creates Markov instance with order" do
      assert markov = Markov.of_order(42)
      assert is_reference(markov)
    end

    test "raise ArgumentError" do
      assert_raise(ArgumentError, fn ->
        Markov.of_order("invalid")
      end)
    end
  end

  describe "#empty?" do
    test "returns true for empty chain" do
      assert Markov.new |> Markov.empty?
    end

    test "returns false when there is some" do
      assert markov = Markov.new
      assert :ok == Markov.feed(markov, ["what the heck"])
      refute Markov.empty?(markov)
    end
  end

  describe "#feed" do
    test "fills chains" do
      assert markov = Markov.new
      assert :ok == Markov.feed(markov, ["what the heck"])
    end
  end

  describe "#feed_str" do
    test "fills chain" do
      assert markov = Markov.new
      assert :ok == Markov.feed_str(markov, "what the heck")
    end
  end

  describe "#feed_file" do
    test "fills chains from file" do
      assert markov = Markov.new
      assert :ok == Markov.feed_file(markov, "test/fixtures/lorem.txt")
      assert ["Temporibus" | _] = Markov.generate_from_token(markov, "Temporibus")
    end

    test "returns error when file not found" do
      assert markov = Markov.new
      assert {:error, :enoent} == Markov.feed_file(markov, "test/fixtures/not_found")
    end
  end

  describe "#generate" do
    test "generates from chain" do
      assert markov = Markov.new
      assert :ok == Markov.feed_str(markov, "what the heck")
      assert ["what", "the", "heck"] == Markov.generate(markov)
    end

    test "returns nil for empty chain" do
      assert markov = Markov.new
      assert nil == Markov.generate(markov)
    end
  end

  describe "#generate_str" do
    test "generates from chain" do
      assert markov = Markov.new
      assert :ok == Markov.feed_str(markov, "what the heck")
      assert "what the heck" == Markov.generate_str(markov)
    end

    test "returns nil for empty chain" do
      assert markov = Markov.new
      assert nil == Markov.generate_str(markov)
    end
  end

  describe "#generate_from_token" do
    test "generates from chain" do
      assert markov = Markov.new
      assert :ok == Markov.feed_str(markov, "what the heck")
      assert ["the", "heck"] == Markov.generate_from_token(markov, "the")
    end

    test "..." do
      assert markov = Markov.new
      assert :ok == Markov.feed_str(markov, "what the heck")
      assert [] == Markov.generate_from_token(markov, "undefined")
    end

    test "returns nil for empty chain" do
      assert markov = Markov.new
      assert [] == Markov.generate_from_token(markov, "what")
    end
  end

  describe "#save" do
    setup do
      path = "test/fixtures/dump"
      File.rm(path)

      {:ok, path: path}
    end

    test "saves to file", %{path: path} do
      assert markov = Markov.new
      assert :ok == Markov.feed_str(markov, "what the heck")
      assert :ok == Markov.save(markov, path)
      assert File.read!(path)
    end

    test "returns error when saving to undefined file" do
      assert markov = Markov.new
      assert :ok == Markov.feed_str(markov, "what the heck")
      assert {:error, :enoent} == Markov.save(markov, "test/fixtures/undefined_folder/undefined")
    end
  end

  describe "#load" do
    setup do
      path = "test/fixtures/dump"
      File.rm(path)

      {:ok, path: path}
    end

    test "loads from file", %{path: path} do
      assert m1 = Markov.new
      assert :ok == Markov.feed_str(m1, "what the heck")
      assert :ok == Markov.save(m1, path)

      assert m2 = Markov.load(path)
      assert ["what", "the", "heck"] == Markov.generate_from_token(m2, "what")
    end

    test "returns error when loads from undefined file" do
      assert {:error, :enoent} == Markov.load("test/fixtures/undefined")
    end
  end
end
