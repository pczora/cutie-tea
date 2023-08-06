use crate::types::{VarByte, MQTTString, MQTTTwoBytes, MQTTByte};

trait PropertyValue {}
impl PropertyValue for VarByte {}
impl PropertyValue for MQTTString {}
impl PropertyValue for MQTTTwoBytes {}
impl PropertyValue for MQTTByte {}

struct Packet {
    fixed_header: FixedHeader,
    variable_header: VariableHeader, 
    payload: Payload
}
struct FixedHeader {
    packet_type_and_flags: u8,
    remaining_length: VarByte
}

struct VariableHeader {
    packet_identifier: u16
}

struct VariableHeaderConnect {
    protocol_name: MQTTString,
    protocol_version: MQTTByte,
    connect_flags: MQTTByte
}

struct Property<T: PropertyValue> {
    identifier: VarByte,
    value: T
}

struct Payload {

}

struct ConnectPacket {
    packet: Packet
}

