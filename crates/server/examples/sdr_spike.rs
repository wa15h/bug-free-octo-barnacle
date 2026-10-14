//! W0-08, server-engineer first task 1(b): the SDR spike on app 480. D3's outcome is its result.
//! One binary, two modes; the repro steps are in the W0-08 PR. CI builds it and never runs it.
//!   Linux, no Steam client:  cargo run -p server --example sdr_spike -- server
//!   desktop, Steam running:  cargo run -p server --example sdr_spike -- client <server Steam ID>
//! ICE is off at both ends, so a connection that comes up is relayed. App 480 only, no token.
//! Exit codes: 0 one message each way; 1 failed or bad arguments (the last line says why).

use std::net::Ipv4Addr;
use std::process::exit;
use std::sync::LazyLock;
use std::thread::sleep;
use std::time::{Duration, Instant};
use steamworks::networking_types::ListenSocketEvent as Listen;
use steamworks::networking_types::NetworkingConnectionState as State;
use steamworks::networking_types::{
    NetworkingConfigEntry, NetworkingConfigValue, NetworkingIdentity, SendFlags,
};
use steamworks::{CallbackResult, Client, Server, ServerMode, SteamId};

const APP_ID: u32 = 480; // Spacewar, Valve's public test app.
const POLL: Duration = Duration::from_millis(1); // Adds at most about 1 ms to each end's turn.
const SEND: SendFlags = SendFlags::RELIABLE_NO_NAGLE; // Reliable, sent at once: no Nagle wait.
static START: LazyLock<Instant> = LazyLock::new(Instant::now);

/// Prints one line, stamped with the milliseconds since the spike started.
macro_rules! log {
    ($($arg:tt)*) => {{ print!("[{:>6} ms] ", START.elapsed().as_millis()); println!($($arg)*) }};
}

fn main() {
    LazyLock::force(&START);
    let mode = std::env::args().nth(1);
    let id = std::env::args().nth(2).and_then(|s| s.parse::<u64>().ok());
    match (mode.as_deref(), id) {
        (Some("server"), None) => server(),
        (Some("client"), Some(id)) => client(SteamId::from_raw(id)),
        _ => fail("usage: sdr_spike server | sdr_spike client <server Steam ID>"),
    }
}

fn server() {
    // App ID from the environment and these two ports, as in steamworks-rs examples/game-server.
    std::env::set_var("SteamAppId", APP_ID.to_string());
    let mode = ServerMode::AuthenticationAndSecure;
    let (server, steam) = Server::init(Ipv4Addr::UNSPECIFIED, 54320, 54321, mode, "0.0.0")
        .unwrap_or_else(|e| fail(&format!("Server::init: {e:?}")));
    server.set_dedicated_server(true);
    server.log_on_anonymous();
    steam.networking_utils().init_relay_network_access();
    let sockets = server.networking_sockets();
    let listen = sockets.create_listen_socket_p2p(0, relay_only());
    let listen = listen.unwrap_or_else(|_| fail("create_listen_socket_p2p gave no socket"));
    log!("server: init ok; logon and relay access started; listen socket on P2P port 0");
    let (mut peer, mut replied) = (None, false);
    loop {
        server.process_callbacks(|cb| {
            log_callback(&cb);
            if let CallbackResult::SteamServersConnected(_) = cb {
                let id = server.steam_id().raw();
                log!("server: Steam ID {id}; run client mode with this number");
            }
        });
        while let Some(event) = listen.try_receive_event() {
            match event {
                Listen::Connecting(request) => log!("server: accept {:?}", request.accept()),
                Listen::Connected(connected) => peer = Some(connected.take_connection()),
                Listen::Disconnected(_) if replied => {
                    log!("server: done, one message each way");
                    exit(0);
                }
                Listen::Disconnected(_) => peer = None,
            }
        }
        if let Some(conn) = peer.as_mut() {
            for message in conn.receive_messages(8).unwrap_or_default() {
                let text = String::from_utf8_lossy(message.data());
                let sent = conn.send_message(b"pong from sdr_spike server", SEND);
                log!("server: received {text:?}; reply sent: {sent:?}");
                replied = true;
            }
        }
        sleep(POLL);
    }
}

fn client(server_id: SteamId) {
    let steam = Client::init_app(APP_ID);
    let steam = steam.unwrap_or_else(|e| fail(&format!("Client::init_app: {e:?}")));
    let to = server_id.raw();
    log!("client: Steam client found; connecting to server {to}");
    steam.networking_utils().init_relay_network_access();
    let remote = NetworkingIdentity::new_steam_id(server_id);
    let sockets = steam.networking_sockets();
    let conn = sockets.connect_p2p(remote, 0, relay_only());
    let mut conn = conn.unwrap_or_else(|_| fail("connect_p2p gave no connection"));
    let mut sent_at = Instant::now();
    while START.elapsed() < Duration::from_secs(60) {
        steam.process_callbacks(|cb| log_callback(&cb));
        while let Some(event) = conn.try_receive_event() {
            if event.new_state == State::Connected {
                sent_at = Instant::now();
                let sent = conn.send_message(b"ping from sdr_spike client", SEND);
                log!("client: ping sent: {sent:?}");
            }
        }
        if let Some(message) = conn.receive_messages(1).unwrap_or_default().pop() {
            let text = String::from_utf8_lossy(message.data());
            let rtt = sent_at.elapsed().as_millis();
            log!("client: received {text:?}; round trip {rtt} ms");
            drop(conn); // Closes the connection, so the server sees us leave and exits 0.
            sleep(Duration::from_millis(500)); // Lets Steam's network thread send the close.
            log!("client: done, one message each way");
            exit(0);
        }
        sleep(POLL);
    }
    fail("no round trip within 60 s; the connection lines above say how far it got")
}

/// ICE off (0 is the SDK's P2P_Transport_ICE_Enable_Disable), so the only route is the relay.
fn relay_only() -> Vec<NetworkingConfigEntry> {
    let ice = NetworkingConfigValue::P2PTransportICEEnable;
    vec![NetworkingConfigEntry::new_int32(ice, 0)]
}

/// Logs the callbacks the spike reports: Steam logon, relay status, and connection states.
fn log_callback(cb: &CallbackResult) {
    match cb {
        CallbackResult::SteamServersConnected(_) => log!("steam: logged on"),
        CallbackResult::SteamServerConnectFailure(f) => log!("steam: logon failed: {f:?}"),
        CallbackResult::SteamServersDisconnected(d) => log!("steam: logged off: {d:?}"),
        CallbackResult::RelayNetworkStatusCallback(status) => log!("relay: {status:?}"),
        CallbackResult::NetConnectionStatusChanged(change) => {
            let (was, now) = (change.old_state, &change.connection_info);
            log!("connection: was {was:?}, now {now:?}");
        }
        _ => {}
    }
}

fn fail(why: &str) -> ! {
    log!("FAIL: {why}");
    exit(1)
}
