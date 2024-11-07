use arduino_hal::prelude::*;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    avr_device::interrupt::disable();
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Firmware panic!").unwrap_infallible();

    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "At {}:{}:{}",
            loc.file(),
            loc.line(),
            loc.column(),
        )
        .unwrap_infallible();
    }

    if let Some(Some(message)) = info.message().map(|m| m.as_str()) {
        ufmt::uwriteln!(&mut serial, "Message: {}", message).unwrap_infallible();
    }

    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(50);
    }
}
