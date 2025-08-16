#![deny(clippy::all)]

use enigo::{Button, Coordinate as EnigoCoord, Direction::Click, Enigo, Mouse, Settings};
use napi::{Error, Result};
use napi_derive::napi;
use once_cell::sync::Lazy;
use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
  },
  thread,
  time::Duration,
};

const MOVE_LOOP_DELAY_MS: u64 = 10;

static AUTOCLICKER_STATE: Lazy<Mutex<Option<Arc<AtomicBool>>>> = Lazy::new(|| Mutex::new(None));

fn is_autoclicker_running() -> bool {
  AUTOCLICKER_STATE
    .lock()
    .unwrap()
    .as_ref()
    .map_or(false, |flag| flag.load(Ordering::SeqCst))
}

fn set_autoclicker_flag(running: bool) {
  if let Some(flag) = AUTOCLICKER_STATE.lock().unwrap().as_ref() {
    flag.store(running, Ordering::SeqCst);
  }
}

fn clear_autoclicker_flag() {
  *AUTOCLICKER_STATE.lock().unwrap() = None;
}

fn create_enigo() -> Result<Enigo> {
  Enigo::new(&Settings::default()).map_err(|e| Error::from_reason(e.to_string()))
}

fn spawn_click_thread(running: Arc<AtomicBool>, interval_ms: u32) {
  thread::spawn(move || {
    let mut enigo = match create_enigo() {
      Ok(e) => e,
      Err(e) => {
        eprintln!("Erreur création Enigo: {:?}", e);
        return;
      }
    };
    let delay = Duration::from_millis(interval_ms as u64);

    while running.load(Ordering::SeqCst) {
      let _ = enigo.button(Button::Left, Click);
      thread::sleep(delay);
    }
  });
}

fn spawn_move_thread(running: Arc<AtomicBool>, x: i32, y: i32) {
  thread::spawn(move || {
    let mut enigo = match create_enigo() {
      Ok(e) => e,
      Err(e) => {
        eprintln!("Erreur création Enigo: {:?}", e);
        return;
      }
    };

    while running.load(Ordering::SeqCst) {
      if let Err(e) = enigo.move_mouse(x, y, EnigoCoord::Abs) {
        eprintln!("Erreur déplacement souris: {:?}", e);
      }
      thread::sleep(Duration::from_millis(MOVE_LOOP_DELAY_MS));
    }
  });
}

#[napi]
pub fn start_autoclicker(interval_ms: u32) {
  if is_autoclicker_running() {
    return;
  }

  let running = Arc::new(AtomicBool::new(true));
  *AUTOCLICKER_STATE.lock().unwrap() = Some(running.clone());

  spawn_click_thread(running, interval_ms);
}

#[napi]
pub fn start_autoclicker_with_coord(interval_ms: u32, x: i32, y: i32) {
  if is_autoclicker_running() {
    return;
  }

  let running = Arc::new(AtomicBool::new(true));
  *AUTOCLICKER_STATE.lock().unwrap() = Some(running.clone());

  spawn_click_thread(running.clone(), interval_ms);
  spawn_move_thread(running, x, y);
}

#[napi]
pub fn stop_autoclicker() {
  set_autoclicker_flag(false);
  clear_autoclicker_flag();
}

#[napi(object)]
pub struct MousePosition {
  pub x: i32,
  pub y: i32,
}

#[napi]
pub fn get_mouse_position() -> Result<MousePosition> {
  let enigo = create_enigo()?;
  let (x, y) = enigo.location().map_err(|e| Error::from_reason(e.to_string()))?;
  Ok(MousePosition { x, y })
}

#[napi]
pub enum Coordinate {
  Absolute,
  Relative,
}

#[napi]
pub fn move_mouse_with_pause(x: i32, y: i32, coordinate: Coordinate) {
  let enigo_coord = match coordinate {
    Coordinate::Absolute => EnigoCoord::Abs,
    Coordinate::Relative => EnigoCoord::Rel,
  };

  set_autoclicker_flag(false);

  if let Ok(mut enigo) = create_enigo() {
    if let Err(e) = enigo.move_mouse(x, y, enigo_coord) {
      eprintln!("Erreur déplacement souris: {:?}", e);
    }
  }

  set_autoclicker_flag(true);
}
