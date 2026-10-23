use poc007_opls::*;

#[test]
fn three_i0_end_to_end_contract() {
    let vault=vault_fixture();
    let cases=canonical_cases();
    let a=run_case(&vault,cases[0].clone()).unwrap();
    let b=run_case(&vault,cases[1].clone()).unwrap();
    let c=run_case(&vault,cases[2].clone()).unwrap();
    assert_eq!(a.shadow.final_state,EpistemicOutcome::FailClosed);
    assert_eq!(b.shadow.final_state,EpistemicOutcome::RigorousNegative);
    assert_eq!(c.shadow.final_state,EpistemicOutcome::Constructive);
    assert!(a.shadow.verified && b.shadow.verified && c.shadow.verified);
    assert!(b.nbes.unwrap().promise_kept);
    assert!(c.nbes.unwrap().promise_kept);
}

#[test]
fn obligation_is_not_positive_only() {
    let mut vault=vault_fixture();
    vault.retain(|c| c.id!="YN07");
    let c=canonical_cases()[2].clone();
    let r=run_case(&vault,c).unwrap();
    assert_eq!(r.shadow.final_state,EpistemicOutcome::FailClosed);
}
