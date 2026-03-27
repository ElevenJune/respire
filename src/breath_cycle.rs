#[derive(Clone,Copy,PartialEq)]
pub struct BreathCycle{
    inhale_duration : u16,
    break1_duration : u16,
    exhale_duration : u16,
    break2_duration : u16
}

#[derive(Clone,Copy,PartialEq)]
#[repr(usize)] 
pub enum CycleState {
    Inhale,
    Break1,
    Exhale,
    Break2,
    None
}

impl CycleState{
    pub fn next(&self) -> CycleState{
        match self {
            CycleState::Inhale => CycleState::Break1,
            CycleState::Break1 => CycleState::Exhale,
            CycleState::Exhale => CycleState::Break2,
            CycleState::Break2 => CycleState::Inhale,
            CycleState::None => CycleState::None
        }
    }

    pub fn roll(&mut self){
        *self = match *self {
            CycleState::Inhale => CycleState::Break1,
            CycleState::Break1 => CycleState::Exhale,
            CycleState::Exhale => CycleState::Break2,
            CycleState::Break2 => CycleState::Inhale,
            CycleState::None => CycleState::None
        };
    }

    pub fn to_string(&self) -> &str{
        match self {
            CycleState::Inhale => "Inhale",
            CycleState::Break1 => "Hold1",
            CycleState::Exhale => "Exhale",
            CycleState::Break2 => "Hold2",
            CycleState::None => ""
        }
    }

    pub fn is_break(&self)->bool{
        self==&CycleState::Break1||self==&CycleState::Break2
    }
}

impl BreathCycle{
    pub fn new(inhale_duration : u16, break1_duration : u16, exhale_duration : u16, break2_duration : u16) -> Self{
        BreathCycle{
            inhale_duration,
            break1_duration,
            exhale_duration,
            break2_duration
            //details : format!("{}-{}-{}-{}",inhale_duration,break1_duration,exhale_duration,break2_duration)
        }
    }

    
    //Setters
    pub fn set_inhale_duration(&mut self, duration : u16){
        self.inhale_duration = duration;
    }
    pub fn set_break1_duration(&mut self, duration : u16){
        self.break1_duration = duration;
    }
    pub fn set_exhale_duration(&mut self, duration : u16){
        self.exhale_duration = duration;
    }
    pub fn set_break2_duration(&mut self, duration : u16){
        self.break2_duration = duration;
    }
    pub fn set_state_duration(&mut self, cycle_state: &CycleState, duration : u16){
        match cycle_state {
            CycleState::Inhale => self.set_inhale_duration(duration),
            CycleState::Break1 => self.set_break1_duration(duration),
            CycleState::Exhale => self.set_exhale_duration(duration),
            CycleState::Break2 => self.set_break2_duration(duration),
            CycleState::None => {},
        }
    }
    pub fn increment_state_duration(&mut self, cycle_state: &CycleState, step : u16){
        let current_duration = self.get_state_duration(cycle_state);
        self.set_state_duration(cycle_state, current_duration+step);
    }
    
    //Getters
    pub fn total_cycle_duration(&self) -> u16{
        self.inhale_duration + self.break1_duration + self.exhale_duration + self.break2_duration
    }
    pub fn to_str(&self) -> &str{
        //&self.details
        "t"
    }
    pub fn inhale_duration(&self) -> u16 {self.inhale_duration}
    pub fn break1_duration(&self) -> u16 {self.break1_duration}
    pub fn exhale_duration(&self) -> u16 {self.exhale_duration}
    pub fn break2_duration(&self) -> u16 {self.break2_duration}
    pub fn get_state_duration(& self, cycle_state: &CycleState) -> u16{
        match cycle_state {
            CycleState::Inhale => self.inhale_duration,
            CycleState::Break1 => self.break1_duration,
            CycleState::Exhale => self.exhale_duration,
            CycleState::Break2 => self.break2_duration,
            CycleState::None => 0,
        }
    }
}