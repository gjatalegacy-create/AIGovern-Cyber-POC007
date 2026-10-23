use poc007_opls::{canonical_cases, run_case, vault_fixture};
fn main() {
    let vault=vault_fixture();
    println!("POC007 OPLS | vault_candidates={}",vault.len());
    for case in canonical_cases() {
        let run=run_case(&vault,case).expect("case execution");
        println!("CASE={} OUTCOME={} VERIFIED={}",run.case.id,run.shadow.final_state,run.shadow.verified);
        println!("  LIGHT={}",run.light.trace_digest);
        println!("  ASHT={}",run.asht.trace_digest);
        println!("  NBES={}",run.nbes.as_ref().map(|n|n.claim_digest.as_str()).unwrap_or("NOT_REACHED"));
        println!("  SHADOW={}",run.shadow.receipt_digest);
        println!("  REASON={}",run.shadow.reason);
        println!("  iZ.achieved={}",run.shadow.iz.achieved);
        println!("  iZ.critique={}",run.shadow.iz.critique);
        println!("  next_i0={}",run.shadow.iz.next_i0_question);
    }
}
