## Progetto "Piastrelle" di AlgoLab

Progetto _"Piastrelle"_ del corso di Algoritmi e Strutture Dati... **in rust** 🦀.

### Build and run

- debug run:
  - `cargo run < inputs/input.txt` _(slow!)_
- optimized build:
  - `cargo build --release`
  - `./target/release/piastrelle < inputs/input.txt`
- _test:_
  - _`cargo test` (work in progress)_

### Runtimes

| Funzioni target | Input | Input lines | Output lines | Runtime |
| --- | --- | --- | --- | --- |
| _tutte_ | [small.in](inputs/small.in) | $39$ | $37$ | $\approx 0.00s$ |
| `blocco`, `bloccoOmog` | [blocco.in](inputs/blocco.in) | $502000$ | $2000$ | $\approx 0.90s$ |
| `propaga`, `propagaBlocco`, `ordina` | [propaga.in](inputs/propaga.in) | $506001$ | $1005002$ | $\approx 9.00s$ |

- _Funzioni target_: funzioni sul quale l'input è incentrato (ne vengono chiamate anche altre per setuppare l'ambiente)
- _Input_: file di input utilizzato
- _Input lines_: numero di righe del file di input
- _Output lines_: numero di righe del file di output
- _Runtime_: tempo di esecuzione approssimato (user time, optimized build + i5-1135G7)
