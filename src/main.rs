mod tictactoe;
mod reinforcement_learner;
mod reinforcement_learning;

fn main() {
    println!("Hello, world!");
    // tictactoe::two_player_tictactoe_game();
    // let q_values = reinforcement_learning::q_learning_learner::q_learning(100000);
    let q_values = reinforcement_learner::q_learning(100000);
    reinforcement_learner::play_vs_human(q_values);
}
