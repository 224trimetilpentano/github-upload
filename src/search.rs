// Default 10 days before

use orbtk::prelude::*;
use crate::tag_search::*;
use std::io::Error;

use crate::styles;
use std::path::Path;
use std::fmt::Display;

enum Action {
    BoxActivated(Entity),
    SwitchChildren,
    Search,
}

#[derive(AsAny)]
pub struct SearchState {
    action: Option<Action>
}
impl Default for SearchState {
    fn default() -> Self {
        SearchState { action: None }
    }
}

impl SearchState {
    fn action(&mut self, act: impl Into<Option<Action>>) {
        self.action = act.into();
    }
}

pub fn block_builder<T: Display>(inp: &T, ctx: &mut Context, stack: Entity) {
    let splits = String::from(format!("{}",inp));
    let splits = splits.split("\n");
    splits.for_each(|b| ctx.append_child_to(TextBlock::new().text(b)
                                                .font_size(15)
                                                .v_align("center")
                                                .h_align("center"), stack))

}

impl State for SearchState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(act)=&self.action {
            match act {
                Action::BoxActivated(entity) => {
                    let mut text_box = orbtk::widgets::text_box(ctx.get_widget(*entity));
                    let text = text_box.text_mut();
                    println!("submitting {}", text);
                    text.clear();
                },
                Action::SwitchChildren => {
                    let now = *search_view(ctx.widget()).cut_children();
                    *search_view(ctx.widget()).cut_children_mut() = !now;
                },
                Action::Search => {
                    let plot_theme = styles::theme::Dark.get();
                    let query = query_builder(ctx);
                    let tagan = TagAn::new(Path::new(&rec_folder()),&query, *search_view(ctx.widget()).cut_children());
                    if let Ok(tagan) = tagan {
                        let grid_en =ctx.child("grid").entity();
                        ctx.clear_children_of(grid_en);
                        // stats and ranking
                        ctx.append_child_to(Stack::new()
                                                    .orientation(Orientation::Vertical)
                                                    .spacing(5.0)
                                                    .v_align("center")
                                                    .id("stack_stats"),grid_en);
                        let stack_stats = ctx.child("stack_stats").entity();
                        block_builder(&tagan, ctx, stack_stats);

                        ctx.append_child_to(Stack::new()
                                                    .orientation(Orientation::Vertical)
                                                    .spacing(5.0)
                                                    .v_align("center")
                                                    .id("chart_stats")
                                                    .attach(Grid::column(0))
                                                    .attach(Grid::row(1)),grid_en);
                        let chart_stats = ctx.child("chart_stats").entity();
                        block_builder(&tagan.t_chart, ctx, chart_stats);

                        ctx.append_child_to(Stack::new()
                                                    .orientation(Orientation::Vertical)
                                                    .spacing(5.0)
                                                    .v_align("center")
                                                    .id("stack_rank")
                                                    .attach(Grid::column(3))
                                                    .attach(Grid::row(0)),grid_en);
                        let stack_rank = ctx.child("stack_rank").entity();
                        let mut ranking = tagan.rank_tags.iter().fold(String::new(), |acc, a| acc + format!("\n {}",a).as_str());
                        ranking = String::from("Tag ranking:\n") + &ranking;
                        block_builder(&ranking, ctx, stack_rank);
                        ctx.append_child_to(Stack::new()
                                                    .orientation(Orientation::Vertical)
                                                    .spacing(5.0)
                                                    .v_align("center")
                                                    .id("stack_last")
                                                    .attach(Grid::column(3))
                                                    .attach(Grid::row(1)),grid_en);
                        let stack_last = ctx.child("stack_last").entity();
                        let last = tagan.last.iter().fold(String::from("Last records:\n"),|a, b| a + "\n" + b);
                        block_builder(&last, ctx, stack_last);
                        // plots
                        if tagan.n_rec != 1 {
                            tagan.t_stats.plot(&Path::new("C:\\Users\\bonal\\OneDrive\\Desktop\\Codice\\Rust\\rec\\TEMP\\T_hist.png"), &plot_theme);
                            tagan.h_stats.plot(&Path::new("C:\\Users\\bonal\\OneDrive\\Desktop\\Codice\\Rust\\rec\\TEMP\\H_hist.png"), &plot_theme);
                            tagan.t_chart.plot(&Path::new("C:\\Users\\bonal\\OneDrive\\Desktop\\Codice\\Rust\\rec\\TEMP\\T_chart.png"), &plot_theme);
                            ctx.append_child_to(ImageWidget::new()
                                    .id("T_stats")
                                    .image("C:\\Users\\bonal\\OneDrive\\Desktop\\Codice\\Rust\\rec\\TEMP\\T_hist.png")
                                    .attach(Grid::column(1))
                                    .attach(Grid::row(0)),grid_en);
                            ctx.append_child_to(ImageWidget::new()
                                    .id("H_stats")
                                    .image("C:\\Users\\bonal\\OneDrive\\Desktop\\Codice\\Rust\\rec\\TEMP\\H_hist.png")
                                    .attach(Grid::column(2))
                                    .attach(Grid::row(0)),grid_en);
                            ctx.append_child_to(ImageWidget::new()
                                    .id("T_chart")
                                    .image("C:\\Users\\bonal\\OneDrive\\Desktop\\Codice\\Rust\\rec\\TEMP\\T_chart.png")
                                    .attach(Grid::column(1))
                                    .attach(Grid::row(1)),grid_en);
                                }
                        *search_view(ctx.widget()).result_mut() = tagan;

                    } else {
                        ctx.child("text").set("text",String16::from("Matches not found"));
                    }
                },
            }
            self.action = None;
        }
    }
}

