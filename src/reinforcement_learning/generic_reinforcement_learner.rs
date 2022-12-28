// Reinforcement learning module

use std::fmt::Display;
use core::hash::Hash;

pub trait State: Display + Eq + Hash + Clone
{
    type A: Action;

    fn initial_state() -> Self;
    fn next_state(&self, action: &Self::A) -> Self;
    fn is_terminal(&self) -> bool;
    fn available_actions(&self) -> Vec<Self::A>;
    fn num_available_actions(&self) -> usize {
        self.available_actions().len()
    }
    fn get_reward(state: &Self, action: &Self::A, next_state: &Self) -> f64;
}


pub trait Action: Display + Eq + Hash + Clone {
}


pub trait Policy<S>
where 
    S: State
{
    fn get_action(&self, values: &mut Vec<(S::A, f64)>) -> S::A;
}


pub trait ReinforcementLearner<S> 
where 
    S: State
{
    fn get_action_value(&self, state: &S, action: &S::A) -> f64;
    fn get_action_values(&self, state: &S) -> Vec<(S::A, f64)>;
    fn update_action_value(&mut self, state: &S, action: &S::A, next_state: &S, reward: f64);
    fn get_state_value(&self, state: &S) -> f64;
    fn update_state_value(&mut self, state: &S, value: f64);
    fn get_best_action(&self, state: &S) -> S::A;
}
