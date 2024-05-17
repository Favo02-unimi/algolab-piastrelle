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
    #[should_panic(expected = "regola invalida (somma coefficienti maggiore di 8)")]
    fn test_regola_invalida5() {
        let mut piano = Piano::new();
        piano.regola(String::from("rosso 1 verde 3 blu 5 fucsia"));
    }

    #[test]
    #[should_panic(expected = "regola invalida (coefficiente invalido)")]
    fn test_regola_invalida6() {
        let mut piano = Piano::new();
        piano.regola(String::from("rosso uno verde 3 blu"));
    }
}

/// Test per le funzioni `_bloccoGenerico`, `blocco`, `bloccoOmogeneo`
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
        assert_eq!(&piano._bloccoGenerico(0, 0, false), &expected);
        assert_eq!(piano.blocco(0, 0), 11);

        piano.colora(0, 0, String::from("a"), 2);
        piano.colora(1, 0, String::from("b"), 4);
        piano.colora(1, 1, String::from("c"), 5);

        assert_eq!(&piano._bloccoGenerico(0, 0, false), &expected);
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
        assert_eq!(&piano._bloccoGenerico(0, 0, true), &expected);
        assert_eq!(piano.bloccoOmogeneo(0, 0), 11);

        piano.colora(0, 0, String::from("a"), 2);
        piano.colora(1, 0, String::from("b"), 4);
        piano.colora(1, 1, String::from("c"), 5);

        let expected = (2, HashSet::from([Piastrella { x: 0, y: 0 }]));
        assert_eq!(&piano._bloccoGenerico(0, 0, true), &expected);
        assert_eq!(piano.bloccoOmogeneo(0, 0), 2);
    }

    #[test]
    fn test_piastrelle_stesso_blocco() {
        let mut piano = Piano::new();
        piano.colora(0, 0, String::from("r"), 2);
        piano.colora(1, 0, String::from("r"), 3);
        piano.colora(1, 1, String::from("r"), 5);

        assert_eq!(
            piano._bloccoGenerico(0, 0, true),
            piano._bloccoGenerico(0, 0, false)
        );
        assert_eq!(
            piano._bloccoGenerico(0, 0, true),
            piano._bloccoGenerico(1, 1, true)
        );
        assert_eq!(
            piano._bloccoGenerico(0, 0, false),
            piano._bloccoGenerico(1, 1, false)
        );
        assert_eq!(
            piano._bloccoGenerico(0, 0, true),
            piano._bloccoGenerico(1, 1, false)
        );

        piano.colora(0, 0, String::from("r"), 2);
        piano.colora(1, 0, String::from("b"), 3);
        piano.colora(1, 1, String::from("r"), 5);

        assert_ne!(
            piano._bloccoGenerico(0, 0, true),
            piano._bloccoGenerico(0, 0, false)
        );
    }

    #[test]
    fn test_piastrelle_blocco_diverso() {
        let mut piano = Piano::new();
        piano.colora(0, 0, String::from("r"), 2);
        piano.colora(1, 0, String::from("r"), 3);
        piano.colora(2, 2, String::from("r"), 5);

        assert_ne!(
            piano._bloccoGenerico(0, 0, true),
            piano._bloccoGenerico(2, 2, true)
        );
        assert_ne!(
            piano._bloccoGenerico(0, 0, false),
            piano._bloccoGenerico(2, 2, false)
        );
    }

    #[test]
    fn test_blocco_spento() {
        let mut piano = Piano::new();
        piano.colora(-1, -1, String::from("r"), 2);
        piano.colora(1, 1, String::from("r"), 5);

        assert_eq!(piano._bloccoGenerico(0, 0, true), (0, HashSet::new()));
        assert_eq!(piano._bloccoGenerico(0, 0, false), (0, HashSet::new()));

        piano.colora(0, 0, String::from("r"), 1);
        assert_ne!(piano._bloccoGenerico(0, 0, true), (0, HashSet::new()));
        assert_ne!(piano._bloccoGenerico(0, 0, false), (0, HashSet::new()));
    }
}
