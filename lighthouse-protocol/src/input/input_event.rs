use serde::{Deserialize, Serialize};

use super::{GamepadEvent, KeyEvent, MouseEvent};

/// A user input event, as generated by the new frontend (LUNA).
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum InputEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Gamepad(GamepadEvent),
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{EventSource, GamepadAxis2DEvent, GamepadAxisEvent, GamepadButtonEvent, GamepadControlEvent, GamepadEvent, InputEvent, KeyEvent, KeyModifiers, MouseButton, MouseEvent, Pos, Vec2};

    #[test]
    fn key_event() {
        assert_eq!(
            serde_json::from_value::<InputEvent>(json!({
                "type": "key",
                "source": 0,
                "down": true,
                "repeat": false,
                "code": "ArrowUp",
                "modifiers": {
                    "alt": false,
                    "ctrl": false,
                    "meta": false,
                    "shift": false,
                },
            })).unwrap(),
            InputEvent::Key(KeyEvent {
                source: EventSource::Int(0),
                down: true,
                repeat: false,
                code: "ArrowUp".into(),
                modifiers: KeyModifiers::default(),
            })
        );
    }

    #[test]
    fn mouse_event() {
        assert_eq!(
            serde_json::from_value::<InputEvent>(json!({
                "type": "mouse",
                "source": 1,
                "button": "left",
                "pos": {
                    "x": 2,
                    "y": 4,
                },
            })).unwrap(),
            InputEvent::Mouse(MouseEvent {
                source: EventSource::Int(1),
                button: MouseButton::Left,
                pos: Pos::new(2.0, 4.0),
            })
        );
    }

    #[test]
    fn gamepad_button_event() {
        assert_eq!(
            serde_json::from_value::<InputEvent>(json!({
                "type": "gamepad",
                "source": 1,
                "control": "button",
                "index": 42,
                "down": true,
                "value": 0.25,
            })).unwrap(),
            InputEvent::Gamepad(GamepadEvent {
                source: EventSource::Int(1),
                control: GamepadControlEvent::Button(GamepadButtonEvent {
                    index: 42,
                    down: true,
                    value: 0.25,
                }),
            })
        );
    }

    #[test]
    fn gamepad_axis_event() {
        assert_eq!(
            serde_json::from_value::<InputEvent>(json!({
                "type": "gamepad",
                "source": 1,
                "control": "axis",
                "index": 42,
                "value": 0.25,
            })).unwrap(),
            InputEvent::Gamepad(GamepadEvent {
                source: EventSource::Int(1),
                control: GamepadControlEvent::Axis(GamepadAxisEvent {
                    index: 42,
                    value: 0.25,
                }),
            })
        );
    }

    #[test]
    fn gamepad_axis_2d_event() {
        assert_eq!(
            serde_json::from_value::<InputEvent>(json!({
                "type": "gamepad",
                "source": 1,
                "control": "axis2d",
                "index": 42,
                "value": {
                    "x": 0.2,
                    "y": -0.2,
                },
            })).unwrap(),
            InputEvent::Gamepad(GamepadEvent {
                source: EventSource::Int(1),
                control: GamepadControlEvent::Axis2D(GamepadAxis2DEvent {
                    index: 42,
                    value: Vec2::new(0.2, -0.2),
                }),
            })
        );
    }
}
