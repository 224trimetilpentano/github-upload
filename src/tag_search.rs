
pub use crate::recs::*;
pub use crate::engine::*;
use crate::styles::ThemeStr;
use std::io::Error;
use std::fmt;
use chrono::Timelike;
use std::default::Default;
use orbtk::prelude::{PropertySource, IntoPropertySource};
use plotters::prelude::*;
use std::path::Path;


// use plotters;
// implementare tratti necessari per metter in GUI
// Ordinare le impl
// Correlazione è sbagliata

// struct per le statisitche sulla durata, comprende anche la correlazione con H
#[derive(Debug, PartialEq, Clone)]
pub struct Tstats {
    sum: Duration,
    median: Duration,
    mean: Duration,
    std: Duration,
    corr_h: f64,
    hist: MyHistogram<Duration>,
}

impl Tstats {
    pub fn plot(&self, filename: &Path, plot_theme: &ThemeStr) -> Result<(),Error> {
        self.hist.plot(filename, plot_theme).or(Err(err_inp("Error during MyHistogram plotting")))?;
        Ok(())
    }
}

impl Default for Tstats {
    fn default() -> Self {
        Tstats {
            sum: Duration::seconds(0),
            median: Duration::seconds(0),
            mean: Duration::seconds(0),
            std: Duration::seconds(0),
            corr_h: 0.0,
            hist: MyHistogram::default(),
        }
    }
}

impl fmt::Display for Tstats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"Total duration: {}", WrapDuration(self.sum))?;
        writeln!(f,"Stats:\n    median: {}",WrapDuration(self.median))?;
        writeln!(f,"    mean: {}",WrapDuration(self.mean))?;
        writeln!(f,"    std: {}",WrapDuration(self.std))?;
        writeln!(f,"    H corr: {:.02}",self.corr_h)
    }
}

// struct per le statisitche sull' ora di inizio
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Hstats {
    hist: MyHistogram<NaiveTime>
}

impl Hstats {
    pub fn plot(&self, filename: &Path, plot_theme: &ThemeStr) -> Result<(),Error> {
        self.hist.plot(filename, plot_theme).or(Err(err_inp("Error during MyHistogram plotting")))?;
        Ok(())
    }
}

// struct per gestire gli istogrammi
// controllare la primitive per disegnare istogramma
#[derive(Debug, PartialEq, Clone)]
struct MyHistogram<T> {
    bins: Vec<T>,
    count: Vec<u32>
}

impl<T> Default for MyHistogram<T> {
    fn default() -> Self {
        MyHistogram {
            bins: Vec::<T>::new(),
            count: Vec::<u32>::new(),
        }
    }
}


// Trait that let  multiplication u32 * f64 with the due truncations
trait U32Mod {

    fn times_f(&self, factor: f64) -> u32;

}

impl U32Mod for u32 {
    fn times_f(&self, factor: f64) -> u32 {

        (*self as f64 * factor) as u32
    }

}


impl MyHistogram<Duration> {
    pub fn plot(&self, filename: &Path, plot_theme: &ThemeStr) -> Result<(),Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (400, 300)).into_drawing_area();
        let bin_size = if self.bins.len()>1 {self.bins[1]-self.bins[0]} else {Duration::minutes(20)};
        root.fill(&plot_theme.background)?;
        println!("{:?}", self.bins);
        println!("{:?}", self.count);
        let x_range = std::ops::Range {
            start: self.bins[0].num_minutes() - 15,
            end: (*self.bins.last().unwrap() + bin_size).num_minutes() + 15};

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption("T Histogram", ("sans-serif", 30.0, &plot_theme.caption))
            .build_cartesian_2d(x_range, 0u32..self.count.iter().max().unwrap().times_f(1.2))?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc("Count")
            .x_desc("Minutes")
            .axis_desc_style(("sans-serif", 15, &plot_theme.label))
            .axis_style(WHITE)
            .label_style(("sans-serif", 15, &plot_theme.label))
            .draw()?;

        let data = self.bins.iter().map(|a| (*a+bin_size/2).num_minutes()).zip(self.count.iter().map(|x: &u32| *x));

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(plot_theme.histogram.clone())
                //.margin(bin_size.num_minutes() as u32)
                .data(data),
        )?;

        root.present()?;
        println!("Result has been saved");

        Ok(())


    }
}

