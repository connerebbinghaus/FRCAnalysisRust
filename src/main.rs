extern crate TheBlueAlliance;
extern crate futures;

fn main() {
    let tba = TheBlueAlliance::TBA::new("WG5pUFbRtNL36CLKw071dPf3gdGeT16ngwuPTWhkQev1pvX2enVnf2hq2oPYtjCH");
    let m = futures::executor::ThreadPool::new().unwrap().spawn(TheBlueAlliance::matches::Match::from_event(tba, "2018migul")).unwrap();
    println!(m.get(0).key);
}