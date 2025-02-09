/// Test per le funzioni `stato`, `colora`, `spegni`
mod stato_colora_spegni {
    #[cfg(test)]
    use crate::*;

    #[test]
    fn test_colora() {
        let mut piano = Piano::new();

        piano.colora(1, 1, String::from("rosso"), 3);

        let res = piano.stato(1, 1);
        assert!(res.is_some());
        assert_eq!(
            res.unwrap(),
            Colorazione {
                colore: String::from("rosso"),
                intensita: 3
            }
        );

        assert!(piano.stato(1, 2).is_none());
    }

    #[test]
    #[should_panic(expected = "intensità non valida")]
    fn test_colora_invalido() {
        let mut piano = Piano::new();

        piano.colora(1, 1, String::from("rosso"), 0);
    }

    #[test]
    fn test_ricolora() {
        let mut piano = Piano::new();

        piano.colora(1, 1, String::from("rosso"), 5);
        piano.colora(1, 1, String::from("verde"), 2);

        let res = piano.stato(1, 1);
        assert!(res.is_some());
        assert_eq!(
            res.unwrap(),
            Colorazione {
                colore: String::from("verde"),
                intensita: 2
            }
        );
    }

    #[test]
    fn test_spento_poi_colora() {
        let mut piano = Piano::new();

        assert!(piano.stato(1, 1).is_none());

        piano.colora(1, 1, String::from("rosso"), 10);

        let res = piano.stato(1, 1);
        assert!(res.is_some());
        assert_eq!(
            res.unwrap(),
            Colorazione {
                colore: String::from("rosso"),
                intensita: 10
            }
        );

        assert!(piano.stato(1, 2).is_none());
    }

    #[test]
    fn test_colora_negativo() {
        let mut piano = Piano::new();

        piano.colora(-1, -1, String::from("rosso"), 8);

        let res = piano.stato(-1, -1);
        assert!(res.is_some());
        assert_eq!(
            res.unwrap(),
            Colorazione {
                colore: String::from("rosso"),
                intensita: 8
            }
        );

        assert!(piano.stato(1, 1).is_none());
    }

    #[test]
    fn test_spegni() {
        let mut piano = Piano::new();

        piano.colora(-1, -1, String::from("rosso"), 5);
        piano.spegni(-1, -1);

        assert!(piano.stato(-1, -1).is_none());
    }
}

/// Test per le funzioni `regola`, `stampa`
mod regola_stampa {
    #[cfg(test)]
    use crate::*;

    #[test]
    fn test_regola() {
        let mut piano = Piano::new();

        piano.regola(String::from("rosso 1 verde 2 rosso 3 fucsia"));

        assert_eq!(
            piano.regole[0],
            Regola {
                requisiti: vec![
                    Requisito {
                        coefficiente: 1,
                        colore: String::from("verde")
                    },
                    Requisito {
                        coefficiente: 2,
                        colore: String::from("rosso")
                    },
                    Requisito {
                        coefficiente: 3,
                        colore: String::from("fucsia")
                    },
                ],
                colore: String::from("rosso"),
                utilizzo: 0
            }
        );

        piano.regola(String::from("verde 6 giallo"));

        assert_eq!(
            piano.regole[1],
            Regola {
                requisiti: vec![Requisito {
                    coefficiente: 6,
                    colore: String::from("giallo")
                },],
                colore: String::from("verde"),
                utilizzo: 0
            }
        );
    }

    #[test]
    fn test_stampa() {
        let mut piano = Piano::new();

        piano.regola(String::from("rosso 1 verde 2 rosso 3 fucsia"));
        assert_eq!(
            piano.stampa(),
            String::from("(\nrosso: 1 verde 2 rosso 3 fucsia\n)")
        );

        piano.regola(String::from("verde 8 blu"));
        assert_eq!(
            piano.stampa(),
            String::from("(\nrosso: 1 verde 2 rosso 3 fucsia\nverde: 8 blu\n)")
        );
    }

