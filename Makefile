# Profesor - WASM-Native Learning Platform
# Makefile with PMAT Compliance Integration

.PHONY: all build build-wasm test test-fast lint fmt coverage coverage-check clean \
        pmat-quality pmat-tdg pmat-defects pmat-rust-score pmat-validate-docs \
        dev release install-deps mutation help

# Default target
all: lint test build

# ============================================================================
# BUILD TARGETS
# ============================================================================

build:
	cargo build --release

build-wasm:
	cargo build --target wasm32-unknown-unknown --release

release: build-wasm
	@echo "WASM bundle built at target/wasm32-unknown-unknown/release/"

# ============================================================================
# DEVELOPMENT TARGETS
# ============================================================================

# Watch mode WASM compilation
dev-watch:
	cargo watch -x 'build --target wasm32-unknown-unknown'

# Local development server (MANDATORY: ruchy only, NO python/npx)
serve:
	ruchy serve dist/ --port 8000 --watch --watch-wasm --verbose

# Full development mode: build WASM and serve
dev: build-wasm serve

install-deps: ## Install required development tools
	rustup target add wasm32-unknown-unknown
	cargo install pmat cargo-llvm-cov cargo-mutants cargo-nextest ruchy
	@echo "NOTE: wasm-bindgen-cli NOT installed - pure WASM only, no JS glue"

# ============================================================================
# QUALITY TARGETS
# ============================================================================

lint:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo clippy --target wasm32-unknown-unknown -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

test: ## Run all tests with output
	cargo test --all-features -- --nocapture

test-fast: ## Run tests quickly (uses nextest if available)
	@echo "⚡ Running fast tests..."
	@if command -v cargo-nextest >/dev/null 2>&1; then \
		RUST_TEST_THREADS=$$(nproc) cargo nextest run \
			--workspace \
			--all-features \
			--status-level skip \
			--failure-output immediate; \
	else \
		cargo test --workspace --all-features; \
	fi

# Coverage requires 95% per global CLAUDE.md requirement
coverage: ## Generate coverage report (≥95% required)
	@echo "📊 Generating coverage report (target: ≥95%)..."
	@echo "    Note: Temporarily disabling mold linker (breaks LLVM coverage)"
	@echo "    Note: wasm.rs excluded (WASM FFI cannot be instrumented on native)"
	@# Temporarily disable mold linker (breaks LLVM coverage)
	@test -f ~/.cargo/config.toml && mv ~/.cargo/config.toml ~/.cargo/config.toml.cov-backup || true
	@cargo llvm-cov --workspace --all-features --ignore-filename-regex 'wasm\.rs$$' --lcov --output-path lcov.info
	@cargo llvm-cov report --ignore-filename-regex 'wasm\.rs$$' --html --output-dir target/coverage/html
	@# Restore mold linker
	@test -f ~/.cargo/config.toml.cov-backup && mv ~/.cargo/config.toml.cov-backup ~/.cargo/config.toml || true
	@echo "✅ Coverage report: target/coverage/html/index.html"
	@echo ""
	@echo "📊 Coverage Summary:"
	@cargo llvm-cov report --ignore-filename-regex 'wasm\.rs$$' --summary-only
	@echo ""
	@COVERAGE=$$(cargo llvm-cov report --ignore-filename-regex 'wasm\.rs$$' --summary-only 2>/dev/null | grep "TOTAL" | awk '{n=0; for(i=1;i<=NF;i++) if($$i ~ /%$$/) {n++; if(n==3) {gsub(/%/,"",$$i); print $$i; exit}}}'); \
	if [ -n "$$COVERAGE" ]; then \
		echo "Line coverage: $$COVERAGE%"; \
		THRESHOLD=95; \
		if [ $$(printf "%.0f" "$$COVERAGE") -lt $$THRESHOLD ]; then \
			echo "❌ FAIL: Coverage $$COVERAGE% below $${THRESHOLD}% threshold"; \
			exit 1; \
		else \
			echo "✅ Coverage threshold met (≥$${THRESHOLD}%)"; \
		fi; \
	fi

