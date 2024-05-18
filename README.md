## Progetto "Piastrelle" di AlgoLab

Progetto _"Piastrelle"_ del corso di Algoritmi e Strutture Dati... **in rust** 🦀.

### Build, Run and Test

- **Run**/**Build**:
  - debug run: `cargo run < yourinput.txt` _(slow!)_
  - optimized run: `cargo build --release`, `./target/release/piastrelle < yourinput.txt`

- **Test**:
  - all tests: `cargo test --release`
  - unit tests: `cargo test --release unit_tests`
  - input/output tests: `cargo test --release io_tests` (tests all inputs in `inputs/`)

- **Clean**:
  - format code: `cargo fmt`
  - check code (best practices, ...): `cargo clippy --all-targets --all-features -- -D warnings`
  - remove all old artifacts (compiled binaries, ...): `cargo clean`
