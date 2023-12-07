use std::ops::Range;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
pub struct RaceRecords {
    records: Vec<RaceRecord>,
}

#[derive(Debug)]
pub struct RaceRecord {
    time: u64,
    distance: u64,
}

#[derive(Debug, Error)]
pub enum RaceRecordParseError {
    #[error("Invalid time: {0}")]
    InvalidTime(String),
    #[error("Invalid distance: {0}")]
    InvalidDistance(String),
    #[error("Invalid input: missing times")]
    MissingTimes,
    #[error("Invalid input: missing distances")]
    MissingDistances,
    #[error("Invalid input: unexpected number of lines: {0} (expecting 2)")]
    UnexpectedNumberOfLines(usize),
    #[error("Mismatch in number of times ({0}) and distances ({1})")]
    MismatchedTimesAndDistances(usize, usize),
}

impl FromStr for RaceRecords {
    type Err = RaceRecordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() != 2 {
            return Err(RaceRecordParseError::UnexpectedNumberOfLines(lines.len()));
        }

        let times = lines[0];
        if !times.starts_with("Time: ") {
            return Err(RaceRecordParseError::MissingTimes);
        }
        let times = times[6..]
            .split_whitespace()
            .map(|time| {
                time.parse::<u64>()
                    .map_err(|_| RaceRecordParseError::InvalidTime(time.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let distances = lines[1];
        if !distances.starts_with("Distance: ") {
            return Err(RaceRecordParseError::MissingDistances);
        }

        let distances = distances[10..]
            .split_whitespace()
            .map(|distance| {
                distance
                    .parse::<u64>()
                    .map_err(|_| RaceRecordParseError::InvalidDistance(distance.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        if times.len() != distances.len() {
            return Err(RaceRecordParseError::MismatchedTimesAndDistances(
                times.len(),
                distances.len(),
            ));
        }

        let records = times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| RaceRecord { time, distance })
            .collect();
        Ok(RaceRecords { records })
    }
}

impl RaceRecord {
    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn distance(&self) -> u64 {
        self.distance
    }

    fn get_hold_times(&self) -> (f64, f64) {
        let time = self.time as f64;
        let distance = self.distance as f64;
        let sq = (time.powi(2) - 4.0 * distance).sqrt();
        let (a, b) = ((self.time as f64 + sq) / 2.0, (self.time as f64 - sq) / 2.0);

        if a > b {
            (b, a)
        } else {
            (a, b)
        }
    }

    pub fn get_winning_hold_times(&self) -> Range<u64> {
        let (hold1, hold2) = self.get_hold_times();
        let hold1 = self.bump_to_winner(hold1.ceil() as u64);
        let hold2 = self.bump_to_loser(hold2.floor() as u64);
        if hold1 > hold2 {
            hold2..hold1
        } else {
            hold1..hold2
        }
    }

    pub fn num_ways_to_beat_record(&self) -> u64 {
        let winning_hold_times = self.get_winning_hold_times();
        winning_hold_times.end - winning_hold_times.start
    }

    fn distance_covered(&self, hold_duration: u64) -> u64 {
        (self.time - hold_duration) * hold_duration
    }

    fn is_winner(&self, hold_duration: u64) -> bool {
        let distance_covered = self.distance_covered(hold_duration);
        distance_covered > self.distance
    }

    fn bump_to_winner(&self, hold_duration: u64) -> u64 {
        if self.is_winner(hold_duration) {
            hold_duration
        } else {
            hold_duration + 1
        }
    }

    fn bump_to_loser(&self, hold_duration: u64) -> u64 {
        if self.is_winner(hold_duration) {
            hold_duration + 1
        } else {
            hold_duration
        }
    }
}

impl RaceRecords {
    pub fn num_ways_to_beat_record(&self) -> u64 {
        self.records
            .iter()
            .map(|record| record.num_ways_to_beat_record())
            .product()
    }

    pub fn patch_bad_kerning(s: &str) -> String {
        s.lines()
            .filter_map(|l| l.split_once(": "))
            .map(|(k, v)| {
                (
                    k,
                    v.split_whitespace()
                        .collect::<Vec<_>>()
                        .join("")
                        .to_string(),
                )
            })
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    fn get_example_records() -> RaceRecords {
        RaceRecords {
            records: vec![
                RaceRecord {
                    time: 7,
                    distance: 9,
                },
                RaceRecord {
                    time: 15,
                    distance: 40,
                },
                RaceRecord {
                    time: 30,
                    distance: 200,
                },
            ],
        }
    }

    fn len(range: &Range<u64>) -> u64 {
        range.end - range.start
    }

    #[test]
    fn test_parse_example() {
        let records = EXAMPLE_INPUT.parse::<RaceRecords>().unwrap();
        assert_eq!(records.records.len(), 3);
        assert_eq!(records.records[0].time, 7);
        assert_eq!(records.records[0].distance, 9);
        assert_eq!(records.records[1].time, 15);
        assert_eq!(records.records[1].distance, 40);
        assert_eq!(records.records[2].time, 30);
        assert_eq!(records.records[2].distance, 200);
    }

    #[test]
    fn test_get_winning_hold_times() {
        let example = RaceRecord {
            time: 7,
            distance: 9,
        };

        let hold_times = example.get_winning_hold_times();
        assert_eq!(len(&hold_times), 4);
        assert_eq!(hold_times, 2..6)
    }

    #[test]
    fn test_get_winning_hold_times_2() {
        let example = RaceRecord {
            time: 15,
            distance: 40,
        };

        let hold_times = example.get_winning_hold_times();
        assert_eq!(len(&hold_times), 8);
        assert_eq!(hold_times, 4..12)
    }

    #[test]
    fn test_get_winning_hold_times_3() {
        let example = RaceRecord {
            time: 30,
            distance: 200,
        };

        let hold_times = example.get_winning_hold_times();
        assert_eq!(len(&hold_times), 9);
        assert_eq!(hold_times, 11..20)
    }

    #[test]
    fn test_num_ways_to_beat_record_example() {
        let records = get_example_records();
        assert_eq!(records.num_ways_to_beat_record(), 288);
    }
}
