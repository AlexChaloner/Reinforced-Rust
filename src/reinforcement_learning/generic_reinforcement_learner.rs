// Reinforcement learning module

pub trait State<A>
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


pub trait Action {
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
    fn get_action(state: &S) -> A;
}


pub trait ReinforcementLearner<S, A> 
where 
    S: State<A>,
    A: Action
{
    fn get_action_value(state: &S, action: &A);
    fn update_action_value(state: &S, action: &A);
    fn get_state_value(state: &S);
    fn update_state_value(state: &S);
}

