use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

/// Piastrella rappresentata da x e y
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Piastrella {
    x: i32,
    y: i32,
}

/// Colorazione di una piastrella: colore e intensità
#[derive(PartialEq, Clone, Debug)]
struct Colorazione {
    colore: String,
    intensita: u32,
}

/// Requisito di una regola: un coefficiente da 0 a 8 e un colore
#[derive(PartialEq, Debug)]
struct Requisito {
    coefficiente: u8,
    colore: String,
}

/// Regola: dei requisiti, un colore "finale" e il suo utilizzo
#[derive(PartialEq, Debug)]
struct Regola {
    requisiti: Vec<Requisito>,
    colore: String,
    utilizzo: u32,
}

/// Piano, l'intero sistema:
/// - delle piastrelle con relativo colore
/// - delle regole con relativo utilizzo
struct Piano {
    piastrelle: HashMap<Piastrella, Colorazione>,
    regole: Vec<Regola>,
}

const ADIACENTI: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// Implementazione metodi per Piano
impl Piano {
    /// Crea un nuovo piano vuoto, senza piastrelle e senza regole
    ///
    /// # Returns
    /// * un `Piano` vuoto
    fn new() -> Self {
        Self {
            piastrelle: HashMap::new(),
            regole: Vec::new(),
        }
    }

    /// Colora una piastrella indicata da `x` e `y`, impostando il suo `colore`
    /// e la sua `intensita` a 1, modificando il Piano
    ///
    /// # Arguments
    /// * `x` - ascisse della piastrella da colorare
    /// * `y` - ordinate della piastrella da colorare
    /// * `colore` - colore della piastrella
    /// * `intensita` - intensità della piastrella
    ///
    /// # Panics
    /// * se `colore` è una stringa vuota
    /// * se `intensita` è minore o uguale a 0
    fn colora(&mut self, x: i32, y: i32, colore: String, intensita: u32) {
        assert!(!colore.is_empty(), "colore non valido");
        assert!(intensita > 0, "intensità non valida");

        self.piastrelle
            .insert(Piastrella { x, y }, Colorazione { intensita, colore });
    }

    /// Spegne una piastrella indicata da `x` e `y`, modificando il Piano
    ///
    /// # Arguments
    /// * `x` - ascisse della piastrella da spegnere
    /// * `y` - ordinate della piastrella da spegnere
    fn spegni(&mut self, x: i32, y: i32) {
        self.piastrelle.remove(&Piastrella { x, y });
    }

    /// Restituisce lo stato (colorazione) di una piastrella, indicata da `x` e `y`
    ///
    /// # Arguments
    /// * `x` - ascisse della piastrella
    /// * `y` - ordinate della piastrella
    ///
    /// # Returns
    /// * `Some(Colorazione)` - se la piastrella è accesa, la sua colorazione
    /// * `None` - se la piastrella è spenta
    fn stato(&self, x: i32, y: i32) -> Option<Colorazione> {
        self.piastrelle.get(&Piastrella { x, y }).cloned()
    }

    /// Aggiunge una regola di colorazione (`Regola`) al piano, parsandola dalla
    /// stringa `regola`. La stringa deve essere nel formato `colore coeff1 col1 coeff2 col2 ...`,
    /// dove tutti i `coeff*` sono numerici e la loro somma non deve eccedere 8
    /// il piano viene modificato
    ///
    /// # Arguments
    /// * `regola` - stringa che rappresenta una regola
    ///
    /// # Panics
    /// * se la regola è malformata:
    ///     * mancano del tutto i coefficienti
    ///     * i coefficienti non sono accoppiati ad un colore
    ///     * non tutti i coefficienti sono numerici
    fn regola(&mut self, regola: String) {
        let parti: Vec<&str> = regola.split(' ').collect();

        assert!(parti.len() > 2, "regola invalida (mancanza coefficienti)");
        assert!(parti.len() % 2 == 1, "regola invalida (coppie malformate)");

        let requisiti = parti
            .iter()
            .skip(1)
            .step_by(2)
            .zip(parti.iter().skip(2).step_by(2))
            .map(|(coefficiente, colore)| Requisito {
                coefficiente: coefficiente
                    .parse()
                    .expect("regola invalida (coefficiente invalido)"),
                colore: String::from(*colore),
            })
            .collect();

        self.regole.push(Regola {
            requisiti,
            colore: String::from(parti[0]),
            utilizzo: 0,
        })
    }

