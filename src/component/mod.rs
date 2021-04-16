use hubcaps::issues::State;

pub mod issue;
pub mod note;

fn to_issue_state(state: &str) -> State {
    match state {
        "open" => State::Open,
        "closed" => State::Closed,
        _ => State::All,
    }
}
