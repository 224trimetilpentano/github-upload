
//Tests!!!
// Per una ricerca "lazy" (che cerca solo le chiavi Some()) sarebbe necessaria una funzione match dinamica, richiede tempo per implementare
// Query funzioni utili (giorni da oggi, durata da 0, durata a partire da etc..)
// Documentation

use crate::recs::*;
use std::fmt;
use std::cmp::Ordering;
use std::default::Default;

#[derive(Debug)]
pub struct Query {
    pub days: Option<[NaiveDate; 2]>,
    pub h: Option<[NaiveTime; 2]>,
    pub t: Option<[Duration; 2]>,
    pub description: Option<Vec<String>>,
    pub not_description: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub not_tags: Option<Vec<String>>,
}

impl Query {
    /// Creates an empty (full of None) Query
    pub fn new() -> Query {
        Query {
            days: None,
            h: None,
            t: None,
            description: None,
            not_description: None,
            tags: None,
            not_tags: None,
        }
    }
}


/// Tagtime
/// Struct useful for tag ordering based on time.
/// Field 0: tag name (String)
/// Field 1: duration (chrono::Duration)
#[derive(Eq, Debug, Clone)]
pub struct Tagtime(pub String, pub Duration);

impl Default for Tagtime {
    fn default() -> Self {
        Tagtime(String::from("Unknown"), Duration::seconds(0))
    }
}

impl Ord for Tagtime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for Tagtime {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl PartialEq for Tagtime {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl fmt::Display for Tagtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"{} , {:?},", WrapDuration(self.1),  &self.0)
    }
}

// Base

fn is_between<T: PartialOrd>(ins: &T, out: &[T;2]) -> bool {
    ins >= &out[0] && ins <= &out[1]
}

fn match_day(q: &Query, r: &Rec) -> bool {
    match &q.days {
        None => true,
        Some(a) => match &r.h {
            None => false,
            Some(b) => is_between(&b.date(),a)
        },
    }
}

fn match_h(q: &Query, r: &Rec) -> bool {
    match &q.h {
        None => true,
        Some(a) => match &r.h {
            None => false,
            Some(b) => is_between(&b.time(),a)
        },
    }
}

fn match_t(q: &Query, r: &Rec) -> bool {
    match &q.t {
        None => return true,
        Some(a) => return is_between(&r.t,a),
    }
}

fn match_des(q: &Query, r: &Rec) -> bool {
    match &q.description {
        None => return true,
        Some(a) => {match &r.description {
            None => return false,
            Some(b) => return a.iter().all(|p| b.contains(p)),
            }
        },
    }
}

fn match_not_des(q: &Query, r: &Rec) -> bool {
    match &q.not_description {
        None => return true,
        Some(a) => {match &r.description {
            None => return true,
            Some(b) => return !a.iter().any(|p| b.contains(p)),
            }
        },
    }
}

fn match_tags(q: &Query, r: &Rec) -> bool {
    match &q.tags {
        None => return true,
        Some(qtags) => {match &r.tags {
            None => return false,
            Some(rtags) => return qtags.iter().all(|qtag| rtags.iter().any(|rtag| *rtag==*qtag)),
            }
        },
    }
}

fn match_not_tags(q: &Query, r: &Rec) -> bool {
    match &q.not_tags {
        None => return true,
        Some(qtags) => {match &r.tags {
            None => return true,
            Some(rtags) => return !qtags.iter().any(|qtag| rtags.iter().any(|rtag| *rtag==*qtag)),
            }
        },
    }
}

/// Funzione che raccoglie i match base e restituisce un bool
fn matcher(r:&Rec, q: &Query) -> bool {
    [match_day(q,r), match_h(q,r), match_t(q,r), match_des(q,r), match_not_des(q,r), match_tags(q,r), match_not_tags(q,r)].iter().all(|a| *a)
}

/// Funzione per effettuare match esclusivi (query1 or query2 or ...)
fn matcher_mult(r: &Rec, vq: &Vec<Query>) -> bool {
    vq.iter().any(|a| matcher(r,&a))
}


pub trait RecMatch {
    fn match_query(&self, q: &Query) -> Option<Vec<Rec>>;

    fn match_mult_query(&self, q: &Vec<Query>) -> Option<Vec<Rec>>;

    /// Get the tags with total duration (in tagtime type), sorted by decreasing total duration
    fn get_tagtimes(&self) -> Vec<Tagtime>;

    /// Retrieve time entries for a tag
    fn get_times_from_tag(&self, tag: &String) -> Option<Vec<Duration>>;

    /// Retrieve all tags
    fn get_tags(&self) -> Option<Vec<&String>>;

    fn get_unders(&self) -> Option<Vec<Rec>>;

    fn flatten(&mut self);

    fn cut_children(&mut self);

    fn display(&self) -> String;

}

impl RecMatch for Vec<Rec> {

    fn match_query(&self, q: &Query) -> Option<Vec<Rec>> {
        let out: Vec<Rec>= self.iter().filter(|a| matcher(a,q)).map(|a| a.clone()).collect();
        if out.is_empty() {None} else {Some(out)}
    }


    fn match_mult_query(&self, q: &Vec<Query>) -> Option<Vec<Rec>> {
        let out: Vec<Rec>= self.iter().filter(|a| matcher_mult(a,q)).map(|a| a.clone()).collect();
        if out.is_empty() {None} else {Some(out)}
    }

    fn get_times_from_tag(&self, tag: &String) -> Option<Vec<Duration>> {
        let search = self.iter().filter(|a| a.tags.is_some()).filter(|a| a.tags.as_ref().unwrap().contains(tag));
        let times: Vec<Duration> = search.map(|a| a.t).collect();
        if times.is_empty() {None} else {Some(times)}

    }


    fn get_tagtimes(&self) -> Vec<Tagtime> {
        let tags = if let Some(a) = self.get_tags() {a} else {return Vec::new()};
        let mut out = Vec::new();
        for i in tags.iter() {
            let times = if let Some(a) = self.get_times_from_tag(i) {a} else {continue};
            let dur = times.iter().map(|a| a.to_std().unwrap()).sum();
            out.push(Tagtime((*i).to_string(),Duration::from_std(dur).unwrap()));
        }
        out.sort();
        out.reverse();
        out
    }

    fn get_tags(&self) -> Option<Vec<&String>> {
        let mut out: Vec<&String> = Vec::new();
        for i in self.iter() {
            match &i.tags {
                Some(a) => {
                    let mut to_insert = a.iter().filter(|t| !out.contains(t)).collect();
                    out.append(&mut to_insert);
                }
                _ => continue,
            }
        }
        if out.is_empty() {None} else {Some(out)}
    }

    fn get_unders(&self) -> Option<Vec<Rec>> {
        let tot: Vec<Rec>= self.iter().filter_map(|a| a.children.as_ref()).flatten().cloned().collect();
        if tot.is_empty() {None} else {Some(tot)}
    }

    fn cut_children(&mut self) {
        for i in self.iter_mut() {
            i.flatten();
        }

    }

    fn flatten(&mut self) {
        let (children, pos) : ( Vec<Vec<Rec>>, Vec<usize>) = self.iter_mut().enumerate()
                                        .filter_map(|(counts, a)| a.flatten().zip(Some(counts)))
                                        .unzip();
        for (count,unders) in children.iter().enumerate() {
            let index = pos[count];
            for i in unders.iter() {
                self.insert(index, i.clone());
            }
        }

    }

    fn display(&self) -> String {
        self.iter().fold(String::new(),|acc, item| acc + &format!("{} \n",item))
    }

}
