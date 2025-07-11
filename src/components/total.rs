use leptos::prelude::*;

use crate::utils::format_euro;
use crate::fees;
use crate::popover;

// Summen

#[component]
pub fn Total(
    v: Memo<Option<u32>>,
    a: Memo<Option<bool>>,
    summe_aussergerichtlich: Memo<f64>,
    summe_rvg13_h: Memo<f64>,
    summe_rvg49_h: Memo<f64>,
    summe_gkg_h: Memo<f64>,
    summe_rvg13_v: Memo<f64>,
    summe_rvg49_v: Memo<f64>,
    summe_gkg_v: Memo<f64>,
    summe_rvg13_netto: Memo<f64>,
    summe_rvg49_netto: Memo<f64>,
    u: Memo<Option<u32>>,
    set_u: SignalSetter<Option<u32>>,
    summe_gkg: Memo<f64>,
    gesamtsumme13: Memo<f64>,
    gesamtsumme49: Memo<f64>
) -> impl IntoView {
    let change_umsatzsteuer = move |ev| {
        set_u.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(19)));
    };

    view! {
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
    }
}