    #[test]
    #[should_panic(expected = "regola invalida (mancanza coefficienti)")]
    fn test_regola_invalida1() {
        let mut piano = Piano::new();
        piano.regola(String::from(""));
    }

    #[test]
    #[should_panic(expected = "regola invalida (mancanza coefficienti)")]
    fn test_regola_invalida2() {
        let mut piano = Piano::new();
        piano.regola(String::from("rosso"));
    }

    #[test]
    #[should_panic(expected = "regola invalida (mancanza coefficienti)")]
    fn test_regola_invalida3() {
        let mut piano = Piano::new();
        piano.regola(String::from("rosso 1"));
    }

    #[test]
    #[should_panic(expected = "regola invalida (coppie malformate)")]
    fn test_regola_invalida4() {
        let mut piano = Piano::new();
        piano.regola(String::from("rosso 1 verde 3 blu 2"));
    }

    #[test]
    #[should_panic(expected = "regola invalida (coefficiente invalido)")]
    fn test_regola_invalida6() {
        let mut piano = Piano::new();
        piano.regola(String::from("rosso uno verde 3 blu"));
    }
}

/// Test per le funzioni `_blocco_generico`, `blocco`, `blocco_omogeneo`
mod blocco {
    #[cfg(test)]
    use crate::*;

    #[test]
    fn test_blocco() {
        let mut piano = Piano::new();
        piano.colora(0, 0, String::from("r"), 2);
        piano.colora(1, 0, String::from("r"), 4);
        piano.colora(1, 1, String::from("r"), 5);

        let expected = (
            11,
            HashSet::from([
                Piastrella { x: 0, y: 0 },
                Piastrella { x: 1, y: 0 },
                Piastrella { x: 1, y: 1 },
            ]),
        );
        assert_eq!(&piano._blocco_generico(0, 0, false), &expected);
        assert_eq!(piano.blocco(0, 0), 11);

        piano.colora(0, 0, String::from("a"), 2);
        piano.colora(1, 0, String::from("b"), 4);
        piano.colora(1, 1, String::from("c"), 5);

        assert_eq!(&piano._blocco_generico(0, 0, false), &expected);
        assert_eq!(piano.blocco(0, 0), 11);
    }

    #[test]
    fn test_blocco_omogeneo() {
        let mut piano = Piano::new();
        piano.colora(0, 0, String::from("r"), 2);
        piano.colora(1, 0, String::from("r"), 4);
        piano.colora(1, 1, String::from("r"), 5);

        let expected = (
            11,
            HashSet::from([
                Piastrella { x: 0, y: 0 },
                Piastrella { x: 1, y: 0 },
                Piastrella { x: 1, y: 1 },
            ]),
        );
        assert_eq!(&piano._blocco_generico(0, 0, true), &expected);
        assert_eq!(piano.blocco_omogeneo(0, 0), 11);

        piano.colora(0, 0, String::from("a"), 2);
        piano.colora(1, 0, String::from("b"), 4);
        piano.colora(1, 1, String::from("c"), 5);

        let expected = (2, HashSet::from([Piastrella { x: 0, y: 0 }]));
        assert_eq!(&piano._blocco_generico(0, 0, true), &expected);
        assert_eq!(piano.blocco_omogeneo(0, 0), 2);
    }

    #[test]
    fn test_piastrelle_stesso_blocco() {
        let mut piano = Piano::new();
        piano.colora(0, 0, String::from("r"), 2);
        piano.colora(1, 0, String::from("r"), 3);
        piano.colora(1, 1, String::from("r"), 5);

        assert_eq!(
            piano._blocco_generico(0, 0, true),
            piano._blocco_generico(0, 0, false)
        );
        assert_eq!(
            piano._blocco_generico(0, 0, true),
            piano._blocco_generico(1, 1, true)
        );
        assert_eq!(
            piano._blocco_generico(0, 0, false),
            piano._blocco_generico(1, 1, false)
        );
        assert_eq!(
            piano._blocco_generico(0, 0, true),
            piano._blocco_generico(1, 1, false)
        );

        piano.colora(0, 0, String::from("r"), 2);
        piano.colora(1, 0, String::from("b"), 3);
        piano.colora(1, 1, String::from("r"), 5);

        assert_ne!(
            piano._blocco_generico(0, 0, true),
            piano._blocco_generico(0, 0, false)
        );
    }

