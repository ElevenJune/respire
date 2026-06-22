use crate::breath_cycle::{BreathCycle, CycleState};
use crate::sink_handle::SinkHandle;

pub struct BreathManager{
    cycles: Vec<BreathCycle>,
    current_cycle_index : Option<usize>,
    current_cycle_state : CycleState,
    current_duration : u16,

    sound_enabled : bool,
    play_sound_next : bool,
    sound_inhale : SinkHandle,
    sound_exhale : SinkHandle,
    sound_hold : SinkHandle
}

impl BreathManager{
    pub fn new() -> Self{
        let mut bm = BreathManager {
            cycles: vec!(),
            current_cycle_index:None,
            current_cycle_state : CycleState::Inhale,
            current_duration : 0,
            sound_enabled : false,
            play_sound_next : true,
            sound_inhale : SinkHandle::new(),
            sound_exhale : SinkHandle::new(),
            sound_hold : SinkHandle::new(),
        };

        bm.populate();
        bm.set_sounds();

        if !bm.cycles.is_empty() {
            bm.current_cycle_index = Some(0);
        }

        bm
    }

    //Getters
    pub fn current_cycle_state(&self) -> CycleState {self.current_cycle_state}
    pub fn current_duration(&self) -> u16 {self.current_duration}
    pub fn current_cycle_index(&self) -> Option<usize> {self.current_cycle_index}
    #[allow(dead_code)]
    pub fn cycles(&self) -> &Vec<BreathCycle> {&self.cycles}

    pub fn current_cycle(&self) -> Option<&BreathCycle> {
        if let Some(index) = self.current_cycle_index {
            self.cycles.get(index)
        } else {
            None
        }
    }

    //Setters
    pub fn increment_current_cycle_state_duration(&mut self, cycle_state: &CycleState, step: i16){
        if let Some(index) = &self.current_cycle_index {
            self.increment_cycle_state_duration(*index, cycle_state, step);
        }
    }
    pub fn increment_cycle_state_duration(&mut self, cycle_index:usize, cycle_state: &CycleState, step: i16){
        let selected_cycle : Option<&mut BreathCycle> = self.cycles.get_mut(cycle_index);
        if let Some(cycle) = selected_cycle{
            cycle.increment_state_duration(cycle_state, step);
        }
    }

    //Actions
    pub fn set_current_cycle_index(&mut self, index: usize){
        if index >= self.cycles.len() {self.current_cycle_index=None;}
        else {self.current_cycle_index=Some(index);}
        self.reset_cycle_state();
        
    }

    pub fn reset_cycle_state(&mut self){
        self.current_cycle_state=CycleState::Inhale;
        self.current_duration=0;

        self.sound_exhale.stop();
        self.sound_hold.stop();
        self.sound_inhale.stop();
        //if self.current_cycle().is_some() {self.sound_inhale.play_once()}
    }

    pub fn toggle_sound_enabled(&mut self){
        self.sound_enabled = ! self.sound_enabled;
    }

    pub fn update_cycle(&mut self, millis_spent: u16) {
        if let Some(current_cycle) = self.current_cycle().copied() {
            if !current_cycle.is_valid() {return;}
            let current_state_duration = current_cycle.get_state_duration(&self.current_cycle_state);

            //Start cycle behaviour
            if self.sound_enabled 
            && self.play_sound_next
            && current_state_duration>=1000{
                match self.current_cycle_state {
                    CycleState::Inhale => self.sound_inhale.play_once(),
                    CycleState::Exhale => self.sound_exhale.play_once(),
                    _ => {self.sound_hold.play_once()}
                }
                self.play_sound_next = false;
            }
            self.current_duration=self.current_duration.saturating_add(millis_spent);

            //End cycle behaviour
            if self.current_duration>current_state_duration {
                let mut next_state = self.current_cycle_state.next();
                while current_cycle.get_state_duration(&next_state) == 0
                && next_state != CycleState::None {
                    next_state.roll();
                }
                self.current_cycle_state=next_state;
                self.play_sound_next = true;
                self.current_duration=self.current_duration.saturating_sub(current_state_duration);
            }
        }
    }

    //Private
    fn populate(&mut self) {
        self.cycles.push(BreathCycle::new(5000, 0, 5000, 0, "Cardiac Coherence"));
        self.cycles.push(BreathCycle::new(4000, 0, 6000, 0, "Long Exhalation"));
        self.cycles.push(BreathCycle::new(4000, 4000, 4000, 4000, "Squared Breathing"));
        self.cycles.push(BreathCycle::new(4000, 7000, 8000, 0, "Triangular Breathing"));
        self.cycles.push(BreathCycle::new(1000, 1000, 1000, 1000, "Custom"));
    }

    fn set_sounds(&mut self){
        self.sound_inhale.set_source_as_filepath("./sounds/laurainhale.mp3");
        self.sound_inhale.set_volume(0.5);
        self.sound_exhale.set_source_as_filepath("./sounds/lauraexhale.mp3");
        self.sound_exhale.set_volume(0.5);
        self.sound_hold.set_source_as_filepath("./sounds/laurahold.mp3");
        self.sound_hold.set_volume(0.5);
    }

    fn _push_cycle(&mut self, bc : &BreathCycle){
        self.cycles.push(*bc);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_ok(){
        let mut bm = BreathManager::new();
        if bm.cycles().len() == 0 {return;}

        bm.set_current_cycle_index(usize::MAX);
        assert_eq!(bm.current_cycle().is_none(),true);
        bm.set_current_cycle_index(0);
        assert_eq!(bm.current_cycle().is_some(),true);

        let cycle = bm.current_cycle().unwrap();
        assert_eq!(cycle.inhale_duration(),5000);
        assert_eq!(cycle.exhale_duration(),5000);
    }

    #[test]
    fn run_ok(){
        let mut bm = BreathManager::new();
        let test_cycle = BreathCycle::new(5000,0,5000,0,"Test");
        bm._push_cycle(&test_cycle);
        bm.set_current_cycle_index(bm.cycles().len()-1);
        let inhale_d = test_cycle.inhale_duration();

        assert_eq!(bm.current_cycle_state(),CycleState::Inhale);
        assert_eq!(test_cycle.inhale_duration(),5000);
        assert_eq!(test_cycle.exhale_duration(),5000);
        assert_eq!(test_cycle.break1_duration(),0);
        assert_eq!(test_cycle.break2_duration(),0);
        bm.update_cycle(inhale_d+100);
        assert_eq!(bm.current_cycle_state(),CycleState::Exhale);
        bm.update_cycle(inhale_d+100);
        assert_eq!(bm.current_cycle_state(),CycleState::Inhale);
    }

    #[test]
    fn test_increment(){
        let mut bm = BreathManager::new();
        let test_cycle = BreathCycle::new(5000,0,5000,0,"Test");
        bm._push_cycle(&test_cycle);
        bm.set_current_cycle_index(bm.cycles().len()-1);
        bm.increment_current_cycle_state_duration(&CycleState::Break1, 777);
        bm.increment_current_cycle_state_duration(&CycleState::Break2, 1000);
        bm.increment_current_cycle_state_duration(&CycleState::Break2, -500);
        if let Some(cycle) = bm.current_cycle() {
            assert_eq!(cycle.break1_duration(),777);
            assert_eq!(cycle.break2_duration(),500);
        }
    }

}