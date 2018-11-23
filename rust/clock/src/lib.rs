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
        Clock::new(self.hours + Clock::roll_over_hours(minutes, self.minutes),
                    Clock::roll_over_minutes(self.minutes + minutes))
    }

    pub fn to_string(self) -> String {
        format!("{}:{}", Clock::stringify_time(self.hours),
                Clock::stringify_time(self.minutes))
    }

    fn roll_over_hours(minutes: i32, previous_minutes: i32) -> i32 {
       match previous_minutes + minutes < 0  {
           false => (previous_minutes + minutes) / 60,

           true => (previous_minutes + minutes) / 60 -1
       }
    }

    fn mathematical_modulo(n: i32, m: i32) -> i32 {
       // equivalent to mod in haskell or % in python
       let mut rem = n % m;
        if ((rem < 0) && (m > 0)) || (rem > 0) && (m < 0) {
            rem += m
        }
        rem
    }

    fn initiate_hours(hours: i32, minutes: i32) -> i32 {
        Clock::mathematical_modulo(hours + Clock::roll_over_hours(minutes, 0), 24)

    }

    fn roll_over_minutes(minutes: i32) -> i32 {
        Clock::mathematical_modulo(minutes, 60)
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
