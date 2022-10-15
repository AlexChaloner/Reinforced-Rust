use std::marker::PhantomData;

use rand::Rng;

use super::generic_reinforcement_learner::{State, Action, Policy};



pub struct EpsilonGreedyPolicy<S, A>
where 
    S: State<A>,
    A: Action
{
    pub epsilon: f64,
    state: PhantomData<S>,
    action: PhantomData<A>
}


impl<S, A> EpsilonGreedyPolicy<S, A>
where 
    S: State<A>,
    A: Action
{
    pub fn new(epsilon: f64) -> Self {
        Self {
            epsilon,
            state: PhantomData,
            action: PhantomData
        }
    }

    fn get_best_action(actions_and_values: &Vec<(A, f64)>) -> A {
        if actions_and_values.is_empty() {
            panic!("No actions available, state is terminal?");
        }
        let mut max: f64 = -1000.0;
        let mut best_actions = Vec::new();
        for (action, value) in actions_and_values {
            if cfg!(debug_assertions) {
                println!("{}: {}", action, value);
            }
            if *value > max {
                max = *value;
                best_actions = Vec::new();
                best_actions.push(action);
            } else if *value == max {
                best_actions.push(action);
            }
        }
        if cfg!(debug_assertions) {
            print!("Best actions: ");
            for action in &best_actions {
                print!("{}, ", action);
            }
            println!();
        }
        let mut thread_rng = rand::thread_rng();
        let length = best_actions.len();
        let chosen_action = thread_rng.gen_range(0..length);
        best_actions[chosen_action].clone()
    }
}


impl<S, A> Policy<S, A> for EpsilonGreedyPolicy<S, A>
where
    S: State<A>,
    A: Action
{
    fn get_action(&self, actions_and_values: &mut Vec<(A, f64)>) -> A {
        // let mut available_actions = state.available_actions();
        let mut thread_rng = rand::thread_rng();
        if actions_and_values.is_empty() {
            panic!("No moves available");
        }
        if actions_and_values.len() == 1 {
            return actions_and_values.remove(0).0;
        }
        let random_value: f64 = thread_rng.gen();
        if random_value > self.epsilon {
            let best_action = Self::get_best_action(actions_and_values);
            if cfg!(debug_assertions) {
                println!("Chosen best action: {}", best_action);
            }
            best_action
        } else {
            let length = actions_and_values.len();
            let chosen_action = thread_rng.gen_range(0..length);
            let action = actions_and_values.remove(chosen_action).0;
            if cfg!(debug_assertions) {
                println!("Chosen random action: {}", action);
            }
            action
        }
    }
}