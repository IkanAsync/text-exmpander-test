use std::collections::HashMap;

use enigo::{Direction, Enigo, Key as enigoKey, Keyboard, Settings};
use rdev::{Event, EventType, Key, listen};

fn key_to_char(key: Key) -> Option<char> {
    match key {
        Key::KeyA => Some('a'),
        Key::KeyB => Some('b'),
        Key::KeyC => Some('c'),
        Key::KeyD => Some('d'),
        Key::KeyE => Some('e'),
        Key::KeyF => Some('f'),
        Key::KeyG => Some('g'),
        Key::KeyH => Some('h'),
        Key::KeyI => Some('i'),
        Key::KeyJ => Some('j'),
        Key::KeyK => Some('k'),
        Key::KeyL => Some('l'),
        Key::KeyM => Some('m'),
        Key::KeyN => Some('n'),
        Key::KeyO => Some('o'),
        Key::KeyP => Some('p'),
        Key::KeyQ => Some('q'),
        Key::KeyR => Some('r'),
        Key::KeyS => Some('s'),
        Key::KeyT => Some('t'),
        Key::KeyU => Some('u'),
        Key::KeyV => Some('v'),
        Key::KeyW => Some('w'),
        Key::KeyX => Some('x'),
        Key::KeyY => Some('y'),
        Key::KeyZ => Some('z'),
        Key::Num0 => Some('0'),
        Key::Num1 => Some('1'),
        Key::Num2 => Some('2'),
        Key::Num3 => Some('3'),
        Key::Num4 => Some('4'),
        Key::Num5 => Some('5'),
        Key::Num6 => Some('6'),
        Key::Num7 => Some('7'),
        Key::Num8 => Some('8'),
        Key::Num9 => Some('9'),
        _ => None,
    }
}
fn main() {
    let mut dict = HashMap::new();
    dict.insert("eml".to_string(), "user@example.com".to_string());
    dict.insert(
        "sig".to_string(),
        "Best regards,\nRust Developer".to_string(),
    );

    let mut buffer = String::new();

    let mut enigo = Enigo::new(&Settings::default()).expect("Gagal menginisialisasi");

    println!("=== Text Expander Sederhana ===");
    println!("Coba ketik 'eml' lalu tekan spasi");

    let callback = move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            // Cek trigger saat Spasi atau Enter ditekan
            if key == Key::Space || key == Key::Return {
                let mut found_expansion = None;

                for (trigger, expansion) in &dict {
                    if buffer.ends_with(trigger) {
                        found_expansion = Some((trigger.len(), expansion.clone()));
                        break;
                    }
                }

                if let Some((trigger_len, expansion)) = found_expansion {
                    // Hapus trigger + 1 karakter (spasi/enter yang baru diketik)
                    for _ in 0..=trigger_len {
                        let _ = enigo.key(enigoKey::Backspace, Direction::Click);
                    }
                    // Ketik teks pengganti
                    let _ = enigo.text(&expansion);
                    buffer.clear();
                } else {
                    // Jika tidak ada trigger, simpan spasi ke buffer
                    buffer.push(' ');
                    if buffer.len() > 50 {
                        buffer.remove(0);
                    }
                }
            }
            // Tangani Backspace
            else if key == Key::Backspace {
                buffer.pop();
            }
            // Tangani huruf/angka
            else if let Some(c) = key_to_char(key) {
                buffer.push(c);
                if buffer.len() > 50 {
                    buffer.remove(0);
                }
            }
        }
    };

    if let Err(e) = listen(callback) {
        println!("Error mendengarkan event keyboard:{:?}", e);
    }
}
