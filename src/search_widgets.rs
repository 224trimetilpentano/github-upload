use crate::day_report::*;
use crate::widget_generators::*;
use std::io::Error;
use crate::tag_search::*;
use crate::styles::*;

use fltk::{app, prelude::*,
    enums::{Align},
    input,
    group};

use super::THEME;

pub fn search_lay(s: &app::Sender<Mess>) -> Box<dyn FnMut(Mess)> {
    // Layout dei widget
    let mut search_gen = group::Pack::new(50,100,1400-20,900,"").with_type(group::PackType::Horizontal);
    search_gen.set_spacing(35);

    search_gen.end();

    let mut search_column = group::Pack::new(0,0,200,50,"");
    let mut search_but = create_button("Search",(200,50));
    search_but.emit(s.clone(), Mess::Search);
    search_column.set_spacing(20);
    let in_vec = search_inputs(&mut search_column);
    let (_ , mut child_but) = create_toggle_button("Include children",(200,50));
    child_but.emit(s.clone(), Mess::Children);
    search_column.end();
    search_gen.add(&search_column);

    let mut cut_children = true;


// RESULTS
    let mut res_col = group::Pack::new(0,0,1200,900,"");
    to_default_style(&mut res_col, &THEME);
    res_col.end();


    spacer(&mut res_col);

// H T analysis
    let mut res_ht = group::Pack::new(0,20,1200,300,"").with_type(group::PackType::Horizontal);
    to_search_pack_style(&mut res_ht, &THEME);
    res_ht.set_label("Duration and hour analysis");
    res_ht.set_spacing(10);
    res_ht.end();
    res_col.add(&res_ht);

    let mut ht_flex = group::Flex::new(0,20,300,300,"");
    let mut main_wid = create_output("");
    let mut ranking_wid = create_output("");
    ht_flex.add(&main_wid);
    ht_flex.add(&ranking_wid);
    ht_flex.end();
    res_ht.add(&ht_flex);

    let mut img_t = MyFrame::new([400,300], 2);
    res_ht.add(&img_t.frame);

    let mut img_h = MyFrame::new([400,300], 2);
    res_ht.add(&img_h.frame);

// Daily analysis
    spacer(&mut res_col);

    let mut res_daily = group::Pack::new(0,0,1200,300,"").with_type(group::PackType::Horizontal);
    to_search_pack_style(&mut res_daily, &THEME);
    res_daily.set_label("Daily records");
    res_daily.end();
    res_col.add(&res_daily);

    let mut daily_pack = group::Pack::new(0,0,300,300,"").with_type(group::PackType::Vertical);
    daily_pack.end();
    let (gr, counter) = create_counter((100,50), (1.0, 31.0), 2.0, "Window size:");
    let mut daily_flex = group::Flex::new(0,0,300,300,"");
    daily_flex.end();
    let mut last_wid = create_output("");
    spacer(&mut daily_pack);
    daily_pack.add(&gr);
    spacer(&mut daily_pack);
    daily_pack.add(&daily_flex);
    daily_flex.add(&last_wid);
    res_daily.add(&daily_pack);

    let mut img_daily = MyFrame::new([800,300], 2);
    res_daily.add(&img_daily.frame);

// General
    res_col.hide();
    let mut scroll = wrap_in_scroll(&res_col, (0,0,1100,700));
    search_gen.add(&scroll);

    let search_close = move |msg| {
            match msg {
                Mess::Search => {
                    let query = query_builder(&in_vec);
                    let tagan = TagAn::new(&rec_folder().join("RecordTime"), &query, cut_children, counter.label().parse::<usize>().unwrap());
                    if let Ok(tagan) = tagan {
                        res_col.show();
                        let ht_str = out_ht(&tagan, &mut img_h.buffer, &mut img_t.buffer);
                        img_t.update_frame();
                        img_h.update_frame();
                        main_wid.set_value(&ht_str[0]);
                        ranking_wid.set_value(&ht_str[1]);


                        let daily_str = out_daily(&tagan, &mut img_daily.buffer);
                        img_daily.update_frame();
                        last_wid.set_value(&daily_str[0]);

                        // daily_flex.auto_layout();
                        scroll.redraw();
                    }
                },
                Mess::Children => {
                    cut_children = !cut_children;
                    search_but.do_callback();
                }
                _ => {}
            }
        };
    Box::new(search_close)
}

fn out_ht(tagan: &TagAn, buf_h: &mut Vec<u8>, buf_t: &mut Vec<u8>) -> Vec<String> {
    let main_str = String::from(format!("{}",tagan));

    let mut ranking = tagan.rank_tags.iter().fold(String::new(), |acc, a| acc + format!("\n {}",a).as_str());
    ranking = String::from("Tag ranking:\n") + &ranking;

    let plot_theme = THEME.get_plot();
    let dim_1 = (800,600);

    tagan.t_stats.bmp_plot(buf_t, dim_1, &plot_theme).unwrap_or_else(|_| println!("Something went wrong"));
    tagan.h_stats.bmp_plot(buf_h, dim_1, &plot_theme).unwrap_or_else(|_| println!("Something went wrong"));
    vec![main_str, ranking]
}

