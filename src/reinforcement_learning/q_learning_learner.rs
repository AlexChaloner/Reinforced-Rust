use std::collections::{HashMap, hash_map::RandomState};

use crate::tictactoe;

use super::generic_reinforcement_learner::{ReinforcementLearner, Action, State, Policy};

#[derive(PartialEq, Eq, Hash)]
pub struct StateAction<S, A>(S, A)
where
    S: State<A>,
    A: Action;


// impl<S, A> Eq for StateAction<S, A>
// where
//     S: State<A> + std::cmp::PartialEq,
//     A: Action + std::cmp::PartialEq
// {
//     // fn assert_receiver_is_total_eq(&self) {}
// }

// impl<S, A> std::hash::Hash for StateAction<S, A>
// where
//     S: State<A> + std::hash::Hash,
//     A: Action + std::hash::Hash
// {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.0.hash(state);
//         self.1.hash(state);
//     }
// }


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

    fn get_action_values(&self, state: S) -> f64 {
        match self.q_values.get(&StateAction (state, action)) {
            Some(value) => *value,
            None => 0.0
        }
    }

    fn update_action_value(&mut self, state: &S, action: &A, value: f64) {
        self.q_values.insert(StateAction(*state, *action), value);
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
            let mut state = tictactoe::TicTacToeBoard::initial_state();
            // Repeat for each step of episode
            let mut terminal = false;
            while !terminal {
                if cfg!(debug_assertions) {
                    println!("{}", state);
                    println!("Player {}'s turn", state.current_player);
                }
                // Choose A from S using policy derived from Q (e.g. epsilon-greedy)
                let action = policy.get_action(&q_values, &state);
        
                // Take action A, observe R, S'
                let mut reward = 0.0;
                let next_state = state.next_state(&action);
                
                match next_state.has_someone_won() {
                    // Give reward then end loop
                    Some(someone) => { 
                        if state.current_player == someone {
                            reward = 1.0;
                        } else if someone == BoardEntry::Blank {
                            reward = 0.0;
                        }
                        terminal = true;
                    }
                    None => {},
                }
                // Q(S, A) = Q(S, A) + alpha * (R + gamma * max_a Q(S', a) - Q(S, A))
                let current_q_value = *q_values.entry(StateAction(state.clone(), action)).or_insert(0.0);
                let next_state_best_q_value = match terminal {
                    false => {
                        if cfg!(debug_assertions) { next_state.pretty_print(); }
                        let best_next_action = get_best_action(&q_values, &next_state, None);
                        *q_values.entry(StateAction(next_state.clone(), best_next_action)).or_insert(0.0)
                    },
                    true => 0.0,
                };
                let new_value = current_q_value +
                    alpha * (reward + gamma * (-1.0 * next_state_best_q_value - current_q_value));
                if cfg!(debug_assertions) {
                    println!("Old Q value: {current_q_value}, new Q Value: {new_value}")
                }
                q_values.insert(StateAction(state, action), new_value);
                // S = S'
                state = next_state.clone();
                // Until S is terminal
            }
            if cfg!(debug_assertions) { state.pretty_print(); }
        }
        return q_values;
    }
}
