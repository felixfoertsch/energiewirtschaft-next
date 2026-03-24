use crate::fehler::ProzessFehler;
use crate::nachricht::Nachricht;

/// Output of a single reducer step.
#[derive(Debug, Clone, PartialEq)]
pub struct ReducerOutput<S> {
	pub state: S,
	pub nachrichten: Vec<Nachricht>,
}

/// The core trait every process crate implements.
pub trait Reducer {
	type State;
	type Event;

	fn reduce(
		state: Self::State,
		event: Self::Event,
	) -> Result<ReducerOutput<Self::State>, ProzessFehler>;
}
