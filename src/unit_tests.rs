#[cfg(test)]
use crate::*;

/// Test per le funzioni `stato`, `colora`, `spegni`
mod stato_colora_spegni {
    use super::*;

    #[test]
    fn test_colora() {
        let mut piano = Piano::new();

        piano.colora(1, 1, String::from("rosso"));

        let res = piano.stato(1, 1);
        assert!(res.is_some());
        assert_eq!(
            res.unwrap(),
            Colorazione {
                colore: String::from("rosso"),
                intensita: 1
            }
        );

        assert!(piano.stato(1, 2).is_none());
    }

    #[test]
    fn test_ricolora() {
        let mut piano = Piano::new();

        piano.colora(1, 1, String::from("rosso"));
        piano.colora(1, 1, String::from("verde"));

        let res = piano.stato(1, 1);
        assert!(res.is_some());
        assert_eq!(
            res.unwrap(),
            Colorazione {
                colore: String::from("verde"),
                intensita: 1
            }
        );
    }

    #[test]
    fn test_spento_poi_colora() {
        let mut piano = Piano::new();

        assert!(piano.stato(1, 1).is_none());

        piano.colora(1, 1, String::from("rosso"));

        let res = piano.stato(1, 1);
        assert!(res.is_some());
        assert_eq!(
            res.unwrap(),
            Colorazione {
                colore: String::from("rosso"),
                intensita: 1
            }
        );

        assert!(piano.stato(1, 2).is_none());
    }

    #[test]
    fn test_colora_negativo() {
        let mut piano = Piano::new();

        piano.colora(-1, -1, String::from("rosso"));

        let res = piano.stato(-1, -1);
        assert!(res.is_some());
        assert_eq!(
            res.unwrap(),
            Colorazione {
                colore: String::from("rosso"),
                intensita: 1
            }
        );

        assert!(piano.stato(1, 1).is_none());
    }

    #[test]
    fn test_spegni() {
        let mut piano = Piano::new();

        piano.colora(-1, -1, String::from("rosso"));
        piano.spegni(-1, -1);

        assert!(piano.stato(-1, -1).is_none());
    }
}