    /// Restituisce le regole di propagazione (`Regole`) contenute nel piano nel formato
    /// ```format
    /// (
    /// colore: coeff1 col1 coeff2 col2 ...
    /// colore: coeff1 col1 coeff2 col2 coeff3 col3
    /// ...
    /// )
    /// ```
    ///
    /// # Returns
    /// * `String` che rappresenta le regole nel formato descritto
    fn stampa(&self) -> String {
        let mut result = String::from("(\n");
        self.regole.iter().for_each(
            |Regola {
                 requisiti, colore, ..
             }| {
                result.push_str(&format!("{colore}:"));
                requisiti.iter().for_each(
                    |Requisito {
                         coefficiente,
                         colore,
                     }| {
                        result.push_str(&format!(" {} {}", coefficiente, colore))
                    },
                );
                result.push('\n');
            },
        );
        result.push(')');
        result
    }

    /// Restituisce l'intensità totale e le piastrelle nel blocco della piastrella
    /// individuata da `x` e `y`
    ///
    /// # Arguments
    /// * `x` - ascisse della piastrella
    /// * `y` - ordinate della piastrella
    /// * `omogeneo` - se il blocco deve essere omogeneo
    ///
    /// # Returns
    /// * l'intensità totale e l'insieme delle piastrelle appartenenti al blocco
    ///     se la piastrella `x`, `y` è accesa
    /// * `0` e l'insieme vuoto se la piastrella `x`, `y` è spenta
    fn _blocco_generico(&self, x: i32, y: i32, omogeneo: bool) -> (u32, HashSet<Piastrella>) {
        let start = Piastrella { x, y };

        let Colorazione {
            colore: colore_omogeneo,
            intensita: mut totale, // inizializza totale a intensità di (x,y)
        } = match self.piastrelle.get(&start) {
            Some(colorazione) => colorazione,
            None => return (0, HashSet::new()),
        };

        let mut coda = VecDeque::from([start.clone()]);
        let mut visitati = HashSet::from([start.clone()]);

        while let Some(Piastrella { x: cx, y: cy }) = coda.pop_front() {
            for (dx, dy) in ADIACENTI {
                let adiacente = Piastrella {
                    x: cx + dx,
                    y: cy + dy,
                };

                if visitati.contains(&adiacente) {
                    continue;
                }

                if let Some(Colorazione { colore, intensita }) = self.piastrelle.get(&adiacente) {
                    if omogeneo && !colore.eq(colore_omogeneo) {
                        continue;
                    }

                    visitati.insert(adiacente.clone());
                    coda.push_back(adiacente.clone());
                    totale += intensita;
                }
            }
        }

        (totale, visitati)
    }

    /// Restituisce l'intensità totale della piastrelle nel blocco **non** omogeneo
    /// della piastrella individuata da `x` e `y`
    ///
    /// # Arguments
    /// * `x` - ascisse della piastrella
    /// * `y` - ordinate della piastrella
    ///
    /// # Returns
    /// * l'intensità totale se la piastrella `x`, `y` è accesa
    /// * `0` se la piastrella `x`, `y` è spenta
    fn blocco(&self, x: i32, y: i32) -> u32 {
        let (totale, ..) = self._blocco_generico(x, y, false);
        totale
    }

    /// Restituisce l'intensità totale della piastrelle nel blocco **omogeneo**
    /// della piastrella individuata da `x` e `y`
    ///
    /// # Arguments
    /// * `x` - ascisse della piastrella
    /// * `y` - ordinate della piastrella
    ///
    /// # Returns
    /// * l'intensità totale se la piastrella `x`, `y` è accesa
    /// * `0` se la piastrella `x`, `y` è spenta
    fn blocco_omogeneo(&self, x: i32, y: i32) -> u32 {
        let (totale, ..) = self._blocco_generico(x, y, true);
        totale
    }

    /// Controlla se esiste una regola di propagazione applicabile alla piastrella
    /// individuata da `x`, `y`, **senza** applicarla (il piano **non** è modificato)
    ///
    /// # Arguments
    /// * `x` - ascisse della piastrella da propagare
    /// * `y` - ordinate della piastrella da propagare
    ///
    /// # Returns
    /// * `Some(indice, colore)` - l'indice della prima regola applicabile e il colore finale
    /// * `None` - se nessuna regola è applicabile
    fn _simula_propagazione(&self, x: i32, y: i32) -> Option<(usize, String)> {
        let mut intorno: HashMap<String, u8> = HashMap::new();

        // "precalcola" valori disponibili nell'intorno di (x,y)
        ADIACENTI
            .iter()
            .map(|(dx, dy)| Piastrella {
                x: x + dx,
                y: y + dy,
            })
            .filter_map(|adiacente| self.piastrelle.get(&adiacente))
            .for_each(|Colorazione { colore, .. }| {
                *intorno.entry(colore.clone()).or_default() += 1
            });

        // trova prima regola applicabile
        self.regole
            .iter()
            .enumerate()
            .find(|(.., Regola { requisiti, .. })| {
                requisiti.iter().all(
                    |Requisito {
                         coefficiente,
                         colore,
                     }| intorno.get(colore).unwrap_or(&0) >= coefficiente,
                )
            })
            .map(|(i, Regola { colore, .. })| (i, colore.clone()))
    }

