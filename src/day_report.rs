

pub use crate::recs::*;
use crate::engine::*;
use orbtk::prelude::{PropertySource, IntoPropertySource};
use std::io::Error;
use std::fmt;
use std::default::Default;

/// Struct for the day report in week report
#[derive(Debug, PartialEq, Clone)]
pub struct DayReport {
    pub day: NaiveDate,
    pub food: Option<Vec<String>>,
    pub bed: [NaiveTime; 2],
    pub ranking: Vec<Tagtime>,
    pub shower: bool,
    pub selfs: Option<Vec<String>>,
}

impl DayReport {

    pub fn new(recs: &Vec<Rec>) -> Result<DayReport, Error> {
        let day = recs[0].h.ok_or(err_inp("Waking hour not found"))?.date();
        let bed = bed_time(recs)?;
        let food = match_food(recs);
        let ranking = recs.get_tagtimes();
        let selfs = get_self(recs);
        let shower = recs.match_query(&query_tag(String::from("Doccia"))).is_some();

        Ok(DayReport {
            day,
            food,
            bed,
            ranking,
            shower,
            selfs,
        })
    }

}

impl Default for DayReport {
    fn default() -> Self {
        DayReport{
            day: chrono::offset::Local::today().naive_local(),
            food: None,
            bed: [NaiveTime::from_hms(1,1,1),NaiveTime::from_hms(1,1,1)],
            ranking: Vec::<Tagtime>::new(),
            shower: false,
            selfs: None,
        }
    }
}

fn disp_weekday(inp: &Weekday) -> String {
    let out = match inp {
        Weekday::Mon => "MONDAY",
        Weekday::Tue => "TUESDAY",
        Weekday::Wed => "WEDNESDAY",
        Weekday::Thu => "THURSDAY",
        Weekday::Fri => "FRIDAY",
        Weekday::Sat => "SATURDAY",
        Weekday::Sun => "SUNDAY",
    };
    String::from(out)
}


impl fmt::Display for DayReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Day report for: {:?}\n  \n ", &self.day)?;
        writeln!(f, "{}\n  \n", disp_weekday(&self.day.weekday()))?;
        writeln!(f, "Woke up at: {:?} \nWent to sleep at: {:?}\n \n", &self.bed[0], &self.bed[1])?;
        let food_str = &self.food.as_ref().or(Some(&vec!["No food recorded".to_string()])).unwrap()
                            .iter().fold(String::new(), |acc, x| acc + &return_string(x,20) + "\n    ");
        writeln!(f, "Food eaten:\n  {}", &food_str)?;
        let selfs = &self.selfs.as_ref().or(Some(&vec!["No selfs recorded".to_string()])).unwrap()
                            .iter().fold(String::new(), |acc, x| acc + x + "\n    ");
        writeln!(f, "Shower:   {}\n \n", if self.shower {"Yes"} else {"No"})?;
        writeln!(f, "Selfs:\n    {}", &selfs)?;
        let ranks = &self.ranking.iter().enumerate()
                                        .filter_map(|(count,x)| if count>10 {None} else {Some(x)})
                                        .fold(String::new(), |acc, x| acc + &format!("        {}",x));
        write!(f, "Tag ranking:\n       Time      Tag\n{}", &ranks)

    }
}

// Word wrap (at spaces)
fn return_string(inp: &String, n_char: usize) -> String {
    let mut out = inp.clone();
    let a = out.len()/n_char;
    if a>1 {(1..a+1).collect::<Vec<usize>>().iter().for_each(|x| out.insert_str(find_space(&out,x*n_char),"\n      "))}
    out
}
// Find the last space before the usize
fn find_space(inp: &String, id: usize) -> usize {
    inp.split_at(id).0.rfind(" ").unwrap()
}


#[derive(Debug, PartialEq, Clone, Default)]
pub struct TotWeekReport {
    pub ranking: Vec<Tagtime>,
    pub selfs: usize,
}

impl TotWeekReport {
    pub fn  new(recs: &Vec<Rec>, selfs: usize) -> Result<TotWeekReport, Error> {
        let ranking = recs.get_tagtimes();

        Ok(TotWeekReport {
            ranking,
            selfs,
        })
    }
}

impl fmt::Display for TotWeekReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Total for the last 7 days:\n")?;
        writeln!(f, "Selfs:  {}  /   7", &self.selfs)?;
        let ranks = &self.ranking.iter().enumerate()
                                        .filter_map(|(count,x)| if count>25 {None} else {Some(x)})
                                        .fold(String::new(), |acc, x| acc + &format!("        {}",x));
        write!(f, "Tag ranking:\n       Time      Tag\n{}", &ranks)

    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct WeekReport {
    days: Vec<NaiveDate>,
    pub day_reports: Vec<Option<DayReport>>,
    pub tot_report: TotWeekReport,
}

