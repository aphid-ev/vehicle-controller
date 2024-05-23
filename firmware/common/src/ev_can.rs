use bxcan::{Frame, StandardId};

use crate::crc8::generate_lookup;




const NISSAN_CRC_LOOKUP: [u8;256] = generate_lookup(0x85);


pub enum EvCanFrame {
    TorqueRequest {torque: i16}
}

impl EvCanFrame
{
    pub fn id(&self) -> Option<StandardId> {
        match self {
            EvCanFrame::TorqueRequest {..} => StandardId::new(0x10d),
        }
    }


    pub fn data(&self, counter: Option<u8>) -> Option<[u8; 8]> {
        match (self, counter) {
            (EvCanFrame::TorqueRequest { torque }, Some(counter)) => Some(Self::torque_request_data(*torque, counter)),
            _ => None
        }
    }

    fn torque_request_data(_torque: i16, counter: u8) -> [u8; 8] {
        let data = [0x6e, 0x6e, 0, 0, counter, 0x44, 0x01, 0];


        data
    }
}

