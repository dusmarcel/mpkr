use fees::{pauschale, umsatzsteuer_brutto};
use leptos::prelude::*;
use leptos_router::{hooks::query_signal_with_options, location::State, NavigateOptions};

mod popover;
mod fees;
mod utils;
mod components;
use crate::components::{
    intro::Intro,
    status::Status,
    value::Value,
    extrajudicial::Extrajudicial,
    main::Main,
    interim::Interim,
    total::Total,
    notes::Notes
};

#[component]
pub fn MPKR() -> impl IntoView {
    // Streitwerte
    let (v, set_v) = query_signal_with_options::<u32>(
        "v", 
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let (t_changed, set_t_changed) = signal(false);
    let (t, set_t) = query_signal_with_options::<u32>(
        "t",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let (p_changed, set_p_changed) = signal(false);
    let (p, set_p) = query_signal_with_options::<u32>(
        "p",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let (s, set_s) = query_signal_with_options::<f64>(
        "s",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let (sv, set_sv) = query_signal_with_options::<f64>(
        "sv",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
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
    let (g, set_g) = query_signal_with_options::<bool>(
        "g",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let (gs, set_gs) = query_signal_with_options::<f64>(
        "gs",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let n2300 = Memo::new( move |_| {
        gs.get().unwrap_or(1.3) * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
    });
    let (ap, set_ap) = query_signal_with_options::<bool>(
        "ap",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let (aa, set_aa) = query_signal_with_options::<bool>(
        "aa",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let (asa, set_asa) = query_signal_with_options::<f64>(
        "asa",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
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
    let (h2, set_h2) = query_signal_with_options::<bool>(
        "h2",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let (h3, set_h3) = query_signal_with_options::<bool>(
        "h3",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    // 1. Instanz Hauptsachverfahren

    //RVG
    let (n3100, set_n3100) = query_signal_with_options::<bool>(
        "n3100",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    let (n3101, set_n3101) = query_signal_with_options::<bool>(
        "n3101",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

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

    let (v2, set_v2) = query_signal_with_options::<bool>(
        "v2",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    // 1. Instanz

    // RVG
    let (n3100v, set_n3100v) = query_signal_with_options::<bool>(
        "n3100v",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    let (n3101v, set_n3101v) = query_signal_with_options::<bool>(
        "n3101v",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
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
        <Value v=v set_v=set_v set_t_changed=set_t_changed t=t set_t=set_t set_p_changed=set_p_changed p=p set_p=set_p s=s set_s=set_s sv=sv set_sv=set_sv />
        <Extrajudicial v=v a=a set_a=set_a g=g set_g=set_g gs=gs set_gs=set_gs n2300=n2300 ap=ap set_ap=set_ap aa=aa set_aa=set_aa asa=asa set_asa=set_asa summe_aussergerichtlich=summe_aussergerichtlich />
        <Main h1=h1 set_h1=set_h1 h2=h2 set_h2=set_h2 h3=h3 set_h3=set_h3 n3100=n3100 set_n3100=set_n3100 n3101=n3101 set_n3101=set_n3101 verfgeb13_h1=verfgeb13_h1 verfgeb49_h1=verfgeb49_h1 />
        <Interim v1=v1 set_v1=set_v1 v2=v2 set_v2=set_v2 n3100v=n3100v set_n3100v=set_n3100v n3101v=n3101v set_n3101v=set_n3101v verfgeb13_v1=verfgeb13_v1 verfgeb49_v1=verfgeb49_v1 summe_rvg13_v=summe_rvg13_v summe_rvg49_v=summe_rvg49_v summe_gkg_v=summe_gkg_v />
        <Total summe_rvg13_netto=summe_rvg13_netto summe_rvg49_netto=summe_rvg49_netto u=u set_u=set_u summe_gkg=summe_gkg gesamtsumme13=gesamtsumme13 gesamtsumme49=gesamtsumme49 />
        <Notes />
    }
}
