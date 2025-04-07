# BMI323 Rust Driver

This is a Rust driver for the Bosch BMM350 Magnetometer. The BMM350 is a highly integrated, low power Magnetometer that provides precise angular rate measurements.

## Features

- Support for both I2C interfaces
- Configurable magnetometer settings
- Reading raw sensor data
- Error handling and device initialization

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bmm350 = "0.0.1"  # Replace with the actual version
```

Here's a basic example of how to use the driver:

```rust
use bmm350::Bmm350;
use embedded_hal::blocking::i2c::I2c;

fn main() {
#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peripherals = unsafe { stm32h563::Peripherals::steal() };
    init_device(&peripherals);

    if i2c1_init(&peripherals).is_err() {
        loop {}
    }

    let i2c = I2C1 {
        i2c: peripherals.I2C1,
    };
    let delay = Delay;

    let mut bmm350_rs = Bmm350::new_with_i2c(i2c, 0x14, delay);

    if let Err(e) = bmm350_rs.init() {
        rprintln!("Error initializing BMM350: {:?}", e);
        loop {}
    }

    if let Err(e) = bmm350_rs.configure_interrupt(bmm350_rs::InterruptLatch::Pulsed, bmm350_rs::InterruptPolarity::ActiveHigh, bmm350_rs::InterruptDrive::PushPull, bmm350_rs::InterruptMap::Unmap) {
        rprintln!("Error configuring interrupt: {:?}", e);
        loop {}
    };

    if let Err(e) = bmm350_rs.enable_interrupt(bmm350_rs::InterruptEnableDisable::Enable) {
        rprintln!("Error enabling interrupt: {:?}", e);
        loop {}
    };

    if let Err(e) = bmm350_rs.set_odr_performance(bmm350_rs::DataRate::ODR100Hz, bmm350_rs::AverageNum::Avg4){
        rprintln!("Error setting ODR and performance: {:?}", e);
        loop {}
    };

    if let Err(e) = bmm350_rs.enable_axes(bmm350_rs::AxisEnableDisable::Enable, bmm350_rs::AxisEnableDisable::Enable, bmm350_rs::AxisEnableDisable::Enable) {
        rprintln!("Error enabling axes: {:?}", e);
        loop {}
    };

    if let Err(e) = bmm350_rs.set_power_mode(bmm350_rs::PowerMode::Normal) {
        rprintln!("Error setting power mode: {:?}", e);
        loop {}
    };
    loop {

        bmm350_rs.read_mag_data()
            .map(|data: bmm350_rs::Sensor3DData| {
                rprintln!("Magnetometer data: x: {}, y: {}, z: {}", data.x, data.y, data.z);
            })
            .unwrap_or_else(|e| {
                rprintln!("Error reading magnetometer data: {:?}", e);
            });

    }

}
```

## License

This project is licensed under Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## References

- [BMM350 Product Page](https://www.bosch-sensortec.com/products/motion-sensors/magnetometers/bmm350/)
- [BMM350 Datasheet](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmm350-ds001.pdf)
