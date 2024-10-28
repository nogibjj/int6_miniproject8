install:
	pip install --upgrade pip && pip install -r requirements.txt

format:
	black mylib/*.py

lint:
	pylint --disable=R,C --ignore-patterns=test_.*?py mylib/*.py

run:
	python mylib/main.py $(filter-out $@,$(MAKECMDGOALS))

test:
	python -m pytest -vv tests/test_main.py

all: install format lint test


rust_version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

rust_format:
	cargo fmt --quiet

rust_install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable 

rust_lint:
	cargo clippy --quiet

rust_test:
	cargo test --quiet

rust_run:
	cargo run --release

rust_build:
	cargo build --release

rust_all: format lint test run


%:
	@: