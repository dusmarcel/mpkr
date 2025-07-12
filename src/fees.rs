pub const AUFFANGSTREITWERT: f64 = 5000.0;

pub fn default_streitwert(t: u32, p: u32, sk: u32) -> f64 { // Thema des Verfahrens, Anzahl Personen, Stand des Streitwertkatalogs
    match t { // Unbefristete Aufenthaltsrechte
        0 => AUFFANGSTREITWERT * p as f64, // Befristete Aufenthaltsrechte
        1 => if sk==0 {
            1.5 * AUFFANGSTREITWERT * p as f64
        } else {
            AUFFANGSTREITWERT * p as f64
        }, // Unbefristete Aufenthaltsrechte
        2 => AUFFANGSTREITWERT * p as f64, // Pass/Passersatz
        3 => 0.5 * AUFFANGSTREITWERT * p as f64, // Duldung und Abschiebung
        4 => 2.0 * AUFFANGSTREITWERT * p as f64, // Einbürgerung
        5 ..= 7 => 5000.0 + ((p as f64 - 1.0) * 1000.0), // Asylrechtliche Themen
        8 => 2500.0 + ((p as f64 - 1.0) * 500.0), // Asylrechtliche Untätigkeitsklage
        _ => AUFFANGSTREITWERT
    }
}

pub fn rvg13_geb(streitwert: f64, rg: u32) -> f64 {
    let mut rvg13_geb;
    match rg {
        0 => {
            let mut tmp_wert = 500.0;
            rvg13_geb = 51.5;
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
        },
        _ => {
            let mut tmp_wert = 500.0;
            rvg13_geb = 49.0;
            while streitwert > tmp_wert {
                if tmp_wert < 2000.0 {
                    rvg13_geb += 39.0;
                    tmp_wert += 500.0;
                } else if tmp_wert < 10000.0 {
                    rvg13_geb += 56.0;
                    tmp_wert += 1000.0;
                } else if tmp_wert < 25000.0 {
                    rvg13_geb += 52.0;
                    tmp_wert += 3000.0;
                } else if tmp_wert < 50000.0 {
                    rvg13_geb += 81.0;
                    tmp_wert += 5000.0;
                } else if tmp_wert <= 200000.0 {
                    rvg13_geb += 94.0;
                    tmp_wert += 15000.0;
                } else if tmp_wert < 500000.0 {
                    rvg13_geb += 132.0;
                    tmp_wert += 30000.0;
                } else {
                    rvg13_geb += 165.0;
                    tmp_wert += 50000.0;
                }
            }
        }
    }
    rvg13_geb
}

pub fn rvg49_geb(streitwert: f64, rg: u32) -> f64 {
    match rg {
        0 => {
            if streitwert <= 4000.0 {
                rvg13_geb(streitwert, rg)
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
        },
        _ => {
            if streitwert <= 4000.0 {
                rvg13_geb(streitwert, rg)
            } else if streitwert <= 5000.0 {
                284.0
            } else if streitwert <= 6000.0 {
                295.0
            } else if streitwert <= 7000.0 {
                306.0
            } else if streitwert <= 8000.0 {
                317.0
            } else if streitwert <= 9000.0 {
                328.0
            } else if streitwert <= 10000.0 {
                339.0
            } else if streitwert <= 13000.0 {
                354.0
            } else if streitwert <= 16000.0 {
                369.0
            } else if streitwert <= 19000.0 {
                384.0
            } else if streitwert <= 22000.0 {
                399.0
            } else if streitwert <= 25000.0 {
                414.0
            } else if streitwert <= 30000.0 {
                453.0
            } else if streitwert <= 35000.0 {
                492.0
            } else if streitwert <= 40000.0 {
                531.0
            } else if streitwert <= 45000.0 {
                570.0
            } else if streitwert <= 50000.0 {
                609.0
            } else {
                659.0
            }
        }
    }
}

pub fn gkg_geb(thema: u32, streitwert: f64, gg: u32) -> f64 {
    if thema >= 5 { // asylrechtliches Thema, also gerichtskostenfrei
        0.0
    } else {
        let mut gkg_geb;
        match gg {
            0 => {
                let mut tmp_wert = 500.0;
                gkg_geb = 40.0;
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
                        gkg_geb += 140.0;
                        tmp_wert += 15000.0;
                    } else if tmp_wert < 500000.0 {
                        gkg_geb += 210.0;
                        tmp_wert += 30000.0;
                    } else {
                        gkg_geb += 210.0;
                        tmp_wert += 50000.0;
                    }
                }
            },
            _ => {
                let mut tmp_wert = 500.0;
                gkg_geb = 38.0;
                while streitwert > tmp_wert {
                    if tmp_wert < 2000.0 {
                        gkg_geb += 20.0;
                        tmp_wert += 500.0;
                    } else if tmp_wert < 10000.0 {
                        gkg_geb += 21.0;
                        tmp_wert += 1000.0;
                    } else if tmp_wert < 25000.0 {
                        gkg_geb += 29.0;
                        tmp_wert += 3000.0;
                    } else if tmp_wert < 50000.0 {
                        gkg_geb += 38.0;
                        tmp_wert += 5000.0;
                    } else if tmp_wert <= 200000.0 {
                        gkg_geb += 132.0;
                        tmp_wert += 15000.0;
                    } else if tmp_wert < 500000.0 {
                        gkg_geb += 198.0;
                        tmp_wert += 30000.0;
                    } else {
                        gkg_geb += 198.0;
                        tmp_wert += 50000.0;
                    }
                }
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