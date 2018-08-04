defmodule MarkovBench do
  use Benchfella

  setup_all do
    m = Markov.new
    Markov.feed(m, ["some", "dummy", "string"])

    {:ok, m}
  end

  bench "#feed" do
    Markov.feed(bench_context, ["some string"])
  end

  bench "#generate" do
    Markov.generate(bench_context)
  end
end
