defmodule Markov do
  use Rustler, otp_app: :markov, crate: :markov

  def new, do: error()
  def of_order(_order), do: error()
  def empty?(_markov), do: error()
  def feed(_markov, _tokens), do: error()
  def feed_str(_markov, _string), do: error()
  def feed_file(_markov, _path), do: error()
  def generate(_markov), do: error()
  def generate_str(_markov), do: error()
  def generate_from_token(_markov, _token), do: error()
  def save(_markov, _path), do: error()
  def load(_path), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
