use std::ops::Range;
use std::u16;

pub(crate) fn write_packet<F: Fn(&mut Vec<u8>)>(
    data: &mut Vec<u8>,
    tag: u8,
    write: F,
) -> Range<usize> {
    let init = data.len();
    let header_tag = (tag << 2) | 0b_1000_0001;
    data.extend(&[header_tag, 0, 0]);
    write(data);
    let len = data.len() - init - 3;
    assert!(len < u16::MAX as usize);
    data[(init + 1)..(init + 3)].copy_from_slice(&(len as u16).to_be_bytes());
    init..data.len()
}

pub(crate) fn prepare_packet<F: Fn(&mut Vec<u8>)>(tag: u8, write: F) -> Vec<u8> {
    let mut packet = vec![0, 0, 0];
    write(&mut packet);
    packet[0] = (tag << 2) | 0b_1000_0001;
    let len = packet.len() - 3;
    packet[1..3].copy_from_slice(&(len as u16).to_be_bytes());
    packet
}

pub(crate) fn write_subpackets<F>(packet: &mut Vec<u8>, write_each_subpacket: F)
where
    F: Fn(&mut Vec<u8>),
{
    packet.extend(&[0, 0]);
    let init = packet.len();
    write_each_subpacket(packet);
    let len = packet.len() - init;
    assert!(len < u16::MAX as usize);
    packet[(init - 2)..init].copy_from_slice(&(len as u16).to_be_bytes());
}

pub(crate) fn write_single_subpacket<F: Fn(&mut Vec<u8>)>(packet: &mut Vec<u8>, tag: u8, write: F) {
    packet.extend(&[0, tag]);
    let init = packet.len() - 1;
    write(packet);
    let len = packet.len() - init;
    assert!(len < 191);
    packet[init - 1] = len as u8;
}

pub(crate) fn write_mpi(data: &mut Vec<u8>, mpi: &[u8]) {
    assert!(mpi.len() < (u16::MAX / 8) as usize);
    assert!(mpi.len() > 0);
    let len = ((mpi.len() * 8 - (mpi[0].leading_zeros() as usize)) as u16).to_be_bytes();
    data.extend(&len);
    data.extend(mpi);
}

// pub(crate) fn bigendian_u32(data: u32) -> BigEndianU32 {
//     data.to_be_bytes()
// }

// pub(crate) fn bigendian_u16(data: u16) -> BigEndianU16 {
//     data.to_be_bytes()
// }
