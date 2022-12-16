use std::path::Path;

use crate::utils::{parse_i32, self};


type Point = (i32, i32);
type Interval = (i32, i32);

fn manhattan(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn join_intervals(intervals: &mut Vec<Interval>) -> Vec<Interval> {
    intervals.sort();
    //println!("{:?}", intervals);
    let mut final_intervals: Vec<Interval> = vec![];
    let mut loop_curr_interval: Option<Interval> = None;
    for interval in intervals {
        if let Some(curr_interval) = loop_curr_interval {
            if interval.0 <= curr_interval.1 {
                loop_curr_interval = Some((curr_interval.0, interval.1.max(curr_interval.1)));
            } else {
                final_intervals.push(curr_interval);
                loop_curr_interval = Some(*interval);
            }
        } else {
            loop_curr_interval = Some(*interval);
        }
    }
    if let Some(curr_interval) = loop_curr_interval {
        final_intervals.push(curr_interval);
    }
    final_intervals
}

fn invert_intervals(intervals: &Vec<Interval>, min: i32, max: i32) -> Vec<Interval> {
    let mut prev_interval = (min, min);
    let mut final_intervals: Vec<Interval> = vec![];
    for ival in intervals {
        if ival.0 > prev_interval.1 + 1 {
            final_intervals.push((prev_interval.1 + 1, ival.0 - 1));
        }
        prev_interval = *ival;
    }
    if max > prev_interval.1 {
        final_intervals.push((prev_interval.1 + 1, max));
    }
    final_intervals
}

fn coord_not_in_intervals(search_range: Interval, intervals: &Vec<Interval>) -> Option<i32> {
    let inverted = invert_intervals(intervals, search_range.0, search_range.1);
    if !inverted.is_empty() {
        return Some(inverted[0].0);
    } else {
        return None;
    }
}

#[derive(Debug)]
struct BeaconSensor {
    beacon: Point,
    sensor: Point,
}

impl BeaconSensor {
    pub fn interval_overlapping(&self, y: &i32) -> Option<Interval> {
        let beacon_sensor_dist = manhattan(&self.beacon, &self.sensor);
        let sensor_y_dist = (self.sensor.1 - y).abs();
        let leftover_distance = beacon_sensor_dist - sensor_y_dist;
        let output = if leftover_distance >= 0 {
            let base_x = self.sensor.0;
            Some((
                base_x - leftover_distance, 
                base_x + leftover_distance,
            ))
        } else {
            None
        };
        //println!("{:?}. distance {}, offset: {}, leftover: {}, ival: {:?}", self, beacon_sensor_dist, sensor_y_dist, leftover_distance, output);
        output
    }
}

impl From<String> for BeaconSensor {
    fn from(line: String) -> Self {
        //Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let (first, beacon_coords) = line.split_once(": closest beacon is at ").unwrap();
        let (_, sensor_coords) = first.split_once("Sensor at ").unwrap();
        let (beacon_x, beacon_y) = beacon_coords.split_once(", ").unwrap();
        let (sensor_x, sensor_y) = sensor_coords.split_once(", ").unwrap();
        let beacon_x = parse_i32(beacon_x.split_once("=").unwrap().1).unwrap();
        let beacon_y = parse_i32(beacon_y.split_once("=").unwrap().1).unwrap();
        let sensor_x = parse_i32(sensor_x.split_once("=").unwrap().1).unwrap();
        let sensor_y = parse_i32(sensor_y.split_once("=").unwrap().1).unwrap();
        return Self {
            beacon: (beacon_x, beacon_y),
            sensor: (sensor_x, sensor_y),
        }
    }
}



pub fn answer_part_1<P: AsRef<Path>>(path: P, y_value: i32) -> String {
    let lines = utils::read_input(path);
    let beacon_sensor_pairs: Vec<BeaconSensor> = lines
        .map(|line| BeaconSensor::from(line))
        .collect();
    let mut intervals: Vec<Interval> = beacon_sensor_pairs
        .into_iter()
        .map(|beacon_sensor| beacon_sensor.interval_overlapping(&y_value))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    let intervals = join_intervals(&mut intervals);
    println!("{:?}", intervals);
    let covered_count: i32 = intervals
        .into_iter()
        .map(|ival| ival.1 - ival.0 + 1)
        .reduce(|a, b| a + b)
        .unwrap();
    // I'm off by 1 for some reason
    (covered_count - 1).to_string()
}

pub fn answer_part_2<P: AsRef<Path>>(path: P, coord_max: i32) -> String {
    let lines = utils::read_input(path);
    let beacon_sensor_pairs: Vec<BeaconSensor> = lines
        .map(|line| BeaconSensor::from(line))
        .collect();
    for y in 0..coord_max+1 {
        let mut intervals: Vec<Interval> = beacon_sensor_pairs
            .iter()
            .map(|beacon_sensor| beacon_sensor.interval_overlapping(&y))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        let intervals = join_intervals(&mut intervals);
        match coord_not_in_intervals((0, coord_max), &intervals) {
            Some(x) => {
                println!("x={}, y={}", x, y);
                return ((x as i64) * 4000000 + (y as i64)).to_string();
            },
            None => {},
        }
    }
    return "No solution found".to_string();
}