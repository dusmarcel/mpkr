pub const AUFFANGSTREITWERT: u32 = 5000;

pub fn default_streitwert(t: u32, p: u32) -> u32 { // Thema des Verfahrens, Anzahl Personen
    match t {
        0 ..= 2 => 5000 + ((p - 1) * 1000),
        3 => 2500 + ((p - 1) * 500),
        4 ..= 6 => AUFFANGSTREITWERT * p,
        7 => AUFFANGSTREITWERT / 2 * p, // Duldung
        8 => AUFFANGSTREITWERT * 2 * p, // Einbürgerung
        _ => AUFFANGSTREITWERT
    }
}