


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum State {
    New,
    Running,
    Waiting,
    Ready,
    Terminated,
}