coverage-check: ## Enforce 95% coverage threshold (BLOCKS on failure)
	@echo "🔒 Enforcing 95% coverage threshold..."
	@echo "    Note: wasm.rs excluded (WASM FFI cannot be instrumented on native)"
	@# Temporarily disable mold linker (breaks LLVM coverage)
	@test -f ~/.cargo/config.toml && mv ~/.cargo/config.toml ~/.cargo/config.toml.cov-backup || true
	@cargo llvm-cov --workspace --all-features --ignore-filename-regex 'wasm\.rs$$' --fail-under-lines 95
	@# Restore mold linker
	@test -f ~/.cargo/config.toml.cov-backup && mv ~/.cargo/config.toml.cov-backup ~/.cargo/config.toml || true
	@echo "✅ Coverage threshold met (≥95%)"

# Mutation testing (target >= 80%)
mutation:
	cargo mutants --timeout 60

# ============================================================================
# PMAT COMPLIANCE TARGETS
# ============================================================================

# Run PMAT quality gates
pmat-quality:
	pmat quality-gate --config .pmat-gates.toml

# Run TDG analysis
pmat-tdg:
	pmat analyze tdg --format table

# Run defect analysis (Known Defects v2.1)
pmat-defects:
	pmat analyze defects --format text

# Run comprehensive Rust project score
pmat-rust-score:
	pmat rust-project-score --format text

# Run full Rust project score with all checks
pmat-rust-score-full:
	pmat rust-project-score --format text --full

# Validate documentation accuracy
pmat-validate-docs:
	pmat context --output deep_context.md --format llm-optimized
	pmat validate-readme --targets README.md CLAUDE.md --deep-context deep_context.md --fail-on-contradiction

# ============================================================================
# CERTEZA QUALITY VALIDATION (per global CLAUDE.md)
# ============================================================================

certeza:
	cd ../certeza && cargo run -- check ../profesor

# ============================================================================
# CI/CD PIPELINE
# ============================================================================

# Full CI pipeline
ci: fmt-check lint pmat-defects test coverage-check pmat-tdg

# Pre-commit validation
pre-commit: fmt-check lint test-fast pmat-defects

# Pre-release validation
pre-release: ci pmat-rust-score-full certeza

# ============================================================================
# CLEANUP
# ============================================================================

clean:
	cargo clean
	rm -f deep_context.md

# ============================================================================
# HELP
# ============================================================================

help:
	@echo "Profesor - WASM-Native Learning Platform (PURE WASM, NO JavaScript)"
	@echo ""
	@echo "Build Targets:"
	@echo "  make build        - Build release binary"
	@echo "  make build-wasm   - Build WASM target"
	@echo "  make release      - Build optimized WASM bundle"
	@echo ""
	@echo "Development (ruchy serve ONLY - NO python/npx):"
	@echo "  make dev          - Build WASM and serve with ruchy"
	@echo "  make dev-watch    - Watch mode WASM compilation"
	@echo "  make serve        - Start ruchy dev server"
	@echo ""
	@echo "Quality Targets:"
	@echo "  make lint           - Run clippy linter"
	@echo "  make fmt            - Format code"
	@echo "  make test           - Run all tests with output"
	@echo "  make test-fast      - Run tests quickly (uses nextest if available)"
	@echo "  make coverage       - Generate coverage report (≥95% required)"
	@echo "  make coverage-check - Enforce 95% threshold (blocks on failure)"
	@echo "  make mutation       - Run mutation testing (≥80% kill rate)"
	@echo ""
	@echo "PMAT Compliance:"
	@echo "  make pmat-quality      - Run quality gates"
	@echo "  make pmat-tdg          - Run TDG analysis"
	@echo "  make pmat-defects      - Run defect analysis"
	@echo "  make pmat-rust-score   - Run Rust project score"
	@echo "  make pmat-validate-docs - Validate documentation"
	@echo "  make certeza           - Run certeza validation"
	@echo ""
	@echo "CI/CD:"
	@echo "  make ci           - Full CI pipeline"
	@echo "  make pre-commit   - Pre-commit checks"
	@echo "  make pre-release  - Pre-release validation"