fn query_builder(ctx: &mut Context) -> Query {
    let mut query = Query::new();
    let [t,nt] = text_parser(search_view(ctx.widget()).tag_id().as_string());
    let [d, nd] = text_parser(search_view(ctx.widget()).desc_id().as_string());
    let dates = date_query_parser(search_view(ctx.widget()).date0().as_string(),
                                    search_view(ctx.widget()).date1().as_string());
    let hours = hour_parser(search_view(ctx.widget()).time0().as_string(),
                                    search_view(ctx.widget()).time1().as_string());
    query.tags = t;
    query.not_tags = nt;
    query.description = d;
    query.not_description = nd;
    query.days = dates;
    query.h = hours;
    query

}


fn text_parser(inp: String) -> [Option<Vec<String>>; 2] {
    if inp.is_empty() {return [None, None]}
    let splits = inp.split(" ").map(|a| String::from(a));
    let (not_cont, cont) : (Vec<String>, Vec<String>) = splits.partition(|a| a.starts_with("!"));
    let not_cont = if not_cont.len()==0 {None} else {Some(not_cont.iter().map(|a| a.get(1..).unwrap().to_string()).collect())};
    let cont = if cont.len()==0 {None} else {Some(cont)};
    [cont, not_cont]

}

fn hour_parser(inp0: String, inp1: String) -> Option<[NaiveTime;2]> {
    if [&inp0, &inp1].iter().all(|a| a.is_empty()) {return None}
    let start_hour = NaiveTime::parse_from_str(&inp0, "%H:%M:%S").unwrap_or(NaiveTime::from_hms(0,0,0));
    let end_hour = NaiveTime::parse_from_str(&inp1, "%H:%M:%S").unwrap_or(NaiveTime::from_hms(23,59,59));
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

fn date_query_parser(inp0: String, inp1: String) -> Option<[NaiveDate;2]> {
    if [&inp0, &inp1].iter().all(|a| a.is_empty()) {return None}
    let start_date = date_parser(&inp0).unwrap_or(NaiveDate::from_ymd(1970,1,1));
    let end_date = date_parser(&inp1).unwrap_or(chrono::offset::Local::today().naive_local());
    Some([start_date, end_date])
}

// Non supporta Option
widget!(SearchView<SearchState> {
    tag_id: String16,
    desc_id: String16,
    date0: String16,
    date1: String16,
    time0: String16,
    time1: String16,
    cut_children: bool,
    result: TagAn
    }
);

impl Template for SearchView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("SearchView")
            .result(TagAn::default())
            .cut_children(true)
            .date1(chrono::offset::Local::today().naive_local().format("%Y/%m/%d").to_string())
            .date0((chrono::offset::Local::today().naive_local()-Duration::days(10))
                        .format("%Y/%m/%d").to_string())
            .child(Stack::new().orientation(Orientation::Vertical)
                .child(TextBlock::new().text("Search").font_size(20.0).build(ctx))
                .child(Stack::new().orientation(Orientation::Horizontal)
                        .child(TextBox::new()
                                .id("tag_bar")
                                .water_mark("Tags")
                                .text(("tag_id", id))
                                .margin((0, 8, 0, 0))
                                .width(500.0)
                                .on_activate(move |states, entity| {state(id, states).action(Action::BoxActivated(entity))})
                                .build(ctx))
                        .child(TextBox::new()
                                .id("desc_bar")
                                .water_mark("Description")
                                .text(("desc_id", id))
                                .margin((0, 8, 0, 0))
                                .on_activate(move |states, entity| {state(id, states).action(Action::BoxActivated(entity))})
                                .build(ctx))
                        .child(TextBox::new()
                                .id("date0_bar")
                                .water_mark("Start date")
                                .text(("date0", id))
                                .margin((0, 8, 0, 0))
                                .on_activate(move |states, entity| {state(id, states).action(Action::BoxActivated(entity))})
                                .build(ctx))
                        .child(TextBox::new()
                                .id("date1_bar")
                                .water_mark("End date")
                                .text(("date1", id))
                                .margin((0, 8, 0, 0))
                                .on_activate(move |states, entity| {state(id, states).action(Action::BoxActivated(entity))})
                                .build(ctx))
                        .child(TextBox::new()
                                .id("time0_bar")
                                .water_mark("Start H:M:S")
                                .text(("time0", id))
                                .margin((0, 8, 0, 0))
                                .on_activate(move |states, entity| {state(id, states).action(Action::BoxActivated(entity))})
                                .build(ctx))
                        .child(TextBox::new()
                                .id("time1_bar")
                                .water_mark("End H:M:S")
                                .text(("time1", id))
                                .margin((0, 8, 0, 0))
                                .on_activate(move |states, entity| {state(id, states).action(Action::BoxActivated(entity))})
                                .build(ctx))
                        .child(Button::new()
                                .text("Search")
                                .margin((0, 8, 0, 0))
                                .on_click(move |states, _| {
                                    state(id, states).action(Action::Search);
                                    true
                                })
                                .build(ctx)
                                )
                        .child(
                            Switch::new()
                                .on_changed(move |states, _entity, _| {
                                    state(id, states).action(Action::SwitchChildren);
                                })
                                .v_align("center")
                                .build(ctx),
                        )
                        .width(1000.0)
                        .spacing(20.0)
                        .build(ctx))
                .child(Grid::new()
                        .id("grid")
                        .columns(Columns::create()
                                    .push(300)
                                    .push(400)
                                    .push(400)
                                    .push(300),)
                        .rows(Rows::create().push(350).push(350))
                        .child(TextBlock::new()
                                .id("text")
                                .attach(Grid::column(0))
                                .attach(Grid::row(0))
                                .text("Insert query").build(ctx))
                        .build(ctx))
                .build(ctx)
            )

    }
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut SearchState {
    states.get_mut(id)
}
