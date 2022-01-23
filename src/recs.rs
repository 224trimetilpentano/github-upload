
//Constructors:
// Move tests in test folder in order to use examples of use, Test ::from_file, ::from_folder
// Documentation


pub use std::time::Duration as stdDuration;
pub use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Duration, Datelike, Weekday, Timelike};
use std::io::{Error, ErrorKind};
use std::fs::read_to_string;
use std::fmt;
pub use std::path::Path;

// Structs


// Duration wrapper, per il fmt
pub struct WrapDuration(pub Duration);

impl fmt::Display for WrapDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = &self.0.num_hours();
        write!(f,"{:02}:{:02}", hours , &self.0.num_minutes()- hours*60)
    }
}

// Word wrap (at spaces)
pub fn return_string(inp: &String, n_char: usize) -> String {
    let mut out = inp.clone();
    let a = out.len()/n_char;
    if a>1 {(1..a+1).collect::<Vec<usize>>().iter().for_each(|x| out.insert_str(find_space(&out,x*n_char),"\n      "))}
    out
}
// Find the last space before the usize
fn find_space(inp: &String, id: usize) -> usize {
    inp.split_at(id).0.rfind(" ").unwrap()
}


#[derive(PartialEq, Debug, Clone)]
pub struct Rec {
    pub h: Option<NaiveDateTime>,
    pub t: Duration,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub children: Option<Vec<Rec>>,
}

// Operazioni tra recs

pub fn time_distance(rec1: &Rec, rec2: &Rec) -> Option<Duration> {
    if rec1.h.is_some() && rec1.h.is_some() {
        Some(rec1.h.unwrap()-rec2.h.unwrap()-rec2.t)
    } else {
        None
    }
}

impl fmt::Display for Rec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"H: {}", display_datetimes_into_time(self.h))?;
        write!(f," T: {}", WrapDuration(self.t))?;
        if self.tags.is_some() {
            let tags_string = self.tags.as_ref().unwrap().iter().fold(String::new(),|acc, item| acc + " " +item);
            write!(f,"\n  Tags: {}",tags_string)?};
        if self.description.is_some() {
            write!(f,"\n  Des: {}", return_string(self.description.as_ref().unwrap(), 15))?
        }
        write!(f,"")

    }
}


fn display_datetimes_into_time(i: Option<NaiveDateTime>) -> String {
    let a = i.unwrap_or(NaiveDate::from_ymd(1970,01,01).and_hms(0,0,0)).time();
    format!("{:02}:{:02}", a.hour(), a.minute())
}

pub fn rec_folder() -> String {
    String::from("C:\\Users\\bonal\\OneDrive\\Desktop\\RecordTime")
}

// Base functions

// Error builders in order to reduce verbosity
pub fn err_inp(msg: &str) -> Error {
    Error::new(ErrorKind::InvalidInput,msg)
}

fn error_print(r: Result<Rec,Error>, n: usize, d: u32) -> Option<Rec> {
    match r {
        Ok(a) => Some(a),
        error => {println!("Failed day {} at record number {}: {:?}",d,n+1,error);
        None},
    }
}

// Parsing functions
fn parse_time(inp: &String) -> Result<NaiveTime,Error> {
    let h_id = inp.find("H ").ok_or(err_inp("Could not find 'H '"))?;
    let inp_cut = inp.get(h_id..h_id+7).ok_or(err_inp("Not enough digits for H parsing"))?.to_string();
    let i_div = inp_cut.find(".").ok_or(err_inp("Could not find the dot for H parsing"))?;
    let h = NaiveTime::from_hms(inp_cut[i_div-2..i_div].parse::<u32>().or(Err(err_inp("Invalid digits for H parsing")))?,
                                inp_cut[i_div+1..i_div+3].parse::<u32>().or(Err(err_inp("Invalid digits for H parsing")))?,0);
    Ok(h)
}

fn parse_dur(inp: &String) -> Result<Duration,Error> {
    let t_id = inp.find("T").ok_or(err_inp("Could not find 'T'"))?;
    let inp_cut = inp.get(t_id..t_id+7).ok_or(err_inp("Not enough digits for T parsing"))?.to_string();
    let t_div = inp_cut.find(".").ok_or(err_inp("Could not find the dot for T parsing"))?;
    let seconds = inp_cut[t_div-2..t_div].parse::<u64>().or(Err(err_inp("Invalid digits for T parsing")))? * 3600
                + inp_cut[t_div+1..t_div+3].parse::<u64>().or(Err(err_inp("Invalid digits for T parsing")))? * 60;
    let h = Duration::from_std(stdDuration::from_secs(seconds)).or(Err(err_inp("Invalid duration")))?;
    Ok(h)
}

fn get_tag(inp: &String) -> Option<Vec<String>> {
    let iter = inp.split("#").skip(1);
    let mut tags : Vec<String> = Vec::new();
    let mut counts=0;
    for i in iter {
        tags.push(i.to_string().trim().to_string());
        counts+=1;
    }
    if counts==0 {return None}
    Some(tags)
}

fn get_des(inp: &String) -> Result<Option<String>, Error> {
    match inp.find("'") {
        Some(a) => {
            let b = inp.rfind("'").unwrap();
            if b>a {
                return Ok(Some(inp[a+1..b].to_string()));
            } else {
                return Err(err_inp("There is only one ' in the description"));
            }},
        None => return Ok(None),

    }

}

// Implementations

