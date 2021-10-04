defmodule Server.Router do

  use Plug.Router

  # Serves static files
  @static_path Path.expand("../../static/", __DIR__)
  plug Plug.Static,
    at: "/static",
    from: @static_path,
    content_types: %{"natural_bg.wasm" => "application/wasm"}

  plug :match
  plug :dispatch


  get "/" do
    serve_html(conn)
  end

  match _ do
    send_resp(conn, 404, "oops")
  end

  # TODO: handle case where the static file is not found
  defp serve_html(conn) do
    conn = put_resp_content_type(conn, "text/html")
    path = @static_path <> "/index.html"

    unless File.exists?(path) do
      send_resp(conn, 404, "HTML file does not exists")
    end

    send_file(conn, 200, path)
  end
end
