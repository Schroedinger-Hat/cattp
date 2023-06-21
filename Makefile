RELEASE_NAME := cattp
EXECUTABLE := ./target/release
AUTOMATED_COMMIT := $(shell git rev-parse --short HEAD)
EMOJIS := 🐱 🐈 😺 😸 😹 😻 😼 😽 🙀 😿 😾
RANDOM_EMOJI := $(shell echo $(EMOJIS) | tr ' ' '\n' | shuf | head -n 1)

.PHONY: release

build:
	cargo build --release
	
release:
	rm -f cattp.tar.gz && \
    cd $(EXECUTABLE) && \
	rm -f cattp.tar.gz && \
	tar -czf cattp.tar.gz $(RELEASE_NAME) && \
	mv cattp.tar.gz ../.. && \
	shasum ../../cattp.tar.gz

get-new-sha:
	curl -sL https://github.com/Schrodinger-Hat/cattp/raw/main/cattp.tar.gz | shasum -a 256

push:
	git add . && \
	git commit -m "$(RANDOM_EMOJI) Automated commit-#$(AUTOMATED_COMMIT)" && \
	git push origin main

emoji:
	@echo $(RANDOM_EMOJI)