impl WeekReport {

    pub fn new(folder: &Path, n_week: i64) -> Result<WeekReport, Error> {

        let today =chrono::offset::Local::today().naive_local()+Duration::days(7*n_week);
        let starting_day = today+Duration::days(-7);
        let mut inp = Vec::from_folder(folder)?;
        inp.flatten();
        let last_week = retrieve_days(&inp, n_week);

        let day_reports: Vec<Option<DayReport>> = last_week.iter().map(|a| day_builder(a)).collect::<Vec<Option<DayReport>>>();
        let days: Vec<NaiveDate> = (0..7).map(|a| starting_day+Duration::days(a)).collect::<Vec<NaiveDate>>();
        let n_selfs= day_reports.iter().filter(|a| a.is_some()).filter(|a| a.as_ref().unwrap().selfs.is_some()).count();
        let tot_report = tot_builder(&inp, n_selfs, &today)?;
        Ok(WeekReport {
        days,
        day_reports,
        tot_report,
        })
    }
}

impl Default for WeekReport {
    fn default() -> Self {
        WeekReport {
            days: Vec::new(),
            day_reports: vec![None; 7],
            tot_report: TotWeekReport::default(),
        }
    }
}

impl IntoPropertySource<WeekReport> for WeekReport {
    fn into_source(self) -> PropertySource<WeekReport> {
        PropertySource::Value(self)
    }
}

// Implement: focus search files based on current date
/// Retrieve the last 7 days and nest them into a Vec of Vec<Rec>
fn retrieve_days<'a>(inp: &Vec<Rec>, n_week: i64) -> Vec<Option<Vec<Rec>>> {
    let today = chrono::offset::Local::today().naive_local();
    let mut query = Query::new();
    let mut out: Vec<Option<Vec<Rec>>> =Vec::new();
    for i in 0..7 {
        let day_iter=today+Duration::days(i-7+n_week*7);
        query.days = Some([day_iter,day_iter]);
        out.push(inp.match_query(&query));
        if out.last().is_none() {println!("Missing data for {:?}",day_iter)};
    }
    out
}

/// Creates a query for one tag
fn query_tag(inp: String) -> Query {
    let mut q = Query::new();
    q.tags = Some(vec![inp]);
    q
}

/// Get food from Vec<Rec>
fn match_food(inp: &Vec<Rec>) -> Option<Vec<String>> {
    let tags = ["Colazione", "Merenda", "Pranzo", "Cena", "Aperitivo"].iter().map(|a| a.to_string());
    let querys: Vec<Query> = tags.map(|a| query_tag(a)).collect();
    let rec_food = if let Some(a)=inp.match_mult_query(&querys) {a} else {return None};
    let food = rec_food.iter().filter_map(|a| a.description.clone()).collect();
    Some(food)


}


fn get_self(inp: &Vec<Rec>) -> Option<Vec<String>> {
    let recs = if let Some(a)=inp.match_query(&query_tag(String::from("Self"))) {a} else {return None};
    let selfs = recs.iter().filter_map(|a| a.description.clone()).collect();
    Some(selfs)
}



/// Get the first hour recorded and last hour + last duration recorded
fn bed_time(recs: &Vec<Rec>) -> Result<[NaiveTime; 2], Error> {
    let b1 =recs[0].h.ok_or(err_inp("Waking hour not found"))?.time();
    let b2 =recs.last().unwrap();
    let b2= b2.h.ok_or(err_inp("Sleep hour not found"))?.time()+b2.t;
    Ok([b1, b2])

}



/// Wrapper for DayReport::new(), used in WeekReport::new() for DayReport creation in line
fn day_builder(inp: &Option<Vec<Rec>>) -> Option<DayReport> {
    match inp {
        None => None,
        Some(a) => match DayReport::new(&a){
            Ok(a) => Some(a),
            Err(b) => {
                println!("Errore: {:?}",b);
                None
            },
        }
    }
}

fn tot_builder(inp: &Vec<Rec>, n_selfs: usize, today: &NaiveDate) -> Result<TotWeekReport, Error> {
    let mut q_last =Query::new();
    q_last.days = Some([*today+Duration::days(-7),*today]);
    Ok(TotWeekReport::new(&inp.match_query(&q_last).ok_or(err_inp("Data not found for the last week"))?,
                    n_selfs)?)
}
