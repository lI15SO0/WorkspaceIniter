
DESTDIR=/usr/local

all:
	@echo Building...
	@cargo build

install: publish
	@echo install into ~/.local/bin
	@mkdir -p ~/.local/bin
	@cp -f target/release/wsinit ~/.local/bin/
	@cp -f target/release/mkwsconfig ~/.local/bin/
	@echo done. make sure ~/.local/bin into your path.

install_global: publish
	@echo install into ${DESTDIR}/bin
	@cp -f target/release/wsinit ${DESTDIR}/bin/
	@cp -f target/release/mkwsconfig ${DESTDIR}/bin/
	@echo done.

publish: target/release/wsinit

target/release/wsinit:
	@echo Release Building...
	@cargo build --release

clean:
	@cargo clean

.PHONY: publish clean all
