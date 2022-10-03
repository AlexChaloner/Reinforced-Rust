mod tictactoe;
mod reinforcement_learner;

fn main() {
    println!("Hello, world!");
    // tictactoe::two_player_tictactoe_game();
    let q_values = reinforcement_learner::q_learning(100000);
    reinforcement_learner::play_vs_human(q_values);
}
