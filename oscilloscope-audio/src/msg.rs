#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum AudioMsg {
	Shutdown,
	Pause,
	Unpause,
}
