


prepare-libc:
	#!/usr/bin/env sh
	if [ ! -f "target/libc.rlib" ]; then
		if [ ! -d "libc_repo" ]; then
			git clone --depth 1 https://github.com/rust-lang/libc.git libc_repo
		fi
		mkdir -p target
		rustc +nightly \
			-O \
			-C opt-level=s \
			-C strip=none \
			-C panic=abort \
			-C linker-plugin-lto \
			-C target-cpu=native \
			-C lto \
			--edition=2021 \
			--crate-type lib \
			--crate-name libc \
			-o target/libc.rlib \
			libc_repo/src/lib.rs
	fi



build-debug: prepare-libc
	rustc -o target/libc-debug.rlib lib_repo/src/lib.rs
	rustc +nightly \
		--extern libc=target/libc.rlib \
		-o target/1brc-debug \
		1brc.rs



build-release: prepare-libc
	rustc +nightly \
		-O \
		-C opt-level=s \
		-C strip=none \
		-C panic=abort \
		-C linker-plugin-lto \
		-C target-cpu=native \
		-C target-feature=+avx \
		-C force-frame-pointers=yes \
		-C lto \
		--extern libc=target/libc.rlib \
		-o target/1brc \
		1brc.rs



run-debug: build-debug
	time target/1brc-debug



run: build-release
	time target/1brc



prepare-challenge-repo:
	#!/usr/bin/env sh
	sudo pacman -S jdk21-openjdk --noconfirm --needed
	if [ ! -d "original_challenge_repo/target" ]; then
		if [ ! -d "original_challenge_repo" ]; then
			git clone --depth 1 https://github.com/gunnarmorling/1brc.git original_challenge_repo
		fi
		sudo archlinux-java set java-21-openjdk
		cd original_challenge_repo && ./mvnw clean verify
		sudo archlinux-java set java-25-openjdk
	fi



generate-test-data count: prepare-challenge-repo
	@echo -e "the generated file (with one billion columns) will require ~12GiB spacen\n\n"
	cd original_challenge_repo && ./create_measurements_fast.sh {{count}}
	mkdir -p data
	mv original_challenge_repo/measurements.txt data/



