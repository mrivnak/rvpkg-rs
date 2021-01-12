install:
	install -v -d -m 755 /usr/share/rvpkg
	install -v -d -m 755 /var/lib/rvpkg
	install -v -m 644 fs/etc/rvpkg.yaml /etc/rvpkg.yaml
	install -v -m 644 fs/usr/share/rvpkg/packages.yaml /usr/share/rvpkg/packages.yaml
	install -v -m 644 src/rvpkg.py /usr/share/rvpkg/rvpkg.py
	install -v -m 644 src/package.py /usr/share/rvpkg/package.py
	install -v -m 700 fs/usr/bin/rvpkg /usr/bin/rvpkg
	touch /var/lib/rvpkg/packages.log
	chmod 644 /var/lib/rvpkg/packages.log
