#![no_std]
#![no_main]

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut ir_sensors = lib::ir::ir_sensors!(dp, pins, [0; 8]);

    let mut min = [u16::MAX; 8];
    let mut max = [u16::MIN; 8];

    loop {
        let readings = ir_sensors.read_raw();
        for n in 0..8 {
            if min[n] > readings[n] {
                min[n] = readings[n];
            }

            if max[n] < readings[n] {
                max[n] = readings[n];
            }
        }

        let mut min_offset = u16::MAX;
        let mut offsets = [0; 8];
        for n in 0..8 {
            offsets[n] = (min[n] + max[n]) / 2;

            if offsets[n] < min_offset {
                min_offset = offsets[n];
            }
        }

        for offset in offsets {
            ufmt::uwrite!(&mut serial, "{}, ", offset - min_offset).unwrap();
        }
        ufmt::uwriteln!(&mut serial, "").unwrap();

        arduino_hal::delay_ms(100);
    }
}