    #[test]
    fn test_piastrelle_blocco_diverso() {
        let mut piano = Piano::new();
        piano.colora(0, 0, String::from("r"), 2);
        piano.colora(1, 0, String::from("r"), 3);
        piano.colora(2, 2, String::from("r"), 5);

        assert_ne!(
            piano._blocco_generico(0, 0, true),
            piano._blocco_generico(2, 2, true)
        );
        assert_ne!(
            piano._blocco_generico(0, 0, false),
            piano._blocco_generico(2, 2, false)
        );
    }

    #[test]
    fn test_blocco_spento() {
        let mut piano = Piano::new();
        piano.colora(-1, -1, String::from("r"), 2);
        piano.colora(1, 1, String::from("r"), 5);

        assert_eq!(piano._blocco_generico(0, 0, true), (0, HashSet::new()));
        assert_eq!(piano._blocco_generico(0, 0, false), (0, HashSet::new()));

        piano.colora(0, 0, String::from("r"), 1);
        assert_ne!(piano._blocco_generico(0, 0, true), (0, HashSet::new()));
        assert_ne!(piano._blocco_generico(0, 0, false), (0, HashSet::new()));
    }
}

/// Test per le funzioni `_simula_propagazione`, `propaga`, `propaga_blocco`
mod propaga {
    #[cfg(test)]
    use crate::*;

    #[test]
    fn test_propaga() {
        let mut piano = Piano::new();
        piano.colora(0, 0, String::from("g"), 1);
        piano.colora(0, 2, String::from("b"), 2);
        piano.colora(1, 1, String::from("g"), 3);
        piano.colora(1, 3, String::from("r"), 4);
        piano.colora(1, 4, String::from("b"), 5);
        piano.colora(2, 0, String::from("b"), 6);
        piano.colora(2, 2, String::from("b"), 7);
        piano.colora(3, 0, String::from("b"), 8);
        piano.colora(3, 1, String::from("r"), 9);
        piano.colora(3, 2, String::from("b"), 10);
        piano.colora(3, 4, String::from("r"), 11);
        piano.colora(4, 4, String::from("r"), 12);

        piano.regola(String::from("z 2 g 1 b"));
        piano.regola(String::from("w 1 g 2 b"));
        piano.regola(String::from("y 1 b 1 r"));
        piano.regola(String::from("g 2 b 1 r"));
        piano.regola(String::from("t 1 b 1 g 1 r"));

        piano.propaga(1, 1);
        assert!(piano
            .regole
            .iter()
            .map(|Regola { utilizzo, .. }| *utilizzo)
            .eq(vec![0, 1, 0, 0, 0]));
        assert_eq!(
            piano.stato(1, 1),
            Some(Colorazione {
                colore: String::from("w"),
                intensita: 3
            })
        );

        piano.propaga(3, 3);
        assert!(piano
            .regole
            .iter()
            .map(|Regola { utilizzo, .. }| *utilizzo)
            .eq(vec![0, 1, 1, 0, 0]));
        assert_eq!(
            piano.stato(3, 3),
            Some(Colorazione {
                colore: String::from("y"),
                intensita: 1
            })
        );
    }

