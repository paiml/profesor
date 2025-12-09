# Profesor - WASM-Native Learning Platform
# Makefile with PMAT Compliance Integration

.PHONY: all build build-wasm test test-fast lint fmt coverage clean \
        pmat-quality pmat-tdg pmat-defects pmat-rust-score pmat-validate-docs \
        dev release install-deps

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

install-deps:
	rustup target add wasm32-unknown-unknown
	cargo install pmat cargo-llvm-cov cargo-mutants ruchy
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

test:
	cargo test --all-features

test-fast:
	cargo test --lib

# Coverage requires 95% per global CLAUDE.md requirement
coverage:
	cargo llvm-cov --all-features --fail-under-lines 95

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
ci: fmt-check lint pmat-defects test coverage pmat-tdg

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
	@echo "  make lint         - Run clippy linter"
	@echo "  make fmt          - Format code"
	@echo "  make test         - Run all tests"
	@echo "  make coverage     - Run coverage (95% minimum)"
	@echo "  make mutation     - Run mutation testing (90% kill rate)"
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
