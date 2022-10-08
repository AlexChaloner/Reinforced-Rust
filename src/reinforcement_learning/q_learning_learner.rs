


struct QLearner {

}

// Let's do a simple Q-learning implementation
pub fn q_learning(num_episodes: u32) -> Q {
    let mut q_values: Q = HashMap::new();

    let alpha = 0.1;
    let gamma = 0.99;

    // Initialise Q(s, a) arbitrarily for any s, a, and for terminal states set Q(s, _) = 0
    let mut player = BoardEntry::X;

    // Repeat for each episode
    for episode in 1..=num_episodes {
        if cfg!(debug_assertions) {
            println!("Episode: {episode} / {num_episodes}");
        } else if episode % 1000 == 0 {
            println!("Episode: {episode} / {num_episodes}");
        }
        
        // Initialise S
        let mut state = tictactoe::create_initial_board();
        // Repeat for each step of episode
        let mut terminal = false;
        while !terminal {
            if cfg!(debug_assertions) {
                state.pretty_print();
                println!("Player {player}'s turn");
            }
            // Choose A from S using policy derived from Q (e.g. epsilon-greedy)
            
            let action = choose_action(&q_values, &state);
            // Take action A, observe R, S'
            let mut reward = 0.0;
            let mut next_state = state.clone();
            next_state.put(action.x, action.y, player);
            
            match tictactoe::has_someone_won(&next_state)  {
                // Give reward then end loop
                Some(someone) => { 
                if player == someone {
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
            player = match player {
                BoardEntry::O => BoardEntry::X,
                BoardEntry::X => BoardEntry::O,
                BoardEntry::Blank => panic!("At the disco"),
            };
            // Until S is terminal
        }
        if cfg!(debug_assertions) { state.pretty_print(); }
    }
    return q_values;
}
