use egui::TextBuffer;
use sfml::{
	graphics::{
		CircleShape, Color, Rect, RenderStates, RenderTarget, RenderWindow, Transformable, View,
	},
	window::{ContextSettings, Event, Style, VideoMode},
};

fn draw_ui(
	sf_ui: &mut egui_sfml::SfEgui,
	w: &mut RenderWindow,
	message: &mut String,
	messages: &mut Vec<String>,
) {
	let di = sf_ui
		.run(w, |_rw, ctx| {
			let win = egui::SidePanel::left("left_panel").resizable(false);
			win.show(ctx, |ui| {
				ui.horizontal(|ui| {
					ui.label("Message");
					let te_re = ui.text_edit_singleline(message);
					if ui.button("Send").clicked()
						|| ui.input(|inp| inp.key_pressed(egui::Key::Enter))
					{
						messages.push(message.take());
						te_re.request_focus();
					}
				});
				for msg in messages.iter() {
					ui.separator();
					ui.label(msg);
				}
			});
		})
		.unwrap();

	sf_ui.draw(di, w, None);
}

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

	let mut circles = vec![CircleShape::new(8.0, 16); 64];
	circles
		.iter_mut()
		.enumerate()
		.for_each(|(i, c)| c.set_position((50.0 * (i + 1) as f32, 50.0)));

	let mut sf_ui = egui_sfml::SfEgui::new(&w);

	let mut message = String::new();
	let mut messages: Vec<String> = Vec::new();

	'event_loop: loop {
		// Procces all pending events
		while let Some(event) = w.poll_event() {
			match event {
				Event::Closed => break 'event_loop,
				Event::Resized { width, height } => {
					let v =
						View::from_rect(Rect::new(0.0, 0.0, width as f32, height as f32)).unwrap();
					w.set_view(&v)
				}
				e => println!("Event: {e:?}"),
			}
			sf_ui.add_event(&event);
		}

		w.clear(Color::rgb(1, 1, 1));

		circles
			.iter()
			.for_each(|c| w.draw_circle_shape(c, &RenderStates::DEFAULT));

		draw_ui(&mut sf_ui, &mut w, &mut message, &mut messages);

		w.display();
	}
}
