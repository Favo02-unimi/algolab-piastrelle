#![allow(non_snake_case)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
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
        write!(f, "{} ", self.colore)?;
        for Requisito { coefficiente, colore } in &self.requisiti {
            write!(f, "{} {}", coefficiente, colore)?;
        }
        Ok(())
    }
}

impl Piano {

    fn colora(&mut self, x: i32, y: i32, colore: String) {
        self.piastrelle.insert(Piastrella { x, y }, Colore { intensita: 1, colore });
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
                colore: String::from(parti[i+1]),
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
                Colore { colore: colore.clone(), intensita: *intensita }
            }
            None => Colore { colore: String::from("spenta"), intensita: 0 },
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
            println!("0");
            return (0, HashSet::new());
        }

        let mut coda = VecDeque::from([start.clone()]);
        let mut visitati = HashSet::from([start.clone()]);
        let Colore { colore: coloreOmogeneo, intensita: mut totale } = &self.piastrelle.get(&start).unwrap();

        while !coda.is_empty() {
            let Piastrella { x: cx, y: cy } = coda.pop_front().unwrap();

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let adiacente = Piastrella{ x: cx+dx, y: cy+dy };

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

        println!("{}", totale);
        (totale, visitati)
    }

    fn blocco(&self, x: i32, y: i32) -> u32 {
        let (totale, ..) = self.bloccoGenerico(x, y, false);
        totale
    }

    fn bloccoOmogeneo(&self, x: i32, y: i32) -> u32 {
        let (totale, ..) = self.bloccoGenerico(x, y, true);
        totale
    }

    fn propagaGenerico(&self, x: i32, y: i32) -> Option<(i32, i32, usize, String)> {
        let mut intorno: HashMap<String, u8> = HashMap::new();

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                let colore = &self.piastrelle.get(&Piastrella { x, y }).unwrap().colore;
                let valore = *intorno.get(colore).unwrap_or(&0);
                intorno.insert(colore.clone(), valore);
            }
        }

        'regole: for (i, Regola { requisiti, colore: coloreTarget, .. }) in self.regole.iter().enumerate() {
            for Requisito { coefficiente, colore } in requisiti {
                if intorno.get(colore).unwrap_or(&0) < coefficiente {
                    continue 'regole; // continue outer loop, skipping return
                }
            }
            return Some((x, y, i, coloreTarget.clone()))
        }

        None
    }

    fn propaga(&mut self, x: i32, y: i32) {
        if let Some((x, y, i, colore)) = self.propagaGenerico(x, y) {
            self.piastrelle.insert(Piastrella { x, y }, Colore { colore, intensita: 1 });
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
            self.piastrelle.insert(Piastrella { x, y }, Colore { colore, intensita: 1 });
        }
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
            },
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
            "o" => println!("TODO ordina"),
            "t" => println!("TODO pista"),
            "L" => println!("TODO lung"),
            "i" => println!("TODO intensità"),
            "m" => println!("TODO perimetro"),
            "q" => return,
            _ => println!("che stai a fa"),
        }
    }
}
