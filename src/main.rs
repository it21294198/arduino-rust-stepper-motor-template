#![no_std]
#![no_main]

use arduino_hal::hal::port::{mode::Output, Pin};
use embedded_hal::digital::OutputPin;
use panic_halt as _;

/// Optimized stepper motor struct using half-step mode for smoother operation
pub struct StepperMotor<'a> {
    coil_pins: [&'a mut Pin<Output>; 4],
    step_delay_micros: u16,  // Changed to microseconds for finer control
    step_index: usize,
}

impl<'a> StepperMotor<'a> {
    pub fn new(pins: [&'a mut Pin<Output>; 4], step_delay_micros: u16) -> StepperMotor<'a> {
        StepperMotor {
            coil_pins: pins,
            step_delay_micros,
            step_index: 0,
        }
    }

    // Optimized step sequence using half-stepping for smoother motion
    // This provides 8 steps per cycle instead of 4, doubling position resolution
    const STEP_SEQUENCE: [[bool; 4]; 8] = [
        [true,  false, false, false], // 1
        [true,  true,  false, false], // 1-2
        [false, true,  false, false], // 2
        [false, true,  true,  false], // 2-3
        [false, false, true,  false], // 3
        [false, false, true,  true],  // 3-4
        [false, false, false, true],  // 4
        [true,  false, false, true],  // 4-1
    ];

    #[inline(always)]
    fn apply_step(&mut self) {
        let current_step = Self::STEP_SEQUENCE[self.step_index];
        // Unroll the loop for better performance
        let _ = self.coil_pins[0].set_state(current_step[0].into());
        let _ = self.coil_pins[1].set_state(current_step[1].into());
        let _ = self.coil_pins[2].set_state(current_step[2].into());
        let _ = self.coil_pins[3].set_state(current_step[3].into());
    }

    #[inline(always)]
    pub fn step_forward(&mut self) {
        self.step_index = (self.step_index + 1) & 7; // Fast modulo 8 using bitwise AND
        self.apply_step();
        arduino_hal::delay_us(self.step_delay_micros as u32);
    }

    #[inline(always)]
    pub fn step_backward(&mut self) {
        self.step_index = (self.step_index + 7) & 7; // Decrement with wraparound using modulo 8
        self.apply_step();
        arduino_hal::delay_us(self.step_delay_micros as u32);
    }

    // Optimized rotate_steps using a faster stepping algorithm
    pub fn rotate_steps(&mut self, steps: i32) {
        if steps > 0 {
            for _ in 0..steps {
                self.step_forward();
            }
        } else {
            for _ in 0..(-steps) {
                self.step_backward();
            }
        }
    }

    // Speed now in microseconds for finer control
    #[inline(always)]
    pub fn set_speed(&mut self, delay_micros: u16) {
        self.step_delay_micros = delay_micros;
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut pin1 = pins.d2.into_output().downgrade();
    let mut pin2 = pins.d3.into_output().downgrade();
    let mut pin3 = pins.d4.into_output().downgrade();
    let mut pin4 = pins.d5.into_output().downgrade();

    // Initialize with microsecond delay (1000 microseconds = 1 millisecond)
    let mut stepper = StepperMotor::new([&mut pin1, &mut pin2, &mut pin3, &mut pin4], 1000);

    loop {
        // Rotate forward 200 steps (100 full steps in half-step mode)
        stepper.rotate_steps(1000);
        
        arduino_hal::delay_ms(500); // Reduced delay between directions
        
        // Rotate backward 200 steps
        stepper.rotate_steps(1000);
        
        arduino_hal::delay_ms(500);
    }
}