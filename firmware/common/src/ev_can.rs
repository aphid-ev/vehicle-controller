use bxcan::{Data, Frame, Id, StandardId};

use crate::crc8::{calc_crc8, generate_lookup};
const NISSAN_CRC_LOOKUP: [u8;256] = generate_lookup(0x85);

#[derive(Debug)]
pub enum EvCanError {
    BadDlc,
    BadCrc,
    NoData,
    ReceiveOnly,
    UnknownFrame,
}

// VCM -> Inverter
const VCM_KEEPALIVE1_ID: StandardId = unsafe {StandardId::new_unchecked(0x11a)};
const TORQUE_REQUEST_ID: StandardId = unsafe {StandardId::new_unchecked(0x14d)};
const VCM_KEEPALIVE2_ID: StandardId = unsafe {StandardId::new_unchecked(0x50b)};

// Inverter -> VCM
const INVERTER_STATUS_ID: StandardId = unsafe {StandardId::new_unchecked(0x1da)};
const INVERTER_TEMPERATURE_ID: StandardId = unsafe {StandardId::new_unchecked(0x55a)};

pub enum EvCanFrame {
    VcmKeepalive1 {counter: u8},
    VcmKeepalive2,
    TorqueRequest {torque: i16, counter: u8},
    InverterStatus { millivolt: u32, rpm: i16, current: i16, error: u8},
    InverterTemperature {motor_temperature: u8, inverter_temperature: u8},
}

impl EvCanFrame
{
    fn to_torque_request_data(torque: i16, counter: u8) -> [u8; 8] {
        let torque_bytes = torque.to_le_bytes();

        let mut data = [0x6e, 0x6e, torque_bytes[0], torque_bytes[1], counter << 6, 0x44, 0x01, 0x00];

        data[7] = calc_crc8(&data[..7], &NISSAN_CRC_LOOKUP);

        data
    }

    fn to_vcm_keepalive1_data(counter: u8) -> [u8; 8] {
        let mut data = [0x4e, 0x40, 0x00, 0xaa, 0xc0, 0x00, counter, 0x00];

        data[7] = calc_crc8(&data[..7], &NISSAN_CRC_LOOKUP);

        data
    }

    fn to_vcm_keepalive2_data() -> [u8; 7] {
        [0x00, 0x00, 0x06, 0xc0, 0x00, 0x00, 0x00]
    }

    fn from_torque_request_data(data: &Data) -> Result<Self, EvCanError> {
        let torque = i16::from_le_bytes([data[2], data[3]]);
        let counter = data[4] >> 6;

        let crc = calc_crc8(&data[..7], &NISSAN_CRC_LOOKUP);

        if crc != data[7] {
            Err(EvCanError::BadCrc)
        } else {
            Ok(EvCanFrame::TorqueRequest { torque, counter })
        }
    }

    fn from_inverter_status_data(data: &Data) -> Result<Self, EvCanError> {
        let millivolt = u16::from_le_bytes([data[0], data[1]]) as u32 * 500;
        let current = i16::from_le_bytes([data[2], data[3]]);
        let rpm = i16::from_le_bytes([data[4], data[5]]);
        let error = data[6];

        Ok(EvCanFrame::InverterStatus { millivolt, rpm, current, error })
    }

    fn from_inverter_temperature_data(data: &Data) -> Result<Self, EvCanError> {
        let motor_temperature = data[0];
        let inverter_temperature = data[1];

        Ok(EvCanFrame::InverterTemperature { motor_temperature, inverter_temperature })
    }
}

impl TryFrom<EvCanFrame> for Frame {
    type Error = EvCanError;

    fn try_from(value: EvCanFrame) -> Result<Self, Self::Error> {
        match value {
            EvCanFrame::TorqueRequest { torque, counter } => Ok(Frame::new_data(TORQUE_REQUEST_ID, EvCanFrame::to_torque_request_data(torque, counter))),
            EvCanFrame::VcmKeepalive1 { counter } => Ok(Frame::new_data(VCM_KEEPALIVE1_ID, EvCanFrame::to_vcm_keepalive1_data(counter))),
            EvCanFrame::VcmKeepalive2 => Ok(Frame::new_data(VCM_KEEPALIVE2_ID, EvCanFrame::to_vcm_keepalive2_data())),
            _ => Err(EvCanError::ReceiveOnly)
        }
    }
}

impl TryFrom<Frame> for EvCanFrame {
    type Error = EvCanError;

    fn try_from(value: Frame) -> Result<Self, Self::Error> {
        if let Id::Standard(id) = value.id() {
            let data = value.data().ok_or(EvCanError::NoData)?;
            match id {
                TORQUE_REQUEST_ID => Ok(EvCanFrame::from_torque_request_data(data)?),
                INVERTER_STATUS_ID => Ok(EvCanFrame::from_inverter_status_data(data)?),
                INVERTER_TEMPERATURE_ID => Ok(EvCanFrame::from_inverter_temperature_data(data)?),
                _ => Err(EvCanError::UnknownFrame)
            }
        } else {
            Err(EvCanError::UnknownFrame)
        }
    }
}

#[cfg(test)]
mod tests {
    use bxcan::Id;

    use super::*;

    #[test]
    fn torque_request() {
        let frame: Frame = EvCanFrame::TorqueRequest { torque: 1000, counter: 2 }.try_into().unwrap();

        assert_eq!(frame.id(), Id::Standard(TORQUE_REQUEST_ID));

        if let EvCanFrame::TorqueRequest { torque, counter } = EvCanFrame::from_torque_request_data(frame.data().unwrap()).unwrap() {
            assert_eq!(torque, 1000);
            assert_eq!(counter, 2);
        } else {
            panic!("Frame should be a TorqueRequest")
        }
    }
}
