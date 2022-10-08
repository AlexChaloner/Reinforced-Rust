// Reinforcement learning module

pub trait Action {
    fn to_string() -> String;
}

pub trait State<A>
where
    A: Action
{
    fn to_string() -> String;
    fn initial_state() -> Self;
    fn next_state(action: &A) -> Self;
    fn is_terminal() -> bool;
    fn available_actions() -> Vec<A>;
    fn num_available_actions() -> usize {
        return Self::available_actions().len();
    }
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

