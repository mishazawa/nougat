.PHONY: test-mastodon run

test-mastodon:
	curl -H 'Accept: application/activity+json' https://mastodon.social/@LemmyDev/109790106847504642 | jq

run:
	uv run nougat