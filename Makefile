all:
	cargo build --release

install:
	install -v -d -m 755 /usr/share/rvpkg
	install -v -d -m 755 /var/lib/rvpkg
	install -v -m 644 target/release /usr/share/rvpkg
	install -v -m 644 fs/etc/rvpkg.toml /etc/rvpkg.toml
	install -v -m 644 fs/usr/share/rvpkg/packages.db /usr/share/rvpkg/packages.db
	ln -s /usr/share/rvpkg/rvpkg /usr/bin/rvpkg
	touch /var/lib/rvpkg/packages.log
	chmod 644 /var/lib/rvpkg/packages.log
