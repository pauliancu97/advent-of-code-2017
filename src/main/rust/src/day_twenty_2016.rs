use crate::utils::read_lines;


#[derive(Clone)]
struct Interval {
    start: u64,
    end: u64
}

impl Interval {
    fn from_string(string: &str) -> Option<Interval> {
        let mut split = string.split('-');
        let start = split.nth(0)?.parse::<u64>().ok()?;
        let end = split.nth(0)?.parse::<u64>().ok()?;
        Some(Interval { start, end })
    }
}

fn get_first_interval<'a>(intervals: &'a[Interval]) -> (Interval, &'a[Interval]) {
    let mut final_end = intervals[0].end;
    let mut index: usize = 1;
    while index < intervals.len() && intervals[index].start <= final_end + 1 {
        final_end = std::cmp::max(final_end, intervals[index].end);
        index += 1;
    }
    let first_interval = Interval { start: intervals[0].start, end: final_end };
    (first_interval, &intervals[index..])
}

fn get_merged_intervals(intervals: &[Interval]) -> Vec<Interval> {
    let mut merged_intervals: Vec<Interval> = Vec::new();
    let mut current_intervals = intervals;
    while !current_intervals.is_empty() {
        let (merged_interval, updated_intervals) = get_first_interval(current_intervals);
        merged_intervals.push(merged_interval);
        current_intervals = updated_intervals;
    }
    merged_intervals
}

fn get_final_intervals(intervals: &[Interval]) -> Vec<Interval> {
    let mut final_intervals = intervals.to_vec();
    final_intervals.sort_by(|first, second| first.start.cmp(&second.start));
    get_merged_intervals(&final_intervals)
}

fn read_intervals(path: &str) -> Vec<Interval> {
    read_lines(path)
        .map_or(
            vec![],
            |strings| {
                strings
                    .iter()
                    .filter_map(|string| Interval::from_string(string))
                    .collect()
            }
        )
}

fn get_num_blocked_ips(intervals: &[Interval]) -> u64 {
    intervals.iter()
        .map(|interval| interval.end - interval.start + 1)
        .sum()
}

fn get_num_allowed_ips(intervals: &[Interval]) -> u64 {
    4294967296u64 - get_num_blocked_ips(intervals)
}

pub fn solve_part_one() {
    let intervals = read_intervals("day_twenty_2016.txt");
    let intervals = get_final_intervals(&intervals);
    let opt_answer = intervals.first().map(|interval| interval.end + 1);
    if let Some(answer) = opt_answer {
        println!("{}", answer);
    }
}

pub fn solve_part_two() {
    let intervals = read_intervals("day_twenty_2016.txt");
    let intervals = get_final_intervals(&intervals);
    println!("{}", get_num_allowed_ips(&intervals));
}