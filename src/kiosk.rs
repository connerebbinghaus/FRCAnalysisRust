use std::collections::HashMap;
use std::io;
use tui::Terminal;
use tui::style::{Style, Color};
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, Table, Row, Paragraph, Text, Gauge};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use futures::Future;

fn complvl_to_string(c: the_blue_alliance::matches::CompLevel) -> &'static str {
    match c {
        the_blue_alliance::matches::CompLevel::QualificationMatch => "QM",
        the_blue_alliance::matches::CompLevel::EighthFinal => "EF",
        the_blue_alliance::matches::CompLevel::QuarterFinal => "QF",
        the_blue_alliance::matches::CompLevel::SemiFinal => "SF",
        the_blue_alliance::matches::CompLevel::Final => "F",
    }
}

fn winner_to_string(c: the_blue_alliance::matches::Winner) -> &'static str {
    match c {
        the_blue_alliance::matches::Winner::Blue => "Blue",
        the_blue_alliance::matches::Winner::Red => "Red",
        the_blue_alliance::matches::Winner::None => "Tie"
    }
}

pub fn run(event_key: &str, tba: &the_blue_alliance::TBA) -> Result<(), io::Error>{
    let screen = crossterm::Screen::default();
    let alternate_screen = screen.enable_alternate_modes(true).unwrap();
    let backend = CrosstermBackend::with_alternate_screen(alternate_screen).unwrap();
    let mut terminal = Terminal::new(backend)?;

    loop {
        let event = the_blue_alliance::event::Event::from_key(tba, event_key).wait().unwrap();

        let matches_f = event.matches(tba);

        let state = ::state::EventState::new(&event, tba, chrono::Utc::now());

        let matches: Vec<_> = matches_f.wait().unwrap();
        
        let mut schedule: Vec<_> = matches.iter().cloned().filter(|m| m.score_breakdown.is_none()).collect();
        schedule.sort_unstable();
        
        let mut results: Vec<_> = matches.iter().cloned().filter(|m| m.score_breakdown.is_some()).collect();
        results.sort_unstable();
        results.reverse();

        let (oprs_prog_send, oprs_progress) = std::sync::mpsc::channel::<f32>();

        let handle = std::thread::spawn(move || {
            oprs_prog_send.send(0.0).unwrap();
            let oprs = ::opr::oprs_from_matches(matches.clone());
            oprs_prog_send.send(0.45).unwrap();
            let dprs = ::opr::dprs_from_matches(matches);
            oprs_prog_send.send(0.95).unwrap();
            let ccwms = ::opr::ccwms_from_oprs_and_dprs(oprs.clone(), dprs.clone());
            oprs_prog_send.send(1.01).unwrap();
            (oprs, dprs, ccwms)
        });

        let time_start = std::time::Instant::now();

        while let Ok(prog) = oprs_progress.recv() {
            terminal.draw(|f| {
                render_status(f, &event, &state, schedule.clone(), results.clone(), prog)
            }).unwrap();
            std::thread::yield_now();
        }

        let (oprs, dprs, ccwms) = handle.join().unwrap();

        std::thread::sleep(std::time::Duration::from_secs(15) - (std::time::Instant::now() - time_start));
        
        terminal.draw(|f| {
            render_oprs(f, &event, oprs, dprs, ccwms)
        }).unwrap();
        
        std::thread::sleep(std::time::Duration::from_secs(30) - (std::time::Instant::now() - time_start));
    }
}

