use bxcan::{Data, Frame, Id, StandardId};

use crate::crc8::{calc_crc8, generate_lookup};

const TORQUE_REQUEST_ID: StandardId = unsafe {StandardId::new_unchecked(0x14d)};


const NISSAN_CRC_LOOKUP: [u8;256] = generate_lookup(0x85);


pub enum EvCanError {
    BadDlc,
    BadCrc,
    NoData,
    UnknownFrame,
}


pub enum EvCanFrame {
    TorqueRequest {torque: i16, counter: u8}
}

impl EvCanFrame
{
    pub fn id(&self) -> StandardId {
        match self {
            EvCanFrame::TorqueRequest {..} => TORQUE_REQUEST_ID
        }
    }


    pub fn data(&self) -> Data {
        match self {
            EvCanFrame::TorqueRequest { torque, counter } => Data::new(&Self::to_torque_request_data(*torque, *counter)),
        }.unwrap() // Will never happen since match enforce all options managed
    }

    fn to_torque_request_data(torque: i16, counter: u8) -> [u8; 8] {
        let torque_bytes = torque.to_le_bytes();

        let mut data = [0x6e, 0x6e, torque_bytes[0], torque_bytes[1], counter << 6, 0x44, 0x01, 0x00];

        data[7] = calc_crc8(&data[..7], &NISSAN_CRC_LOOKUP);

        data
    }

    fn from_torque_request_data(data: &Data) -> Result<Self, EvCanError> {
        i16::
        let torque = 0;
        let counter = 0;

        // TODO

        Ok(EvCanFrame::TorqueRequest { torque, counter })
    }
}

impl From<EvCanFrame> for Frame {
    fn from(value: EvCanFrame) -> Self {
        Frame::new_data(value.id(), value.data())
    }
}

impl TryFrom<Frame> for EvCanFrame {
    type Error = EvCanError;

    fn try_from(value: Frame) -> Result<Self, Self::Error> {
        if let Id::Standard(id) = value.id() {
            let data = value.data().ok_or(EvCanError::NoData)?;
            match id {
                TORQUE_REQUEST_ID => Ok(EvCanFrame::from_torque_request_data(data)?),
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
        let frame: Frame = EvCanFrame::TorqueRequest { torque: 1000, counter: 2 }.into();

        assert_eq!(frame.id(), Id::Standard(StandardId::new(0x1d4).unwrap()));
        assert_eq!(frame.data().unwrap(), [0x6e, 0x6e, 0x00, 0x00, 0x00, 0x44, 0x01, 0x00]);
    }
}
