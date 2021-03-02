all:
	cargo build --release

install:
	install -v -d -m 755 /usr/share/rvpkg
	install -v -d -m 755 /var/lib/rvpkg
	install -v -d -m 755 /usr/share/rvpkg/packages.db
	install -v -d -m 755 /var/lib/rvpkg/packages_log.db
	install -v -m 755 target/release/rvpkg /usr/share/rvpkg/rvpkg
	install -v -m 644 fs/etc/rvpkg.toml /etc/rvpkg.toml
	ln -sf /usr/share/rvpkg/rvpkg /usr/bin/rvpkg
	/usr/bin/rvpkg import --replace fs/usr/share/rvpkg/packages.csv