    /// Propaga una piastrella, applicando la *prima* regola applicabile, modifica
    /// il piano senza restituire nulla
    ///
    /// # Arguments
    /// * `x` - ascisse della piastrella da propagare
    /// * `y` - ordinate della piastrella da propagare
    fn propaga(&mut self, x: i32, y: i32) {
        // se una regola è stata trovata applicabile
        if let Some((i, colore)) = self._simula_propagazione(x, y) {
            // intensità invariata se accesa o 1
            let intensita = *self
                .piastrelle
                .get(&Piastrella { x, y })
                .map(|Colorazione { intensita, .. }| intensita)
                .unwrap_or(&1);

            self.piastrelle
                .insert(Piastrella { x, y }, Colorazione { colore, intensita });
            self.regole[i].utilizzo += 1;
        }
    }

    /// Propaga un blocco, applicando a ciascuna piastrella del blocco la *prima*
    /// regola applicabile. I cambiamenti non sono applicati fino alla *fine* di
    /// tutte le operazioni, ovvero la propagazione di una piastrella del blocco
    /// **non** può far scattare la propagazione di un'altra piastrella nello stesso blocco.
    /// Modifica il piano senza restituire nulla
    ///
    /// # Arguments
    /// * `x` - ascisse della piastrella da propagare
    /// * `y` - ordinate della piastrella da propagare
    fn propaga_blocco(&mut self, x: i32, y: i32) {
        // calcola blocco di (x,y)
        let (.., blocco) = self._blocco_generico(x, y, false);

        // trova la regola applicabile ad ogni piastrella del blocco
        let applicazioni: Vec<((i32, i32), (usize, String))> = blocco
            .into_iter()
            .map(|Piastrella { x, y }| (x, y, self._simula_propagazione(x, y)))
            .filter(|(.., regola)| regola.is_some())
            .map(|(x, y, regola)| ((x, y), regola.unwrap()))
            .collect();

        // applica le regole
        for ((x, y), (i, colore)) in applicazioni {
            let intensita = *self
                .piastrelle
                .get(&Piastrella { x, y })
                .map(|Colorazione { intensita, .. }| intensita)
                .unwrap_or(&1);

            self.piastrelle
                .insert(Piastrella { x, y }, Colorazione { colore, intensita });
            self.regole[i].utilizzo += 1;
        }
    }

    fn ordina(&mut self) {
        self.regole.sort_by(|a, b| a.utilizzo.cmp(&b.utilizzo));
    }

    fn pista(&self, x: i32, y: i32, s: String) -> Option<u32> {
        let mut cx = x;
        let mut cy = y;

        let mut totale_intensita: u32 = match self.piastrelle.get(&Piastrella { x, y }) {
            Some(Colorazione { intensita, .. }) => *intensita,
            None => return None,
        };

        for dir in s.split(' ') {
            match dir {
                "NN" => (cx += 0, cy += 1),
                "SS" => (cx += 0, cy += -1),
                "EE" => (cx += 1, cy += 0),
                "WW" => (cx += -1, cy += 0),
                "NE" => (cx += 1, cy += 1),
                "NW" => (cx += -1, cy += 1),
                "SE" => (cx += 1, cy += -1),
                "SW" => (cx += -1, cy += -1),
                _ => return None,
            };

            match self.piastrelle.get(&Piastrella { x: cx, y: cy }) {
                Some(Colorazione { intensita, .. }) => totale_intensita += intensita,
                None => return None,
            }
        }

        Some(totale_intensita)
    }

    fn lung(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Option<u32> {
        let start_dist = match self.piastrelle.get(&Piastrella { x: x1, y: y1 }) {
            Some(Colorazione { intensita, .. }) => intensita,
            None => return None,
        };

        if x1 == x2 && y1 == y2 {
            return Some(*start_dist);
        }

        let mut coda = BinaryHeap::from([Reverse((*start_dist, x1, y1))]);
        let mut visitati: HashSet<Piastrella> = HashSet::from([Piastrella { x: x1, y: y1 }]);

        while !coda.is_empty() {
            let (dist, cx, cy) = coda.pop().unwrap().0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let adiacente = Piastrella {
                        x: cx + dx,
                        y: cy + dy,
                    };

                    if visitati.contains(&adiacente) {
                        continue;
                    }

                    if let Some(Colorazione { intensita, .. }) = self.piastrelle.get(&adiacente) {
                        if cx + dx == x2 && cy + dy == y2 {
                            return Some(dist + intensita);
                        }

                        visitati.insert(adiacente.clone());
                        coda.push(Reverse((dist + intensita, cx + dx, cy + dy)));
                    }
                }
            }
        }

