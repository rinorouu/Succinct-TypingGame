use sp1_sdk::{ProverClient, SP1Stdin};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ProofData {
    pub player_name: String,
    pub score: u32,
}

pub fn generate_sp1_proof(player_name: String, score: u32) -> String {
    // Create a new stdin for the prover
    let mut stdin = SP1Stdin::new();
    
    // Write the input data
    stdin.write(&ProofData { player_name, score });
    
    // Get the client and execute the prover
    let client = ProverClient::new();
    let (_, proof) = client.execute("typing-speed-test-program", &stdin).unwrap();
    
    // Serialize the proof to a string
    proof
}
