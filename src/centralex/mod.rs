pub mod packages;

pub use super::client::{End, Heartbeat, Reject};

use packages::{RemAck, RemCall, RemConfirm, RemConnect};

package! {Centralex,
    Heartbeat = 0x00,
    End = 0x03,
    Reject = 0x04,
    RemConnect = 0x81,
    RemConfirm = 0x82,
    RemCall = 0x83,
    RemAck = 0x84,
}

#[test]
fn construct_package() {
    let mut package = RemAck {}.to_package();

    package.downcast_mut::<RemAck>().unwrap();
}
