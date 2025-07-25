use std::sync::mpsc::{Receiver, SyncSender};

use log::{info, warn};
use msg::AudioMsg;

pub mod msg;

pub fn thread_start(tx: SyncSender<AudioMsg>, rx: Receiver<AudioMsg>) {
	loop {
		match rx.recv().expect("main thread channel closed early") {
			AudioMsg::Shutdown => {
				info!("Audio thread shutting down.");
				break;
			}
			x => warn!("No handler for message {x:?}"),
		}
	}
}
