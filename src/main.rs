#![allow(non_snake_case)]

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
    ///     * tutti i coefficienti sono numerici
    ///     * la somma dei coefficienti supera 8
    fn regola(&mut self, regola: String) {
        let parti: Vec<&str> = regola.split(' ').collect();
        let mut sommaCoefficienti = 0;

        assert!(parti.len() > 2, "regola invalida (mancanza coefficienti)");
        assert!(parti.len() % 2 == 1, "regola invalida (coppie malformate)");

        let requisiti = parti
            .iter()
            .skip(1)
            .step_by(2)
            .zip(parti.iter().skip(2).step_by(2))
            .map(|(coefficiente, colore)| {
                sommaCoefficienti += coefficiente
                    .parse::<u8>()
                    .expect("regola invalida (coefficiente invalido)");

                Requisito {
                    coefficiente: coefficiente.parse().unwrap(),
                    colore: String::from(*colore),
                }
            })
            .collect();

        assert!(
            sommaCoefficienti <= 8,
            "regola invalida (somma coefficienti maggiore di 8)"
        );

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
    fn _bloccoGenerico(&self, x: i32, y: i32, omogeneo: bool) -> (u32, HashSet<Piastrella>) {
        let start = Piastrella { x, y };

        if !self.piastrelle.contains_key(&start) {
            return (0, HashSet::new());
        }

        let mut coda = VecDeque::from([start.clone()]);
        let mut visitati = HashSet::from([start.clone()]);
        let Colorazione {
            colore: coloreOmogeneo,
            intensita: mut totale,
        } = &self.piastrelle.get(&start).unwrap();

        while !coda.is_empty() {
            let Piastrella { x: cx, y: cy } = coda.pop_front().unwrap();

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let adiacente = Piastrella {
                        x: cx + dx,
                        y: cy + dy,
                    };

                    if visitati.contains(&adiacente) {
                        continue;
                    }

                    if let Some(Colorazione { colore, intensita }) = self.piastrelle.get(&adiacente)
                    {
                        if omogeneo && !colore.eq(coloreOmogeneo) {
                            continue;
                        }

                        visitati.insert(adiacente.clone());
                        coda.push_back(adiacente.clone());
                        totale += intensita;
                    }
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
        let (totale, ..) = self._bloccoGenerico(x, y, false);
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
    fn bloccoOmogeneo(&self, x: i32, y: i32) -> u32 {
        let (totale, ..) = self._bloccoGenerico(x, y, true);
        totale
    }

    fn propagaGenerico(&self, x: i32, y: i32) -> Option<(i32, i32, usize, String)> {
        let mut intorno: HashMap<String, u8> = HashMap::new();

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                if let Some(Colorazione { colore, .. }) = self.piastrelle.get(&Piastrella {
                    x: x + dx,
                    y: y + dy,
                }) {
                    let valore = *intorno.get(colore).unwrap_or(&0) + 1;
                    intorno.insert(colore.clone(), valore);
                }
            }
        }

        'regole: for (
            i,
            Regola {
                requisiti,
                colore: coloreTarget,
                ..
            },
        ) in self.regole.iter().enumerate()
        {
            for Requisito {
                coefficiente,
                colore,
            } in requisiti
            {
                if intorno.get(colore).unwrap_or(&0) < coefficiente {
                    continue 'regole; // continue outer loop, skipping return
                }
            }
            return Some((x, y, i, coloreTarget.clone()));
        }

        None
    }

    fn propaga(&mut self, x: i32, y: i32) {
        if let Some((x, y, i, colore)) = self.propagaGenerico(x, y) {
            self.piastrelle.insert(
                Piastrella { x, y },
                Colorazione {
                    colore,
                    intensita: 1,
                },
            );
            self.regole[i].utilizzo += 1;
        }
    }

    fn propagaBlocco(&mut self, x: i32, y: i32) {
        let (.., visitati) = self._bloccoGenerico(x, y, false);
        let mut applicazioni: Vec<(i32, i32, usize, String)> = Vec::new();

        for Piastrella { x, y } in visitati {
            if let Some(applicazione) = self.propagaGenerico(x, y) {
                applicazioni.push(applicazione)
            }
        }

        for (x, y, i, colore) in applicazioni {
            self.regole[i].utilizzo += 1;
            self.piastrelle.insert(
                Piastrella { x, y },
                Colorazione {
                    colore,
                    intensita: 1,
                },
            );
        }
    }

    fn ordina(&mut self) {
        self.regole.sort_by(|a, b| a.utilizzo.cmp(&b.utilizzo));
    }

    fn pista(&self, x: i32, y: i32, s: String) -> Option<u32> {
        let mut cx = x;
        let mut cy = y;

        let mut totaleIntensita: u32 = match self.piastrelle.get(&Piastrella { x, y }) {
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
                Some(Colorazione { intensita, .. }) => totaleIntensita += intensita,
                None => return None,
            }
        }

        Some(totaleIntensita)
    }

    fn lung(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Option<u32> {
        let startDist = match self.piastrelle.get(&Piastrella { x: x1, y: y1 }) {
            Some(Colorazione { intensita, .. }) => intensita,
            None => return None,
        };

        if x1 == x2 && y1 == y2 {
            return Some(*startDist);
        }

        let mut coda = BinaryHeap::from([Reverse((*startDist, x1, y1))]);
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
                logger(piano.bloccoOmogeneo(x, y).to_string());
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
                piano.propagaBlocco(x, y);
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
