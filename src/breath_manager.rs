use crate::breath_cycle::{BreathCycle, CycleState};

pub struct BreathManager{
    cycles: Vec<BreathCycle>,
    current_cycle_index : Option<usize>,
    current_cycle_state : CycleState,
    current_duration : i16
}

impl BreathManager{
    pub fn new() -> Self{
        let mut bm = BreathManager {
            cycles: vec!(),
            current_cycle_index:None,
            current_cycle_state : CycleState::Inhale,
            current_duration : 0,
        };
        bm.populate();
        if bm.cycles.len() > 0 {
            bm.current_cycle_index = Some(0);
        }
        bm
    }

    //Getters
    pub fn cycles(&self) -> &Vec<BreathCycle> {&self.cycles}
    pub fn current_cycle_state(&self) -> CycleState {self.current_cycle_state}
    pub fn current_duration(&self) -> i16 {self.current_duration}

    pub fn current_cycle(&self) -> Option<&BreathCycle> {
        if let Some(index) = self.current_cycle_index {
            self.cycles.get(index)
        } else {
            None
        }
    }


    //Actions
    pub fn update_cycle(&mut self, millis_spent: i16) {
        if let Some(index) = self.current_cycle_index {
            let current_cycle = self.cycles()[index];

            self.current_duration+=millis_spent;
            let current_state_duration = current_cycle.get_state_duration(&self.current_cycle_state);

            if self.current_duration>current_state_duration {
                self.current_duration=0;
                self.current_cycle_state=self.current_cycle_state.next();
            }
        }
    }

    //Private
    fn populate(&mut self) {
        self.cycles.push(BreathCycle::new(1000, 1000, 1000, 1000));
        self.cycles.push(BreathCycle::new(6000, 0, 6000, 0));
        self.cycles.push(BreathCycle::new(4000, 4000, 4000, 4000));
    }

}