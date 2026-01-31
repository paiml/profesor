# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Profesor is a WASM-native learning management system (LMS) built on the PAIML Sovereign AI Stack. It provides Coursera-like functionality (courses, quizzes, labs, simulations) compiled to WebAssembly with zero JavaScript dependencies.

**Status**: Specification phase - no code implemented yet. See `docs/profesor-spec.md` for the full design.

## CRITICAL: Pure WASM Policy (STOP THE LINE)

### ABSOLUTE PROHIBITION: NO JAVASCRIPT

**FORBIDDEN** (will cause immediate rejection):
- ❌ Any `.js` or `.ts` files
- ❌ `npm`, `yarn`, `pnpm`, or Node.js tooling
- ❌ JavaScript glue code or wasm-bindgen JS output
- ❌ Any npm package dependencies
- ❌ Webpack, Vite, Rollup, or JS bundlers

**REQUIRED**:
- ✅ Pure Rust → `wasm32-unknown-unknown` compilation
- ✅ Raw WASM module instantiation (no JS bridge)
- ✅ `ruchy serve` for local development

### Component Source Priority (MANDATORY)

Before adding ANY external dependency, check existing PAIML components:

1. **FIRST**: `../batuta/src/` - wasm.rs, types.rs, config.rs, tui/, content/
2. **SECOND**: `../wos/` - Extreme TDD, proptest patterns, mutation testing
3. **THIRD**: PAIML stack crates (trueno, aprender, presentar)
4. **LAST**: External crates (must be WASM-compatible, no JS)

### Local Development Server

**MANDATORY**: Use `ruchy serve` (adapted from wos)

```bash
cargo install ruchy
ruchy serve dist/ --port 8000 --watch --watch-wasm --verbose
```

**FORBIDDEN**: python http.server, npx http-server, any JS-based server

## Build Commands

```bash
# Build for WASM
make build-wasm

# Run tests
make test

# Lint (both native and WASM targets)
make lint

# Format
make fmt

# Full CI pipeline
make ci
```

## PMAT Compliance

This project enforces PMAT quality standards. Run quality gates before committing.

### Quality Standards

| Metric | Threshold | Rationale |
|--------|-----------|-----------|
| Test Coverage | >= 95% | Per global CLAUDE.md requirement |
| Mutation Score | >= 80% | ICST 2024 best practices |
| Cyclomatic Complexity | <= 15 | Keep code teachable |
| TDG Grade | >= A- | Technical Debt Grade |
| unwrap() calls | 0 | Zero tolerance (Cloudflare lesson) |
| WASM Bundle Size | < 2 MB | Per spec Section 10 |

### PMAT Commands

```bash
# Run quality gates
make pmat-quality

# Run TDG analysis
make pmat-tdg

# Run defect analysis (Known Defects v2.1)
make pmat-defects

# Run Rust project score
make pmat-rust-score

# Validate documentation accuracy
make pmat-validate-docs

# Run certeza validation (per global CLAUDE.md)
make certeza
```

### Configuration Files

| File | Purpose |
|------|---------|
| `pmat.toml` | Main PMAT configuration |
| `.pmat-gates.toml` | Quality gate thresholds |
| `.pmat-metrics.toml` | O(1) metric thresholds |
| `pmat-quality.toml` | Detailed quality settings |

### Pre-Commit Workflow

```bash
# Quick pre-commit check
make pre-commit

# Full pre-release validation
make pre-release
```

## Architecture

### PAIML Stack Dependencies

| Crate | Purpose |
|-------|---------|
| `trueno` | SIMD-accelerated scoring, state vectors |
| `aprender` | Adaptive learning, clustering (no rayon for WASM) |
| `presentar` | UI widgets, layout, events |
| `presentar-core` | Core types, traits |
| `presentar-widgets` | Quiz UI components |
| `presentar-yaml` | Course manifest parsing |
| `alimentar` | Data serialization |

