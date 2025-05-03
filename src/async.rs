use embedded_hal_async::i2c::{I2c, SevenBitAddress};
use crate::{
    CommandType,
    PowerDown,
    DacStatus,
    encode::{encode_command, encode_fast_command},
};

/// Set the dac register
pub async fn set_dac<I: I2c<SevenBitAddress>>(i2c: &mut I, address: u8, power: PowerDown, data: u16) -> Result<(), I::Error> {
    let bytes = encode_command(CommandType::WriteDac, power, data);
    i2c.write(address, &bytes).await
}

/// Set the dac and eeprom registers
pub async fn set_dac_and_eeprom<I: I2c<SevenBitAddress>>(i2c: &mut I, address: u8, power: PowerDown, data: u16) -> Result<(), I::Error> {
    let bytes = encode_command(CommandType::WriteDacAndEEPROM, power, data);
    i2c.write(address, &bytes).await
}

/// Use the two byte fast command to set the dac register
pub async fn set_dac_fast<I: I2c<SevenBitAddress>>(i2c: &mut I, address: u8, power: PowerDown, data: u16) -> Result<(), I::Error> {
    let bytes = encode_fast_command(power, data);
    i2c.write(address, &bytes).await
}

/// Send read command and return the dac status
pub async fn read<I: I2c<SevenBitAddress>>(i2c: &mut I, address: u8) -> Result<DacStatus, I::Error> {
    let mut buffer: [u8; 5] = [0; 5];
    i2c.read(address, &mut buffer).await?;

    Ok(buffer.into())
}