impl Rec {
    /// COnstructor for Rec
    pub fn new(inp: String, day: &NaiveDate) -> Result<Rec,Error> {
        let mut iter = inp.split("/");

        let main_str = iter.next().unwrap().to_string();

        let mut children : Vec<Rec> = Vec::new();
        let mut counts = 0;
        let h = Some(day.and_time(parse_time(&main_str)?));
        for i in iter.map(|s| s.to_string()) {

            children.push(Rec{
                h ,
                t : parse_dur(&i)?,
                description: get_des(&i)?,
                tags: get_tag(&i),
                children: None,

            }) ;
            counts+=1;
        }

        let a =  Rec {
            h ,
            t: parse_dur(&main_str)?,
            description: get_des(&main_str)?,
            tags: get_tag(&main_str),
            children: if counts==0 { None } else {Some(children)},
        };
        Ok(a)
    }

    pub fn flatten(&mut self) -> Option<Vec<Rec>> {
        let children = self.children.clone();
        if let Some(a) = children {
            for i in a.iter() {
                self.t = self.t - i.t;
            }
            self.children = None;
            Some(a)
        } else {
            None
        }
    }

}


/// Take an instance of a day input and return the corresponding Recs
fn read_day(count: usize, day: &str, file_name: &Vec<u32>) -> Option<Vec<Rec>> {
    let mut lines = day.lines();
    let day_it = match lines.next().unwrap().parse::<u32>() {
        Ok(a) => NaiveDate::from_ymd(file_name[0] as i32 + 2000, file_name[1], a),
        Err(b) => {
            println!("Day number {} in {:?} failed 'day parsing': {:?}",count, file_name,b);
            return None},
    };
    let recs = lines.enumerate().filter_map(|(b,a)| error_print(Rec::new(a.to_string(),&day_it),b,day_it.day())).collect();
    Some(recs)
}

pub trait RecBuilder {

    fn from_file(path: &Path) -> Result<Vec<Rec>, Error>;

    fn from_folder(path: &Path) -> Result<Vec<Rec>, Error>;
}


/// Constructor for Vec<Rec> from files and folders
impl RecBuilder for Vec<Rec> {

    fn from_file(path: &Path) -> Result<Vec<Rec>, Error> {
        let file_name = path.file_name().unwrap().to_str().unwrap().split(".").next().unwrap();
        let file_name : Vec<u32> = file_name.split("-")
                                            .filter_map(|a| a.parse::<u32>().ok())
                                            .collect();

        let out: Vec<Rec>= read_to_string(path)?.split("|").enumerate()
                                                .filter_map(|(count,day)| read_day(count,day,&file_name))
                                                .flatten().collect();
        Ok(out)
    }

    fn from_folder(path: &Path) -> Result<Vec<Rec>, Error> {
        let files = path.read_dir().expect("read_dir failed");
        let mut out = Vec::new();
        for entry in files {
            if let Ok(entry) = entry {
                let path_file=entry.path();
                match Vec::from_file(&path_file) {
                    Ok(mut a) => out.append(&mut a),
                    Err(e) => {println!("Cannot open file {:?}: {:?}",path_file,e);
                    continue}
                };
            }
        }
        Ok(out)
    }
}


//Tests

#[cfg(test)]
mod tests {
    use std::time::Duration as stdDuration;
    use chrono::{NaiveDate, NaiveTime, Duration};
    use super::*;

    // Test basics
    #[test]
    fn parse_time_should() {
        let out = parse_time(&String::from("H 09.45 T 00.20 'ahah' #test1 #test2 ")).unwrap();
        assert_eq!(out, NaiveTime::from_hms(9,45,0));

    }

    #[test]
    fn parse_dur_should() {
        let out = parse_dur(&String::from("H 09.45 T 00.10 'ahah' #test1 #test2 ")).unwrap();
        assert_eq!(out, Duration::from_std(stdDuration::from_secs(600)).unwrap());

    }

    #[test]
    fn get_des_should() {
        let out = get_des(&String::from(" H 09.45 T 00.10 'ahah' #test1 #test2 ")).unwrap();
        assert_eq!(out, Some(String::from("ahah")));

        let out = get_des(&String::from(" H 09.45 T 00.10 #test1 #test2 ")).unwrap();
        assert_eq!(out, None);

    }


    #[test]
    fn get_tag_should() {
        let out = get_tag(&String::from(" H 09.45 T 00.10 'ahah' #test1 #test2 "));
        assert_eq!(out, Some(vec![String::from("test1"), String::from("test2")]));

        let out = get_tag(&String::from(" H 09.45 T 00.10 'ahah' "));
        assert_eq!(out, None);


    }

    // Test complete
    #[test]
    fn rec_new_should() {
        let inp = String::from(" H 07.30 T 02.30 'Test' #test1 #test2 / T 00.05 'Test1' #test1 #test2 / T 01.45 'Test2' #test1 #test2 ");
        let day = NaiveDate::from_ymd(2012,05,03);
        let res = crate::recs::Rec::new(inp,&day).unwrap();
        let under_p = crate::recs::Rec{
            h: None,
            t: Duration::from_std(stdDuration::from_secs(300)).unwrap(),
            description: Some(String::from("Test1")),
            tags: Some(vec![String::from("test1"), String::from("test2")]),
            children: None,
        };
        let under_np = crate::recs::Rec{
            h: None,
            t: Duration::from_std(stdDuration::from_secs(6300)).unwrap(),
            description: Some(String::from("Test2")),
            tags: Some(vec![String::from("test1"), String::from("test2")]),
            children: None,
        };
        let should_out = crate::recs::Rec {
            h: Some(NaiveDateTime::new(day, NaiveTime::from_hms(07,30,00))),
            t: Duration::from_std(stdDuration::from_secs(9000)).unwrap(),
            description: Some(String::from("Test")),
            tags: Some(vec![String::from("test1"), String::from("test2")]),
            children: Some(vec![under_p, under_np]),
        };
        assert_eq!(res,should_out);
    }
}
