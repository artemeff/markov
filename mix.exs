defmodule Markov.MixProject do
  use Mix.Project

  def project do
    [
      app: :markov,
      version: "0.1.0",
      elixir: "~> 1.6",
      start_permanent: Mix.env() == :prod,
      compilers: [:rustler] ++ Mix.compilers,
      rustler_crates: rustler_crates(),
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.18.0"},
      {:benchfella, "~> 0.3.5", only: :dev},
    ]
  end

  defp rustler_crates do
    [
      markov: [path: "native", mode: rustc_mode(Mix.env)]
    ]
  end

  defp rustc_mode(:prod), do: :release
  defp rustc_mode(_), do: :debug
end
