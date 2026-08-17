use std::os::fd::AsFd;
use std::time::Duration;

use smithay::backend::input;
use smithay::input::pointer::PointerHandle;
use smithay::input::SeatHandler;
use smithay::utils::{Logical, Point};

use crate::input::backend_ext::NiriInputDevice;

pub struct VirtualPointer {}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct VirtualPointerDevice {}

pub struct PointerMotionEvent {
    pub dx: f64,
    pub dy: f64,
    pub time: u64,
}

impl input::InputBackend for VirtualPointer {
    type Device = VirtualPointerDevice;
    type KeyboardKeyEvent = input::UnusedEvent;
    type PointerAxisEvent = input::UnusedEvent;
    type PointerButtonEvent = input::UnusedEvent;
    type PointerMotionEvent = PointerMotionEvent;
    type PointerMotionAbsoluteEvent = input::UnusedEvent;
    type GestureSwipeBeginEvent = input::UnusedEvent;
    type GestureSwipeUpdateEvent = input::UnusedEvent;
    type GestureSwipeEndEvent = input::UnusedEvent;
    type GesturePinchBeginEvent = input::UnusedEvent;
    type GesturePinchUpdateEvent = input::UnusedEvent;
    type GesturePinchEndEvent = input::UnusedEvent;
    type GestureHoldBeginEvent = input::UnusedEvent;
    type GestureHoldEndEvent = input::UnusedEvent;
    type TouchDownEvent = input::UnusedEvent;
    type TouchUpEvent = input::UnusedEvent;
    type TouchMotionEvent = input::UnusedEvent;
    type TouchCancelEvent = input::UnusedEvent;
    type TouchFrameEvent = input::UnusedEvent;
    type TabletToolAxisEvent = input::UnusedEvent;
    type TabletToolProximityEvent = input::UnusedEvent;
    type TabletToolTipEvent = input::UnusedEvent;
    type TabletToolButtonEvent = input::UnusedEvent;
    type SwitchToggleEvent = input::UnusedEvent;
    type SpecialEvent = input::UnusedEvent;
}

impl AsFd for VirtualPointerDevice {
    fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
        todo!()
    }
}

impl input::Device for VirtualPointerDevice {
    fn id(&self) -> String {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }

    fn has_capability(&self, _capability: smithay::backend::input::DeviceCapability) -> bool {
        todo!()
    }

    fn usb_id(&self) -> Option<(u32, u32)> {
        todo!()
    }

    fn syspath(&self) -> Option<std::path::PathBuf> {
        todo!()
    }
}

impl NiriInputDevice for VirtualPointerDevice {
    fn output(&self, state: &crate::niri::State) -> Option<smithay::output::Output> {
        state.niri.output_under_cursor()
    }
}

impl smithay::backend::input::PointerMotionEvent<VirtualPointer> for PointerMotionEvent {
    fn delta_x(&self) -> f64 {
        self.dx
    }

    fn delta_y(&self) -> f64 {
        self.dy
    }

    fn delta_x_unaccel(&self) -> f64 {
        self.dx
    }

    fn delta_y_unaccel(&self) -> f64 {
        self.dy
    }
}

impl smithay::backend::input::Event<VirtualPointer> for PointerMotionEvent {
    fn time(&self) -> u64 {
        self.time
    }

    fn device(&self) -> <VirtualPointer as input::InputBackend>::Device {
        VirtualPointerDevice {}
    }
}

impl VirtualPointer {
    pub fn new_pointer_motion_event<S: SeatHandler>(
        pointer: PointerHandle<S>,
        new_pos: Point<f64, Logical>,
        time: Duration,
    ) -> <Self as input::InputBackend>::PointerMotionEvent {
        let current_pos = pointer.current_location();
        PointerMotionEvent {
            dx: new_pos.x - current_pos.x,
            dy: new_pos.y - current_pos.y,
            time: time.as_micros() as u64,
        }
    }
}