**Not WASM-compatible as runtime dependencies**:
- `batuta` - **Use as component source** (copy/adapt from `../batuta/src/`)
- `repartir`, `entrenar`, `certeza` - Do not use

### Planned Crate Structure

```
profesor/
├── crates/
│   ├── profesor-core/    # Course, Quiz, Lab, Progress types
│   ├── profesor-quiz/    # Quiz engine, grading, adaptive difficulty
│   ├── profesor-lab/     # WASM sandbox execution, test runner
│   ├── profesor-sim/     # State machine and physics simulations
│   ├── profesor-ui/      # Presentar widgets for quiz/lab/progress
│   └── profesor/         # Main WASM entry point
└── courses/              # Course content in YAML format
```

### Key Design Principles (Toyota Way)

| Principle | Application |
|-----------|-------------|
| **Jidoka** | Auto-grading with immediate feedback; halt on errors like NaN |
| **Poka-Yoke** | Structured formats prevent submission errors |
| **Genchi Genbutsu** | Live code execution, not screenshots |
| **Heijunka** | Modules should be ~2-4 hours for consistent pacing |
| **Mieruka** | Visual management via progress dashboards and color-coded test results |
| **Kaizen** | Spaced repetition improves retention continuously |
| **Muda** | Performance targets eliminate wait waste |

### Performance Targets

| Metric | Target |
|--------|--------|
| WASM Bundle Size | < 2 MB |
| Quiz Render | < 16ms (60fps) |
| Code Execution | < 5s |
| Memory Usage | < 64 MB |
| Time to Interactive | < 3s |

## Course Content Format

Courses are defined in YAML under `courses/`:
- `course.yaml` - Course metadata and module structure
- `module-XX/lesson-XX.yaml` - Lesson content
- `module-XX/quiz-XX.yaml` - Quiz definitions with question types
- `module-XX/lab-XX.yaml` - Lab instructions and test suites

Supported question types: MultipleChoice, MultipleSelect, CodeCompletion, Ordering, Matching, FreeformCode

Supported lab languages: Rust, Python, JavaScript, TypeScript, SQL, Markdown

## Extreme TDD (Adapted from wos)

No production code without failing test first:

1. **RED**: Write failing test (unit + property + mutation)
2. **GREEN**: Minimum implementation to pass
3. **REFACTOR**: Optimize while maintaining all test passes
4. **VERIFY**: Run `make quality` + PMAT gates
5. **COMMIT**: Atomic commit with `[PROF-XXX]` ticket prefix

### Testing Requirements

| Test Type | Requirement |
|-----------|-------------|
| Unit Tests | 95%+ coverage |
| Property Tests | 10K inputs (proptest) |
| Mutation Tests | 90%+ kill rate |
| Fuzz Tests | All parsers/input handlers |

## Code Quality Rules

### Zero Tolerance

- **No `.unwrap()` calls** - Use `.expect()` with descriptive message or `?` operator
- **No SATD comments** - No TODO, FIXME, HACK, XXX in code
- **No dead code** - Remove unused functions and modules
- **No JavaScript** - Pure WASM only

### Complexity Limits

- Maximum cyclomatic complexity: 15
- Maximum cognitive complexity: 15
- Maximum nesting depth: 4
- Maximum function lines: 80
- Maximum file lines: 500

### Documentation Requirements

- All public API items must have `///` documentation
- Include runnable examples in rustdoc
- Document unsafe code with safety comments


## Stack Documentation Search

Query this component's documentation and the entire Sovereign AI Stack using batuta's RAG Oracle:

```bash
# Index all stack documentation (run once, persists to ~/.cache/batuta/rag/)
batuta oracle --rag-index

# Search across the entire stack
batuta oracle --rag "your question here"

# Examples
batuta oracle --rag "SIMD matrix multiplication"
batuta oracle --rag "how to train a model"
batuta oracle --rag "tokenization for BERT"

# Check index status
batuta oracle --rag-stats
```

The RAG index includes CLAUDE.md, README.md, and source files from all stack components plus Python ground truth corpora for cross-language pattern matching.
