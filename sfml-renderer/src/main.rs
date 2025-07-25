use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};

use bezier::{BezierCurve, BezierPoint, Point};
use egui::TextBuffer;
use log::{debug, error, info};
use oscilloscope_audio::msg::AudioMsg;
use rust_fontconfig::{FcFontCache, FcPattern, PatternMatch};
use sfml::window::Key;
use sfml::{
	cpp::FBox,
	graphics::{
		CircleShape, Color, Font, PrimitiveType, Rect, RenderStates, RenderTarget, RenderWindow,
		Text, Transformable, Vertex, View,
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

fn load_font() -> Option<FBox<Font>> {
	let cache = FcFontCache::build();
	let fonts = cache.query_all(
		&FcPattern {
			monospace: PatternMatch::True,
			..Default::default()
		},
		&mut Vec::new(),
	);

	info!("Found {} monospace fonts", fonts.len());
	let font_name = cache.get_metadata_by_id(&fonts.first()?.id)?.name.clone();
	info!("Picking font {font_name:?}");

	let font_src = cache.get_font_by_id(&fonts.first()?.id)?;

	match font_src {
		rust_fontconfig::FontSource::Disk(path) => {
			info!(
				"Loading font from: {} @ index: {}",
				path.path, path.font_index
			);
			match sfml::graphics::Font::from_file(&path.path) {
				Ok(f) => Some(f),
				Err(e) => {
					error!("Failed to load font: {e}");
					None
				}
			}
		}
		rust_fontconfig::FontSource::Memory(_font) => {
			unimplemented!("Loading fonts from memory is not supported")
		}
	}
}

fn move_point(idx: usize, points: &mut [BezierPoint], m: Point) {
	let point = match idx {
		1 => &mut points[0].origin,
		2 => &mut points[0].handle_b,
		3 => &mut points[1].handle_a,
		4 => &mut points[1].origin,
		_ => panic!("Invalid move index"),
	};
	*point = *point + m
}

fn main() {
	env_logger::init();

	let (local_tx, audio_rx): (SyncSender<AudioMsg>, Receiver<AudioMsg>) =
		std::sync::mpsc::sync_channel(8); // Outgoing
	let (audio_tx, local_rx): (SyncSender<AudioMsg>, Receiver<AudioMsg>) =
		std::sync::mpsc::sync_channel(32); // Incoming
	let audio_thread_handle = std::thread::spawn(move || {
		oscilloscope_audio::thread_start(audio_tx, audio_rx);
	});

	let font = load_font().expect("Unable to load font");

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

	let mut bez_circles = vec![CircleShape::new(8.0, 16); 4];

	let mut sf_ui = egui_sfml::SfEgui::new(&w);

	let mut message = String::new();
	let mut messages: Vec<String> = Vec::new();

	let mut bezier = {
		let mut bezier = BezierCurve::new();
		bezier.add_point(((300.0, 300.0), (500.0, 200.0)).into());
		bezier.add_point(((800.0, 800.0), (600.0, 1000.0)).into());
		bezier
	};

	let mut current_point = 0;
	'render_loop: loop {
		// Procces all pending events
		while let Some(event) = w.poll_event() {
			match event {
				Event::Closed => {
					break 'render_loop;
				}
				Event::Resized { width, height } => {
					let v =
						View::from_rect(Rect::new(0.0, 0.0, width as f32, height as f32)).unwrap();
					w.set_view(&v)
				}
				Event::KeyPressed { code: key, .. } => match key {
					Key::Num1 => {
						current_point = 1;
						debug!("Changed current point to : {}", current_point);
					}
					Key::Num2 => {
						current_point = 2;
						debug!("Changed current point to : {}", current_point);
					}
					Key::Num3 => {
						current_point = 3;
						debug!("Changed current point to : {}", current_point);
					}
					Key::Num4 => {
						current_point = 4;
						debug!("Changed current point to : {}", current_point);
					}
					Key::Up => {
						move_point(current_point, bezier.as_mut_slice(), (0.0, -10.0).into())
					}
					Key::Down => {
						move_point(current_point, bezier.as_mut_slice(), (0.0, 10.0).into())
					}
					Key::Left => {
						move_point(current_point, bezier.as_mut_slice(), (-10.0, 0.0).into())
					}
					Key::Right => {
						move_point(current_point, bezier.as_mut_slice(), (10.0, 0.0).into())
					}
					_ => (),
				},
				e => debug!("Event: {e:?}"),
			}
			sf_ui.add_event(&event);
		}
		loop {
			match local_rx.try_recv() {
				Ok(msg) => {}
				Err(TryRecvError::Empty) => break,
				Err(TryRecvError::Disconnected) => break 'render_loop,
			}
		}
		w.clear(Color::rgb(1, 1, 1));

		let mut t = Text::new("Hello, World!", &font, 32);
		t.set_position((500.0, 200.0));
		w.draw_text(&t, &RenderStates::DEFAULT);

		let verts: Vec<Vertex> = bezier
			.mesh()
			.flat_map(|(a, b)| [a, b])
			.map(|p| Vertex::with_pos(p.into()))
			.collect();

		w.draw_primitives(&verts, PrimitiveType::LINE_STRIP, &RenderStates::DEFAULT);
		circles
			.iter()
			.for_each(|c| w.draw_circle_shape(c, &RenderStates::DEFAULT));

		bezier
			.as_slice()
			.iter()
			.flat_map(|p| [p.handle_a, p.origin, p.handle_b])
			.skip(1)
			.take(4)
			.zip(bez_circles.iter_mut())
			.for_each(|(p, c)| c.set_position(Into::<(f32,f32)>::into(p)));

		bez_circles
			.iter()
			.for_each(|c| w.draw_circle_shape(c, &RenderStates::DEFAULT));

		// draw_ui(&mut sf_ui, &mut w, &mut message, &mut messages);

		w.display();
	}
	w.close();

	{
		info!("Requesting audio thread shutdown");
		match local_tx.send(AudioMsg::Shutdown) {
			Ok(()) => info!("Shutdown request sent"),
			Err(e) => info!("Audio thread channel disconnected already: {e}"),
		};
		if let Err(e) = audio_thread_handle.join() {
			error!("Audio thread panicked with: {e:?}")
		};
		info!("Audio thread stopped.");
	}
}