// Per ora precisione di un'ora, meglio fare bins dedicati (sopratutto per i pasti e le cose da sera)
impl MyHistogram<NaiveTime> {
    pub fn plot(&self, filename: &Path, plot_theme: &ThemeStr) -> Result<(),Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (400, 300)).into_drawing_area();
        root.fill(&plot_theme.background)?;
        println!("{:?}", self.bins);
        println!("{:?}", self.count);



        let bins_h: Vec<u32> = self.bins.iter().map(|a| a.hour()).collect();
        let x_range = std::ops::Range {
            start: bins_h[0] - 1,
            end: bins_h.last().unwrap() + 1};

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption("H Histogram", ("sans-serif", 30.0, &plot_theme.caption))
            .build_cartesian_2d(x_range, 0u32..self.count.iter().max().unwrap().times_f(1.2))?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc("Count")
            .x_desc("Hour")
            .axis_desc_style(("sans-serif", 15, &plot_theme.label))
            .axis_style(WHITE)
            .label_style(("sans-serif", 15, &plot_theme.label))
            .draw()?;

        let data = bins_h.iter().map(|a: &u32| *a).zip(self.count.iter().map(|x: &u32| *x));

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(plot_theme.histogram.clone())
                .margin(100/self.bins.len() as u32)
                .data(data),
        )?;

        root.present()?;
        println!("Result has been saved");

        Ok(())


    }
}

// struct for temporal evolution chart
#[derive(Debug, PartialEq, Clone)]
pub struct Tchart {
    dates: Vec<NaiveDate>,
    durations: Vec<Duration>,
    range: Duration,
    median: Duration,
    mean: Duration,
    std: Duration,
    median_in_day: Duration,
    mean_in_day: Duration,
    std_in_day: Duration,
}

impl Default for Tchart {
    fn default() -> Tchart {
        Tchart {
        dates: Vec::<NaiveDate>::new(),
        durations: Vec::<Duration>::new(),
        range: Duration::zero(),
        median: Duration::zero(),
        mean: Duration::zero(),
        std: Duration::zero(),
        median_in_day: Duration::zero(),
        mean_in_day: Duration::zero(),
        std_in_day: Duration::zero(),
        }
    }
}

impl Tchart {
    fn new(durs: &Vec<Duration>, dates: &Vec<NaiveDate>, range: &[NaiveDate; 2]) -> Tchart {
        let date_min = range[0].clone();
        let date_max = range[1].clone();
        let date_ax: Vec<NaiveDate> = date_min.iter_days().take_while(|a| *a<=date_max).collect();
        let n = date_ax.len();
        let mut dur_ax = vec![Duration::zero(); n];
        for (count, i) in dates.iter().enumerate() {
            let eureka = date_ax.iter().enumerate().find(|(_bucket,value)| value==&i).unwrap().0;
            dur_ax[eureka] = dur_ax[eureka] + durs[count];
        }
        let non_zero_days = dur_ax.iter().cloned().filter(|a| a.num_minutes()>0).collect();
        let [median_in_day, mean_in_day, std_in_day] = chart_stats(&non_zero_days);
        let [median, mean, std] = chart_stats(&dur_ax);
        println!("{:?}",dur_ax);
        Tchart{
            dates: date_ax,
            durations: dur_ax,
            range: date_max - date_min,
            median,
            mean,
            std,
            mean_in_day,
            median_in_day,
            std_in_day,
        }
    }

    pub fn plot(&self, filename: &Path, plot_theme: &ThemeStr) -> Result<(),Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (800, 300)).into_drawing_area();
        root.fill(&plot_theme.background)?;

        let x_range = std::ops::Range {
            start: self.dates[0],
            end: *self.dates.last().unwrap()};

        let y_range = std::ops::Range {
            start: Duration::zero(),
            end: *self.durations.iter().max().unwrap()};

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(80)
            .margin(5)
            .caption("T timeseries", ("sans-serif", 30.0, &plot_theme.caption))
            .build_cartesian_2d(x_range, y_range)?;

        fn  y_formatter<'r>(a:&'r Duration)-> String {
            format!("{}",WrapDuration(*a)).to_string()
        }

        chart
            .configure_mesh()
            .disable_y_mesh()
            .y_desc("Duration")
            .y_label_formatter(&y_formatter)
            .x_desc("Days")
            .axis_desc_style(("sans-serif", 15, &plot_theme.label))
            .axis_style(WHITE)
            .label_style(("sans-serif", 15, &plot_theme.label))
            .draw()?;

        let data =self.dates.iter().zip(self.durations.iter());



        chart.draw_series(
            LineSeries::new(data.map(|(a,b)| (*a,*b)), plot_theme.tchart_line.clone())
        )?;

        root.present()?;
        println!("Result has been saved");

        Ok(())


    }

}

