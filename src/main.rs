#![allow(non_snake_case)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt;
use std::io::{self, BufRead};

// piastrella rappresentata da x e y
#[derive(Eq, Hash, PartialEq, Clone)]
struct Piastrella {
    x: i32,
    y: i32,
}

// colore di una piastrella: colore e intensità
struct Colore {
    colore: String,
    intensita: u32,
}

// requisito di una regola: un coefficiente da 0 a 8 e un colore
struct Requisito {
    coefficiente: u8,
    colore: String,
}

// una regola: dei requisiti, un colore "finale" e il suo utilizzo
struct Regola {
    requisiti: Vec<Requisito>,
    colore: String,
    utilizzo: u32,
}

// piano, l'intero sistema:
// - delle piastrelle con relativo colore
// - delle regole con relativo utilizzo
struct Piano {
    piastrelle: HashMap<Piastrella, Colore>,
    regole: Vec<Regola>,
}

impl fmt::Display for Regola {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.colore)?;
        for Requisito {
            coefficiente,
            colore,
        } in &self.requisiti
        {
            write!(f, " {} {}", coefficiente, colore)?;
        }
        Ok(())
    }
}

impl Piano {
    fn colora(&mut self, x: i32, y: i32, colore: String) {
        self.piastrelle.insert(
            Piastrella { x, y },
            Colore {
                intensita: 1,
                colore,
            },
        );
    }

    fn spegni(&mut self, x: i32, y: i32) {
        self.piastrelle.remove(&Piastrella { x, y });
    }

    fn regola(&mut self, regola: String) {
        let parti: Vec<&str> = regola.split(' ').collect();

        let mut requisiti: Vec<Requisito> = Vec::new();

        for i in (1..parti.len()).step_by(2) {
            requisiti.push(Requisito {
                coefficiente: parti[i].parse().unwrap(),
                colore: String::from(parti[i + 1]),
            })
        }

        self.regole.push(Regola {
            requisiti,
            colore: String::from(parti[0]),
            utilizzo: 0,
        })
    }

    fn stato(&self, x: i32, y: i32) -> Colore {
        match self.piastrelle.get(&Piastrella { x, y }) {
            Some(Colore { colore, intensita }) => {
                println!("{} {}", colore, intensita);
                Colore {
                    colore: colore.clone(),
                    intensita: *intensita,
                }
            }
            None => Colore {
                colore: String::from("spenta"),
                intensita: 0,
            },
        }
    }

    fn stampa(&self) {
        println!("(");
        for regola in &self.regole {
            println!("{}", regola);
        }
        println!(")");
    }

    fn bloccoGenerico(&self, x: i32, y: i32, omogeneo: bool) -> (u32, HashSet<Piastrella>) {
        let start = Piastrella { x, y };

        if !self.piastrelle.contains_key(&start) {
            return (0, HashSet::new());
        }

        let mut coda = VecDeque::from([start.clone()]);
        let mut visitati = HashSet::from([start.clone()]);
        let Colore {
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

                    if let Some(Colore { colore, intensita }) = self.piastrelle.get(&adiacente) {
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

    fn blocco(&self, x: i32, y: i32) -> u32 {
        let (totale, ..) = self.bloccoGenerico(x, y, false);
        println!("{}", totale);
        totale
    }

    fn bloccoOmogeneo(&self, x: i32, y: i32) -> u32 {
        let (totale, ..) = self.bloccoGenerico(x, y, true);
        println!("{}", totale);
        totale
    }

    fn propagaGenerico(&self, x: i32, y: i32) -> Option<(i32, i32, usize, String)> {
        let mut intorno: HashMap<String, u8> = HashMap::new();

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                if let Some(Colore { colore, .. }) = self.piastrelle.get(&Piastrella {
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
            // println!("applica a {} {} regola {}", x, y, i);
            return Some((x, y, i, coloreTarget.clone()));
        }

        None
    }

    fn propaga(&mut self, x: i32, y: i32) {
        if let Some((x, y, i, colore)) = self.propagaGenerico(x, y) {
            self.piastrelle.insert(
                Piastrella { x, y },
                Colore {
                    colore,
                    intensita: 1,
                },
            );
            self.regole[i].utilizzo += 1;
        }
    }

    fn propagaBlocco(&mut self, x: i32, y: i32) {
        let (.., visitati) = self.bloccoGenerico(x, y, false);
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
                Colore {
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
            Some(Colore { intensita, .. }) => *intensita,
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
                Some(Colore { intensita, .. }) => totaleIntensita += intensita,
                None => return None,
            }
        }

        println!("{}", totaleIntensita);
        Some(totaleIntensita)
    }

    fn lung(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Option<u32> {
        let startDist = match self.piastrelle.get(&Piastrella { x: x1, y: y1 }) {
            Some(Colore { intensita, .. }) => intensita,
            None => return None,
        };

        if x1 == x2 && y1 == y2 {
            println!("{}", startDist);
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

                    if let Some(Colore { intensita, .. }) = self.piastrelle.get(&adiacente) {
                        if cx + dx == x2 && cy + dy == y2 {
                            println!("{}", dist + intensita);
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

fn main() {
    let mut piano = Piano {
        piastrelle: HashMap::new(),
        regole: Vec::new(),
    };

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parti: Vec<&str> = line.split(' ').collect();

        match parti[0] {
            "C" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                let colore: String = String::from(parti[3]);
                piano.colora(x, y, colore);
            }
            "S" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                piano.spegni(x, y);
            }
            "r" => {
                piano.regola(parti[1..].join(" "));
            }
            "?" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                piano.stato(x, y);
            }
            "s" => {
                piano.stampa();
            }
            "b" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                piano.blocco(x, y);
            }
            "B" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                piano.bloccoOmogeneo(x, y);
            }
            "p" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                piano.propaga(x, y);
            }
            "P" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                piano.propagaBlocco(x, y);
            }
            "o" => {
                piano.ordina();
            }
            "t" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                piano.pista(x, y, parti[3..].join(" "));
            }
            "L" => {
                let x1: i32 = parti[1].parse().unwrap();
                let y1: i32 = parti[2].parse().unwrap();
                let x2: i32 = parti[3].parse().unwrap();
                let y2: i32 = parti[4].parse().unwrap();
                piano.lung(x1, y1, x2, y2);
            }
            "i" => println!("TODO intensità"),
            "m" => println!("TODO perimetro"),
            "q" => return,
            _ => println!("che stai a fa"),
        }
    }
}
