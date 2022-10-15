use std::collections::{HashMap, hash_map::RandomState};

use rand::Rng;

use super::generic_reinforcement_learner::{ReinforcementLearner, Action, State, Policy};

#[derive(PartialEq, Eq, Hash)]
pub struct StateAction<S, A>(S, A)
where
    S: State<A>,
    A: Action;


pub struct QLearner<S, A>
where
    S: State<A>,
    A: Action
{
    pub q_values: HashMap<StateAction<S, A>, f64, RandomState>,
    pub alpha: f64,
    pub gamma: f64
}


impl<S, A> ReinforcementLearner<S, A> for QLearner<S, A>
where
    S: State<A>,
    A: Action,
{
    fn get_action_value(&self, state: &S, action: &A) -> f64 {
        match self.q_values.get(&StateAction (state.clone(), action.clone())) {
            Some(value) => *value,
            None => 0.0
        }
    }

    fn get_action_values(&self, state: &S) -> Vec<(A, f64)> {
        let mut values: Vec<(A, f64)> = Vec::new();
        for action in state.available_actions() {
            let value = self.get_action_value(state, &action);
            values.push((action, value));
        }
        values
    }

    fn get_best_action(&self, state: &S) -> A {
        let actions_and_values = self.get_action_values(state);
        if actions_and_values.is_empty() {
            panic!("No actions available, state is terminal?");
        }
        let mut max: f64 = -1000.0;
        let mut best_actions = Vec::new();
        for (action, value) in actions_and_values {
            if cfg!(debug_assertions) {
                println!("{}: {}", action, value);
            }
            if value > max {
                max = value;
                best_actions = Vec::new();
                best_actions.push(action);
            } else if value == max {
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

    fn update_action_value(&mut self, state: &S, action: &A, next_state: &S, reward: f64) {
        let current_q_value = self.get_action_value(state, action);
        if cfg!(debug_assertions) { println!("{next_state}"); }
        let new_value = current_q_value +
            self.alpha * (reward + self.gamma * (-1.0 * self.get_state_value(next_state) - current_q_value));
        if cfg!(debug_assertions) {
            println!("Old Q value: {current_q_value}, new Q Value: {new_value}")
        }
        let state_action = StateAction(state.clone(), action.clone());
        self.q_values.insert(state_action, new_value);
    }

    fn get_state_value(&self, state: &S) -> f64 {
        if state.is_terminal() {
            return 0.0;
        }
        let best_action = self.get_best_action(state);
        self.get_action_value(state, &best_action)
    }

    fn update_state_value(&mut self, _state: &S, _value: f64) {
        panic!("Q Learner cannot directly update state value.")
    }
}


impl<S, A> QLearner<S, A>
where
    S: State<A>,
    A: Action,
{
    // Let's do a simple Q-learning implementation
    pub fn q_learning(&mut self, policy: &dyn Policy<S, A>, num_episodes: u32) {
        // Initialise Q(s, a) arbitrarily for any s, a, and for terminal states set Q(s, _) = 0
    
        // Repeat for each episode
        for episode in 1..=num_episodes {
            if cfg!(debug_assertions) || episode % 1000 == 0 {
                println!("Episode: {episode} / {num_episodes}");
            }
            
            // Initialise S
            let mut state: S = State::initial_state();
            // Repeat for each step of episode
            while !state.is_terminal() {
                if cfg!(debug_assertions) {
                    println!("{}", state);
                }
                // Choose A from S using policy derived from Q (e.g. epsilon-greedy)
                let mut action_values = self.get_action_values(&state);
                let action = policy.get_action(&mut action_values);
        
                // Take action A, observe R, S'
                let next_state = state.next_state(&action);
                let reward = S::get_reward(&state, &action, &next_state);
                
                // Q(S, A) = Q(S, A) + alpha * (R + gamma * max_a Q(S', a) - Q(S, A))
                self.update_action_value(&state, &action, &next_state, reward);
                
                // S = S'
                state = next_state;
                // Until S is terminal
            }
            if cfg!(debug_assertions) { println!("{state}"); }
        }
    }
}