fn out_daily(tagan: &TagAn, bufs: &mut Vec<u8>) -> Vec<String> {

    let tchart_str = String::from(format!("{}",tagan.t_chart));

    let last = tagan.last.iter().fold(String::from("Last records:\n"),|a, b| a + "\n" + b);

    let plot_theme = THEME.get_plot();
    if tagan.n_rec != 1 {
        let dim_2 = (1600,600);
        tagan.t_chart.bmp_plot(bufs, dim_2, &plot_theme).unwrap_or_else(|_| println!("Something went wrong"));
    }
    vec![tchart_str, last]
}

fn spacer<P: GroupExt>(parent: &mut P) {
    parent.add(&group::Tile::new(0,0,parent.w(),30,""));
}

fn search_inputs(parent: &mut group::Pack) -> Vec<input::Input> {
    let input_width = 200;
    let in_tag = create_input(&String::from("Tag")).with_size(input_width, 50);
    parent.add(&in_tag);
    let in_des = create_input(&String::from("Description")).with_size(input_width, 50);
    parent.add(&in_des);
    let mut in_date_st = create_input(&String::from("Start Date (Y/M/D)")).with_size(input_width, 50);
    in_date_st.set_value(&(chrono::offset::Local::today().naive_local()-Duration::days(10))
                .format("%Y/%m/%d").to_string());
    parent.add(&in_date_st);
    let mut in_date_en = create_input(&String::from("End Date (Y/M/D)")).with_size(input_width, 50);
    in_date_en.set_value(&(chrono::offset::Local::today().naive_local())
                .format("%Y/%m/%d").to_string());
    parent.add(&in_date_en );
    let in_hour_st = create_input(&String::from("Start Hour (H:M)")).with_size(input_width, 50);
    parent.add(&in_hour_st);
    let in_hour_en = create_input(&String::from("End Hour (H:M)")).with_size(input_width, 50);
    parent.add(&in_hour_en);
    vec![in_tag, in_des, in_date_st, in_date_en, in_hour_st, in_hour_en]

}

fn query_builder(col: &Vec<input::Input>) -> Query {
    let mut query = Query::new();
    let widget_vals: Vec<String> = col.iter().map(|i| input_parser(i)).collect();
    let [t,nt] = text_parser(&widget_vals[0]);
    let [d, nd] = text_parser(&widget_vals[1]);
    let dates = date_query_parser(&widget_vals[2],&widget_vals[3]);
    let hours = hour_parser(&widget_vals[4],&widget_vals[5]);
    query.tags = t;
    query.not_tags = nt;
    query.description = d;
    query.not_description = nd;
    query.days = dates;
    query.h = hours;
    println!("{:?}",query);
    query

}

fn input_parser(inp: &input::Input) -> String {
    if inp.value()==inp.label() {
        String::from("")
    } else {
        inp.value()
    }
}

fn text_parser(inp: &String) -> [Option<Vec<String>>; 2] {
    if inp.is_empty() {return [None, None]}
    let splits = inp.split(" ").map(|a| String::from(a));
    let (not_cont, cont) : (Vec<String>, Vec<String>) = splits.partition(|a| a.starts_with("!"));
    let not_cont = if not_cont.len()==0 {None} else {Some(not_cont.iter().map(|a| a.get(1..).unwrap().to_string()).collect())};
    let cont = if cont.len()==0 {None} else {Some(cont)};
    [cont, not_cont]

}

fn add_semicolon(a: String) -> String {
    if !a.contains(":") {
        a + ":00"
    } else {
        a
    }
}

fn hour_parser(inp0: &String, inp1: &String) -> Option<[NaiveTime;2]> {
    if [&inp0, &inp1].iter().all(|a| a.is_empty()) {return None}
    let start_hour = NaiveTime::parse_from_str(&(add_semicolon(inp0.clone()) + ":00"), "%H:%M:%S").unwrap_or(NaiveTime::from_hms(0,0,0));
    let end_hour = NaiveTime::parse_from_str(&(add_semicolon(inp1.clone()) + ":00"), "%H:%M:%S").unwrap_or(NaiveTime::from_hms(23,59,59));
    Some([start_hour, end_hour])
}

fn date_parser(inp: &String) -> Result<NaiveDate, Error> {
    let mut numbers = [0; 3];
    for (c,i) in inp.split("/").enumerate() {
        let num = i.parse::<u32>().or(Err(err_inp("Invalid date format")))?;
        numbers[c]=num;
    }
    NaiveDate::from_ymd_opt(numbers[0] as i32, numbers[1], numbers[2]).ok_or(err_inp("Invalid date"))

}

fn date_query_parser(inp0: &String, inp1: &String) -> Option<[NaiveDate;2]> {
    if [&inp0, &inp1].iter().all(|a| a.is_empty()) {return None}
    let start_date = date_parser(&inp0).unwrap_or(NaiveDate::from_ymd(1970,1,1));
    let end_date = date_parser(&inp1).unwrap_or(chrono::offset::Local::today().naive_local());
    Some([start_date, end_date])
}
