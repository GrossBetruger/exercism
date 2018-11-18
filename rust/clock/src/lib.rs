#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: Clock::initiate_hours(hours, minutes),
            minutes: Clock::roll_over_minutes(minutes)
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        Clock::new(self.hours + Clock::roll_over_hours(minutes), self.minutes + Clock::roll_over_minutes(minutes))
    }

    pub fn to_string(self) -> String {
        format!("{}:{}", Clock::stringify_time(self.hours),
                Clock::stringify_time(self.minutes))
    }


    fn roll_over_hours(minutes: i32) -> i32 {
        match minutes >= 0 {
            true => minutes / 60,
            false => {
                match minutes / 60 {
                    0 => -1,
                    _ => (minutes / 60) -1
                }
            }
        }

    }

    fn initiate_hours(hours: i32, minutes: i32) -> i32 {
        match hours >= 0 {
            true => (24 + hours + Clock::roll_over_hours(minutes)) % 24,
            false => 24 + ((hours + Clock::roll_over_hours(minutes)) % 24)
        }
    }

    fn roll_over_minutes(minutes: i32) -> i32 {
        match minutes >= 0 {
            true => minutes % 60,
            false => 60 + (minutes % 60)
        }
    }


    fn stringify_time(time_representation: i32) -> String {
        match time_representation.to_string().len() {
            1 => {
                ("0".to_owned() + &time_representation.to_string()).into()
            }

            _ => time_representation.to_string()
        }
    }

}