impl fmt::Display for Tchart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"Every day stats:\n    median: {}",WrapDuration(self.median))?;
        writeln!(f,"    mean : {}",WrapDuration(self.mean))?;
        writeln!(f,"    std: {}",WrapDuration(self.std))?;
        writeln!(f,"In day stats:\n    median: {}",WrapDuration(self.median_in_day))?;
        writeln!(f,"    mean: {}",WrapDuration(self.mean_in_day))?;
        writeln!(f,"    std: {}",WrapDuration(self.std_in_day))
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TagAn {
    pub n_rec: usize,
    pub t_stats: Tstats,
    pub h_stats: Hstats,
    pub rank_tags: Vec<Tagtime>,
    pub t_chart: Tchart,
    pub last: Vec<String>,
}


impl IntoPropertySource<TagAn> for TagAn {
    fn into_source(self) -> PropertySource<TagAn> {
        PropertySource::Value(self)
    }
}

impl fmt::Display for TagAn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"T stats for {} records:", &self.n_rec)?;
        writeln!(f,"{}", &self.t_stats)
    }
}

impl TagAn {

    pub fn new(folder: &Path, query: &Query, cut: bool) -> Result<TagAn, Error> {
        let mut found = search(folder, query)?;
        if cut {
            found.cut_children();
        }
        let n_rec = found.len();
        let last = found.iter().rev().take(5).cloned().map(|a| a.description.unwrap_or(String::from(""))).collect();
        let (mut durs, times, dates) = get_t_h_d(&found);
        let t_chart = Tchart::new(&durs, &dates, &query.days.unwrap());
        let (t_stats, h_stats) =  ht_new(&mut durs, &times);
        let rank_tags = found.get_tagtimes().iter().take(10).cloned().collect();

        Ok(TagAn {
            n_rec,
            t_stats,
            h_stats,
            rank_tags,
            t_chart,
            last,

        })
    }
}


// Searching function
fn search(folder: &Path, query: &Query) -> Result<Vec<Rec>,Error> {
    let inp = Vec::from_folder(folder)?;
    let search = inp.match_query(&query);
    search.ok_or(err_inp("Results matching the tag(s) were not found"))

}

// Extracts durations, times and dates
fn get_t_h_d(inp: &Vec<Rec>) -> (Vec<Duration>, Vec<NaiveTime>, Vec<NaiveDate>) {
    let dur : Vec<Duration> = inp.iter().map(|a| a.t).collect();

    let (dates, times) = inp.iter().map(|a| (a.h.unwrap().date(), a.h.unwrap().time())).unzip();
    (dur, times, dates)
}

// Builder of Hstats and Tstats, all in one in order to avoid redundancy
fn ht_new(durs: &mut Vec<Duration>, times: &Vec<NaiveTime>) -> (Tstats, Hstats) {
    let [n, sum, mean, std, corr] = stats_t(durs, times);
    let n = n as i32;
    let hist = t_hist(&durs, n);
    durs.sort();
    let med: Duration;
    if n == 1 || n == 2 {
        med = durs[0];
    } else {
        med = match n%2 {
            1 => durs[(n/2+1) as usize],
            0 => (durs[(n/2) as usize]+durs[(n/2+1) as usize])/2,
            _ => Duration::zero(),
        };
    }
    (Tstats {
        sum: Duration::seconds(sum as i64),
        median: med,
        mean: Duration::seconds(mean as i64),
        std: Duration::seconds(std as i64),
        corr_h: corr,
        hist,
    }, Hstats {
        hist: h_hist(&times)
    })

}


