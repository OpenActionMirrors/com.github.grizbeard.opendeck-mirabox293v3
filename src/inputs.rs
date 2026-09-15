use mirajazz::{error::MirajazzError, types::DeviceInput};

use crate::mappings::KEY_COUNT;

/// This device numbers its keys differently depending on the direction of travel.
///
/// For *writing images*, it numbers them row-major from the bottom left, while OpenDeck
/// numbers them row-major from the top left:
///
/// ```text
///   device        opendeck
///  10 11 12 13 14    0  1  2  3  4
///   5  6  7  8  9    5  6  7  8  9
///   0  1  2  3  4   10 11 12 13 14
/// ```
///
/// For *reporting key presses*, however, it counts from the top left just like OpenDeck
/// does, so presses need no remapping at all -- only the 1-based correction. The middle
/// row is unaffected either way, which is what makes the asymmetry easy to miss.
const KEY_MAP: [u8; KEY_COUNT] = [10, 11, 12, 13, 14, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4];

pub fn process_input(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    log::info!("Processing input: {}, {}", input, state);

    match input as usize {
        (0..=KEY_COUNT) => read_button_press(input, state),
        _ => Err(MirajazzError::BadData),
    }
}

fn read_button_states(states: &[u8]) -> Vec<bool> {
    let mut bools = vec![];

    for i in 0..KEY_COUNT {
        bools.push(states[i + 1] != 0);
    }

    bools
}

/// Converts opendeck key index to device key index
pub fn opendeck_to_device(key: u8) -> u8 {
    if key < KEY_COUNT as u8 {
        KEY_MAP[key as usize]
    } else {
        key
    }
}

/// Converts device key index to opendeck key index.
///
/// Presses are reported 1-based in OpenDeck's own order, so this is just the shift.
pub fn device_to_opendeck(key: usize) -> usize {
    key - 1
}

fn read_button_press(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    let mut button_states = vec![0x01];
    button_states.extend(vec![0u8; KEY_COUNT + 1]);

    if input == 0 {
        return Ok(DeviceInput::ButtonStateChange(read_button_states(
            &button_states,
        )));
    }

    let pressed_index: usize = device_to_opendeck(input as usize);

    // `device_to_opendeck` is 0-based, so add 1
    button_states[pressed_index + 1] = state;

    Ok(DeviceInput::ButtonStateChange(read_button_states(
        &button_states,
    )))
}
