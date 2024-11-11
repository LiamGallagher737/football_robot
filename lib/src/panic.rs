use core::panic::PanicInfo;

use crate::{display::Display, terminal::Terminal};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    avr_device::interrupt::disable();
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.d20.into_pull_up_input(),
        pins.d21.into_pull_up_input(),
        50000,
    );

    let mut terminal = Terminal::new().with_usb(serial);
    if let Ok(display) = Display::new(i2c) {
        terminal = terminal.with_display(display);
    }

    let _ = ufmt::uwriteln!(&mut terminal, "Firmware panic!");

    if let Some(loc) = info.location() {
        let _ = ufmt::uwriteln!(
            &mut terminal,
            "At {}:{}:{}",
            loc.file(),
            loc.line(),
            loc.column(),
        );
    }

    if let Some(Some(message)) = info.message().map(|m| m.as_str()) {
        let _ = ufmt::uwriteln!(&mut terminal, "Message: {}", message);
    }

    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(50);
    }
}
