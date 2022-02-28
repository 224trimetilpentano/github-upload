use crate::day_report::*;
use std::path::Path;
use crate::styles::*;
use crate::widget_generators::*;
use std::io::Error;
use crate::tag_search::*;

use fltk::{app, prelude::*,
    // enums::{FrameType, Align},
    input,
    group};

static PATH_T_HIST: &'static str = "C:\\Users\\bonal\\OneDrive\\Desktop\\Codice\\Rust\\rec\\TEMP\\T_hist.png";
static PATH_H_HIST: &'static str = "C:\\Users\\bonal\\OneDrive\\Desktop\\Codice\\Rust\\rec\\TEMP\\H_hist.png";
static PATH_T_CHART: &'static str = "C:\\Users\\bonal\\OneDrive\\Desktop\\Codice\\Rust\\rec\\TEMP\\T_chart.png";
static PATH_PLOT_ERROR: &'static str = "C:\\Users\\bonal\\OneDrive\\Desktop\\Codice\\Rust\\rec\\TEMP\\error.png";

pub fn search_lay(theme: &'static Theme, s: &app::Sender<Mess>) -> Box<dyn FnMut(Mess)> {
    // Layout dei widget
    let mut search_gen = group::Pack::new(50,100,1400-20,900,"").with_type(group::PackType::Horizontal);
    search_gen.set_spacing(35);

    search_gen.end();

    let mut search_column = group::Pack::new(0,0,200,50,"");
    let mut search_but = create_button("Search", theme);
    search_but.emit(s.clone(), Mess::Search);
    search_column.set_spacing(20);
    search_column.end();

    let in_vec = search_inputs(&mut search_column, &theme);
    search_gen.add(&search_column);

    let mut res_col = group::Group::new(250,100,1000,900,"").with_type(group::PackType::Horizontal);
    let res_1row = group::Flex::new(300,100,150,830,"").column();
    let mut main_wid = create_output("", &theme);
    res_1row.end();

    let mut res_2row = group::Flex::new(450,100,150,830,"").column();
    let mut ranking_wid = create_output("", &theme);
    let mut last_wid = create_output("", &theme);
    res_2row.end();

    res_col.end();
    search_gen.add(&res_col);

    let search_close = move |msg| {
            match msg {
                Mess::Search => {
                    let query = query_builder(&in_vec);
                    let (out_str, image_paths) = search_outputs(&query, &theme);

                    let mut res_3row = group::Flex::new(650,100,800,700,"").column();
                    res_2row.set_pad(70);
                    let res_4row = group::Flex::new(650,100,800,350,"").row();
                    create_image(&Path::new(image_paths[0]), [400,300]);
                    create_image(&Path::new(image_paths[1]), [400,300]);
                    res_4row.end();
                    create_image(&Path::new(image_paths[2]), [800,300]);
                    res_3row.end();
                    res_col.add(&res_3row);
                    res_3row.redraw();

                    main_wid.set_value(&out_str[0]);
                    ranking_wid.set_value(&out_str[1]);
                    last_wid.set_value(&out_str[2]);

                },
                _ => {}
            }
        };
    Box::new(search_close)
}

fn search_outputs(query: &Query, theme: &Theme) -> (Vec<String>, [&'static str;3]) {


    let tagan = TagAn::new(Path::new(&rec_folder()), &query, true);

    if let Ok(tagan) = tagan {
        let mut main_str = String::from(format!("{}",tagan));
        let tchart_str = String::from(format!("{}",tagan.t_chart));
        main_str = main_str + &tchart_str;

        let mut ranking = tagan.rank_tags.iter().fold(String::new(), |acc, a| acc + format!("\n {}",a).as_str());
        ranking = String::from("Tag ranking:\n") + &ranking;

        let last = tagan.last.iter().fold(String::from("Last records:\n"),|a, b| a + "\n" + b);

        let plot_theme = theme.get_plot();
        if tagan.n_rec != 1 {
            tagan.t_stats.plot(&Path::new(PATH_T_HIST), &plot_theme).unwrap_or_else(|_| println!("Something went wrong"));
            tagan.h_stats.plot(&Path::new(PATH_H_HIST), &plot_theme).unwrap_or_else(|_| println!("Something went wrong"));
            tagan.t_chart.plot(&Path::new(PATH_T_CHART), &plot_theme).unwrap_or_else(|_| println!("Something went wrong"));
            (vec![main_str, ranking, last], [PATH_T_HIST, PATH_H_HIST, PATH_T_CHART])
        } else {
            (vec![main_str, ranking, last], [PATH_PLOT_ERROR;3])
        }
    } else {
        (vec![String::from(""); 3], [PATH_PLOT_ERROR;3])
    }
}


fn search_inputs(parent: &mut group::Pack, style: &Theme) -> Vec<input::Input> {
    let input_width = 200;
    let in_tag = create_input(&String::from("Tag"), style).with_size(input_width, 50);
    parent.add(&in_tag);
    let in_des = create_input(&String::from("Description"), style).with_size(input_width, 50);
    parent.add(&in_des);
    let mut in_date_st = create_input(&String::from("Start Date (Y/M/D)"), style).with_size(input_width, 50);
    in_date_st.set_value(&(chrono::offset::Local::today().naive_local()-Duration::days(10))
                .format("%Y/%m/%d").to_string());
    parent.add(&in_date_st);
    let mut in_date_en = create_input(&String::from("End Date (Y/M/D)"), style).with_size(input_width, 50);
    in_date_en.set_value(&(chrono::offset::Local::today().naive_local())
                .format("%Y/%m/%d").to_string());
    parent.add(&in_date_en );
    let in_hour_st = create_input(&String::from("Start Hour (H:M)"), style).with_size(input_width, 50);
    parent.add(&in_hour_st);
    let in_hour_en = create_input(&String::from("End Hour (H:M)"), style).with_size(input_width, 50);
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
