extern mod sdl;
mod map;

use map;
use map::*;

use core::str;
use core::io;
use core::uint::range;
use core::libc::{c_char};
use sdl::sdl;
use sdl::ll;
use sdl::video;
use sdl::img;
use sdl::keyboard::*;
use sdl::event;
use sdl::video::{DoubleBuf, HWSurface, AsyncBlit};

use sdl::util::Rect;

const SCREEN_WIDTH: uint = 800;
const SCREEN_HEIGHT: uint = 600;
const SCREEN_BPP: uint = 32;

fn load_or_die(file : ~str) -> ~video::Surface {
	match img::load_img(str::concat(&[~"data/", copy file, ~".png"])) {
		result::Ok(image) => {
			image
		},
		result::Err(str) => {
			die!(str);
		}
	}
}

fn draw_each_on_view(
		screen : &video::Surface,
		view : &View,
		pos : &Position,
		map : &mut map::Map,
		f : &a/fn(position : map::Position, tile : map::Tile) -> Option<&a/video::Surface>)
	{
		do map.each_in_vrect(pos, 5, 5) | position : map::Position, tile : &mut map::Tile | {
			match f(position , *tile) {
				None => {},
				Some(surface) => view.draw(screen, &position, surface)
			};
		}
	}

fn main() {
	io::print("Hi!\n");
	sdl::sdl::init(&[sdl::sdl::InitEverything]);

	let screen = match video::set_video_mode(
			SCREEN_WIDTH as int, SCREEN_HEIGHT as int, SCREEN_BPP as int,
			&[],&[DoubleBuf]
			) {
		result::Ok(image) => {
			image
		},
		result::Err(str) => {
			io::print(str);
			return;
		}

	};

	let fog = load_or_die(~"fog");
	let floor = load_or_die(~"floor");
	let wall = load_or_die(~"wall");
	let notvisibe = load_or_die(~"notvisible");
	let human = load_or_die(~"human");

	let map = map::Map::new();

	let player = Creature::new(Position {x: 6, y: 6}, N);

	player.set_map(map);

	loop {
		player.update_visibility();

		screen.fill(0);

		let view = player.view();

		do draw_each_on_view(screen, &*view, &player.position, map) | _ : map::Position, tile : map::Tile| {
			if tile.is_wall() {
				Some(&*wall)
			} else {
				Some(&*floor)
			}
		}
		do draw_each_on_view(screen, &*view, &player.position, map) |position : map::Position, _ : map::Tile| {
			if !player.sees(&position) {
				Some(&*notvisibe)
			} else {
				None
			}
		}

		view.draw(screen, &player.position, human);

		do draw_each_on_view(screen, &*view, &player.position, map) |position : map::Position, _ : map::Tile| {
			if !player.knowns(&position) {
				//Some(&*fog)
				Some(&*notvisibe)
			} else {
				None
			}
		}

		screen.flip();

		match event::poll_event() {
			event::KeyDownEvent(ref key_event) => {
				match key_event.keycode {
					SDLKEscape => {
						return;
					},
					SDLKk | SDLKUp => {
						player.move_forward();
					},
					SDLKh | SDLKLeft => {
						player.turn_left();
					},
					SDLKl | SDLKRight => {
						player.turn_right();
					},
					SDLKj | SDLKDown => {
						player.move_backwards();
					},
					k => {
						io::print(fmt!("%d\n", k as int));
					}
				};
			},
			event::NoEvent => {},
			_ => {}
		}
	}
}
