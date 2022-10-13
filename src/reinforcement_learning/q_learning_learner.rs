use std::collections::{HashMap, hash_map::RandomState};

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
    q_values: HashMap<StateAction<S, A>, f64, RandomState>,
    alpha: f64,
    gamma: f64
}


impl<S, A> ReinforcementLearner<S, A> for QLearner<S, A>
where
    S: State<A>,
    A: Action
{
    fn get_action_value(&self, state: S, action: A) -> f64 {
        match self.q_values.get(&StateAction (state, action)) {
            Some(value) => *value,
            None => 0.0
        }
    }

    fn get_action_values(&self, state: S) -> Vec<(A, f64)> {
        let values: Vec<(A, f64)> = Vec::new();
        for action in state.available_actions() {
            values.push((action, self.get_action_value(state, action)));
        }
        return values;
    }

    fn get_best_action(&self, state: &S) -> A {
        return self.get_action_values(*state).iter()
            .max_by(|v, w| v.1.partial_cmp(&w.1).unwrap())
            .unwrap().0;
    }

    fn update_action_value(&mut self, state: &S, action: &A, next_state: &S, reward: f64) {
        let current_q_value = self.get_action_value(*state, *action);
        if cfg!(debug_assertions) { println!("{next_state}"); }
        let best_next_action = self.get_best_action(state);
        let next_state_best_q_value = self.get_action_value(*next_state, best_next_action);
        let new_value = current_q_value +
            self.alpha * (reward + self.gamma * (-1.0 * next_state_best_q_value - current_q_value));
        if cfg!(debug_assertions) {
            println!("Old Q value: {current_q_value}, new Q Value: {new_value}")
        }
        self.q_values.insert(StateAction(*state, *action), new_value);
    }

    fn get_state_value(&self, state: &S) -> f64 {
        todo!()
    }

    fn update_state_value(&mut self, state: &S, value: f64) {
        todo!()
    }
}


impl<S, A> QLearner<S, A>
where
    S: State<A>,
    A: Action,
{
    // Let's do a simple Q-learning implementation
    pub fn q_learning(&self, policy: &dyn Policy<S, A>, num_episodes: u32) {
        // Initialise Q(s, a) arbitrarily for any s, a, and for terminal states set Q(s, _) = 0
    
        // Repeat for each episode
        for episode in 1..=num_episodes {
            if cfg!(debug_assertions) {
                println!("Episode: {episode} / {num_episodes}");
            } else if episode % 1000 == 0 {
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
                let action = policy.get_action(self.get_action_values(state));
        
                // Take action A, observe R, S'
                
                let next_state = state.next_state(&action);
                let reward = S::get_reward(state, &action, next_state);
                
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
