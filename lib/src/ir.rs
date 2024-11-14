use arduino_hal::{
    clock::MHz16,
    hal::{
        port::{PF0, PF1, PF2, PF3, PF4, PF5, PF6, PF7},
        Adc,
    },
    port::{mode::Analog, Pin},
};

pub struct IrSensors {
    pub adc: Adc<MHz16>,
    pub ir1: Pin<Analog, PF0>,
    pub ir2: Pin<Analog, PF1>,
    pub ir3: Pin<Analog, PF2>,
    pub ir4: Pin<Analog, PF3>,
    pub ir5: Pin<Analog, PF4>,
    pub ir6: Pin<Analog, PF5>,
    pub ir7: Pin<Analog, PF6>,
    pub ir8: Pin<Analog, PF7>,
    pub offsets: [u16; 8],
}

impl IrSensors {
    pub fn read(&mut self) -> [u16; 8] {
        [
            self.read_single(IrSensor::IR1),
            self.read_single(IrSensor::IR2),
            self.read_single(IrSensor::IR3),
            self.read_single(IrSensor::IR4),
            self.read_single(IrSensor::IR5),
            self.read_single(IrSensor::IR6),
            self.read_single(IrSensor::IR7),
            self.read_single(IrSensor::IR8),
        ]
    }

    pub fn read_single(&mut self, ir: IrSensor) -> u16 {
        let raw_value = match ir {
            IrSensor::IR1 => self.ir1.analog_read(&mut self.adc),
            IrSensor::IR2 => self.ir2.analog_read(&mut self.adc),
            IrSensor::IR3 => self.ir3.analog_read(&mut self.adc),
            IrSensor::IR4 => self.ir4.analog_read(&mut self.adc),
            IrSensor::IR5 => self.ir5.analog_read(&mut self.adc),
            IrSensor::IR6 => self.ir6.analog_read(&mut self.adc),
            IrSensor::IR7 => self.ir7.analog_read(&mut self.adc),
            IrSensor::IR8 => self.ir8.analog_read(&mut self.adc),
        };
        raw_value - self.offsets[ir as usize]
    }
}

pub enum IrSensor {
    IR1,
    IR2,
    IR3,
    IR4,
    IR5,
    IR6,
    IR7,
    IR8,
}

#[macro_export]
macro_rules! ir_sensors {
    ($dp:expr, $pins:expr, $offsets:expr) => {{
        let mut adc = arduino_hal::Adc::new($dp.ADC, Default::default());
        $crate::ir::IrSensors {
            ir1: $pins.a0.into_analog_input(&mut adc),
            ir2: $pins.a1.into_analog_input(&mut adc),
            ir3: $pins.a2.into_analog_input(&mut adc),
            ir4: $pins.a3.into_analog_input(&mut adc),
            ir5: $pins.a4.into_analog_input(&mut adc),
            ir6: $pins.a5.into_analog_input(&mut adc),
            ir7: $pins.a6.into_analog_input(&mut adc),
            ir8: $pins.a7.into_analog_input(&mut adc),
            offsets: $offsets,
            adc,
        }
    }};
}
