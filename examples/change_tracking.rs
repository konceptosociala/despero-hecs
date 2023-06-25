use despero_hecs::*;
use std::sync::{Arc, Mutex};

fn main() {
	let world = Arc::new(Mutex::new(World::new()));

	// create entities
	let _a = world.lock().unwrap().spawn((15, false));
	let _b = world.lock().unwrap().spawn((16, true));
	
	let world2 = world.clone();
	let world3 = world.clone();
	
	// multiply by 2 (15 -> 30, 16 -> 32)
	std::thread::spawn(move || {
		std::thread::sleep(std::time::Duration::from_millis(2000));
		for (_, mut int) in &mut world2.lock().unwrap().query::<&mut i32>(){
			*int *= 2;
		}
	});
	
	// main loop
	loop {	
		// check changes
		for (_, (int, changed)) in &mut world.lock().unwrap().query::<(&i32, Changed<i32>)>() {
			if changed {
				println!("{int}");
			}
		}
		// clear trackers
		world3.lock().unwrap().clear_trackers();
	}
}
