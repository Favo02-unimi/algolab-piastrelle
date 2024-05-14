#![allow(non_snake_case)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::fmt;

// piastrella rappresentata da x e y
type Piastrella = (i32, i32);

// colore di una piastrella: colore e intensità
type Colore = (String, u32);

// requisito di una regola: un coefficiente da 0 a 8 e un colore
type Requisito = (u8, String);

// una regola: dei requisiti e un colore "finale"
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
        for (coefficiente, colore) in &self.requisiti {
            write!(f, "{} {}", coefficiente, colore)?;
        }
        Ok(())
    }
}

impl Piano {

    fn colora(&mut self, x: i32, y: i32, colore: String) -> () {
        self.piastrelle.insert((x,y), (colore, 1));
    }

    fn spegni(&mut self, x: i32, y: i32) -> () {
        self.piastrelle.remove(&(x,y));
    }

    fn regola(&mut self, regola: String) -> () {
        let parti: Vec<&str> = regola.split(" ").collect();

        let mut requisiti: Vec<Requisito> = Vec::new();

        for i in (1..parti.len()).step_by(2) {
            requisiti.push((parti[i].parse().unwrap(), String::from(parti[i+1])))
        }

        self.regole.push(Regola {
            requisiti,
            colore: String::from(parti[0]),
            utilizzo: 0,
        })
    }

    fn stato(&self, x: i32, y: i32) -> Colore {
        match self.piastrelle.get(&(x,y)) {
            Some(colore) => {
                println!("{} {}", colore.0, colore.1);
                colore.clone()
            },
            None => (String::from("spenta"), 0)
        }
    }

    fn stampa(&self) -> () {
        println!("(");
        for regola in &self.regole {
            println!("{}", regola);
        }
        println!(")");
    }

    fn bloccoGenerico(&self, x: i32, y: i32, omogeneo: bool) -> u32 {
        if !self.piastrelle.contains_key(&(x, y)) {
            println!("blocco (omog: {}) per {} {}: {}", omogeneo, x, y, 0);
            return 0
        }

        let mut coda = VecDeque::from([(x, y)]);
        let mut visitati = HashSet::from([(x, y)]);
        let colore = &self.piastrelle.get(&(x, y)).unwrap().0;

        let mut totale = 0;

        while !coda.is_empty() {
            let (cx, cy) = coda.pop_front().unwrap();
            totale += self.piastrelle.get(&(cx, cy)).unwrap().1;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if visitati.contains(&(cx+dx, cy+dy)) {
                        continue;
                    }

                    match self.piastrelle.get(&(cx+dx, cy+dy)) {
                        Some((curColore, _)) => {
                            if omogeneo && !curColore.eq(colore) {
                                continue;
                            }

                            visitati.insert((cx+dx, cy+dy));
                            coda.push_back((cx+dx, cy+dy));
                        },
                        None => (),
                    }
                }
            }
        }

        println!("blocco (omog: {}) per {} {}: {}", omogeneo, x, y, totale);
        totale
    }

    fn blocco(&self, x: i32, y: i32) -> u32 {
        self.bloccoGenerico(x, y, false)
    }

    fn bloccoOmogeneo(&self, x: i32, y: i32) -> u32 {
        self.bloccoGenerico(x, y, true)
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
        let parti: Vec<&str> = line.split(" ").collect();

        match parti[0] {
            "C" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                let colore: String = String::from(parti[3]);
                piano.colora(x, y, colore);
            },
            "S" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                piano.spegni(x, y);
            },
            "r" => {
                piano.regola(parti[1..].join(" "))
            },
            "?" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                piano.stato(x, y);
            },
            "s" => {
                piano.stampa();
            },
            "b" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                piano.blocco(x, y);
            },
            "B" => {
                let x: i32 = parti[1].parse().unwrap();
                let y: i32 = parti[2].parse().unwrap();
                piano.bloccoOmogeneo(x, y);
            },
            "p" => println!("TODO propaga"),
            "P" => println!("TODO propaga blocco"),
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