        None
    }
}

fn run(input: Option<String>, output: Option<String>) {
    let mut piano = Piano::new();

    let input: Box<dyn BufRead> = match input {
        Some(filename) => match File::open(filename) {
            Ok(file) => Box::new(BufReader::new(file)),
            Err(..) => panic!("errore aprendo il file"),
        },
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut output: Option<File> = match output {
        Some(filename) => match File::create(filename) {
            Ok(file) => Some(file),
            Err(..) => panic!("errore aprendo il file"),
        },
        None => None,
    };

    let mut logger = |mut s: String| {
        s.push('\n');
        match output {
            Some(ref mut file) => {
                if file.write_all(s.as_bytes()).is_err() {
                    panic!("errore scrivendo nel file")
                }
            }
            None => {
                print!("{}", s);
            }
        }
    };

    for line in input.lines() {
        let line = line.unwrap();
        let parti: Vec<&str> = line.split(' ').collect();

        match parti[0] {
            "C" => {
                assert!(parti.len() == 5, "input non valido");
                let x: i32 = parti[1].parse().expect("input non valido");
                let y: i32 = parti[2].parse().expect("input non valido");
                let colore: String = String::from(parti[3]);
                let i: u32 = parti[4].parse().expect("input non valido");
                piano.colora(x, y, colore, i);
            }
            "S" => {
                assert!(parti.len() == 3, "input non valido");
                let x: i32 = parti[1].parse().expect("input non valido");
                let y: i32 = parti[2].parse().expect("input non valido");
                piano.spegni(x, y);
            }
            "r" => {
                assert!(parti.len() > 1, "input non valido");
                piano.regola(parti[1..].join(" "));
            }
            "?" => {
                assert!(parti.len() == 3, "input non valido");
                let x: i32 = parti[1].parse().expect("input non valido");
                let y: i32 = parti[2].parse().expect("input non valido");
                if let Some(Colorazione { colore, intensita }) = piano.stato(x, y) {
                    logger(format!("{} {}", colore, intensita));
                }
            }
            "s" => {
                assert!(parti.len() == 1, "input non valido");
                logger(piano.stampa());
            }
            "b" => {
                assert!(parti.len() == 3, "input non valido");
                let x: i32 = parti[1].parse().expect("input non valido");
                let y: i32 = parti[2].parse().expect("input non valido");
                logger(piano.blocco(x, y).to_string());
            }
            "B" => {
                assert!(parti.len() == 3, "input non valido");
                let x: i32 = parti[1].parse().expect("input non valido");
                let y: i32 = parti[2].parse().expect("input non valido");
                logger(piano.blocco_omogeneo(x, y).to_string());
            }
            "p" => {
                assert!(parti.len() == 3, "input non valido");
                let x: i32 = parti[1].parse().expect("input non valido");
                let y: i32 = parti[2].parse().expect("input non valido");
                piano.propaga(x, y);
            }
            "P" => {
                assert!(parti.len() == 3, "input non valido");
                let x: i32 = parti[1].parse().expect("input non valido");
                let y: i32 = parti[2].parse().expect("input non valido");
                piano.propaga_blocco(x, y);
            }
            "o" => {
                assert!(parti.len() == 1, "input non valido");
                piano.ordina();
            }
            "t" => {
                assert!(parti.len() > 3, "input non valido");
                let x: i32 = parti[1].parse().expect("input non valido");
                let y: i32 = parti[2].parse().expect("input non valido");
                if let Some(intensita) = piano.pista(x, y, parti[3..].join(" ")) {
                    logger(intensita.to_string());
                }
            }
            "L" => {
                assert!(parti.len() == 5, "input non valido");
                let x1: i32 = parti[1].parse().expect("input non valido");
                let y1: i32 = parti[2].parse().expect("input non valido");
                let x2: i32 = parti[3].parse().expect("input non valido");
                let y2: i32 = parti[4].parse().expect("input non valido");
                if let Some(dist) = piano.lung(x1, y1, x2, y2) {
                    logger(dist.to_string());
                }
            }
            "i" => println!("TODO intensità"),
            "m" => println!("TODO perimetro"),
            "q" => return,
            _ => println!("che stai a fa"),
        }
    }
}

fn main() {
    run(None, None)
}

#[cfg(test)]
mod io_tests;
mod unit_tests;
