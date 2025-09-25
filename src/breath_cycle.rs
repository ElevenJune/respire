#[derive(Clone,Copy,PartialEq)]
pub struct BreathCycle{
    inhale_duration : i16,
    break1_duration : i16,
    exhale_duration : i16,
    break2_duration : i16,
}

#[derive(Clone,Copy,PartialEq)]
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

    pub fn is_break(&self)->bool{
        self==&CycleState::Break1||self==&CycleState::Break2
    }
}

impl BreathCycle{
    pub fn new(inhale_duration : i16, break1_duration : i16, exhale_duration : i16, break2_duration : i16) -> Self{
        BreathCycle{
            inhale_duration,
            break1_duration,
            exhale_duration,
            break2_duration
        }
    }

    
    //Setters
    pub fn set_inhale_duration(&mut self, duration : i16){
        self.inhale_duration = duration;
    }
    pub fn set_break1_duration(&mut self, duration : i16){
        self.break1_duration = duration;
    }
    pub fn set_exhale_duration(&mut self, duration : i16){
        self.exhale_duration = duration;
    }
    pub fn set_break2_duration(&mut self, duration : i16){
        self.break2_duration = duration;
    }
    
    //Getters
    pub fn total_cycle_duration(&self) -> i16{
        self.inhale_duration + self.break1_duration + self.exhale_duration + self.break2_duration
    }
    pub fn inhale_duration(&self) -> i16 {self.inhale_duration}
    pub fn break1_duration(&self) -> i16 {self.break1_duration}
    pub fn exhale_duration(&self) -> i16 {self.exhale_duration}
    pub fn break2_duration(&self) -> i16 {self.break2_duration}
    pub fn get_state_duration(& self, cycle_state: &CycleState) -> i16{
        match cycle_state {
            CycleState::Inhale => self.inhale_duration,
            CycleState::Break1 => self.break1_duration,
            CycleState::Exhale => self.exhale_duration,
            CycleState::Break2 => self.break2_duration,
            CycleState::None => 0,
        }
    }
}