fn render_status<B: tui::backend::Backend>(mut f: tui::Frame<B>, event: &the_blue_alliance::event::Event, state: &::state::EventState, schedule: Vec<the_blue_alliance::matches::Match>, results: Vec<the_blue_alliance::matches::Match>, next_progress: f32) {
    let size = f.size();

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)].as_ref())
        .margin(0)
        .split(size);

    let main_chunk = main_chunks[0];
    let toolbar_chunk = main_chunks[1];

    let mut main_block = Block::default()
        .title(&event.name)
        .borders(Borders::NONE);
    main_block.render(&mut f, main_chunk);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(65), Constraint::Min(85)].as_ref())
        .margin(1)
        .split(main_chunk);

    let chunks2 = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Min(3)].as_ref())
        .margin(0)
        .split(chunks[1]);


    let mut ranking_block = Block::default()
        .title("Rankings")
        .borders(Borders::ALL);
    ranking_block.render(&mut f, chunks[0]);

    let mut schedule_block = Block::default()
        .title("Schedule")
        .borders(Borders::ALL);
    schedule_block.render(&mut f, chunks2[0]);

    let mut results_block = Block::default()
        .title("Results")
        .borders(Borders::ALL);
    results_block.render(&mut f, chunks2[1]);

    if !state.ranking.is_empty() {
    Table::new(
            ["Rank", "Team", "RS", "Cargo", "Panel", "Climb", "Sandstorm", "Played"].iter(),
            state.ranking.iter()
                .map(|t| {
                    (&t.team,
                    if let ::state::TeamRankingData::S2019(ref d) = t.ranking {d} else {panic!("Wrong season!")})
                })
                .enumerate()
                .map(|(i, (n, r))| Row::Data(vec![
                    i.to_string(), 
                    n.clone(), 
                    r.ranking_score.to_string(),
                    r.cargo_points.to_string(),
                    r.panel_points.to_string(),
                    r.climb_points.to_string(),
                    r.sandstorm_points.to_string(),
                    r.played.to_string()].into_iter()))
        )
        .block(ranking_block)
        .header_style(Style::default().fg(Color::Yellow))
        .style(Style::default().fg(Color::White))
        .column_spacing(1)
        .widths(&[4, 12, 6, 6, 6, 6, 10, 6])
        .render(&mut f, chunks[0]);
    } else {
        Paragraph::new([Text::Raw(std::borrow::Cow::Borrowed("NO DATA"))].iter())
            .alignment(Alignment::Center)
            .block(ranking_block)
            .render(&mut f, chunks[0]);
    }

    Table::new(
        ["Level", "Set", "Number", "Red1", "Red2", "Red3", "Blue1", "Blue2", "Blue3"].iter(),
        schedule.into_iter()
            .map(|m| {
                    let na = "N/A".to_owned();
                    Row::Data(vec![
                        complvl_to_string(m.comp_level).to_owned(),
                        m.set_number.to_string(),
                        m.match_number.to_string(),
                        m.alliances.as_ref().map(|a| a.red.team_keys[0].clone()).or_else(|| Some(na.clone())).unwrap(),
                        m.alliances.as_ref().map(|a| a.red.team_keys[1].clone()).or_else(|| Some(na.clone())).unwrap(),
                        m.alliances.as_ref().map(|a| a.red.team_keys[2].clone()).or_else(|| Some(na.clone())).unwrap(),
                        m.alliances.as_ref().map(|a| a.blue.team_keys[0].clone()).or_else(|| Some(na.clone())).unwrap(),
                        m.alliances.as_ref().map(|a| a.blue.team_keys[1].clone()).or_else(|| Some(na.clone())).unwrap(),
                        m.alliances.as_ref().map(|a| a.blue.team_keys[2].clone()).or_else(|| Some(na.clone())).unwrap()
                    ].into_iter())
                }
            )
    )
    .block(schedule_block)
    .header_style(Style::default().fg(Color::Yellow))
    .style(Style::default().fg(Color::White))
    .column_spacing(1)
    .widths(&[6, 4, 8, 8, 8, 8, 8, 8, 8])
    .render(&mut f, chunks2[0]);

    Table::new(
        ["Level", "Set", "Number", "Red1", "Red2", "Red3", "Blue1", "Blue2", "Blue3", "Winner"].iter(),
        results.into_iter()
            .map(|m| {
                    let na = "N/A".to_owned();
                    Row::Data(vec![
                        complvl_to_string(m.comp_level).to_owned(),
                        m.set_number.to_string(),
                        m.match_number.to_string(),
                        m.alliances.as_ref().map(|a| a.red.team_keys[0].clone()).or_else(|| Some(na.clone())).unwrap(),
                        m.alliances.as_ref().map(|a| a.red.team_keys[1].clone()).or_else(|| Some(na.clone())).unwrap(),
                        m.alliances.as_ref().map(|a| a.red.team_keys[2].clone()).or_else(|| Some(na.clone())).unwrap(),
                        m.alliances.as_ref().map(|a| a.blue.team_keys[0].clone()).or_else(|| Some(na.clone())).unwrap(),
                        m.alliances.as_ref().map(|a| a.blue.team_keys[1].clone()).or_else(|| Some(na.clone())).unwrap(),
                        m.alliances.as_ref().map(|a| a.blue.team_keys[2].clone()).or_else(|| Some(na.clone())).unwrap(),
                        winner_to_string(m.winning_alliance.unwrap_or(the_blue_alliance::matches::Winner::None)).to_owned(),
                    ].into_iter())
                }
            )
    )
    .block(results_block)
    .header_style(Style::default().fg(Color::Yellow))
    .style(Style::default().fg(Color::White))
    .column_spacing(1)
    .widths(&[6, 4, 8, 8, 8, 8, 8, 8, 8, 8])
    .render(&mut f, chunks2[1]);

    let toolbar_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Max(50), Constraint::Min(45)].as_ref())
        .margin(0)
        .split(toolbar_chunk);

    if next_progress < 1.0 {
        Gauge::default()
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .percent((next_progress * 100.0) as u16)
            .render(&mut f, toolbar_chunks[0]);
    }

    Paragraph::new([Text::Raw(std::borrow::Cow::Owned(chrono::Local::now().to_rfc2822()))].iter())
            .alignment(Alignment::Right)
            .render(&mut f, toolbar_chunks[1]);

}

