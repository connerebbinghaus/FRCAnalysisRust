extern crate the_blue_alliance;
extern crate core;

use core::cmp::Ordering;

fn main() {
    let tba = the_blue_alliance::TBA::new("WG5pUFbRtNL36CLKw071dPf3gdGeT16ngwuPTWhkQev1pvX2enVnf2hq2oPYtjCH");
    let mut matches = the_blue_alliance::matches::Match::from_event(tba, "2018migul".to_string()).unwrap();

    matches.sort_by(|a, b| {
        match a.comp_level.cmp(&b.comp_level) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                match a.set_number.cmp(&b.set_number) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => {
                        a.match_number.cmp(&b.match_number)
                    },
                }
            },
        }
    });

    for m in matches {
        if m.team_keys().unwrap().contains(&&"frc4453".to_string()) {
            println!("{}", m.key);
        }
    }
}
