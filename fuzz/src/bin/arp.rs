/*
   Copyright (c) 2019 Alex Forster <alex@alexforster.com>

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

   SPDX-License-Identifier: Apache-2.0
*/

use pdu::*;

pub fn fuzz(data: &[u8]) {
    match ArpPdu::new(&data) {
        Ok(arp_pdu) => {
            arp_pdu.hardware_type();
            arp_pdu.protocol_type();
            arp_pdu.hardware_length();
            arp_pdu.protocol_length();
            arp_pdu.opcode();
            arp_pdu.sender_hardware_address();
            arp_pdu.sender_protocol_address();
            arp_pdu.target_hardware_address();
            arp_pdu.target_protocol_address();
        }
        Err(_) => {}
    }
}

fn main() {
    loop {
        honggfuzz::fuzz!(|data: &[u8]| {
            fuzz(&data);
        });
    }
}
