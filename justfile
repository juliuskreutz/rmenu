build:
	cargo build --release

install:
	cp -f target/release/rmenu /usr/local/bin/
	cp -f target/release/rmenu_path /usr/local/bin/
	cp -f rmenu_run /usr/local/bin/

uninstall:
	rm -f /usr/local/bin/rmenu /usr/local/bin/rmenu_run

clean:
    cargo clean
