use sfml::{
	graphics::{CircleShape, Color, RenderStates, RenderTarget, RenderWindow, Transformable},
	window::{ContextSettings, Event, Style, VideoMode},
};

fn main() {
	let mut w = RenderWindow::new(
		VideoMode::desktop_mode(),
		"Ossiloscope",
		Style::RESIZE | Style::CLOSE,
		&ContextSettings {
			depth_bits: 24,
			stencil_bits: 0,
			antialiasing_level: 3,
			major_version: 4,
			minor_version: 6,
			attribute_flags: ContextSettings::ATTRIB_DEFAULT,
			srgb_capable: true,
		},
	)
	.expect("Failed to create window");

	let mut circle = CircleShape::new(8.0, 8);
	circle.set_position((50.0, 50.0));
	'event_loop: loop {
		// Procces all pending events
		while let Some(event) = w.poll_event() {
			match event {
				Event::Closed => break 'event_loop,
				e => println!("Event: {e:?}"),
			}
		}

		w.clear(Color::rgb(1, 1, 1));
		w.draw_circle_shape(&circle, &RenderStates::DEFAULT);
		w.display();
	}
}
