RELEASE_NAME := cattp
EXECUTABLE := ./target/release

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
