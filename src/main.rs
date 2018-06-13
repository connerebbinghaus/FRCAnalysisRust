extern crate the_blue_alliance;
extern crate futures;

fn main() {
    let tba = the_blue_alliance::TBA::new("WG5pUFbRtNL36CLKw071dPf3gdGeT16ngwuPTWhkQev1pvX2enVnf2hq2oPYtjCH");
    let m = the_blue_alliance::matches::Match::from_event(tba, "2018migul".to_string()).unwrap();
    println!("Got match {}", m.get(0).unwrap().key);
    println!("Score breakdown: {:?}", m.get(0).unwrap().score_breakdown)
}