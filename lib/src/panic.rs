use arduino_hal::prelude::*;
use core::panic::PanicInfo;

use crate::display::Display;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    avr_device::interrupt::disable();
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.d20.into_pull_up_input(),
        pins.d21.into_pull_up_input(),
        50000,
    );

    let mut display = Display::new(i2c);

    ufmt::uwriteln!(&mut serial, "Firmware panic!").unwrap_infallible();
    if let Ok(display) = &mut display {
        let _ = ufmt::uwriteln!(display, "Firmware panic!");
    }

    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "At {}:{}:{}",
            loc.file(),
            loc.line(),
            loc.column(),
        )
        .unwrap_infallible();

        if let Ok(display) = &mut display {
            let _ = ufmt::uwriteln!(display, "At {}:{}:{}", loc.file(), loc.line(), loc.column());
        }
    }

    if let Some(Some(message)) = info.message().map(|m| m.as_str()) {
        ufmt::uwriteln!(&mut serial, "Message: {}", message).unwrap_infallible();
        if let Ok(display) = &mut display {
            let _ = ufmt::uwriteln!(display, "Message: {}", message);
        }
    }

    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(50);
    }
}
