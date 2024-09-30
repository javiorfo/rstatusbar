# Version
VERSION = 0.1.0

# Variables
CARGO := cargo
TARGET := target/release
INSTALL_DIR := /usr/local/bin
BINARY := rstatusbar

all: build

build:
	$(CARGO) build --release

install: build
	install -m 0755 $(TARGET)/$(BINARY) $(INSTALL_DIR)

uninstall:
	rm -f $(INSTALL_DIR)/$(BINARY)

clean:
	$(CARGO) clean

.PHONY: all build install uninstall clean
