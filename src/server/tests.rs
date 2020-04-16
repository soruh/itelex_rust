use super::*;
use std::io::Cursor;
use std::net::Ipv4Addr;

fn test_all(package: Package, serialized: Vec<u8>) {
    {
        let mut cursor = Cursor::new(serialized.clone());
        assert_eq!(
            Package::deserialize_le(&mut cursor).expect("Package::deserialize_le failed"),
            package,
            "deserialize_le created unexpected result"
        );

        let mut res = Vec::with_capacity(serialized.len());

        package
            .serialize_le(&mut res)
            .expect("package.serialize_le failed");

        assert_eq!(res, serialized, "serialize_le created unexpected result");
    }
}

#[test]
fn type_1() {
    let serialized: Vec<u8> = vec![
        // header:
        1, 8, // number:
        0x0f, 0xf0, 0x00, 0xff, // pin:
        0x0f, 0xf0, // port:
        0xf0, 0x0f,
    ];

    let package = ClientUpdate {
        number: 0xff_00_f0_0f,
        pin: 0xf0_0f,
        port: 0x0f_f0,
    }
    .into();

    test_all(package, serialized);
}

#[test]

fn type_2() {
    let serialized: Vec<u8> = vec![
        // header:
        2, 4, // ipaddress
        0xff, 0x00, 0xf0, 0x0f,
    ];

    let package = AddressConfirm {
        ipaddress: Ipv4Addr::from([0xff, 0x00, 0xf0, 0x0f]),
    }
    .into();

    test_all(package, serialized);
}

#[test]

fn type_3() {
    let serialized: Vec<u8> = vec![
        // header:
        3, 5, // number:
        0x44, 0x33, 0x22, 0x11, // version:
        0xf7,
    ];

    let package = PeerQuery {
        number: 0x11_22_33_44,
        version: 0xf7,
    }
    .into();

    test_all(package, serialized);
}

#[test]

fn type_4() {
    let serialized: Vec<u8> = vec![4, 0];

    let package = PeerNotFound {}.into();

    test_all(package, serialized);
}

#[test]

fn type_5() {
    let serialized: Vec<u8> = vec![
        // header:
        5, 100, // number:
        4, 3, 2, 1, // name:
        84, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // flags:
        2, 0, // client_type:
        5, // hostname:
        104, 111, 115, 116, 46, 110, 97, 109, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // ipaddress:
        8, 9, 0x0a, 0x0b, // port:
        0x0d, 0x0c, // extension:
        0x0e, // pin:
        0x10, 0x0f, //timestamp:
        0x14, 0x13, 0x12, 0x11,
    ];

    let package = PeerReply {
        number: 0x01_02_03_04,
        name: String::from("Test").into(),
        flags: PeerReply::flags(true),
        client_type: ClientType::BaudotDynIp,
        hostname: String::from("host.name").into(),
        ipaddress: Ipv4Addr::from(0x08_09_0a_0b),
        port: 0x0c_0d,
        extension: 0x0e,
        pin: 0x0f_10,
        timestamp: 0x11_12_13_14,
    }
    .into();

    test_all(package, serialized);
}

#[test]

fn type_6() {
    let serialized: Vec<u8> = vec![6, 5, 0x0f, 0x11, 0x22, 0x33, 0x44];

    let package = FullQuery {
        server_pin: 0x44_33_22_11,
        version: 0x0f,
    }
    .into();

    test_all(package, serialized);
}

#[test]

fn type_7() {
    let serialized: Vec<u8> = vec![7, 5, 0x0f, 0x11, 0x22, 0x33, 0x44];

    let package = Login {
        server_pin: 0x44_33_22_11,
        version: 0x0f,
    }
    .into();

    test_all(package, serialized);
}

#[test]

fn type_8() {
    let serialized: Vec<u8> = vec![8, 0];

    let package = Acknowledge {}.into();

    test_all(package, serialized);
}

#[test]

fn type_9() {
    let serialized: Vec<u8> = vec![9, 0];

    let package = EndOfList {}.into();

    test_all(package, serialized);
}

#[test]

fn type_10() {
    let serialized: Vec<u8> = vec![
        // header:
        10, 41,  // version:
        240, // pattern:
        80, 97, 116, 116, 101, 114, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let package = PeerSearch {
        pattern: String::from("Pattern").into(),
        version: 0xf0,
    }
    .into();

    test_all(package, serialized);
}

#[test]

fn type_255() {
    let serialized: Vec<u8> = vec![
        // header:
        0xff, 22, // message:
        65, 110, 32, 69, 114, 114, 111, 114, 32, 104, 97, 115, 32, 111, 99, 99, 117, 114, 101, 100,
        33, 0,
    ];

    let package = Error {
        message: String::from("An Error has occured!"),
    }
    .into();

    test_all(package, serialized);
}
