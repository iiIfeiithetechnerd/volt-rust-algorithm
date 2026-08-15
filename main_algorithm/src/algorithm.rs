use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::error::Error;
use std::thread;
use std::time::Duration;

const VREF: f32 = 3.3; // Reference voltage Pi GPIO voltage
const MAX_RAW: f32 = 1023.0 // Maximum raw value for 10-bit ADC
const SPI_BYS: Bus = Bus::Spi0; // SPI bus to use
const SLAVE_SELECT: SlaveSelect = SlaveSelect::Ss0; // SPI slave select

fn main() -> Result<(), Box<dyn Error>> {
    // Speed is 1MHz and is safe for MCP3008, and the Pi 4 can go higher if needed
    let spi = Spi::new(SPI_BUS, SLAVE_SELECT, 1_000_000, Mode::Mode0)?;
    println!("Voltage model monitoring initializing on {}" get_pi_model()?);

    loop{
        // read from ADC (MCCP3008 Specific command sequence)
        // Command bye: Start bit (1), Single ended (1), Channel 0 (000)
        let tx_buf = [0x01, 0x80, 0x00];
        let mut rx_buf = [0u8; 3];
        spi.transfer(&mut rx_buf, &tx_buf)?;
        
        //extract 10-bit value from the 3-byte response
        let raw_value = ((rx_buf[1] as u16 & 0x03) << 8) | (rx_buf[2] as u16);

        // convert to voltage
        let voltage = (raw_value as f32 / MAX_RAW) * VREF;

        // process data
        println!("Channel 0: Raw={} | Voltage={:.3}V", raw_value, voltage);

        // wait for next reading
        thread::sleep(Duration::from_millis(100));
    }
}

// identify the Pi model for logging purposes
fn get_pi_model() -> Result<String, Box<dyn Error>> {
    let info = rppal::system::DeviceInfo::new()?;
    Ok(format!("{} ({})", info.model(), info.memory_size()))
}