pub const AUFFANGSTREITWERT: f64 = 5000.0;

pub fn default_streitwert(t: u32, p: u32) -> f64 { // Thema des Verfahrens, Anzahl Personen
    match t {
        0 ..= 2 => 5000.0 + ((p as f64 - 1.0) * 1000.0),
        3 => 2500.0 + ((p as f64 - 1.0) * 500.0),
        4 ..= 6 => AUFFANGSTREITWERT * p as f64,
        7 => AUFFANGSTREITWERT / 2.0 * p as f64, // Duldung
        8 => AUFFANGSTREITWERT * 2.0 * p as f64, // Einbürgerung
        _ => AUFFANGSTREITWERT
    }
}

pub fn rvg13_geb(streitwert: f64) -> f64 {
    let mut tmp_wert = 500.0;
    let mut rvg13_geb = 51.5;
    while streitwert > tmp_wert {
        if tmp_wert < 2000.0 {
            rvg13_geb += 41.5;
            tmp_wert += 500.0;
        } else if tmp_wert < 10000.0 {
            rvg13_geb += 59.5;
            tmp_wert += 1000.0;
        } else if tmp_wert < 25000.0 {
            rvg13_geb += 55.0;
            tmp_wert += 3000.0;
        } else if tmp_wert < 50000.0 {
            rvg13_geb += 86.0;
            tmp_wert += 5000.0;
        } else if tmp_wert <= 200000.0 {
            rvg13_geb += 99.5;
            tmp_wert += 15000.0;
        } else if tmp_wert < 500000.0 {
            rvg13_geb += 140.0;
            tmp_wert += 30000.0;
        } else {
            rvg13_geb += 175.0;
            tmp_wert += 50000.0;
        }
    }
    rvg13_geb
}

pub fn rvg49_geb(streitwert: f64) -> f64 {
    if streitwert <= 4000.0 {
        rvg13_geb(streitwert)
    } else if streitwert <= 5000.0 {
        319.0
    } else if streitwert <= 6000.0 {
        330.0
    } else if streitwert <= 7000.0 {
        341.0
    } else if streitwert <= 8000.0 {
        352.0
    } else if streitwert <= 9000.0 {
        363.0
    } else if streitwert <= 10000.0 {
        374.0
    } else if streitwert <= 13000.0 {
        389.0
    } else if streitwert <= 16000.0 {
        404.0
    } else if streitwert <= 19000.0 {
        419.0
    } else if streitwert <= 22000.0 {
        434.0
    } else if streitwert <= 25000.0 {
        449.0
    } else if streitwert <= 30000.0 {
        488.0
    } else if streitwert <= 35000.0 {
        527.0
    } else if streitwert <= 40000.0 {
        566.0
    } else if streitwert <= 45000.0 {
        605.0
    } else if streitwert <= 50000.0 {
        644.0
    } else if streitwert <= 65000.0 {
        692.0
    } else if streitwert <= 80.000 {
        739.0
    } else {
        786.0
    }
}

pub fn gkg_geb(thema: u32, streitwert: f64) -> f64 {
    if thema <= 3 { // asylrechtliches Thema, also gerichtskostenfrei
        0.0
    } else {
        let mut tmp_wert = 500.0;
        let mut gkg_geb = 40.0;
        while streitwert > tmp_wert {
            if tmp_wert < 2000.0 {
                gkg_geb += 21.0;
                tmp_wert += 500.0;
            } else if tmp_wert < 10000.0 {
                gkg_geb += 22.5;
                tmp_wert += 1000.0;
            } else if tmp_wert < 25000.0 {
                gkg_geb += 30.5;
                tmp_wert += 3000.0;
            } else if tmp_wert < 50000.0 {
                gkg_geb += 40.5;
                tmp_wert += 5000.0;
            } else if tmp_wert <= 200000.0 {
                gkg_geb += 14.0;
                tmp_wert += 15000.0;
            } else if tmp_wert < 500000.0 {
                gkg_geb += 210.0;
                tmp_wert += 30000.0;
            } else {
                gkg_geb += 210.0;
                tmp_wert += 50000.0;
            }
        }
        gkg_geb
    }
}

pub fn pauschale(gebuehren: f64) -> f64 {
    if gebuehren * 0.2 > 20.0 {
        20.0
    } else {
        gebuehren * 0.2
    }
}

pub fn umsatzsteuer(steuersatz: u32, nettobetrag: f64) -> f64 {
    nettobetrag / 100.0 * steuersatz as f64
}

pub fn umsatzsteuer_brutto(steuersatz: u32, nettobetrag: f64) -> f64 {
    nettobetrag + umsatzsteuer(steuersatz, nettobetrag)
}