PREFIX = /usr
MANDIR = $(PREFIX)/share/man
CARGO = cargo
TARGET_DIR = target

all:
	@echo Run \'make install\' to install Neofetch.
	@echo Run \'make build\' to build in debug mode.
	@echo Run \'make release\' to build optimized release version.

# Build debug version
build:
	$(CARGO) build

# Build optimized release version
release:
	$(CARGO) build --release

# Build release with debug info (for profiling)
release-debug:
	$(CARGO) build --profile release-with-debug

# Clean build artifacts
clean:
	$(CARGO) clean

# Install from release build
install: release
	@mkdir -p $(DESTDIR)$(PREFIX)/bin
	@mkdir -p $(DESTDIR)$(MANDIR)/man1
	@cp -p $(TARGET_DIR)/release/neofetch $(DESTDIR)$(PREFIX)/bin/neofetch
	@cp -p neofetch.1 $(DESTDIR)$(MANDIR)/man1
	@chmod 755 $(DESTDIR)$(PREFIX)/bin/neofetch

# Install from debug build (for development)
install-debug: build
	@mkdir -p $(DESTDIR)$(PREFIX)/bin
	@mkdir -p $(DESTDIR)$(MANDIR)/man1
	@cp -p $(TARGET_DIR)/debug/neofetch $(DESTDIR)$(PREFIX)/bin/neofetch
	@cp -p neofetch.1 $(DESTDIR)$(MANDIR)/man1
	@chmod 755 $(DESTDIR)$(PREFIX)/bin/neofetch

uninstall:
	@rm -rf $(DESTDIR)$(PREFIX)/bin/neofetch
	@rm -rf $(DESTDIR)$(MANDIR)/man1/neofetch.1*
