use std::time::Duration;

use bevy::{prelude::*, input::gamepad::{GamepadRumbleRequest, GamepadRumbleIntensity}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, gamepad)
        .run();
}

fn gamepad(
    pads: Res<Gamepads>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadButton>>,
    joy: Res<Axis<GamepadAxis>>,
    mut rumble: EventWriter<GamepadRumbleRequest>
) {
    for pad in pads.iter() {
        if buttons.just_pressed(GamepadButton::new(pad, GamepadButtonType::LeftTrigger)) {
            rumble.send(GamepadRumbleRequest::Add {
                duration: Duration::from_secs_f32(3.0),
                intensity: GamepadRumbleIntensity::MAX,
                gamepad: pad,
            });
            println!("LB");
        }
        if buttons.just_pressed(GamepadButton::new(pad, GamepadButtonType::RightTrigger)) {
            rumble.send(GamepadRumbleRequest::Add {
                duration: Duration::from_secs_f32(3.0),
                intensity: GamepadRumbleIntensity::WEAK_MAX,
                gamepad: pad,
            });
        }
        let rx = joy.get(
            GamepadAxis::new(pad, GamepadAxisType::RightStickX)
        ).unwrap();
        let ry = joy.get(
            GamepadAxis::new(pad, GamepadAxisType::RightStickY)
        ).unwrap();
        let r = Vec2::new(rx, ry).length();
        let lx = joy.get(
            GamepadAxis::new(pad, GamepadAxisType::LeftStickX)
        ).unwrap();
        let ly = joy.get(
            GamepadAxis::new(pad, GamepadAxisType::LeftStickY)
        ).unwrap();
        let l = (Vec2::new(lx, ly).length() / 2.0_f32.sqrt()).powf(2.0);
        let lt2 = axes.get(
            GamepadButton::new(
                pad,
                GamepadButtonType::LeftTrigger2
            )
        ).unwrap().abs();
        let rt2 = axes.get(
            GamepadButton::new(
                pad,
                GamepadButtonType::RightTrigger2
            )
        ).unwrap().abs();
        if l > 0.1 {
            rumble.send(
                GamepadRumbleRequest::Add {
                    duration: Duration::from_secs_f32(0.1),
                    intensity: GamepadRumbleIntensity::strong_motor(l),
                    gamepad: pad,
                }
            );
            println!("L");
        }
        if r > 0.01 {
            rumble.send(
                GamepadRumbleRequest::Add {
                    duration: Duration::from_secs_f32(0.1),
                    intensity: GamepadRumbleIntensity::weak_motor(r),
                    gamepad: pad,
                }
            );
        }
        if lt2 > 0.01 {
            rumble.send(
                GamepadRumbleRequest::Add {
                    duration: Duration::from_secs_f32(1.0),
                    intensity: GamepadRumbleIntensity::strong_motor(lt2),
                    gamepad: pad,
                }
            );
        }
        if rt2 > 0.01 {
            rumble.send(
                GamepadRumbleRequest::Add {
                    duration: Duration::from_secs_f32(1.0),
                    intensity: GamepadRumbleIntensity::weak_motor(rt2),
                    gamepad: pad,
                }
            );
        }
    }
}