use fees::{pauschale, umsatzsteuer_brutto};
use leptos::prelude::*;
use leptos_router::{hooks::query_signal_with_options, location::State, NavigateOptions};

mod popover;
mod fees;
mod utils;
use utils::format_euro;
mod components;
use crate::components::{
    intro::Intro,
    status::Status
};

#[component]
pub fn MPKR() -> impl IntoView {
    // Streitwerte
    let (v, set_v) = query_signal_with_options::<u32>(
        "v", 
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_verfahren = move |ev| set_v.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(0)));

    let (t_changed, set_t_changed) = signal(false);
    let (t, set_t) = query_signal_with_options::<u32>(
        "t",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_thema = move |ev| {
        set_t.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(4)));
        set_t_changed.set(true);
    };

    let (p_changed, set_p_changed) = signal(false);
    let (p, set_p) = query_signal_with_options::<u32>(
        "p",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_personen = move |ev| {
        set_p.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(1)));
        set_p_changed.set(true);
    };
    
    let (s, set_s) = query_signal_with_options::<f64>(
        "s",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_streitwert = move |ev| set_s.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(fees::AUFFANGSTREITWERT)));

    let (sv, set_sv) = query_signal_with_options::<f64>(
        "sv",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_streitwert_vorl = move |ev| set_sv.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(fees::AUFFANGSTREITWERT / 2.0)));

    Effect::new(move |_| {
        if t_changed.get() || p_changed.get() {
            set_s.set(Some(fees::default_streitwert(t.get().unwrap_or(4), p.get().unwrap_or(1))));
            set_sv.set(Some(fees::default_streitwert(t.get().unwrap_or(4), p.get().unwrap_or(1)) / 2.0));
        }
    });

    // Aussergerichtliche Vertretung
    let (a, set_a) = query_signal_with_options::<bool>(
        "a",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_aussergerichtlich = move |ev| {
        if v.get().unwrap_or(0) != 1 {
            set_a.set(Some(event_target_checked(&ev)));
        } else {
            set_a.set(Some(false));
        }
    };

    let (g, set_g) = query_signal_with_options::<bool>(
        "g",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_geschaeftsgebuehr = move |ev| set_g.set(Some(event_target_checked(&ev)));

    let (gs, set_gs) = query_signal_with_options::<f64>(
        "gs",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_gebuehrensatz = move |ev| set_gs.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(1.3))); 

    let n2300 = Memo::new( move |_| {
        gs.get().unwrap_or(1.3) * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
    });

    let (ap, set_ap) = query_signal_with_options::<bool>(
        "ap",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_a_pauschale = move |ev| set_ap.set(Some(event_target_checked(&ev)));

    let (aa, set_aa) = query_signal_with_options::<bool>(
        "aa",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_a_auslagen = move |ev| set_aa.set(Some(event_target_checked(&ev)));

    let (asa, set_asa) = query_signal_with_options::<f64>(
        "asa",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_aussergerichtlich_sonstige_auslagen = move |ev| {
        set_asa.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(0.0)));
        if event_target_value(&ev).parse::<f64>().unwrap_or(0.0) != 0.0 {
            set_aa.set(Some(true));
        } else {
            set_aa.set(Some(false));
        }
    };

    let summe_aussergerichtlich = Memo::new(move |_| {
        let mut summe = 0.0;
        if g.get().unwrap_or(true) { summe += n2300.get() };
        if ap.get().unwrap_or(true) { summe += pauschale(n2300.get()) };
        if aa.get().unwrap_or(false) { summe += asa.get().unwrap_or(0.0) };
        summe
    });

    // Hauptsacheverfahren

    // Welche Instanzen?
    let (h1, set_h1) = query_signal_with_options::<bool>(
        "h1",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h1 = move |ev| set_h1.set(Some(event_target_checked(&ev)));

    let (h2, set_h2) = query_signal_with_options::<bool>(
        "h2",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h2 = move |ev| set_h2.set(Some(event_target_checked(&ev)));

    let (h3, set_h3) = query_signal_with_options::<bool>(
        "h3",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h3 = move |ev| set_h3.set(Some(event_target_checked(&ev)));

    // 1. Instanz Hauptsachverfahren

    //RVG
    let (n3100, set_n3100) = query_signal_with_options::<bool>(
        "n3100",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3100 = move |ev| set_n3100.set(Some(event_target_checked(&ev)));

    let (n3101, set_n3101) = query_signal_with_options::<bool>(
        "n3101",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3101 = move |ev| set_n3101.set(Some(event_target_checked(&ev)));

    let verfgeb13_h1 = Memo::new( move |_| {
        if n3101.get().unwrap_or(false) {
            0.8 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            if n3100.get().unwrap_or(true) {
                1.3 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
            } else {
                0.0
            }
        }
    });

    let verfgeb49_h1 = Memo::new( move |_| {
        if n3101.get().unwrap_or(false) {
            0.8 * fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            if n3100.get().unwrap_or(true) {
                1.3 * fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
            } else {
                0.0
            }
        }
    });

    let (anr, set_anr) = query_signal_with_options::<bool>(
        "anr",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_anrechnung = move |ev| set_anr.set(Some(event_target_checked(&ev)));

    let anrechnung13 = Memo::new(move |_| {
        if anr.get().unwrap_or(a.get().unwrap_or(false)) && a.get().unwrap_or(false) && g.get().unwrap_or(true) && (n3100.get().unwrap_or(true) || n3101.get().unwrap_or(false)) {
            let mut anrechnungsbetrag = 0.5 * n2300.get();
            if anrechnungsbetrag > 0.75 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT)) {
                anrechnungsbetrag = 0.75 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT));
            }
            anrechnungsbetrag
        } else {
            0.0
        }        
    });

    let anrechnung49 = Memo::new(move |_| {
        let mut anrechnungsbetrag = anrechnung13.get();
        let differenz = verfgeb13_h1.get() - verfgeb49_h1.get();
        anrechnungsbetrag -= differenz;
        if anrechnungsbetrag <= 0.0 {
            anrechnungsbetrag = 0.0;
        }
        anrechnungsbetrag      
    });

    let (n3104, set_n3104) = query_signal_with_options::<bool>(
        "n3104",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3104 = move |ev| set_n3104.set(Some(event_target_checked(&ev)));

    let tgeb13_h1 = Memo::new( move |_| {
        if n3104.get().unwrap_or(true) {
            1.2 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            0.0
        }
    });

    let tgeb49_h1 = Memo::new( move |_| {
        if n3104.get().unwrap_or(true) {
            1.2 * fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            0.0
        }
    });

    let (h1p, set_h1p) = query_signal_with_options::<bool>(
        "h1p",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h1_pauschale = move |ev| set_h1p.set(Some(event_target_checked(&ev)));

    let pauschale13_h1 = Memo::new( move |_| {
        if h1p.get().unwrap_or(true) {
            fees::pauschale(verfgeb13_h1.get () + tgeb13_h1.get())
        } else {
            0.0
        }
    });

    let pauschale49_h1 = Memo::new( move |_| {
        if h1p.get().unwrap_or(true) {
            fees::pauschale(verfgeb49_h1.get () + tgeb49_h1.get())
        } else {
            0.0
        }
    });

    let (h1a, set_h1a) = query_signal_with_options::<bool>(
        "h1a",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h1_auslagen = move |ev| set_h1a.set(Some(event_target_checked(&ev)));

    let (h1sa, set_h1sa) = query_signal_with_options::<f64>(
        "h1sa",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h1_sonstige_auslagen = move |ev| {
        set_h1sa.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(0.0)));
        if event_target_value(&ev).parse::<f64>().unwrap_or(0.0) != 0.0 {
            set_h1a.set(Some(true));
        } else {
            set_h1a.set(Some(false));
        }
    };

    // GKG
    let (n5110, set_n5110) = query_signal_with_options::<bool>(
        "n5110",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n5110 = move |ev| set_n5110.set(Some(event_target_checked(&ev)));

    let (n5111, set_n5111) = query_signal_with_options::<bool>(
        "n5111",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n5111 = move |ev| set_n5111.set(Some(event_target_checked(&ev)));

    let gkg_h1 = Memo::new ( move |_| {
        if n5111.get().unwrap_or(false) {
            1.0 * fees::gkg_geb(t.get().unwrap_or(4), s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else if n5110.get().unwrap_or(true) {
            3.0 * fees::gkg_geb(t.get().unwrap_or(4), s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            0.0
        }
    });

    // 2. Instanz Hauptsacheverfahren

    // RVG
    let (n3200, set_n3200) = query_signal_with_options::<bool>(
        "n3200",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3200 = move |ev| set_n3200.set(Some(event_target_checked(&ev)));

    let (n3201, set_n3201) = query_signal_with_options::<bool>(
        "n3201",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3201 = move |ev| set_n3201.set(Some(event_target_checked(&ev)));

    let verfgeb13_h2 = Memo::new( move |_| {
        if n3201.get().unwrap_or(false) {
            1.1 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            if n3200.get().unwrap_or(true) {
                1.6 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
            } else {
                0.0
            }
        }
    });

    let verfgeb49_h2 = Memo::new( move |_| {
        if n3201.get().unwrap_or(false) {
            1.1 * fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            if n3200.get().unwrap_or(true) {
                1.6 * fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
            } else {
                0.0
            }
        }
    });

    let (n3202, set_n3202) = query_signal_with_options::<bool>(
        "n3202",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3202 = move |ev| set_n3202.set(Some(event_target_checked(&ev)));

    let tgeb13_h2 = Memo::new( move |_| {
        if n3202.get().unwrap_or(true) {
            1.2 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            0.0
        }
    });

    let tgeb49_h2 = Memo::new( move |_| {
        if n3202.get().unwrap_or(true) {
            1.2 * fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            0.0
        }
    });

    let (h2p, set_h2p) = query_signal_with_options::<bool>(
        "h2p",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h2_pauschale = move |ev| set_h2p.set(Some(event_target_checked(&ev)));

    let pauschale13_h2 = Memo::new( move |_| {
        if h2p.get().unwrap_or(true) {
            fees::pauschale(verfgeb13_h2.get () + tgeb13_h2.get())
        } else {
            0.0
        }
    });

    let pauschale49_h2 = Memo::new( move |_| {
        if h2p.get().unwrap_or(true) {
            fees::pauschale(verfgeb49_h2.get () + tgeb49_h2.get())
        } else {
            0.0
        }
    });

    let (h2a, set_h2a) = query_signal_with_options::<bool>(
        "h2a",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h2_auslagen = move |ev| set_h2a.set(Some(event_target_checked(&ev)));

    let (h2sa, set_h2sa) = query_signal_with_options::<f64>(
        "h2sa",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h2_sonstige_auslagen = move |ev| {
        set_h2sa.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(0.0)));
        if event_target_value(&ev).parse::<f64>().unwrap_or(0.0) != 0.0 {
            set_h2a.set(Some(true));
        } else {
            set_h2a.set(Some(false));
        }
    };

    // GKG
    let (n5122, set_n5122) = query_signal_with_options::<bool>(
        "n5122",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    let (n5120, set_n5120) = query_signal_with_options::<bool>(
        "n5120",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    let (n5121, set_n5121) = query_signal_with_options::<bool>(
        "n5121",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    let (n5123, set_n5123) = query_signal_with_options::<bool>(
        "n5123",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    let (n5124, set_n5124) = query_signal_with_options::<bool>(
        "n5124",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    let change_n5122 = move |ev| {
        let value = event_target_checked(&ev);
        set_n5122.set(Some(value));
        if value {
            set_n5120.set(Some(false));
            set_n5121.set(Some(false));
        }
    };

    let change_n5120 = move |ev| {
        let value = event_target_checked(&ev);
        set_n5120.set(Some(value));
        if value {
            set_n5122.set(Some(false));
            set_n5121.set(Some(false));
            set_n5123.set(Some(false));
            set_n5124.set(Some(false));
        }
    };

    let change_n5121 = move |ev| {
        let value = event_target_checked(&ev);
        set_n5121.set(Some(value));
        if value {
            set_n5122.set(Some(false));
            set_n5120.set(Some(false));
            set_n5123.set(Some(false));
            set_n5124.set(Some(false));
        }
    };

    let change_n5123 = move |ev| {
        let value = event_target_checked(&ev);
        set_n5123.set(Some(value));
        if value {
            set_n5122.set(Some(true));
            set_n5120.set(Some(false));
            set_n5121.set(Some(false));
            set_n5124.set(Some(false));
        }
    };

    let change_n5124 = move |ev| {
        let value = event_target_checked(&ev);
        set_n5124.set(Some(value));
        if value {
            set_n5122.set(Some(true));
            set_n5120.set(Some(false));
            set_n5121.set(Some(false));
            set_n5123.set(Some(false));
        }
    };

    let gkg_h2 = Memo::new ( move |_| {
        if n5123.get().unwrap_or(false) || n5120.get().unwrap_or(false) {
            1.0 * fees::gkg_geb(t.get().unwrap_or(4), s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else if n5121.get().unwrap_or(false) {
            0.5 * fees::gkg_geb(t.get().unwrap_or(4), s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else if n5124.get().unwrap_or(false) {
            2.0 * fees::gkg_geb(t.get().unwrap_or(4), s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else if n5122.get().unwrap_or(true) {
            4.0 * fees::gkg_geb(t.get().unwrap_or(4), s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            0.0
        }
    });

    // 3. Instanz Hauptsacheverfahren

    // RVG
    let (n3206, set_n3206) = query_signal_with_options::<bool>(
        "n3206",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3206 = move |ev| set_n3206.set(Some(event_target_checked(&ev)));

    let (n3207, set_n3207) = query_signal_with_options::<bool>(
        "n3207",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3207 = move |ev| set_n3207.set(Some(event_target_checked(&ev)));

    let verfgeb13_h3 = Memo::new( move |_| {
        if n3207.get().unwrap_or(false) {
            1.1 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            if n3206.get().unwrap_or(true) {
                1.6 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
            } else {
                0.0
            }
        }
    });

    let verfgeb49_h3 = Memo::new( move |_| {
        if n3207.get().unwrap_or(false) {
            1.1 * fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            if n3206.get().unwrap_or(true) {
                1.6 * fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
            } else {
                0.0
            }
        }
    });

    let (n3210, set_n3210) = query_signal_with_options::<bool>(
        "n3210",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3210 = move |ev| set_n3210.set(Some(event_target_checked(&ev)));

    let tgeb13_h3 = Memo::new( move |_| {
        if n3210.get().unwrap_or(true) {
            1.5 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            0.0
        }
    });

    let tgeb49_h3 = Memo::new( move |_| {
        if n3210.get().unwrap_or(true) {
            1.5 * fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            0.0
        }
    });

    let (h3p, set_h3p) = query_signal_with_options::<bool>(
        "h2p",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h3_pauschale = move |ev| set_h3p.set(Some(event_target_checked(&ev)));

    let pauschale13_h3 = Memo::new( move |_| {
        if h3p.get().unwrap_or(true) {
            fees::pauschale(verfgeb13_h3.get () + tgeb13_h3.get())
        } else {
            0.0
        }
    });

    let pauschale49_h3 = Memo::new( move |_| {
        if h3p.get().unwrap_or(true) {
            fees::pauschale(verfgeb49_h3.get () + tgeb49_h3.get())
        } else {
            0.0
        }
    });

    let (h3a, set_h3a) = query_signal_with_options::<bool>(
        "h2a",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h3_auslagen = move |ev| set_h3a.set(Some(event_target_checked(&ev)));

    let (h3sa, set_h3sa) = query_signal_with_options::<f64>(
        "h3sa",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h3_sonstige_auslagen = move |ev| {
        set_h3sa.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(0.0)));
        if event_target_value(&ev).parse::<f64>().unwrap_or(0.0) != 0.0 {
            set_h3a.set(Some(true));
        } else {
            set_h3a.set(Some(false));
        }
    };

    // GKG
    let (n5130, set_n5130) = query_signal_with_options::<bool>(
        "n5130",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    let (n5131, set_n5131) = query_signal_with_options::<bool>(
        "n5131",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    let (n5132, set_n5132) = query_signal_with_options::<bool>(
        "n5132",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    let change_n5130 = move |ev| {
        let value = event_target_checked(&ev);
        set_n5130.set(Some(value));
    };

    let change_n5131 = move |ev| {
        let value = event_target_checked(&ev);
        set_n5131.set(Some(value));
        if value {
            set_n5130.set(Some(true));
            set_n5132.set(Some(false));
        }
    };

    let change_n5132 = move |ev| {
        let value = event_target_checked(&ev);
        set_n5132.set(Some(value));
        if value {
            set_n5130.set(Some(true));
            set_n5131.set(Some(false));
        }
    };

    let gkg_h3 = Memo::new ( move |_| {
        if n5132.get().unwrap_or(false) {
            3.0 * fees::gkg_geb(t.get().unwrap_or(4), s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else if n5131.get().unwrap_or(false) {
            1.0 * fees::gkg_geb(t.get().unwrap_or(4), s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else if n5130.get().unwrap_or(true) {
            5.0 * fees::gkg_geb(t.get().unwrap_or(4), s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            0.0
        }
    });

    // Summen Hauptsacheverfahren
    let summe_rvg13_h = Memo::new(move |_| {
        let mut summe = 0.0;
        if h1.get().unwrap_or(true) {
            summe += verfgeb13_h1.get() - anrechnung13.get() + tgeb13_h1.get() + pauschale13_h1.get();
            if h1a.get().unwrap_or(false) {
                summe += h1sa.get().unwrap_or(0.0)
            }
        }
        if h2.get().unwrap_or(false) {
            summe += verfgeb13_h2.get() + tgeb13_h2.get() + pauschale13_h2.get();
            if h2a.get().unwrap_or(false) {
                summe += h2sa.get().unwrap_or(0.0)
            }
        }
        if h3.get().unwrap_or(false) {
            summe += verfgeb13_h3.get() + tgeb13_h3.get() + pauschale13_h3.get();
            if h3a.get().unwrap_or(false) {
                summe += h3sa.get().unwrap_or(0.0)
            }
        }
        summe
    });

    let summe_rvg49_h = Memo::new(move |_| {
        let mut summe = 0.0;
        if h1.get().unwrap_or(true) {   
            summe += verfgeb49_h1.get() - anrechnung49.get() + tgeb49_h1.get() + pauschale49_h1.get();
            if h1a.get().unwrap_or(false) {
                summe += h1sa.get().unwrap_or(0.0)
            }
        }
        if h2.get().unwrap_or(false) {
            summe += verfgeb49_h2.get() + tgeb49_h2.get() + pauschale49_h2.get();
            if h2a.get().unwrap_or(false) {
                summe += h2sa.get().unwrap_or(0.0)
            }
        }
        if h3.get().unwrap_or(false) {
            summe += verfgeb49_h3.get() + tgeb49_h3.get() + pauschale49_h3.get();
            if h3a.get().unwrap_or(false) {
                summe += h3sa.get().unwrap_or(0.0)
            }
        }      
        summe        
    });

    let summe_gkg_h = Memo::new(move |_| {
        let mut summe = 0.0;
        if h1.get().unwrap_or(true) { summe += gkg_h1.get(); }
        if h2.get().unwrap_or(false) { summe += gkg_h2.get(); }
        if h3.get().unwrap_or(false) { summe += gkg_h3.get(); }
        summe
    });

    // Vorläufiger Rechtsschutz

    // Welche Instanzen?
    let (v1, set_v1) = query_signal_with_options::<bool>(
        "v1",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_v1 = move |ev| set_v1.set(Some(event_target_checked(&ev)));

    let (v2, set_v2) = query_signal_with_options::<bool>(
        "v2",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_v2 = move |ev| set_v2.set(Some(event_target_checked(&ev)));

    // 1. Instanz

    // RVG
    let (n3100v, set_n3100v) = query_signal_with_options::<bool>(
        "n3100v",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3100v = move |ev| set_n3100v.set(Some(event_target_checked(&ev)));

    let (n3101v, set_n3101v) = query_signal_with_options::<bool>(
        "n3101v",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3101v = move |ev| set_n3101v.set(Some(event_target_checked(&ev)));

    let verfgeb13_v1 = Memo::new( move |_| {
        if n3101v.get().unwrap_or(false) {
            0.8 * fees::rvg13_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
        } else {
            if n3100v.get().unwrap_or(true) {
                1.3 * fees::rvg13_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
            } else {
                0.0
            }
        }
    });

    let verfgeb49_v1 = Memo::new( move |_| {
        if n3101v.get().unwrap_or(false) {
            0.8 * fees::rvg49_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
        } else {
            if n3100v.get().unwrap_or(true) {
                1.3 * fees::rvg49_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
            } else {
                0.0
            }
        }
    });

    let (n3104v, set_n3104v) = query_signal_with_options::<bool>(
        "n3104v",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3104v = move |ev| set_n3104v.set(Some(event_target_checked(&ev)));

    let tgeb13_v1 = Memo::new( move |_| {
        if n3104v.get().unwrap_or(false) {
            1.2 * fees::rvg13_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
        } else {
            0.0
        }
    });

    let tgeb49_v1 = Memo::new( move |_| {
        if n3104v.get().unwrap_or(false) {
            1.2 * fees::rvg49_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
        } else {
            0.0
        }
    });

    let (v1p, set_v1p) = query_signal_with_options::<bool>(
        "v1p",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_v1_pauschale = move |ev| set_v1p.set(Some(event_target_checked(&ev)));

    let pauschale13_v1 = Memo::new( move |_| {
        if v1p.get().unwrap_or(true) {
            fees::pauschale(verfgeb13_v1.get () + tgeb13_v1.get())
        } else {
            0.0
        }
    });

    let pauschale49_v1 = Memo::new( move |_| {
        if v1p.get().unwrap_or(true) {
            fees::pauschale(verfgeb49_v1.get () + tgeb49_v1.get())
        } else {
            0.0
        }
    });

    let (v1a, set_v1a) = query_signal_with_options::<bool>(
        "v1a",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_v1_auslagen = move |ev| set_v1a.set(Some(event_target_checked(&ev)));

    let (v1sa, set_v1sa) = query_signal_with_options::<f64>(
        "v1sa",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_v1_sonstige_auslagen = move |ev| {
        set_v1sa.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(0.0)));
        if event_target_value(&ev).parse::<f64>().unwrap_or(0.0) != 0.0 {
            set_v1a.set(Some(true));
        } else {
            set_v1a.set(Some(false));
        }
    };

    // GKG
    let (n5210, set_n5210) = query_signal_with_options::<bool>(
        "n5210",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n5210 = move |ev| set_n5210.set(Some(event_target_checked(&ev)));

    let (n5211, set_n5211) = query_signal_with_options::<bool>(
        "n5211",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n5211 = move |ev| set_n5211.set(Some(event_target_checked(&ev)));

    let gkg_v1 = Memo::new ( move |_| {
        if n5211.get().unwrap_or(false) {
            0.5 * fees::gkg_geb(t.get().unwrap_or(4), sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
        } else if n5210.get().unwrap_or(true) {
            1.5 * fees::gkg_geb(t.get().unwrap_or(4), sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
        } else {
            0.0
        }
    });

    // 2. Instanz

    // RVG
    let (n3200v, set_n3200v) = query_signal_with_options::<bool>(
        "n3200v",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3200v = move |ev| set_n3200v.set(Some(event_target_checked(&ev)));

    let (n3201v, set_n3201v) = query_signal_with_options::<bool>(
        "n3201v",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3201v = move |ev| set_n3201v.set(Some(event_target_checked(&ev)));

    let verfgeb13_v2 = Memo::new( move |_| {
        if n3201v.get().unwrap_or(false) {
            1.1 * fees::rvg13_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
        } else {
            if n3200v.get().unwrap_or(true) {
                1.6 * fees::rvg13_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
            } else {
                0.0
            }
        }
    });

    let verfgeb49_v2 = Memo::new( move |_| {
        if n3201v.get().unwrap_or(false) {
            1.1 * fees::rvg49_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
        } else {
            if n3200v.get().unwrap_or(true) {
                1.6 * fees::rvg49_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
            } else {
                0.0
            }
        }
    });

    let (n3202v, set_n3202v) = query_signal_with_options::<bool>(
        "n3202v",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3202v = move |ev| set_n3202v.set(Some(event_target_checked(&ev)));

    let tgeb13_v2 = Memo::new( move |_| {
        if n3202v.get().unwrap_or(false) {
            1.2 * fees::rvg13_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
        } else {
            0.0
        }
    });

    let tgeb49_v2 = Memo::new( move |_| {
        if n3202v.get().unwrap_or(false) {
            1.2 * fees::rvg49_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
        } else {
            0.0
        }
    });

    let (v2p, set_v2p) = query_signal_with_options::<bool>(
        "v2p",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_v2_pauschale = move |ev| set_v2p.set(Some(event_target_checked(&ev)));

    let pauschale13_v2 = Memo::new( move |_| {
        if v2p.get().unwrap_or(true) {
            fees::pauschale(verfgeb13_v2.get () + tgeb13_v2.get())
        } else {
            0.0
        }
    });

    let pauschale49_v2 = Memo::new( move |_| {
        if v2p.get().unwrap_or(true) {
            fees::pauschale(verfgeb49_v2.get () + tgeb49_v2.get())
        } else {
            0.0
        }
    });

    let (v2a, set_v2a) = query_signal_with_options::<bool>(
        "v1a",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_v2_auslagen = move |ev| set_v2a.set(Some(event_target_checked(&ev)));

    let (v2sa, set_v2sa) = query_signal_with_options::<f64>(
        "v2sa",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_v2_sonstige_auslagen = move |ev| {
        set_v2sa.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(0.0)));
        if event_target_value(&ev).parse::<f64>().unwrap_or(0.0) != 0.0 {
            set_v2a.set(Some(true));
        } else {
            set_v2a.set(Some(false));
        }
    };

    // GKG
    let (n5240, set_n5240) = query_signal_with_options::<bool>(
        "n5240",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n5240 = move |ev| set_n5240.set(Some(event_target_checked(&ev)));

    let (n5241, set_n5241) = query_signal_with_options::<bool>(
        "n5241",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n5241 = move |ev| set_n5241.set(Some(event_target_checked(&ev)));

    let gkg_v2 = Memo::new ( move |_| {
        if n5241.get().unwrap_or(false) {
            1.0 * fees::gkg_geb(t.get().unwrap_or(4), sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
        } else if n5240.get().unwrap_or(true) {
            2.0 * fees::gkg_geb(t.get().unwrap_or(4), sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
        } else {
            0.0
        }
    });

    // Summen vorläufiger Rechtsschutz
    let summe_rvg13_v = Memo::new(move |_| {
        let mut summe = 0.0;
        if v1.get().unwrap_or(true) {
            summe += verfgeb13_v1.get() + tgeb13_v1.get() + pauschale13_v1.get();
            if v1a.get().unwrap_or(false) {
                summe += v1sa.get().unwrap_or(0.0)
            }
        }
        if v2.get().unwrap_or(false) {
            summe += verfgeb13_v2.get() + tgeb13_v2.get() + pauschale13_v2.get();
            if v2a.get().unwrap_or(false) {
                summe += v2sa.get().unwrap_or(0.0)
            }
        }
        summe
    });

    let summe_rvg49_v = Memo::new(move |_| {
        let mut summe = 0.0;
        if v1.get().unwrap_or(true) {   
            summe += verfgeb49_v1.get() + tgeb49_v1.get() + pauschale49_v1.get();
            if v1a.get().unwrap_or(false) {
                summe += v1sa.get().unwrap_or(0.0)
            }
        }
        if v2.get().unwrap_or(false) {
            summe += verfgeb49_v2.get() + tgeb49_v2.get() + pauschale49_v2.get();
            if v2a.get().unwrap_or(false) {
                summe += v2sa.get().unwrap_or(0.0)
            }
        }
        summe
    });

    let summe_gkg_v = Memo::new(move |_| {
        let mut summe = 0.0;
        if v1.get().unwrap_or(true) { summe += gkg_v1.get(); }
        if v2.get().unwrap_or(false) { summe += gkg_v2.get(); }
        summe
    });

    // Summen
    let summe_rvg13_netto = Memo::new( move |_| {
        let mut summe = 0.0;
        if a.get().unwrap_or(false) { summe += summe_aussergerichtlich.get() };
        if v.get().unwrap_or(0) != 1 { summe += summe_rvg13_h.get() };
        if v.get().unwrap_or(0) != 0 { summe += summe_rvg13_v.get() };
        summe
    });

    let summe_rvg49_netto = Memo::new( move |_| {
        let mut summe = 0.0;
        if a.get().unwrap_or(false) { summe += summe_aussergerichtlich.get() };
        if v.get().unwrap_or(0) != 1 { summe += summe_rvg49_h.get() };
        if v.get().unwrap_or(0) != 0 { summe += summe_rvg49_v.get() };
        summe
    });

    let (u, set_u) = query_signal_with_options::<u32>(
        "u",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_umsatzsteuer = move |ev| {
        set_u.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(19)));
    };

    let summe_gkg = Memo::new( move |_| {
        let mut summe = 0.0;
        if v.get().unwrap_or(0) != 1 { summe += summe_gkg_h.get() };
        if v.get().unwrap_or(0) != 0 { summe += summe_gkg_v.get() };
        summe
    });

    let gesamtsumme13 = Memo::new ( move |_| {
        let mut summe = 0.0;
        summe += umsatzsteuer_brutto(u.get().unwrap_or(19), summe_rvg13_netto.get());
        summe += summe_gkg.get();
        summe
    });

    let gesamtsumme49 = Memo::new ( move |_| {
        let mut summe = 0.0;
        summe += umsatzsteuer_brutto(u.get().unwrap_or(19), summe_rvg49_netto.get());
        summe += summe_gkg.get();
        summe
    });

    view! {
        <Intro />
        <Status />
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Streitwerte"
            </h2>
            <p>
                "Grundlage für die Berechnung der Gebühren ist der sogenannte Streitwert. Wir müssen also zuerst die Streitwerte für deine Angelegenheit ermitteln. Plural, weil gerade in gerichtlichen Verfahren häufig zusätzlich zur Klage noch ein Antrag auf Gewährung vorläufigen Rechtsschutzes erforderlich ist. Manchmal wird auch nur ein Antrag auf vorläufigen Rechtsschutz gestellt. Deswegen kann man hier verschiedene Optionen wählen. Außerdem hängt die konkrete Höhe des Streitwerts auch von der Anzahl der Personen ab, die in einem Verfahren zusammengefasst werden."
            </p>
            <p>
                <label for="verfahren">"Wähle aus, ob die Gebühren nur für ein Hauptsacheverfahren, nur für dein Verfahren
                zum vorläufigen Rechtsschutz, oder für beides berechnet werden sollen."</label>
            </p>
            <p>
                <select class="p-1 border-2 border-stone-400 rounded-lg" aria-label="Auswahl der Verfahrensart" id="verfahren" on:change=change_verfahren>
                    <option value="0" selected=move || v.get().unwrap_or(0) == 0>"Nur Hauptsacheverfahren"</option>
                    <option value="1" selected=move || v.get().unwrap_or(0) == 1>"Nur Verfahren zum vorläufigen Rechtsschutz"</option>
                    <option value="2" selected=move || v.get().unwrap_or(0) == 2>"Hauptsacheverfahren und Verfahren zum vorläufigen Rechtsschutz"</option>
                </select>
            </p>
            <p>
                <label for="thema">"Wähle ein Thema, dann versucht der Rechner, dir die passenden Streitwerte vorzuschlagen.
                Du kannst aber auch manuell selbst Streitwerte angeben."</label>
            </p>
            <p>
                <select class="p-1 border-2 border-stone-400 rounded-lg" aria-label="Auswahl des Themas" id="thema" on:change=change_thema>
                    <option value="0" selected=move || t.get().unwrap_or(4) == 0>"Asylrecht: Zulässigkeit (z.B. Dublin, Drittstaatenfall, Folgeantrag)"</option>
                    <option value="1" selected=move || t.get().unwrap_or(4) == 1>"Asylrecht: Anerkennungsverfahren"</option>
                    <option value="2" selected=move || t.get().unwrap_or(4) == 2>"Asylrecht: Widerruf/Rücknahme"</option>
                    <option value="3" selected=move || t.get().unwrap_or(4) == 3>"Asylrecht: Untätigkeitsklage"</option>
                    <option value="4" selected=move || t.get().unwrap_or(4) == 4>"Aufenthaltstitel inkl. Untätigkeitsklage"</option>
                    <option value="5" selected=move || t.get().unwrap_or(4) == 5>"Ausweisung"</option>
                    <option value="6" selected=move || t.get().unwrap_or(4) == 6>"Pass/Passersatz"</option>
                    <option value="7" selected=move || t.get().unwrap_or(4) == 7>"Duldung und Abschiebung inkl. Ausbildungs-/Beschäftigungsduldung,
                        Untätigkeitsklage"</option>
                    <option value="8" selected=move || t.get().unwrap_or(4) == 8>"Einbürgerung und Feststellung der Staatsangehörigkeit"</option>
                </select>
            </p>
            <div>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th class="px-1">
                                <label for="anzahl">"Anzahl Personen"</label>
                            </th>
                            <th class="px-1">
                                "Streitwerte"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 13 RVG)"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 49 RVG / PKH)"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (GKG)"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td></td>
                            <td  class="px-1">
                                { move || match v.get().unwrap_or(0) {
                                    0 => "Hauptsache",
                                    1 => "vorläufiger Rechtsschutz",
                                    _ => "Hauptsache"
                                }}
                            </td>
                            <td></td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="px-1">
                                <input type="number" min="1" value=move || p.get().unwrap_or(1) class="border-2 border-stone-400 rounded-lg px-1" on:change=change_personen />
                                <button popovertarget="zahl-der-personen" class="border-2 border-stone-400 rounded-lg px-1 ml-1">"?"</button>
                                <div id="zahl-der-personen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Zahl der Personen"</h4>
                                    <p>{ popover::PERSONS }</p>
                                </div>
                            </td>
                            <td class="px-1">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || if v.get().unwrap_or(0) != 1 {
                                        s.get().unwrap_or(fees::default_streitwert(t.get().unwrap_or(4), p.get().unwrap_or(1)))
                                    } else {
                                        sv.get().unwrap_or(fees::default_streitwert(t.get().unwrap_or(4), p.get().unwrap_or(1)) / 2.0)
                                    }
                                    on:change=change_streitwert
                                    prop:value=move || if v.get().unwrap_or(0) != 1 {
                                        format_euro(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
                                    } else {
                                        format_euro(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
                                    }
                                />
                            </td>
                            <td class="px-1 text-right">
                                { move || if v.get().unwrap_or(0) != 1 {
                                        format_euro(fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT)))
                                    } else {
                                        format_euro(fees::rvg13_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0)))
                                    }
                                }
                            </td>
                            <td class="px-1 text-right">
                                { move || if v.get().unwrap_or(0) != 1 {
                                        format_euro(fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT)))
                                    } else {
                                        format_euro(fees::rvg49_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0)))
                                    }
                                }
                            </td>
                            <td class="px-1 text-right">
                                { move || if v.get().unwrap_or(0) != 1 {
                                        format_euro(fees::gkg_geb(t.get().unwrap_or(4), s.get().unwrap_or(fees::AUFFANGSTREITWERT)))
                                    } else {
                                        format_euro(fees::gkg_geb(t.get().unwrap_or(4), sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0)))
                                    }
                                }
                            </td>                     
                        </tr>
                        <tr class=move || if v.get().unwrap_or(0) == 2 { "visible" } else { "hidden" }>
                            <td></td>
                            <td class="px-1">
                                "vorläufiger Rechtsschutz"
                            </td>
                            <td></td>
                            <td></td>
                            <td></td> 
                        </tr>
                        <tr class=move || if v.get().unwrap_or(0) == 2 { "visible" } else { "hidden" }>
                            <td></td>
                            <td class="px-1">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || sv.get().unwrap_or(fees::default_streitwert(t.get().unwrap_or(4), p.get().unwrap_or(1)) / 2.0)
                                    on:change=change_streitwert_vorl
                                    prop:value=move || format_euro(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
                                />
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(fees::rvg13_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(fees::rvg49_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(fees::gkg_geb(t.get().unwrap_or(4), sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))) }
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>

        // Aussergerichtliche Vertretung
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Außergerichtliche Vertretung"
            </h2>
            <p>
                <input
                    type="checkbox"
                    id="aussergerichtlich"
                    on:change=change_aussergerichtlich
                    prop:checked=move || a.get().unwrap_or(false)
                />
                <label for="aussergerichtlich" class="ml-1">"Außergerichtliche Vertretung"</label>
                <button popovertarget="aussergerichtliche-vertretung" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                <div id="aussergerichtliche-vertretung" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                    <h4 class="text-xl font-medium">"Außergerichtliche Vertretung"</h4>
                    <p>{ popover::AUSSERGERICHTLICH }</p>
                </div>            
            </p>
            // Abschnitt für die Berechnung der Gebühren der außergerichtlichen Vertretung.
            // Er soll nur angezeigt werden, wenn die Box für außergerichtliche Vertretung (a)
            // und nicht nur Hauptsacheverfahren (v != 1) ausgewählt wurde
            <p class=move || if a.get().unwrap_or(false) && v.get().unwrap_or(0) != 1 { "visible" } else { "hidden" }>
                <table>
                    <tbody>
                        <tr>
                            <td class="px-1">
                                <input
                                    type="checkbox"
                                    id="geschaeftsgebuehr"
                                    on:change=change_geschaeftsgebuehr
                                    prop:checked=move || g.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">"Geschäftsgebühr, Nr. 2300 VV RVG"</td>
                            <td class="px-1">
                                Gebührensatz
                            </td>
                            <td class="px-1">
                                <input
                                    type="number"
                                    class="p-1 border-2 border-stone-400 rounded-lg"
                                    step="0.1"
                                    min="0.5"
                                    max="2.5"
                                    value="1.3"
                                    on:change=change_gebuehrensatz
                                    prop:value=move || gs.get().unwrap_or(1.3)
                                />
                                <button popovertarget="gebuehrensatz" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="gebuehrensatz" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Gebührensatz für die Geschäftsgebühr"</h4>
                                    <p>{ popover::GEBUEHRENSATZ }</p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                { move || if g.get().unwrap_or(true) { format_euro(n2300.get()) } else { "0,00".to_string() } }
                            </td>
                        </tr>
                        <tr>
                            <td class="px-1">
                                <input
                                    type="checkbox"
                                    id="a_pauschale"
                                    on:change=change_a_pauschale
                                    prop:checked=move || ap.get().unwrap_or(true)
                                />
                            </td>                            
                            <td class="px-1">"Auslagenpauschale, Nr. 7002 VV RVG"</td>
                            <td></td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || if ap.get().unwrap_or(true) { format_euro(pauschale(n2300.get())) } else { "0,00".to_string() } }
                            </td>
                        </tr>
                        <tr>
                            <td class="px-1">
                                <input
                                    type="checkbox"
                                    id="a_auslagen"
                                    on:change=change_a_auslagen
                                    prop:checked=move || aa.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <span>"Sonstige Auslagen, z. B. Nr. 7000, 7003 ff. VV RVG"</span>
                                <button popovertarget="aauslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="aauslagen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Sonstige Auslagen"</h4>
                                    <p>"Zum Beispiel:"
                                        <ul>
                                            <li>"7000 Pauschale für die Herstellung und Überlassung von Dokumenten:"
                                                <ul>
                                                    <li>"für Kopien und Ausdrucke"</li>        
                                                    <li>"für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR"</li>
                                                    <li>"für jede weitere Seite 0,15 EUR"</li>
                                                    <li>"für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR"</li>
                                                    <li>"für jede weitere Seite in Farbe 0,30 EUR"</li>
                                                </ul>
                                            </li>
                                            <li>"7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR."</li>
                                            <li>"7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe."</li>
                                            <li>"7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise"
                                                <ol>
                                                    <li>"von nicht mehr als 4 Stunden 30,00 EUR"</li>
                                                    <li>"von mehr als 4 bis 8 Stunden 50,00 EUR"</li>
                                                    <li>"von mehr als 8 Stunden 80,00 EUR"</li>
                                                </ol>
                                                "Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden."</li>
                                            <li>"7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe."</li>
                                            "Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet."
                                        </ul>
                                    </p>
                                </div>
                            </td>
                            <td></td>
                            <td></td>
                            <td class="px-1">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || asa.get().unwrap_or(0.0)
                                    on:change=change_aussergerichtlich_sonstige_auslagen
                                    prop:value=move || if aa.get().unwrap_or(false) { format_euro(asa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
                                />
                            </td>
                        </tr>                        
                        <tr class="font-semibold">
                            <td></td>
                            <td class="px-1">Summe</td>
                            <td></td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(summe_aussergerichtlich.get()) }
                            </td>
                        </tr>  
                    </tbody>
                </table>
            </p>
        </div>

        // Hauptsacheverfahren
        <div class=move || if v.get().unwrap_or(0) != 1 { // Container einblenden, wenn nicht "nur vorläufiger Rechtsschutz" ausgewählt ist 
                "container visible max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300"
            } else {
                "hidden"
            }
        >
            <h2 class="pt-4 text-2xl font-medium">
                "Hauptsacheverfahren"
            </h2>
            <p>
                <input
                    type="checkbox"
                    id="h1"
                    on:change=change_h1
                    prop:checked=move || h1.get().unwrap_or(true)
                />
                <label for="h1" class="mx-1">1. Instanz</label>
                <input
                    type="checkbox"
                    id="h2"
                    on:change=change_h2
                    prop:checked=move || h2.get().unwrap_or(false)
                />
                <label for="h2" class="mx-1">2. Instanz</label>
                <input
                    type="checkbox"
                    id="h3"
                    on:change=change_h3
                    prop:checked=move || h3.get().unwrap_or(false)
                />
                <label for="h3" class="mx-1">3. Instanz</label>
            </p>
            <p class=move || if h1.get().unwrap_or(true) { "visible" } else { "hidden" }>
                <h3 class="text-xl font-medium">
                    "1. Instanz"
                </h3>
                <h4 class="text-l font-bold">
                    "Rechtsanwaltsvergütungsgesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 13 RVG)"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 49 RVG)"
                            </th>
                            <th class="pl-1">
                                "Differenz"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3100"
                                    on:change=change_n3100
                                    prop:checked=move || n3100.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3100">"Verfahrensgebühr, Nr. 3100"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,3"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb49_h1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h1.get() - verfgeb49_h1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3101"
                                    on:change=change_n3101
                                    prop:checked=move || n3101.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3101">"Ermäßigte Verfahrensgebühr, Nr. 3101"</label>
                                <button popovertarget="ermaessigung3101" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung3101" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Verfahrensgebühr Nr. 3100"</h4>
                                    <p>{ popover::ERMAESSIGUNG3101 }</p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                "0,8"
                            </td>
                            <td></td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="anr"
                                    on:change=change_anrechnung
                                    prop:checked=move || anr.get().unwrap_or(a.get().unwrap_or(false))
                                />
                            </td>
                            <td colspan="2" class="px-1">
                                <label for="anr">"Anrechnung der Geschäfts- auf die Verfahrensgebühr"</label>
                                <button popovertarget="anrechnung" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="anrechnung" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Vorbemerkung 3 Abs. 4 VV RVG"</h4>
                                    <p>{ popover::ANRECHNUNG }</p>
                                </div>                                
                            </td>
                            <td class="px-1 text-right">
                                <span class="mr-1">"-"</span>
                                { move || format_euro(anrechnung13.get()) }
                            </td>
                            <td class="px-1 text-right">
                                <span class="mr-1">"-"</span>
                                { move || format_euro(anrechnung49.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(anrechnung13.get() - anrechnung49.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3104"
                                    on:change=change_n3104
                                    prop:checked=move || n3104.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3104">"Terminsgebühr, Nr. 3104"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,2"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb49_h1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h1.get() - tgeb49_h1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="h1_pauschale"
                                    on:change=change_h1_pauschale
                                    prop:checked=move || h1p.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="h1_pauschale">"Auslagenpauschale, Nr. 7002"</label>
                            </td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale13_h1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h1.get() - pauschale13_h1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="h1_auslagen"
                                    on:change=change_h1_auslagen
                                    prop:checked=move || h1a.get().unwrap_or(false)
                                />
                            </td>
                            <td colspan="2" class="px-1">
                                <label for="h1_auslagen">"Sonstige Auslagen"</label>
                                <button popovertarget="h1auslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="h1auslagen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Sonstige Auslagen"</h4>
                                    <p>"Zum Beispiel:"
                                        <ul>
                                            <li>"7000 Pauschale für die Herstellung und Überlassung von Dokumenten:"
                                                <ul>
                                                    <li>"für Kopien und Ausdrucke"</li>        
                                                    <li>"für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR"</li>
                                                    <li>"für jede weitere Seite 0,15 EUR"</li>
                                                    <li>"für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR"</li>
                                                    <li>"für jede weitere Seite in Farbe 0,30 EUR"</li>
                                                </ul>
                                            </li>
                                            <li>"7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR."</li>
                                            <li>"7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe."</li>
                                            <li>"7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise"
                                                <ol>
                                                    <li>"von nicht mehr als 4 Stunden 30,00 EUR"</li>
                                                    <li>"von mehr als 4 bis 8 Stunden 50,00 EUR"</li>
                                                    <li>"von mehr als 8 Stunden 80,00 EUR"</li>
                                                </ol>
                                                "Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden."</li>
                                            <li>"7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe."</li>
                                            "Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet."
                                        </ul>
                                    </p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || h1sa.get().unwrap_or(0.0)
                                    on:change=change_h1_sonstige_auslagen
                                    prop:value=move || if h1a.get().unwrap_or(false) { format_euro(h1sa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
                                />
                            </td>
                            <td colspan="2"></td>
                        </tr>
                    </tbody>
                </table>
                <h4 class="text-l font-bold">
                    "Gerichtskostengesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5110"
                                    on:change=change_n5110
                                    prop:checked=move || n5110.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1 max-w-64">
                                <label for="n5110">"Verfahren im Allgemeinen, Nr. 5110"</label>
                            </td>
                            <td class="px-1 text-right">
                                "3,0"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(gkg_h1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5111"
                                    on:change=change_n5111
                                    prop:checked=move || n5111.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-64">
                                <label for="n5111">"Ermäßigte Gebühr, Nr. 5111"</label>
                                <button popovertarget="ermaessigung5111" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5111" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5110"</h4>
                                    <p>{ popover::ERMAESSIGUNG5111 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "1,0"
                            </td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </p>
            <p class=move || if h2.get().unwrap_or(false) { "visible" } else { "hidden" }>
                <h3 class="text-xl font-medium">
                    "2. Instanz"
                </h3>
                <h4 class="text-l font-bold">
                    "Rechtsanwaltsvergütungsgesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 13 RVG)"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 49 RVG)"
                            </th>
                            <th class="pl-1">
                                "Differenz"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3200"
                                    on:change=change_n3200
                                    prop:checked=move || n3200.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3200">"Verfahrensgebühr, Nr. 3200"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,6"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb49_h2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h2.get() - verfgeb49_h2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3201"
                                    on:change=change_n3201
                                    prop:checked=move || n3201.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3201">"Ermäßigte Verfahrensgebühr, Nr. 3201"</label>
                                <button popovertarget="ermaessigung3201" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung3201" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Verfahrensgebühr Nr. 3200"</h4>
                                    <p>{ popover::ERMAESSIGUNG3201 }</p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                "1,1"
                            </td>
                            <td></td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3202"
                                    on:change=change_n3202
                                    prop:checked=move || n3202.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3202">"Terminsgebühr, Nr. 3202"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,2"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb49_h2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h2.get() - tgeb49_h2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="h2_pauschale"
                                    on:change=change_h2_pauschale
                                    prop:checked=move || h2p.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="h2_pauschale">"Auslagenpauschale, Nr. 7002"</label>
                            </td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale13_h2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h2.get() - pauschale13_h2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="h2_auslagen"
                                    on:change=change_h2_auslagen
                                    prop:checked=move || h2a.get().unwrap_or(false)
                                />
                            </td>
                            <td colspan="2" class="px-1">
                                <label for="h2_auslagen">"Sonstige Auslagen"</label>
                                <button popovertarget="h2auslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="h2auslagen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Sonstige Auslagen"</h4>
                                    <p>"Zum Beispiel:"
                                        <ul>
                                            <li>"7000 Pauschale für die Herstellung und Überlassung von Dokumenten:"
                                                <ul>
                                                    <li>"für Kopien und Ausdrucke"</li>        
                                                    <li>"für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR"</li>
                                                    <li>"für jede weitere Seite 0,15 EUR"</li>
                                                    <li>"für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR"</li>
                                                    <li>"für jede weitere Seite in Farbe 0,30 EUR"</li>
                                                </ul>
                                            </li>
                                            <li>"7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR."</li>
                                            <li>"7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe."</li>
                                            <li>"7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise"
                                                <ol>
                                                    <li>"von nicht mehr als 4 Stunden 30,00 EUR"</li>
                                                    <li>"von mehr als 4 bis 8 Stunden 50,00 EUR"</li>
                                                    <li>"von mehr als 8 Stunden 80,00 EUR"</li>
                                                </ol>
                                                "Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden."</li>
                                            <li>"7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe."</li>
                                            "Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet."
                                        </ul>
                                    </p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || h2sa.get().unwrap_or(0.0)
                                    on:change=change_h2_sonstige_auslagen
                                    prop:value=move || if h2a.get().unwrap_or(false) { format_euro(h2sa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
                                />
                            </td>
                            <td colspan="2"></td>
                        </tr>
                    </tbody>
                </table>
                <h4 class="text-l font-bold">
                    "Gerichtskostengesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5122"
                                    on:change=change_n5122
                                    prop:checked=move || n5122.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5122">"Verfahren im Allgemeinen, Nr. 5122"</label>
                            </td>
                            <td class="px-1 text-right">
                                "4,0"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(gkg_h2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5120"
                                    on:change=change_n5120
                                    prop:checked=move || n5120.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5120">"Verfahren über die Zulassung der Berufung, soweit der Antrag abgeleht wird, Nr. 5120"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,0"
                            </td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5121"
                                    on:change=change_n5121
                                    prop:checked=move || n5121.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5121">"Verfahren über die Zulassung der Berufung, soweit der Antrag zurückgenommen oder das Verfahren durch anderweitige Erledigung beendet wird, Nr. 5121"</label>
                            </td>
                            <td class="px-1 text-right">
                                "0,5"
                            </td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5123"
                                    on:change=change_n5123
                                    prop:checked=move || n5123.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5123">"Ermäßigte Gebühr, Nr. 5123"</label>
                                <button popovertarget="ermaessigung5123" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5123" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5122"</h4>
                                    <p>{ popover::ERMAESSIGUNG5123 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "1,0"
                            </td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5124"
                                    on:change=change_n5124
                                    prop:checked=move || n5124.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5124">"Weitere Beendigung des Verfahrens, Nr. 5124"</label>
                                <button popovertarget="ermaessigung5124" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5124" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5122"</h4>
                                    <p>{ popover::ERMAESSIGUNG5124 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "2,0"
                            </td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </p>
            <p class=move || if h3.get().unwrap_or(false) { "visible" } else { "hidden" }>
                <h3 class="text-xl font-medium">
                    "3. Instanz"
                </h3>
                <h4 class="text-l font-bold">
                    "Rechtsanwaltsvergütungsgesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 13 RVG)"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 49 RVG)"
                            </th>
                            <th class="pl-1">
                                "Differenz"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3206"
                                    on:change=change_n3206
                                    prop:checked=move || n3206.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3206">"Verfahrensgebühr, Nr. 3206"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,6"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h3.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb49_h3.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h3.get() - verfgeb49_h3.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3207"
                                    on:change=change_n3207
                                    prop:checked=move || n3207.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3207">"Ermäßigte Verfahrensgebühr, Nr. 3207"</label>
                                <button popovertarget="ermaessigung3207" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung3207" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Verfahrensgebühr Nr. 3206"</h4>
                                    <p>{ popover::ERMAESSIGUNG3207 }</p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                "1,1"
                            </td>
                            <td></td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3210"
                                    on:change=change_n3210
                                    prop:checked=move || n3210.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3210">"Terminsgebühr, Nr. 3210"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,5"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h3.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb49_h3.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h3.get() - tgeb49_h3.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="h3_pauschale"
                                    on:change=change_h3_pauschale
                                    prop:checked=move || h3p.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="h3_pauschale">"Auslagenpauschale, Nr. 7002"</label>
                            </td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale13_h3.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h3.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h3.get() - pauschale13_h3.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="h3_auslagen"
                                    on:change=change_h3_auslagen
                                    prop:checked=move || h3a.get().unwrap_or(false)
                                />
                            </td>
                            <td colspan="2" class="px-1">
                                <label for="h3_auslagen">"Sonstige Auslagen"</label>
                                <button popovertarget="h3auslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="h3auslagen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Sonstige Auslagen"</h4>
                                    <p>"Zum Beispiel:"
                                        <ul>
                                            <li>"7000 Pauschale für die Herstellung und Überlassung von Dokumenten:"
                                                <ul>
                                                    <li>"für Kopien und Ausdrucke"</li>        
                                                    <li>"für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR"</li>
                                                    <li>"für jede weitere Seite 0,15 EUR"</li>
                                                    <li>"für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR"</li>
                                                    <li>"für jede weitere Seite in Farbe 0,30 EUR"</li>
                                                </ul>
                                            </li>
                                            <li>"7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR."</li>
                                            <li>"7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe."</li>
                                            <li>"7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise"
                                                <ol>
                                                    <li>"von nicht mehr als 4 Stunden 30,00 EUR"</li>
                                                    <li>"von mehr als 4 bis 8 Stunden 50,00 EUR"</li>
                                                    <li>"von mehr als 8 Stunden 80,00 EUR"</li>
                                                </ol>
                                                "Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden."</li>
                                            <li>"7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe."</li>
                                            "Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet."
                                        </ul>
                                    </p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || h3sa.get().unwrap_or(0.0)
                                    on:change=change_h3_sonstige_auslagen
                                    prop:value=move || if h3a.get().unwrap_or(false) { format_euro(h3sa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
                                />
                            </td>
                            <td colspan="2"></td>
                        </tr>
                    </tbody>
                </table>
                <h4 class="text-l font-bold">
                    "Gerichtskostengesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5130"
                                    on:change=change_n5130
                                    prop:checked=move || n5130.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5130">"Verfahren im Allgemeinen, Nr. 5130"</label>
                            </td>
                            <td class="px-1 text-right">
                                "5,0"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(gkg_h3.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5131"
                                    on:change=change_n5131
                                    prop:checked=move || n5131.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5131">"Ermäßigte Gebühr, Nr. 5131"</label>
                                <button popovertarget="ermaessigung5131" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5131" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5130"</h4>
                                    <p>{ popover::ERMAESSIGUNG5131 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "1,0"
                            </td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5132"
                                    on:change=change_n5132
                                    prop:checked=move || n5132.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5132">"Weitere Beendigung des Verfahrens, Nr. 5132"</label>
                                <button popovertarget="ermaessigung5132" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5132" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5130"</h4>
                                    <p>{ popover::ERMAESSIGUNG5132 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "3,0"
                            </td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </p>
            <h3 class="text-xl font-medium">
                "Summen"
            </h3>
            <p class="grid grid-cols-3 font-semibold">
                <div>
                    "Rechtsanwaltsvergütungsgesetz (§ 13 RVG)"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_rvg13_h.get()) }
                </div>
                <div></div>
                <div>
                    "Rechtsanwaltsvergütungsgesetz (§ 49 RVG)"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_rvg49_h.get()) }
                </div>
                <div></div>
                <div>
                    "Gerichtskostengesetz"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_gkg_h.get()) }
                </div>
                <div></div>        
            </p>
        </div>

        // Vorläufiger Rechtsschutz
        <div class=move || if v.get().unwrap_or(0) != 0 { // Container einblenden, wenn nicht "nur Hauptsacheverfahren" ausgewählt ist 
                "container visible max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300"
            } else {
                "hidden"
            }
        >
            <h2 class="pt-4 text-2xl font-medium">
                "Vorläufiger Rechtsschutz"
            </h2>
            <p>
                <input
                    type="checkbox"
                    id="v1"
                    on:change=change_v1
                    prop:checked=move || v1.get().unwrap_or(true)
                />
                <span class="mx-1">1. Instanz</span>
                <input
                    type="checkbox"
                    id="v2"
                    on:change=change_v2
                    prop:checked=move || v2.get().unwrap_or(false)
                />
                <span class="mx-1">2. Instanz</span>
            </p>
            <p class=move || if v1.get().unwrap_or(true) { "visible" } else { "hidden" }>
                <h3 class="text-xl font-medium">
                    "1. Instanz"
                </h3>
                <h4 class="text-l font-bold">
                    "Rechtsanwaltsvergütungsgesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 13 RVG)"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 49 RVG)"
                            </th>
                            <th class="pl-1">
                                "Differenz"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3100v"
                                    on:change=change_n3100v
                                    prop:checked=move || n3100v.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3100v">"Verfahrensgebühr, Nr. 3100"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,3"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_v1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb49_v1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_v1.get() - verfgeb49_v1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3101"
                                    on:change=change_n3101v
                                    prop:checked=move || n3101v.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3101">"Ermäßigte Verfahrensgebühr, Nr. 3101"</label>
                                <button popovertarget="ermaessigung3101v" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung3101v" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Verfahrensgebühr Nr. 3100"</h4>
                                    <p>{ popover::ERMAESSIGUNG3101 }</p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                "0,8"
                            </td>
                            <td></td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3104v"
                                    on:change=change_n3104v
                                    prop:checked=move || n3104v.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3104v">"Terminsgebühr, Nr. 3104"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,2"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_v1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb49_v1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_v1.get() - tgeb49_v1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="v1_pauschale"
                                    on:change=change_v1_pauschale
                                    prop:checked=move || v1p.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="v1_pauschale">"Auslagenpauschale, Nr. 7002"</label>
                            </td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale13_v1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_v1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_v1.get() - pauschale13_v1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="v1_auslagen"
                                    on:change=change_v1_auslagen
                                    prop:checked=move || v1a.get().unwrap_or(false)
                                />
                            </td>
                            <td colspan="2" class="px-1">
                                <label for="v1_auslagen">"Sonstige Auslagen"</label>
                                <button popovertarget="v1auslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="v1auslagen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Sonstige Auslagen"</h4>
                                    <p>"Zum Beispiel:"
                                        <ul>
                                            <li>"7000 Pauschale für die Herstellung und Überlassung von Dokumenten:"
                                                <ul>
                                                    <li>"für Kopien und Ausdrucke"</li>        
                                                    <li>"für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR"</li>
                                                    <li>"für jede weitere Seite 0,15 EUR"</li>
                                                    <li>"für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR"</li>
                                                    <li>"für jede weitere Seite in Farbe 0,30 EUR"</li>
                                                </ul>
                                            </li>
                                            <li>"7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR."</li>
                                            <li>"7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe."</li>
                                            <li>"7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise"
                                                <ol>
                                                    <li>"von nicht mehr als 4 Stunden 30,00 EUR"</li>
                                                    <li>"von mehr als 4 bis 8 Stunden 50,00 EUR"</li>
                                                    <li>"von mehr als 8 Stunden 80,00 EUR"</li>
                                                </ol>
                                                "Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden."</li>
                                            <li>"7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe."</li>
                                            "Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet."
                                        </ul>
                                    </p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || v1sa.get().unwrap_or(0.0)
                                    on:change=change_v1_sonstige_auslagen
                                    prop:value=move || if v1a.get().unwrap_or(false) { format_euro(v1sa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
                                />
                            </td>
                            <td colspan="2"></td>
                        </tr>
                    </tbody>
                </table>
                <h4 class="text-l font-bold">
                    "Gerichtskostengesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5210"
                                    on:change=change_n5210
                                    prop:checked=move || n5210.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1 max-w-64">
                                <label for="n5210">"Verfahren im Allgemeinen, Nr. 5210"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,5"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(gkg_v1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5211"
                                    on:change=change_n5211
                                    prop:checked=move || n5211.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-64">
                                <label for="n5211">"Ermäßigte Gebühr, Nr. 5211"</label>
                                <button popovertarget="ermaessigung5211" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5211" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5210"</h4>
                                    <p>{ popover::ERMAESSIGUNG5211 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "0,5"
                            </td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </p>
            <p class=move || if v2.get().unwrap_or(false) { "visible" } else { "hidden" }>
                <h3 class="text-xl font-medium">
                    "2. Instanz"
                </h3>
                <h4 class="text-l font-medium">
                    "Rechtsanwaltsvergütungsgesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 13 RVG)"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 49 RVG)"
                            </th>
                            <th class="pl-1">
                                "Differenz"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3200v"
                                    on:change=change_n3200v
                                    prop:checked=move || n3200v.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3200">"Verfahrensgebühr, Nr. 3200"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,6"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_v2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb49_v2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_v2.get() - verfgeb49_v2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3201v"
                                    on:change=change_n3201v
                                    prop:checked=move || n3201v.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3201v">"Ermäßigte Verfahrensgebühr, Nr. 3201"</label>
                                <button popovertarget="ermaessigung3201v" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung3201v" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Verfahrensgebühr Nr. 3200"</h4>
                                    <p>{ popover::ERMAESSIGUNG3201 }</p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                "1,1"
                            </td>
                            <td></td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3202v"
                                    on:change=change_n3202v
                                    prop:checked=move || n3202v.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3202v">"Terminsgebühr, Nr. 3202"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,2"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_v2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb49_v2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_v2.get() - tgeb49_v2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="v2_pauschale"
                                    on:change=change_v2_pauschale
                                    prop:checked=move || v2p.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="v2_pauschale">"Auslagenpauschale, Nr. 7002"</label>
                            </td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale13_v2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_v2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_v2.get() - pauschale13_v2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="v2_auslagen"
                                    on:change=change_v2_auslagen
                                    prop:checked=move || v2a.get().unwrap_or(false)
                                />
                            </td>
                            <td colspan="2" class="px-1">
                                <label for="v2_auslagen">"Sonstige Auslagen"</label>
                                <button popovertarget="v2auslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="v2auslagen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Sonstige Auslagen"</h4>
                                    <p>"Zum Beispiel:"
                                        <ul>
                                            <li>"7000 Pauschale für die Herstellung und Überlassung von Dokumenten:"
                                                <ul>
                                                    <li>"für Kopien und Ausdrucke"</li>        
                                                    <li>"für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR"</li>
                                                    <li>"für jede weitere Seite 0,15 EUR"</li>
                                                    <li>"für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR"</li>
                                                    <li>"für jede weitere Seite in Farbe 0,30 EUR"</li>
                                                </ul>
                                            </li>
                                            <li>"7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR."</li>
                                            <li>"7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe."</li>
                                            <li>"7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise"
                                                <ol>
                                                    <li>"von nicht mehr als 4 Stunden 30,00 EUR"</li>
                                                    <li>"von mehr als 4 bis 8 Stunden 50,00 EUR"</li>
                                                    <li>"von mehr als 8 Stunden 80,00 EUR"</li>
                                                </ol>
                                                "Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden."</li>
                                            <li>"7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe."</li>
                                            "Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet."
                                        </ul>
                                    </p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || v2sa.get().unwrap_or(0.0)
                                    on:change=change_v2_sonstige_auslagen
                                    prop:value=move || if v2a.get().unwrap_or(false) { format_euro(v2sa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
                                />
                            </td>
                            <td colspan="2"></td>
                        </tr>
                    </tbody>
                </table>
                <h4 class="text-l font-medium">
                    "Gerichtskostengesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5240"
                                    on:change=change_n5240
                                    prop:checked=move || n5240.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5240">"Verfahren im Allgemeinen, Nr. 5240"</label>
                            </td>
                            <td class="px-1 text-right">
                                "2,0"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(gkg_v2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5241"
                                    on:change=change_n5241
                                    prop:checked=move || n5241.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5241">"Ermäßigte Gebühr, Nr. 5241"</label>
                                <button popovertarget="ermaessigung5241" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5241" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5240"</h4>
                                    <p>{ popover::ERMAESSIGUNG5241 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "0,5"
                            </td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </p>
            <h3 class="text-xl font-medium">
                "Summen"
            </h3>
            <p class="grid grid-cols-3 font-semibold">
                <div>
                    "Rechtsanwaltsvergütungsgesetz (§ 13 RVG)"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_rvg13_v.get()) }
                </div>
                <div></div>
                <div>
                    "Rechtsanwaltsvergütungsgesetz (§ 49 RVG)"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_rvg49_v.get()) }
                </div>
                <div></div>
                <div>
                    "Gerichtskostengesetz"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_gkg_v.get()) }
                </div>
                <div></div>        
            </p>
        </div>

        // Summen
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Summe"
            </h2>
            <div class="grid grid-cols-5">
                <div class="col-span-2"></div>
                <div class="text-right font-semibold">
                    "Wertgebühren (§ 13 RVG)"
                </div>
                <div class="text-right font-semibold">
                    "Wertgebühren (§ 49 RVG / PKH)"
                </div>
                <div class="text-right font-semibold">
                    "Differenz"
                </div>
                <div class="col-span-5 pt-4 text-xl font-medium">
                    "Summe Rechtsanwaltsvergütungsgesetz"
                </div>
                <div class=move || if a.get().unwrap_or(false) == true && v.get().unwrap_or(0) != 1 { "visible col-span-2" } else { "hidden col-span-2" }>
                    "Außergerichtliche Vertretung"
                </div>
                <div class=move || if a.get().unwrap_or(false) == true  && v.get().unwrap_or(0) != 1 { "visible text-right" } else { "hidden" }>
                    { move || format_euro(summe_aussergerichtlich.get()) }
                </div>
                <div class=move || if a.get().unwrap_or(false) == true  && v.get().unwrap_or(0) != 1 { "visible text-right" } else { "hidden" }>
                    { move || format_euro(summe_aussergerichtlich.get()) }
                </div>
                <div class=move || if a.get().unwrap_or(false) == true  && v.get().unwrap_or(0) != 1 { "visible text-right" } else { "hidden" }>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible col-span-2"} else { "hidden col-span-2" }>
                    "Hauptsacheverfahren"
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_rvg13_h.get()) }
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_rvg49_h.get()) }
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_rvg13_h.get() - summe_rvg49_h.get()) }
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible col-span-2"} else { "hidden col-span-2" }>
                    "Vorläufiger Rechtsschutz"
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visibl text-right"} else { "hidden" }>
                    { move || format_euro(summe_rvg13_v.get()) }
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_rvg49_v.get()) }
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible text-right" } else { "hidden" }>
                    { move || format_euro(summe_rvg13_v.get() - summe_rvg49_v.get()) }
                </div>
                <div class="col-span-2 italic">
                    "Summe netto"
                </div>
                <div class="text-right italic">
                    { move || format_euro(summe_rvg13_netto.get()) }
                </div>
                <div class="text-right italic">
                    { move || format_euro(summe_rvg49_netto.get()) }
                </div>            
                <div class="text-right italic">
                    { move || format_euro(summe_rvg13_netto.get() - summe_rvg49_netto.get()) }
                </div>
                <div>
                    "Umsatzsteuer, Nr. 7008 VV RVG"
                </div>
                <div>
                    <input type="number" min="0" value=move || u.get().unwrap_or(19) class="px-1 w-16 border-2 border-stone-400 rounded-lg" on:change=change_umsatzsteuer />
                    <span class="ml-1">%</span>
                    <button popovertarget="umsatzsteuer" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                    <div id="umsatzsteuer" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                        <h4 class="text-xl font-medium">Steuersatz der Umsatzsteuer</h4>
                        <p>{ popover::UMSATZSTEUER }</p>
                    </div>
                </div>
                <div class="text-right">
                    { move || format_euro(fees::umsatzsteuer(u.get().unwrap_or(19), summe_rvg13_netto.get())) }
                </div>
                <div class="text-right">
                    { move || format_euro(fees::umsatzsteuer(u.get().unwrap_or(19), summe_rvg49_netto.get())) }
                </div>
                <div class="text-right">
                    { move || format_euro(fees::umsatzsteuer(u.get().unwrap_or(19), summe_rvg13_netto.get()) - fees::umsatzsteuer(u.get().unwrap_or(19), summe_rvg49_netto.get())) }
                </div>
                <div class="col-span-2 font-semibold">
                    "Summe brutto"
                </div>
                <div class="text-right font-semibold">
                    { move || format_euro(fees::umsatzsteuer_brutto(u.get().unwrap_or(19), summe_rvg13_netto.get())) }
                </div>
                <div class="text-right italic">
                    { move || format_euro(fees::umsatzsteuer_brutto(u.get().unwrap_or(19), summe_rvg49_netto.get())) }
                </div>            
                <div class="text-right italic">
                    { move || format_euro(fees::umsatzsteuer_brutto(u.get().unwrap_or(19), summe_rvg13_netto.get()) - fees::umsatzsteuer_brutto(u.get().unwrap_or(19), summe_rvg49_netto.get())) }
                </div> 
                <div class="col-span-5 pt-4 text-xl font-medium">
                    "Summe Gerichtskostengesetz"
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible col-span-2"} else { "hidden col-span-2" }>
                    "Hauptsacheverfahren"
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_gkg_h.get()) }
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_gkg_h.get()) }
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible"} else { "hidden" }>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible col-span-2"} else { "hidden col-span-2" }>
                    "Vorläufiger Rechtsschutz"
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visibl text-right"} else { "hidden" }>
                    { move || format_euro(summe_gkg_v.get()) }
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_gkg_v.get()) }
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible" } else { "hidden" }>
                </div>
                <div class="col-span-2 font-semibold">
                    "Summe"
                </div>
                <div class="text-right font-semibold">
                    { move || format_euro(summe_gkg.get()) }
                </div>
                <div class="text-right italic">
                    { move || format_euro(summe_gkg.get()) }
                </div>            
                <div></div>             
                <div class="col-span-2 pt-4 text-xl font-medium">
                    "Gesamtsumme"
                </div>
                <div class="pt-4 text-right text-xl font-medium">
                    { move || format_euro(gesamtsumme13.get()) }
                </div>
                <div class="pt-4 text-right text-l italic">
                    { move || format_euro(gesamtsumme49.get()) }
                </div>
                <div class="pt-4 text-right text-l italic">
                    { move || format_euro(gesamtsumme13.get() - gesamtsumme49.get()) }
                </div>
            </div>
        </div>

        // Rechtliche Hinweise
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Rechtliche Hinweise"
            </h2>
            <p>
                "Dieser Prozesskostenrechner berechnet gesetzliche Gebühren auf der Grundlage des 
                Rechtsanwaltsvergütungsgesetzes ("<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://dejure.org/gesetze/RVG">"RVG"</a>
                "), des Gerichtskostengesetzes ("<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://dejure.org/gesetze/GKG">"GKG"</a>
                "), des "<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://www.bverwg.de/rechtsprechung/streitwertkatalog">"Streitwertkatalogs"</a>
                " des Bundesverwaltungsgerichts und meiner Erfahrung mit der Interpretation dieser Vorgaben durch die
                Verwaltungsgerichte vornehmlich in NRW. Der Rechner dient nur einer unverbindlichen
                Orientierung und kann eine fachkundige Beratung nicht ersetzen. Seine Nutzung erfolgt insofern auf eigene
                Gefahr. Es kann eine Vielzahl von Gründen geben, warum ein Gericht höhere oder niedrigere Kosten festsetzt,
                als von diesem Rechner ermittelt. Auch möchten viele Rechtsanwält*innen Vergütungsvereinbarungen schließen,
                die zum Teil deutlich von den gesetzlich vorgesehenen Gebühren abweichen. Der Rechner geht zudem auch davon
                aus, dass sich die Behördenseite nicht anwaltlich vertreten lässt. Tatsächlich lassen sich Behörden in
                migrationsrechtlichen Streitigkeiten erfahungsgemäß selten anwaltlich vertreten. Völlig ausgeschlossen ist
                es aber auch nicht. Ggf. würden hierdurch weitere Kosten entstehen, die dieser Rechner nicht berücksichtigt.
                Außerdem berücksichtigt dieser Rechner keine Gebühren, die durch Behörden im Verwaltungsverfahren erhoben
                werden. Widerspruchs- und Remonstrationsverfahren werden ebenfalls (noch?) nicht abgebildet, was vor allem
                daran liegt, dass ich hauptsächlich in NRW tätig bin, wo es kaum noch Widerspruchsverfahren gibt. Die
                Anrechnung der Geschäftsgebühr auf die Verfahrensgebühr (Vorbemerkung 4 zu Teil 3 VV RVG) erfolgt immer auf
                die 1. Instanz des Hauptsacheverfahrens, da dies auch in der Praxis nahezu immer der Fall sein wird. Soweit
                zumindest theoretisch auch Fälle konstruiert werden können, in denen die Anrechnung auf die Verfahrensgebühr
                in einer höheren Insatz erfolgt, bleiben diese Fälle hier um der Einfachheit willen unberücksichtigt."
            </p>
            <p>
                "Die Abkürzung PKH steht für Prozesskostenhilfe. Da die Wertgebühren bei Prozesskostenhilfe teilweise abweichen,
                werden die entsprechenden Gebühren gesondert ausgewiesen. Für außergerichtliche Vertretung gibt es keine
                Prozesskostenhilfe, daher werden für die außergerichtliche Vertretung immer die Wergebühren nach § 13 RVG ausgewiesen.
                Für die Abrechnung gegenüber Mandant*innen sind in aller Regel die Gebühren nach § 13 RVG in der jeweils linken Spalte maßgeblich.
                Die PKH-Gebühren werden meist nur für die Abrechnung gegenüber der Staatskasse benötigt, wenn Prozesskostenhilfe bewilligt wurde.
                Dass die PKH-Gebühren und ihre Differenz zu den Wertgebühren nach § 13 RVG hier gesondert ausgewiesen werden, versteht sich
                daher eher als Serviceleistung für Anwaltskanzleien. Für Rechtssuchende hingegen sind sie grundsätzlich irrelevant, da
                es für sie nur auf die Wertgebühren gemäß § 13 RVG ankommt."
            </p>
            <p>
                "Der Rechner geht äußerst sparsam mit deinen Daten um. Zwar werden einige technisch benötigte Daten,
                insbesondere deine IP-Adresse, an meinen Server gesendet und von meinem Server verarbeitet. Das ist nötig,
                um die zum Rechner gehörenden Dateien an deinen Browser oder dein sonstiges Gerät, mit welchem du den
                Rechner ausführst, schicken zu können. Der Rechner wird aber vollständig auf deinem Gerät ausgeführt. Das
                bedeutet, dass alle Daten, die du in den Rechner eingibst, und die Ergebnisse, die von meinem Rechner
                erzeugt werden, vollständig bei dir verbleiben, und erst gar nicht an meinen Server geschickt, geschweige
                denn verarbeitet oder gespeichert werden."
            </p>
            <p>
                "Der Rechner ist zudem auch als Freie Software unter den Lizenzen Apache, Version 2.0, und MIT
                veröffentlicht. Du kannst dir die Software also auch aus dem "<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://github.com/dusmarcel/mpkr25">
                "Repository"</a>" herunterladen und sie dann ganz auf einem Gerät deiner Wahl ausführen.
                In diesem Falle hast du mit meinem Server gar nichts mehr zu tun, und die Notwendigkeit, Daten an meinen
                Server zu übertragen, entfällt ganz."
            </p>
            <p>
                "Und schließlich geht es hier noch zu meinem „"<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://aufentha.lt/index.php/impressum/">"Impressum"</a>"“."
            </p>
        </div>
    }
}