    #[test]
    fn test_propaga_blocco() {
        let mut piano = Piano::new();
        piano.colora(0, 0, String::from("g"), 1);
        piano.colora(0, 2, String::from("b"), 2);
        piano.colora(1, 1, String::from("g"), 3);
        piano.colora(1, 3, String::from("r"), 4);
        piano.colora(1, 4, String::from("b"), 5);
        piano.colora(2, 0, String::from("b"), 6);
        piano.colora(2, 2, String::from("b"), 7);
        piano.colora(3, 0, String::from("b"), 8);
        piano.colora(3, 1, String::from("r"), 9);
        piano.colora(3, 2, String::from("b"), 10);
        piano.colora(3, 4, String::from("r"), 11);
        piano.colora(4, 4, String::from("r"), 12);

        piano.regola(String::from("z 2 g 1 b"));
        piano.regola(String::from("w 1 g 2 b"));
        piano.regola(String::from("y 1 b 1 r"));
        piano.regola(String::from("g 2 b 1 r"));
        piano.regola(String::from("t 1 b 1 g 1 r"));

        piano.propaga_blocco(1, 1);
        assert!(piano
            .regole
            .iter()
            .map(|Regola { utilizzo, .. }| *utilizzo)
            .eq(vec![0, 1, 4, 0, 0]));
        assert_eq!(
            piano.stato(1, 1),
            Some(Colorazione {
                colore: String::from("w"),
                intensita: 3
            })
        );
        assert_eq!(
            piano.stato(2, 0),
            Some(Colorazione {
                colore: String::from("y"),
                intensita: 6
            })
        );
        assert_eq!(
            piano.stato(2, 2),
            Some(Colorazione {
                colore: String::from("y"),
                intensita: 7
            })
        );
        assert_eq!(
            piano.stato(3, 0),
            Some(Colorazione {
                colore: String::from("y"),
                intensita: 8
            })
        );
        assert_eq!(
            piano.stato(2, 0),
            Some(Colorazione {
                colore: String::from("y"),
                intensita: 6
            })
        );
        assert_eq!(
            piano.stato(3, 2),
            Some(Colorazione {
                colore: String::from("y"),
                intensita: 10
            })
        );
    }

    #[test]
    fn test_propaga_vuoto() {
        let mut piano = Piano::new();

        piano.regola(String::from("z 2 g 1 b"));
        piano.regola(String::from("w 1 g 2 b"));
        piano.regola(String::from("y 1 b 1 r"));
        piano.regola(String::from("g 2 b 1 r"));
        piano.regola(String::from("t 1 b 1 g 1 r"));

        piano.propaga_blocco(1, 1);
        piano.propaga_blocco(0, 0);
        piano.propaga_blocco(-10, 30);
        assert!(piano
            .regole
            .iter()
            .map(|Regola { utilizzo, .. }| *utilizzo)
            .eq(vec![0, 0, 0, 0, 0]));
    }
}

/// Test per la funzione `ordina`
mod ordina {
    #[cfg(test)]
    use crate::*;

