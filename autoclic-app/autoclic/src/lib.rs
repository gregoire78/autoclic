#![deny(clippy::all)]

use napi_derive::napi;
use std::{sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread, time};
use enigo::{
    Button,
    Direction::{ Click },
    Enigo, Mouse, Settings,
};
use once_cell::sync::Lazy;

static AUTCLICKER_STATE: Lazy<Mutex<Option<Arc<AtomicBool>>>> = Lazy::new(|| Mutex::new(None));

#[napi]
pub fn start_autoclicker(interval_ms: u32) {
    let mut guard = AUTCLICKER_STATE.lock().unwrap();

    if guard.as_ref().map_or(false, |flag| flag.load(Ordering::SeqCst)) {
        return; // déjà en cours
    }

    let running = Arc::new(AtomicBool::new(true));
    *guard = Some(running.clone());

    thread::spawn(move || {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        let delay = time::Duration::from_millis(interval_ms as u64);

        while running.load(Ordering::SeqCst) {
            let _ = enigo.button(Button::Left, Click);
            thread::sleep(delay);
        }
    });
}

#[napi]
pub fn stop_autoclicker() {
    let mut guard = AUTCLICKER_STATE.lock().unwrap();

    if let Some(flag) = guard.as_ref() {
        flag.store(false, Ordering::SeqCst);
    }

    *guard = None;
}