fn render_oprs<B: tui::backend::Backend>(mut f: tui::Frame<B>, event: &the_blue_alliance::event::Event, mut oprs: HashMap<String, f32>, dprs: HashMap<String, f32>, ccwms: HashMap<String, f32>) {
    let size = f.size();

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)].as_ref())
        .margin(0)
        .split(size);

    let main_chunk = main_chunks[0];
    let toolbar_chunk = main_chunks[1];

    let mut main_block = Block::default()
        .title(&event.name)
        .borders(Borders::NONE);
    main_block.render(&mut f, main_chunk);

    let main_chunk2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10)].as_ref())
        .margin(1)
        .split(main_chunk);


    Paragraph::new([Text::Raw(std::borrow::Cow::Owned(chrono::Local::now().to_rfc2822()))].iter())
            .alignment(Alignment::Right)
            .render(&mut f, toolbar_chunk);

    let mut oprs_block = Block::default()
        .title("Rankings")
        .borders(Borders::ALL);
    oprs_block.render(&mut f, main_chunk2[0]);

    let mut oprs: Vec<(String, f32)> = oprs.into_iter().collect();

    oprs.sort_unstable_by(|(_, opra), (_, oprb)| opra.partial_cmp(&oprb).unwrap());
    oprs.reverse();

    Table::new(
        ["Team", "OPR", "DPR", "CCWM"].iter(),
        oprs.into_iter()
            .map(|(t, opr)| {
                    let na = "N/A".to_owned();
                    Row::Data(vec![
                        t.clone(),
                        opr.to_string(),
                        dprs.get(&t).map(|dpr| dpr.to_string()).unwrap_or_else(|| na.clone()),
                        ccwms.get(&t).map(|dpr| dpr.to_string()).unwrap_or_else(|| na.clone()),
                    ].into_iter())
                }
            )
    )
    .block(oprs_block)
    .header_style(Style::default().fg(Color::Yellow))
    .style(Style::default().fg(Color::White))
    .column_spacing(1)
    .widths(&[10, 6, 6, 6])
    .render(&mut f, main_chunk2[0]);


}