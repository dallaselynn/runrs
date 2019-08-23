extern crate rand;

use rand::Rng;


/// calculate x percentage of some number
/// ```
/// assert_eq!(runrs::percentage_of(100.0, 1.0), 1.0);
/// assert_eq!(runrs::percentage_of(100.0, 100.0), 100.0);
/// assert_eq!(runrs::percentage_of(100.0, 200.0), 200.0);
/// ```
pub fn percentage_of(of: f32, percent: f32) -> f32 {
    (percent / 100.0) * of
}

#[derive(Default)]
pub struct LifeAdjustments {
    pub is_married: bool,
    pub minor_children: f32,
    pub hours_worked_per_week: f32,
    pub age: f32,
    pub pounds_overweight: f32,
    pub following_plan: bool,
    pub alcohol_units_prior: f32,
    pub had_pacer: bool,
    pub east_african: bool,
    pub years_as_runner: f32,
    pub number_of_medications: f32,
    pub destination_race: bool,
    pub religious: bool,
    pub very_attractive: bool,
    pub weeks_of_training: f32,
    pub has_coach: bool,
    pub percentage_of_workouts_posted: f32,
    pub miles_travelled_to_race: f32,
    pub temperature_in_farenheit: f32,
}

///
/// ```
/// let la = runrs::LifeAdjustments::new();
/// assert_eq!(la.age, 0)
/// ```
impl LifeAdjustments {
    pub fn new() -> LifeAdjustments {
        LifeAdjustments {
            ..Default::default()
        }
    }

    /// Return the life adjusted time in seconds.
    pub fn calculate(&self, base_time_in_seconds: u32) -> u32 {
        let base_time = base_time_in_seconds as f32;
        let adjustment_seconds =
            self.is_married_adjustment(base_time) +
            self.minor_children_adjustment(base_time) +
            self.hours_worked_per_week_adjustment(base_time) +
            self.age_adjustment(base_time) +
            self.pounds_overweight_adjustment(base_time) +
            self.following_plan_adjustment(base_time) +
            self.alcohol_units_prior_adjustment(base_time) +
            self.had_pacer_adjustment(base_time) +
            self.east_african_adjustment(base_time) +
            self.years_as_runner_adjustment(base_time) +
            self.number_of_medications_adjustment(base_time) +
            self.destination_race_adjustment(base_time) +
            self.very_attractive_adjustment(base_time) +
            self.weeks_of_training_adjustment(base_time) +
            self.has_coach_adjustment(base_time) +
            self.percentage_of_workouts_posted_adjustment(base_time) +
            self.miles_travelled_to_race_adjustment(base_time);

        (base_time - adjustment_seconds) as u32
    }

    /// nothing for up to 1000 miles, 1% per 1000 miles after that
    pub fn miles_travelled_to_race_adjustment(&self, base_time: f32) -> f32 {
        match self.miles_travelled_to_race as u32 {
            0 ... 1000 => 0.0,
            _ => percentage_of(base_time, -(self.miles_travelled_to_race / 1000.0))
        }
    }

    /// What percent of workouts for this training cycle were posted to social media?
    pub fn percentage_of_workouts_posted_adjustment(&self, base_time: f32) -> f32 {
        match self.percentage_of_workouts_posted as u32 {
            0 => percentage_of(base_time, -1.0),
            1 ... 20 => percentage_of(base_time, 0.5),
            21 ... 50 => percentage_of(base_time, 0.8),
            51 ... 75 => percentage_of(base_time, 0.9),
            _ => percentage_of(base_time, 1.01)
        }
    }

    pub fn has_coach_adjustment(&self, base_time: f32) -> f32 {
        if self.has_coach { percentage_of(base_time, 2.0) } else { 0.00 }
    }

    pub fn weeks_of_training_adjustment(&self, base_time: f32) -> f32 {
        match self.weeks_of_training as u32 {
            0 ... 11 => percentage_of(base_time, -2.03),
            12 ... 18 => percentage_of(base_time, -1.02),
            _ => 0.00
        }
    }

    pub fn very_attractive_adjustment(&self, base_time: f32) -> f32 {
        if self.very_attractive { percentage_of(base_time, 1.05) } else { 0.00 }
    }

    pub fn destination_race_adjustment(&self, base_time: f32) -> f32  {
        if self.destination_race { percentage_of(base_time, -0.50) } else { 0.00 }
    }

    pub fn number_of_medications_adjustment(&self, base_time: f32) -> f32 {
        percentage_of(base_time, self.number_of_medications * -0.1)
    }

    pub fn years_as_runner_adjustment(&self, base_time: f32) -> f32 {
        match self.years_as_runner as u32 {
            0 ... 3 => percentage_of(base_time, -1.02),
            4 ... 9 => percentage_of(base_time, -0.20),
            _ => 0.00
        }
    }

    pub fn east_african_adjustment(&self, base_time: f32) -> f32 {
        if self.east_african { percentage_of(base_time, 0.96) } else { 0.00 }
    }

    pub fn had_pacer_adjustment(&self, base_time: f32) -> f32 {
        if self.had_pacer { percentage_of(base_time, 0.25) } else { 0.00 }
    }

    pub fn alcohol_units_prior_adjustment(&self, base_time: f32) -> f32 {
        percentage_of(base_time, -(self.alcohol_units_prior * 0.07))
    }

    pub fn following_plan_adjustment(&self, base_time: f32) -> f32 {
        if self.following_plan { 0.00 } else { percentage_of(base_time, -0.15) }
    }

