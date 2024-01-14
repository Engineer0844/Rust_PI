
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED21: u8 = 21;
const GPIO_LED20: u8 = 20;

fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the GPIO pin and configure it as an output.
    let mut pin21 = Gpio::new()?.get(GPIO_LED21)?.into_output();
    let mut pin20 = Gpio::new()?.get(GPIO_LED20)?.into_output();


    loop {
        pin21.toggle();
        thread::sleep(Duration::from_millis(1000));

        pin20.toggle();
        thread::sleep(Duration::from_millis(1000));


    }
}