all: build

.PHONY: build
build:
	$(shell [ "$EUID" = "0" ] && echo "build can not be run as root" && exit 1)
	@echo ":: Rebuilding in release mode..."
	@cargo build --release

.PHONY: install
install:
	@echo ":: Installing binaries..."
	@mkdir -p /usr/local/bin
	@cp -f target/release/runlog /usr/local/bin
	@chmod 755 /usr/local/bin/runlog
	@echo ":: Done"

.PHONY: uninstall
uninstall:
	@echo ":: Removing binaries..."
	@rm -f /usr/local/bin/runlog
	@echo ":: Done"