    #[test]
    fn test_ordina() {
        let mut piano = Piano::new();
        piano.colora(0, 0, String::from("x"), 1);
        piano.colora(0, 1, String::from("y"), 3);
        piano.colora(0, 2, String::from("z"), 4);

        piano.regola(String::from("x 1 x"));
        piano.regola(String::from("y 1 y"));
        piano.regola(String::from("z 1 z"));

        piano.propaga(1, 1);
        assert_eq!(
            piano.regole,
            vec![
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("x")
                    }],
                    colore: String::from("x"),
                    utilizzo: 1
                },
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("y")
                    }],
                    colore: String::from("y"),
                    utilizzo: 0
                },
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("z")
                    }],
                    colore: String::from("z"),
                    utilizzo: 0
                }
            ]
        );

        piano.ordina();
        assert_eq!(
            piano.regole,
            vec![
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("y")
                    }],
                    colore: String::from("y"),
                    utilizzo: 0
                },
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("z")
                    }],
                    colore: String::from("z"),
                    utilizzo: 0
                },
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("x")
                    }],
                    colore: String::from("x"),
                    utilizzo: 1
                },
            ]
        );

        piano.propaga(1, 1);
        assert_eq!(
            piano.regole,
            vec![
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("y")
                    }],
                    colore: String::from("y"),
                    utilizzo: 1
                },
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("z")
                    }],
                    colore: String::from("z"),
                    utilizzo: 0
                },
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("x")
                    }],
                    colore: String::from("x"),
                    utilizzo: 1
                },
            ]
        );

        piano.ordina();
        assert_eq!(
            piano.regole,
            vec![
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("z")
                    }],
                    colore: String::from("z"),
                    utilizzo: 0
                },
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("y")
                    }],
                    colore: String::from("y"),
                    utilizzo: 1
                },
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("x")
                    }],
                    colore: String::from("x"),
                    utilizzo: 1
                },
            ]
        );

        piano.propaga(1, 0);
        assert_eq!(
            piano.regole,
            vec![
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("z")
                    }],
                    colore: String::from("z"),
                    utilizzo: 0
                },
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("y")
                    }],
                    colore: String::from("y"),
                    utilizzo: 2
                },
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("x")
                    }],
                    colore: String::from("x"),
                    utilizzo: 1
                },
            ]
        );

        piano.ordina();
        assert_eq!(
            piano.regole,
            vec![
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("z")
                    }],
                    colore: String::from("z"),
                    utilizzo: 0
                },
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("x")
                    }],
                    colore: String::from("x"),
                    utilizzo: 1
                },
                Regola {
                    requisiti: vec![Requisito {
                        coefficiente: 1,
                        colore: String::from("y")
                    }],
                    colore: String::from("y"),
                    utilizzo: 2
                },
            ]
        );
    }

    #[test]
    fn test_ordina_blocco() {
        let mut piano = Piano::new();
        piano.colora(0, 0, String::from("g"), 1);
        piano.colora(0, 2, String::from("b"), 2);
        piano.colora(1, 1, String::from("g"), 3);
        piano.colora(1, 3, String::from("r"), 4);
        piano.colora(1, 4, String::from("b"), 5);
        piano.colora(2, 0, String::from("b"), 6);
        piano.colora(2, 2, String::from("b"), 7);
        piano.colora(3, 0, String::from("b"), 8);
        piano.colora(3, 1, String::from("r"), 9);
        piano.colora(3, 2, String::from("b"), 10);
        piano.colora(3, 4, String::from("r"), 11);
        piano.colora(4, 4, String::from("r"), 12);

        piano.regola(String::from("z 2 g 1 b"));
        piano.regola(String::from("w 1 g 2 b"));
        piano.regola(String::from("y 1 b 1 r"));
        piano.regola(String::from("g 2 b 1 r"));
        piano.regola(String::from("t 1 b 1 g 1 r"));

        piano.ordina();
        assert!(piano
            .regole
            .iter()
            .map(
                |Regola {
                     utilizzo, colore, ..
                 }| (*utilizzo, colore.clone())
            )
            .eq(vec![
                (0, String::from('z')),
                (0, String::from('w')),
                (0, String::from('y')),
                (0, String::from('g')),
                (0, String::from('t'))
            ]));

        piano.propaga_blocco(1, 1);
        assert!(piano
            .regole
            .iter()
            .map(
                |Regola {
                     utilizzo, colore, ..
                 }| (*utilizzo, colore.clone())
            )
            .eq(vec![
                (0, String::from('z')),
                (1, String::from('w')),
                (4, String::from('y')),
                (0, String::from('g')),
                (0, String::from('t'))
            ]));

        piano.ordina();
        assert!(piano
            .regole
            .iter()
            .map(
                |Regola {
                     utilizzo, colore, ..
                 }| (*utilizzo, colore.clone())
            )
            .eq(vec![
                (0, String::from('z')),
                (0, String::from('g')),
                (0, String::from('t')),
                (1, String::from('w')),
                (4, String::from('y')),
            ]));
    }
}
