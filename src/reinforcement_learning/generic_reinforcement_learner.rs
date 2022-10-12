// Reinforcement learning module

use std::fmt::Display;
use core::hash::Hash;

pub trait State<A>: Display + Eq + Hash
where
    A: Action
{
    fn initial_state() -> Self;
    fn next_state(&self, action: &A) -> Self;
    fn is_terminal(&self) -> bool;
    fn available_actions(&self) -> Vec<A>;
    fn num_available_actions(&self) -> usize {
        return self.available_actions().len();
    }
}


pub trait Action: Display + Eq + Hash {
}


pub trait Reward<S, A>
where
    S: State<A>,
    A: Action
{
    fn reward_function(state: &S, action: &A, next_state: &S) -> f64;
}


pub trait Policy<S, A>
where 
    S: State<A>,
    A: Action
{
    fn get_action(&self, values: Vec<(A, f64)>) -> A;
}


pub trait ReinforcementLearner<S, A> 
where 
    S: State<A>,
    A: Action
{
    fn get_action_value(&self, state: S, action: A) -> f64;
    fn update_action_value(&mut self, state: &S, action: &A, value: f64);
    fn get_state_value(&self, state: &S) -> f64;
    fn update_state_value(&mut self, state: &S, value: f64);
}