    pub fn pounds_overweight_adjustment(&self, base_time: f32) -> f32 {
        percentage_of(base_time, -(self.pounds_overweight * 0.02))
    }

    pub fn age_adjustment(&self, base_time: f32) -> f32 {
        match self.age as u32 {
            1 ... 30 => percentage_of(base_time, 0.95),
            31 ... 41 => percentage_of(base_time, 0.25),
            42 ... 50 => percentage_of(base_time, -0.75),
            51 ... 60 => percentage_of(base_time, -1.25),
            61 ... 70 => percentage_of(base_time, -2.25),
            71 ... 110 => percentage_of(base_time, -4.25),
            _ => 0.00
        }
    }

    pub fn hours_worked_per_week_adjustment(&self, base_time: f32) -> f32 {
        percentage_of(base_time, -(self.hours_worked_per_week * 0.02))
    }

    pub fn is_married_adjustment(&self, base_time: f32) -> f32 {
        let adjustment = percentage_of(base_time, 0.90);
        if self.is_married { adjustment * -1.0 } else { adjustment }
    }

    pub fn minor_children_adjustment(&self, base_time: f32) -> f32 {
        percentage_of(base_time, -(self.minor_children * 0.77))
    }

    /// Return formatted life adjusted time.
    pub fn life_adjusted_time(&self, base_time: u32) -> String {
        seconds_to_time(self.calculate(base_time).into())
    }

    /// Return formatted life adjusted time.
    pub fn life_adjusted_time_from_string(&self, base_time_string: &str) -> String {
        let base_time = time_to_seconds(base_time_string).unwrap() as u32;
        seconds_to_time(self.calculate(base_time).into())
    }

}

/// Convert human readable time to seconds
/// ```
/// assert_eq!(runrs::time_to_seconds(&"3:45:00".to_owned()), Ok(13500));
/// ```
pub fn time_to_seconds(t: &str) -> Result<u64, String> {
    let pieces: Vec<u64> = t
        .split(':')
        .map(|x| x.parse::<u64>())
        .filter_map(Result::ok)
        .collect();
    if pieces.len() != 3 {
        return Err("Invalid format".to_string());
    }
    // currently rust can only destructure a vector
    let time = (pieces[0] * 3600) + (pieces[1] * 60) + pieces[2];

    Ok(time)
}

/// Convert a number of seconds to HH:MM:SS
///
/// ```
/// assert_eq!(runrs::seconds_to_time(60), "00:01:00".to_owned());
/// assert_eq!(runrs::seconds_to_time(0), "00:00:00".to_owned());
/// ```
pub fn seconds_to_time(seconds: u64) -> String {
    let mut s = seconds;
    let hours = s / 3600;
    s -= hours * 3600;
    let minutes = s / 60;
    let secs = s % 60;

    return format!("{:02}:{:02}:{:02}", hours, minutes, secs);
}

/// Calculate how much a race costs per unit.
pub fn cost_per_unit(price: f64, distance: f64) -> f64 {
    // weird rust way to round to 2 decimal places
    ((price / distance) * 100.0).round() / 100.0
}

/// Given a previous race's time and distance, predict the finish time of a future race of a given distance.
/// Formula is
/// T2 = T1 x (D2/D1)1.06
/// where T1 is the given time, D1 is the given distance, D2 is the distance to
/// predict a time for, and T2 is the calculated time for D2.
pub fn predicted_finish_time(
    previous_race_time: u32,
    previous_race_distance: f64,
    next_race_distance: f64
) -> f64 {
    let dist = next_race_distance / previous_race_distance;
    (f64::from(previous_race_time) * dist.powf(1.06)).ceil()
}

/// Calculate the pace per unit required to run the target pace.
/// ie. to run a 3:00:00 hour marathon (26.2 miles) you have to run
///
/// ```
/// assert_eq!(runrs::target_pace(&"3:00:00".to_owned(), 26.2), Ok("00:06:52".to_owned()));
/// assert_eq!(runrs::target_pace(&"3:00:00".to_owned(), 42.125), Ok("00:04:16".to_owned()));
/// ```
pub fn target_pace(finish_time: &str, distance: f64) -> Result<String, String> {
    let finish_seconds = time_to_seconds(&finish_time)?;
    let seconds_per_unit = (finish_seconds as f64 / distance).floor();

    Ok(seconds_to_time(seconds_per_unit as u64))
}

/// Return a vector of strings that represents a chart of what time a runner
/// should should cross each unit
pub fn pace_chart(finish_time: &str, distance: f64) -> Vec<String> {
    /* pace chart return */
    let mut paces: Vec<String> = Vec::new();
    /* seconds to finish */
    let finish_seconds = time_to_seconds(&finish_time).unwrap();
    /* seconds per unit of distance */
    let seconds_per_unit = finish_seconds as f64 / distance;

    let mut i = seconds_per_unit as u64;
    while i < finish_seconds {
        paces.push(seconds_to_time(i));
        i += seconds_per_unit as u64;
    }

    paces
}

const STRATEGIES: [&str; 5] = [
    "Go out at your target pace and hold it the whole race.",
    "Go out too fast and hold on as long as you can.",
    "Just have fun out there.",
    "Go out hard and hit the wall.",
    "Your entire self-worth is based on getting a PR...but have fun out there."
];

pub fn race_strategy() -> &'static str {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0, STRATEGIES.len());

    STRATEGIES[idx]
}
