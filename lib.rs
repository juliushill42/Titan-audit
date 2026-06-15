use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
#[derive(Serialize, Deserialize, Clone)]
pub struct AuditEvent {
    pub position: u64,
    pub timestamp: String,
    pub action: String,
    pub data: String,
    pub prev_hash: String,
}
#[derive(Serialize, Deserialize)]
pub struct AuditLedger {
    pub events: Vec<AuditEvent>,
    pub root_hash: String,
}
pub fn compute_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
pub fn append_event(ledger: &mut AuditLedger, action: &str, data: &str, timestamp: &str) -> u64 {
    let position = ledger.events.len() as u64;
    let prev_hash = ledger.events.last().map(|e| e.prev_hash.clone()).unwrap_or_else(|| compute_hash("genesis"));
    let event_data = format!("{}|{}|{}|{}", position, timestamp, action, data);
    let event_hash = compute_hash(&event_data);
    let event = AuditEvent { position, timestamp: timestamp.to_string(), action: action.to_string(), data: data.to_string(), prev_hash: prev_hash };
    ledger.events.push(event);
    ledger.root_hash = event_hash;
    position
}
pub fn verify_ledger(ledger: &AuditLedger) -> bool {
    for (i, event) in ledger.events.iter().enumerate() {
        if event.position != i as u64 { return false; }
        let prev_hash = ledger.events.get(i - 1).map(|e| e.prev_hash.clone()).unwrap_or_else(|| compute_hash("genesis"));
        if event.prev_hash != prev_hash { return false; }
    }
    true
}
