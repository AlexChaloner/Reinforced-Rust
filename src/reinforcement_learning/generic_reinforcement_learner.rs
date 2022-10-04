// Reinforcement learning module

use rand::Rng;

pub trait Action {

}

pub trait State {

}

pub trait Policy {
  fn get_action(state: &State) {}
}

pub trait ReinforcementLearner {
  fn get_action_value(state: &State, action: &Action) {}
  fn update_action_value(state: &State, action: &Action) {}
  fn get_state_value(state: &State) {}
  fn update_state_value() {}
}

