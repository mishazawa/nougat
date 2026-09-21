.PHONY: run ngrok lab post-mastodon

run:
	cargo run --bin server

post-mastodon:
	cargo run --bin post-mastodon

ngrok:
	ngrok http 5000

lab:
	uv run jupyter lab --notebook-dir=.
