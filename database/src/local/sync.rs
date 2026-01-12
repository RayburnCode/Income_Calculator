// This file contains the legacy libp2p-based P2P sync implementation.
// It is currently disabled by default. To use it, enable the "p2p-sync" feature in Cargo.toml
// The recommended approach is to use the REST API based sync instead.

#![cfg(feature = "p2p-sync")]

use libp2p::{
    SwarmBuilder, SwarmEvent, NetworkBehaviour,
    TokioTcpConfig, YamuxConfig,
    Multiaddr, PeerId, Swarm, Transport,
    RequestResponse, RequestResponseEvent, RequestResponseMessage,
    Ping, PingEvent,
    Identify, IdentifyEvent,
    core::upgrade,
    futures::StreamExt,
    identity,
    noise,
    request_response::cbor,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use sea_orm::{Database, DatabaseConnection};

// Define your data model for sync (e.g., a simplified income entry)
#[derive(Serialize, Deserialize, Clone, Debug)]
struct SyncData {
    id: i32,
    amount: f64,
    // Add other fields from your entities
}

// Define the behaviour
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Event")]
struct Behaviour {
    ping: Ping,
    identify: Identify,
    request_response: RequestResponse<cbor::Behaviour<SyncData, String>>,
}

#[derive(Debug)]
enum Event {
    Ping(PingEvent),
    Identify(IdentifyEvent),
    RequestResponse(RequestResponseEvent<SyncData, String>),
}

impl From<PingEvent> for Event {
    fn from(event: PingEvent) -> Self {
        Event::Ping(event)
    }
}

impl From<IdentifyEvent> for Event {
    fn from(event: IdentifyEvent) -> Self {
        Event::Identify(event)
    }
}

impl From<RequestResponseEvent<SyncData, String>> for Event {
    fn from(event: RequestResponseEvent<SyncData, String>) -> Self {
        Event::RequestResponse(event)
    }
}

// Sync handler
pub struct P2pSync {
    swarm: Swarm<Behaviour>,
    db: DatabaseConnection,
}

impl P2pSync {
    pub async fn new(db_url: &str) -> Result<Self, Box<dyn Error>> {
        let db = Database::connect(db_url).await?;
        
        // Generate a peer ID
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        // Build transport (TCP over Tailscale)
        let noise_keys = noise::Config::new(&local_key)?;
        let transport = TokioTcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(noise_keys)
            .multiplex(YamuxConfig::default())
            .boxed();
        
        // Create behaviour
        let behaviour = Behaviour {
            ping: Ping::default(),
            identify: Identify::new(identity::PublicKey::from(local_key.public())),
            request_response: RequestResponse::new(cbor::Behaviour::<SyncData, String>::default()),
        };
        
        // Create swarm
        let mut swarm = SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(Default::default(), noise_keys, YamuxConfig::default())?
            .with_dns()?
            .with_behaviour(|_key| behaviour)?
            .build();
        
        // Listen on Tailscale IP/port (e.g., /ip4/100.64.0.1/tcp/0) - adjust to your Tailscale IP
        swarm.listen_on("/ip4/100.64.0.1/tcp/0".parse()?)?;
        
        Ok(P2pSync { swarm, db })
    }
    
    pub async fn run(mut self) -> Result<(), Box<dyn Error>> {
        // Connect to peer via Tailscale DNS (e.g., other device's IP or DNS name)
        let peer_addr: Multiaddr = "/dns/iphone-income.your-tailnet.ts.net/tcp/0".parse()?;  // Replace with actual
        self.swarm.dial(peer_addr)?;
        
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::Behaviour(Event::RequestResponse(RequestResponseEvent::Message { message, .. })) => {
                    match message {
                        RequestResponseMessage::Request { request, channel, .. } => {
                            // Handle incoming sync data
                            self.handle_incoming_data(request).await?;
                            // Send ack
                            self.swarm.behaviour_mut().request_response.send_response(channel, "OK".to_string());
                        }
                        RequestResponseMessage::Response { response, .. } => {
                            println!("Received response: {}", response);
                        }
                    }
                }
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    println!("Connected to peer: {}", peer_id);
                    // Send local changes
                    let changes = self.get_local_changes().await?;
                    for change in changes {
                        self.swarm.behaviour_mut().request_response.send_request(&peer_id, change);
                    }
                }
                _ => {}
            }
        }
    }
    
    async fn handle_incoming_data(&self, data: SyncData) -> Result<(), Box<dyn Error>> {
        // Apply to DB, e.g., insert or update
        // Using SeaORM
        // Example: Assume an entity
        Ok(())
    }
    
    async fn get_local_changes(&self) -> Result<Vec<SyncData>, Box<dyn Error>> {
        // Query your DB for recent changes (e.g., using SeaORM)
        // Return as Vec<SyncData>
        Ok(vec![])  // Placeholder
    }
}

// In your main app (e.g., database/src/lib.rs), start sync
pub async fn start_sync(db_url: &str) -> Result<(), Box<dyn Error>> {
    let sync = P2pSync::new(db_url).await?;
    sync.run().await
}