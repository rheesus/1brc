


build-debug:
	mkdir -p target
	rustc -o target/1brc-debug 1brc.rs



build-release:
	mkdir -p target
	rustc \
		-O \
		-C opt-level=s \
		-C strip=none \
		-C panic=abort \
		-C linker-plugin-lto \
		-C target-cpu=native \
		-C lto \
		-o target/1brc \
		1brc.rs



run-debug: build-debug
	time target/1brc-debug



run: build-release
	time target/1brc



prepare-challenge-repo:
	#!/usr/bin/env sh
	if [ ! -d "original_challenge_repo" ]; then
		git clone --depth 1 https://github.com/gunnarmorling/1brc.git original_challenge_repo
	fi
	sudo pacman -S jdk21-openjdk --noconfirm --needed
	if [ ! -d "original_challenge_repo/target" ]; then
		sudo archlinux-java set java-21-openjdk
		cd original_challenge_repo && ./mvnw clean verify
		sudo archlinux-java set java-25-openjdk
	fi



generate-test-data count: prepare-challenge-repo
	@echo -e "the generated file (with one billion columns) will require ~12GiB spacen\n\n"
	cd original_challenge_repo && ./create_measurements_fast.sh {{count}}
	mkdir -p data
	mv original_challenge_repo/measurements.txt data/