// MyHistograms

// Check bin number appropriateness
fn t_hist(inp: &Vec<Duration>, n: i32) -> MyHistogram<Duration> {
    let tmax = inp.iter().max().unwrap();
    let tmin = inp.iter().min().unwrap();
    let n_bins = (1.0+3.322*(n as f64).log10()).floor() as i32;
    if n_bins == 0 { return MyHistogram {
        bins: vec![*tmax],
        count: vec![inp.len() as u32],
    }}
    let div = (*tmax - *tmin) / n_bins;
    let bins: Vec<Duration> = (0..n_bins).map(|a| div*a+*tmin).collect();
    let mut count = vec![0_u32;n_bins as usize];
    for i in inp.iter() {
        let eureka = bins.iter().enumerate().rev().find(|(_bucket,value)| value<=&i).unwrap().0;
        count[eureka]+=1;
    }
    MyHistogram{
        bins,
        count,
    }
}

fn h_hist(inp: &Vec<NaiveTime>) -> MyHistogram<NaiveTime> {
    let tmax = inp.iter().max().unwrap();
    let tmin = inp.iter().min().unwrap();
    let n_bins = (*tmax - *tmin).num_hours() as i32;
    if n_bins == 0 { return MyHistogram {
        bins: vec![*tmax],
        count: vec![inp.len() as u32],
    }}
    let bins: Vec<NaiveTime> = (0..n_bins).map(|a| *tmin + Duration::hours(1)*a).collect();
    let mut count = vec![0_u32;n_bins as usize];
    for i in inp.iter() {
        let eureka = bins.iter().enumerate().rev().find(|(_bucket,value)| value<=&i).expect(&format!("Errore {:?}",i)).0;
        count[eureka]+=1;
    }
    MyHistogram{
        bins,
        count,
    }
}

// Calcuates statistics for times
fn stats_t(ts: &Vec<Duration>, hs: &Vec<NaiveTime>) -> [f64; 5] {
    let n = ts.len() as f64;
    let ts: Vec<f64> = ts.iter().map(|t| f64::from(t.num_seconds() as i32)).collect();
    let hs: Vec<f64> = hs.iter().map(|h| (h.hour() * 3600 + h.minute()*60 +h.second()) as f64).collect();
    let sum_t: f64 = ts.iter().sum();
    let mean_t: f64 = sum_t/n;
    let mut std_t: f64 = ts.iter().map(|h| h.powi(2)).sum::<f64>()/n-mean_t.powi(2);
    std_t = std_t.sqrt();
    let mean_h: f64 = hs.iter().sum::<f64>()/n;
    let mut std_h: f64 = hs.iter().map(|h| h.powi(2)).sum::<f64>()/n-mean_h.powi(2);
    std_h = std_h.sqrt();
    let hs_norm: Vec<f64> = hs.iter().map(|h| (h-mean_h)/std_h).collect();
    let corr = ts.iter().zip(hs_norm.iter()).map(|(a,b)| a*b).sum::<f64>()/std_t/n;
    [n, sum_t, mean_t, std_t, corr]

}

// Results in [median, mean, std]
fn chart_stats(inp: &Vec<Duration>) -> [Duration; 3] {
    let n = inp.len() as f64;
    if n==1.0 {
        return [inp[0],inp[0],Duration::seconds(0)]
    }
    let ts: Vec<f64> = inp.iter().map(|t| f64::from(t.num_seconds() as i32)).collect();
    let sum_t: f64 = ts.iter().sum();
    let mean_t: f64 = sum_t/n;
    let mut std_t: f64 = ts.iter().map(|h| h.powi(2)).sum::<f64>()/n-mean_t.powi(2);
    std_t = std_t.sqrt();
    let mut inp_sort = inp.clone();
    inp_sort.sort();
    let n = n as i32;
    let med: Duration;
    if n==1 || n==2 {
        med = inp[0];
    } else {
        med = match n%2 {
            1 => inp_sort[(n/2+1) as usize],
            0 => (inp_sort[(n/2) as usize]+inp_sort[(n/2+1) as usize])/2,
            _ => Duration::zero(),
        };
    }
    [med, Duration::seconds(mean_t as i64), Duration::seconds(std_t as i64)]
}
