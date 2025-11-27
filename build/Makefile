# MateriaTrack Makefile
# "Forging time tracking tools in Mako Energy"

CARGO := cargo
VERSION := $(shell grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
BINARY := materiatrack
TARGET_DIR := target
RELEASE_DIR := release
BUILD_DIR := build

PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
MANDIR ?= $(PREFIX)/share/man/man1
COMPLETIONS_DIR ?= $(PREFIX)/share

UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

ifeq ($(UNAME_S),Linux)
    TARGET := $(UNAME_M)-unknown-linux-gnu
endif
ifeq ($(UNAME_S),Darwin)
    TARGET := $(UNAME_M)-apple-darwin
endif
ifeq ($(UNAME_S),FreeBSD)
    TARGET := $(UNAME_M)-unknown-freebsd
endif

.PHONY: all build release install uninstall clean test check lint fmt \
        doc man completions package help

all: build

help:
	@echo "MateriaTrack v$(VERSION) Build System"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Build targets:"
	@echo "  build        Build debug binary"
	@echo "  release      Build optimized release binary"
	@echo "  all-targets  Cross-compile for all targets"
	@echo ""
	@echo "Quality targets:"
	@echo "  test         Run all tests"
	@echo "  check        Run cargo check"
	@echo "  lint         Run clippy linter"
	@echo "  fmt          Format code"
	@echo "  fmt-check    Check code formatting"
	@echo "  audit        Security audit dependencies"
	@echo ""
	@echo "Install targets:"
	@echo "  install      Install to $(PREFIX)"
	@echo "  uninstall    Remove from $(PREFIX)"
	@echo ""
	@echo "Documentation:"
	@echo "  doc          Generate Rust documentation"
	@echo "  man          Install man page"
	@echo "  completions  Generate shell completions"
	@echo ""
	@echo "Packaging:"
	@echo "  package      Create release archives"
	@echo "  clean        Remove build artifacts"
	@echo ""
	@echo "Variables:"
	@echo "  PREFIX=$(PREFIX)"
	@echo "  BINDIR=$(BINDIR)"
	@echo "  MANDIR=$(MANDIR)"

build:
	@echo "Building MateriaTrack (debug)..."
	$(CARGO) build

release:
	@echo "Building MateriaTrack (release)..."
	$(CARGO) build --release
	@strip $(TARGET_DIR)/release/$(BINARY) 2>/dev/null || true
	@echo "Binary: $(TARGET_DIR)/release/$(BINARY)"
	@ls -lh $(TARGET_DIR)/release/$(BINARY)

all-targets:
	@echo "Cross-compiling for all targets..."
	./$(BUILD_DIR)/build.sh all

test:
	@echo "Running tests..."
	$(CARGO) test --all-features

check:
	@echo "Running cargo check..."
	$(CARGO) check --all-features

lint:
	@echo "Running clippy..."
	$(CARGO) clippy --all-features -- -D warnings

fmt:
	@echo "Formatting code..."
	$(CARGO) fmt

fmt-check:
	@echo "Checking code format..."
	$(CARGO) fmt -- --check

audit:
	@echo "Running security audit..."
	$(CARGO) audit 2>/dev/null || echo "Install cargo-audit: cargo install cargo-audit"

doc:
	@echo "Generating documentation..."
	$(CARGO) doc --no-deps --open

install: release
	@echo "Installing MateriaTrack to $(PREFIX)..."
	install -d $(DESTDIR)$(BINDIR)
	install -m 755 $(TARGET_DIR)/release/$(BINARY) $(DESTDIR)$(BINDIR)/
	@ln -sf $(BINDIR)/$(BINARY) $(DESTDIR)$(BINDIR)/mtrack 2>/dev/null || true
	@echo "Installed: $(DESTDIR)$(BINDIR)/$(BINARY)"
	@if [ -f man/$(BINARY).1 ]; then \
		install -d $(DESTDIR)$(MANDIR); \
		install -m 644 man/$(BINARY).1 $(DESTDIR)$(MANDIR)/; \
		echo "Installed: $(DESTDIR)$(MANDIR)/$(BINARY).1"; \
	fi
	@echo ""
	@echo "Installation complete!"
	@echo "Run 'materiatrack --help' to get started"

uninstall:
	@echo "Uninstalling MateriaTrack..."
	rm -f $(DESTDIR)$(BINDIR)/$(BINARY)
	rm -f $(DESTDIR)$(BINDIR)/mtrack
	rm -f $(DESTDIR)$(MANDIR)/$(BINARY).1
	@echo "Uninstall complete"

man:
	@if [ -f man/$(BINARY).1 ]; then \
		install -d $(DESTDIR)$(MANDIR); \
		install -m 644 man/$(BINARY).1 $(DESTDIR)$(MANDIR)/; \
		echo "Man page installed: $(DESTDIR)$(MANDIR)/$(BINARY).1"; \
	else \
		echo "Man page not found: man/$(BINARY).1"; \
	fi

completions: release
	@echo "Generating shell completions..."
	@mkdir -p completions
	@$(TARGET_DIR)/release/$(BINARY) completions bash > completions/$(BINARY).bash 2>/dev/null || true
	@$(TARGET_DIR)/release/$(BINARY) completions zsh > completions/_$(BINARY) 2>/dev/null || true
	@$(TARGET_DIR)/release/$(BINARY) completions fish > completions/$(BINARY).fish 2>/dev/null || true
	@echo "Completions generated in completions/"

package: release
	@echo "Creating release package..."
	@mkdir -p $(RELEASE_DIR)
	@ARCHIVE="$(BINARY)-$(VERSION)-$(TARGET)"
	@mkdir -p $(RELEASE_DIR)/$$ARCHIVE
	@cp $(TARGET_DIR)/release/$(BINARY) $(RELEASE_DIR)/$$ARCHIVE/
	@cp README.md LICENSE $(RELEASE_DIR)/$$ARCHIVE/ 2>/dev/null || true
	@cp -r man $(RELEASE_DIR)/$$ARCHIVE/ 2>/dev/null || true
	@cp -r completions $(RELEASE_DIR)/$$ARCHIVE/ 2>/dev/null || true
	@cd $(RELEASE_DIR) && tar -czvf $$ARCHIVE.tar.gz $$ARCHIVE
	@rm -rf $(RELEASE_DIR)/$$ARCHIVE
	@echo "Package: $(RELEASE_DIR)/$$ARCHIVE.tar.gz"

clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean
	rm -rf $(RELEASE_DIR)
	@echo "Clean complete"

version:
	@echo "MateriaTrack v$(VERSION)"
	@echo "Target: $(TARGET)"
	@echo "Rust: $$(rustc --version)"
	@echo "Cargo: $$(cargo --version)"
