//! POC 007 — Obligation Promise Law to Success (OPLS)
//!
//! Scope: standalone, dependency-free experimental capsule derived from the
//! ESS-MAI v1.8.9 source semantics. It does NOT claim full-engine closure.
//!
//! Core thesis under test:
//!   lawful_success != positive_support_alone
//!   lawful_success requires a bounded promise that exact positive and exact
//!   negative evidence for the same Xi/Yi case are both processed, traced,
//!   and finally verified.

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis { X, Y }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity { Positive, Negative }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    pub id: String,
    pub axis: Axis,
    pub polarity: Polarity,
    pub term: String,
    pub scope: String,
    pub mass_milli: u16,
    pub evidence: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct I0Case {
    pub id: String,
    pub xi_term: String,
    pub yi_term: String,
    pub scope: String,
    pub question: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZeroNoiVector {
    pub state: u8,
    pub knowledge_creation: u8,
    pub verdict: u8,
    pub authority: u8,
    pub persistence: u8,
    pub trigger: u8,
}

impl ZeroNoiVector {
    /// v1.8.9 documented vector: zero state/knowledge/verdict/authority/persistence;
    /// trigger only.
    pub const fn canonical() -> Self {
        Self { state:0, knowledge_creation:0, verdict:0, authority:0, persistence:0, trigger:1 }
    }
    pub const fn valid(self) -> bool {
        self.state == 0 && self.knowledge_creation == 0 && self.verdict == 0
            && self.authority == 0 && self.persistence == 0 && self.trigger == 1
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollisionObligation {
    pub case_id: String,
    pub scope: String,
    pub xi_term: String,
    pub yi_term: String,
    pub require_exact_positive: bool,
    pub require_exact_negative: bool,
    pub same_case_required: bool,
    pub same_scope_required: bool,
}

/// Light-side obligation law.
///
/// Project-specific composition inspired by deontic/input-output logic but
/// deliberately bounded to one ESS-MAI case. It grants no reasoning/verdict.
pub fn activate_collision_obligation(case: &I0Case, z: ZeroNoiVector) -> Result<CollisionObligation, String> {
    if !z.valid() { return Err("0-NOI vector escaped zero-authority trigger semantics".into()); }
    Ok(CollisionObligation {
        case_id: case.id.clone(),
        scope: case.scope.clone(),
        xi_term: case.xi_term.clone(),
        yi_term: case.yi_term.clone(),
        require_exact_positive: true,
        require_exact_negative: true,
        same_case_required: true,
        same_scope_required: true,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbabLawWitness {
    pub case_id: String,
    pub field: String,
    pub space: String,
    pub requested_xi: String,
    pub requested_yi: String,
    pub condition: String,
    pub consequence: String,
    pub obligation_digest: String,
    pub law_digest: String,
}

impl DbabLawWitness {
    /// DBAB does not reason here. It witnesses the requested field/space,
    /// obligation and consequence before BESA promises that exact law object.
    pub fn from_obligation(case: &I0Case, o: &CollisionObligation) -> Self {
        let obligation_digest = sha256_hex(format!(
            "OPLS/COLLISION|{}|{}|{}|{}|{}|{}|{}|{}",
            o.case_id, o.scope, o.xi_term, o.yi_term,
            o.require_exact_positive, o.require_exact_negative,
            o.same_case_required, o.same_scope_required
        ).as_bytes());
        let field = "ESS-MAI/request-bound-epistemic".to_string();
        let space = case.scope.clone();
        let condition = "exact positive AND exact negative must be request-bound to the same Xi/Yi case".to_string();
        let consequence = "if unmet: fail-closed with scoped gap evidence and next-i0 question".to_string();
        let law_digest = sha256_hex(format!(
            "OPLS/DBAB|{}|{}|{}|{}|{}|{}|{}|{}",
            case.id, field, space, case.xi_term, case.yi_term,
            condition, consequence, obligation_digest
        ).as_bytes());
        Self {
            case_id: case.id.clone(), field, space,
            requested_xi: case.xi_term.clone(), requested_yi: case.yi_term.clone(),
            condition, consequence, obligation_digest, law_digest,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BesaPromise {
    pub case_id: String,
    pub dbab_law_digest: String,
    pub promise_digest: String,
}

impl BesaPromise {
    /// BESA keeps its own role: it gives its word to the exact DBAB-witnessed
    /// processing law. DBAB does not choose on behalf of BESA.
    pub fn give(case: &I0Case, dbab: &DbabLawWitness) -> Result<Self, String> {
        if case.id != dbab.case_id { return Err("BESA refused DBAB case drift".into()); }
        let promise_digest = sha256_hex(format!(
            "OPLS/BESA/PROMISE|{}|{}", case.id, dbab.law_digest
        ).as_bytes());
        Ok(Self { case_id: case.id.clone(), dbab_law_digest: dbab.law_digest.clone(), promise_digest })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LightTrace {
    pub case_id: String,
    pub zero_noi: ZeroNoiVector,
    pub collision: CollisionObligation,
    pub dbab: DbabLawWitness,
    pub besa: BesaPromise,
    pub trace_digest: String,
}

pub fn light_trace(case: &I0Case) -> Result<LightTrace, String> {
    let zero_noi = ZeroNoiVector::canonical();
    let collision = activate_collision_obligation(case, zero_noi)?;
    let dbab = DbabLawWitness::from_obligation(case, &collision);
    let besa = BesaPromise::give(case, &dbab)?;
    let trace_digest = sha256_hex(format!(
        "OPLS/LIGHT/TRACE|{}|{}|{}|{}|{}",
        case.id, collision.scope, dbab.law_digest, besa.promise_digest, zero_noi.trigger
    ).as_bytes());
    Ok(LightTrace { case_id: case.id.clone(), zero_noi, collision, dbab, besa, trace_digest })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EpistemicOutcome { Constructive, RigorousNegative, FailClosed }

impl fmt::Display for EpistemicOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Constructive => write!(f, "CONSTRUCTIVE"),
            Self::RigorousNegative => write!(f, "RIGOROUS_NEGATIVE"),
            Self::FailClosed => write!(f, "FAIL_CLOSED"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactPair {
    pub x_pos: Candidate,
    pub y_pos: Candidate,
    pub x_neg: Candidate,
    pub y_neg: Candidate,
}

fn exact<'a>(vault: &'a [Candidate], axis: Axis, polarity: Polarity, term: &str, scope: &str) -> Option<&'a Candidate> {
    vault.iter().find(|c| c.axis == axis && c.polarity == polarity && c.term == term && c.scope == scope)
}

pub fn exact_pair(vault: &[Candidate], case: &I0Case, light: &LightTrace) -> Result<ExactPair, String> {
    if light.case_id != case.id { return Err("ASHT rejected Light trace case drift".into()); }
    if light.besa.dbab_law_digest != light.dbab.law_digest { return Err("ASHT rejected BESA/DBAB law drift".into()); }
    let xp = exact(vault, Axis::X, Polarity::Positive, &case.xi_term, &case.scope)
        .ok_or_else(|| format!("request-bound relevance found no exact positive candidate for Xi={}", case.xi_term))?;
    let yp = exact(vault, Axis::Y, Polarity::Positive, &case.yi_term, &case.scope)
        .ok_or_else(|| format!("request-bound relevance found no exact positive candidate for Yi={}", case.yi_term))?;
    let xn = exact(vault, Axis::X, Polarity::Negative, &case.xi_term, &case.scope)
        .ok_or_else(|| format!("request-bound relevance found no exact negative candidate for -Xi={}", case.xi_term))?;
    let yn = exact(vault, Axis::Y, Polarity::Negative, &case.yi_term, &case.scope)
        .ok_or_else(|| format!("request-bound relevance found no exact negative candidate for -Yi={}", case.yi_term))?;
    Ok(ExactPair { x_pos:xp.clone(), y_pos:yp.clone(), x_neg:xn.clone(), y_neg:yn.clone() })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AshtTrace {
    pub case_id: String,
    pub parent_light_digest: String,
    pub exact_pair: bool,
    pub positive_mass_milli: u16,
    pub negative_mass_milli: u16,
    pub outcome: EpistemicOutcome,
    pub reason: String,
    pub trace_digest: String,
}

pub fn asht_reason(vault: &[Candidate], case: &I0Case, light: &LightTrace) -> AshtTrace {
    match exact_pair(vault, case, light) {
        Err(reason) => {
            let trace_digest = sha256_hex(format!(
                "OPLS/ASHT/FAIL|{}|{}|{}", case.id, light.trace_digest, reason
            ).as_bytes());
            AshtTrace {
                case_id:case.id.clone(), parent_light_digest:light.trace_digest.clone(), exact_pair:false,
                positive_mass_milli:0, negative_mass_milli:0, outcome:EpistemicOutcome::FailClosed,
                reason, trace_digest,
            }
        }
        Ok(pair) => {
            let positive_mass_milli = ((pair.x_pos.mass_milli as u32 + pair.y_pos.mass_milli as u32) / 2) as u16;
            let negative_mass_milli = ((pair.x_neg.mass_milli as u32 + pair.y_neg.mass_milli as u32) / 2) as u16;
            let outcome = if negative_mass_milli >= positive_mass_milli {
                EpistemicOutcome::RigorousNegative
            } else {
                EpistemicOutcome::Constructive
            };
            let reason = format!(
                "exact pair collision completed: +mass={} -mass={} => {}",
                positive_mass_milli, negative_mass_milli, outcome
            );
            let trace_digest = sha256_hex(format!(
                "OPLS/ASHT/TRACE|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                case.id, light.trace_digest,
                pair.x_pos.id, pair.y_pos.id, pair.x_neg.id, pair.y_neg.id,
                positive_mass_milli, negative_mass_milli, outcome
            ).as_bytes());
            AshtTrace { case_id:case.id.clone(), parent_light_digest:light.trace_digest.clone(), exact_pair:true,
                positive_mass_milli, negative_mass_milli, outcome, reason, trace_digest }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NbesClaim {
    pub case_id: String,
    pub parent_asht_digest: String,
    pub promise_kept: bool,
    pub fulfilment: String,
    pub claim_digest: String,
}

pub fn nbes_claim(case: &I0Case, light: &LightTrace, asht: &AshtTrace) -> Result<NbesClaim, String> {
    if asht.outcome == EpistemicOutcome::FailClosed { return Err("NBES unreachable: ASHT fail-closed before fulfilment".into()); }
    if asht.parent_light_digest != light.trace_digest { return Err("NBES rejected Light/ASHT trace break".into()); }
    if light.besa.dbab_law_digest != light.dbab.law_digest { return Err("NBES rejected promise/law drift".into()); }
    if !asht.exact_pair { return Err("NBES rejected incomplete collision obligation".into()); }
    let fulfilment = match asht.outcome {
        EpistemicOutcome::Constructive => "PROMISE_KEPT_CONSTRUCTIVE_ADVANCE",
        EpistemicOutcome::RigorousNegative => "PROMISE_KEPT_RIGOROUS_NEGATIVE_ADVANCE",
        EpistemicOutcome::FailClosed => unreachable!(),
    }.to_string();
    let claim_digest = sha256_hex(format!(
        "OPLS/NBES|{}|{}|{}|{}", case.id, asht.trace_digest, light.besa.promise_digest, fulfilment
    ).as_bytes());
    Ok(NbesClaim { case_id:case.id.clone(), parent_asht_digest:asht.trace_digest.clone(), promise_kept:true, fulfilment, claim_digest })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IzState {
    pub achieved: String,
    pub critique: String,
    pub next_i0_question: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShadowReceipt {
    pub case_id: String,
    pub verified: bool,
    pub final_state: EpistemicOutcome,
    pub reason: String,
    pub iz: IzState,
    pub receipt_digest: String,
}

pub fn shadow_verify(case: &I0Case, light: &LightTrace, asht: &AshtTrace, nbes: Option<&NbesClaim>) -> ShadowReceipt {
    let mut verified = asht.parent_light_digest == light.trace_digest
        && light.besa.dbab_law_digest == light.dbab.law_digest
        && light.zero_noi.valid();

    let (reason, iz) = match asht.outcome {
        EpistemicOutcome::FailClosed => {
            if nbes.is_some() { verified = false; }
            let q = format!(
                "Which exact additional evidence in scope '{}' supplies the missing request-bound Xi/Yi positive+negative pair for case '{}' so a new i0 can lawfully continue?",
                case.scope, case.id
            );
            (
                format!("verified fail-closed: {}", asht.reason),
                IzState {
                    achieved:"request/obligation/promise trace preserved; no fabricated candidate".into(),
                    critique:asht.reason.clone(),
                    next_i0_question:q,
                }
            )
        }
        EpistemicOutcome::RigorousNegative => {
            match nbes {
                Some(c) if c.parent_asht_digest == asht.trace_digest && c.promise_kept => {},
                _ => verified = false,
            }
            (
                "verified rigorous-negative advance; negative critique remains living knowledge".into(),
                IzState {
                    achieved:format!("positive support mass={} retained as context", asht.positive_mass_milli),
                    critique:format!("negative mass={} is superior and must guide next advancement", asht.negative_mass_milli),
                    next_i0_question:format!("What new evidence can answer the verified negative critique for case '{}' without changing scope '{}' silently?", case.id, case.scope),
                }
            )
        }
        EpistemicOutcome::Constructive => {
            match nbes {
                Some(c) if c.parent_asht_digest == asht.trace_digest && c.promise_kept => {},
                _ => verified = false,
            }
            (
                "verified constructive advance; relevant negative critique is retained rather than erased".into(),
                IzState {
                    achieved:format!("constructive support mass={} > negative mass={}", asht.positive_mass_milli, asht.negative_mass_milli),
                    critique:format!("negative critique mass={} retained as advancement boundary", asht.negative_mass_milli),
                    next_i0_question:format!("What additional indicia would stress-test the constructive result for case '{}' inside scope '{}'?", case.id, case.scope),
                }
            )
        }
    };

    let receipt_digest = sha256_hex(format!(
        "OPLS/SHADOW|{}|{}|{}|{}|{}|{}",
        case.id, light.trace_digest, asht.trace_digest,
        nbes.map(|n| n.claim_digest.as_str()).unwrap_or("NO_NBES"),
        verified, asht.outcome
    ).as_bytes());
    ShadowReceipt { case_id:case.id.clone(), verified, final_state:asht.outcome, reason, iz, receipt_digest }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaseRun {
    pub case: I0Case,
    pub light: LightTrace,
    pub asht: AshtTrace,
    pub nbes: Option<NbesClaim>,
    pub shadow: ShadowReceipt,
}

pub fn run_case(vault: &[Candidate], case: I0Case) -> Result<CaseRun, String> {
    let light = light_trace(&case)?;
    let asht = asht_reason(vault, &case, &light);
    let nbes = if asht.outcome == EpistemicOutcome::FailClosed { None } else { Some(nbes_claim(&case, &light, &asht)?) };
    let shadow = shadow_verify(&case, &light, &asht, nbes.as_ref());
    Ok(CaseRun { case, light, asht, nbes, shadow })
}

pub fn vault_fixture() -> Vec<Candidate> {
    let mut out = Vec::with_capacity(40);
    for i in 1..=10u16 {
        let x = format!("x{:02}", i);
        let y = format!("y{:02}", i);
        let px = match i { 3 => 600, 7 => 950, _ => 700 + i * 5 };
        let py = match i { 3 => 580, 7 => 930, _ => 690 + i * 5 };
        let nx = match i { 3 => 920, 7 => 330, _ => 500 };
        let ny = match i { 3 => 900, 7 => 310, _ => 490 };
        out.push(Candidate { id:format!("XP{:02}",i), axis:Axis::X, polarity:Polarity::Positive, term:x.clone(), scope:"scope-alpha".into(), mass_milli:px, evidence:format!("positive evidence for {}",x) });
        out.push(Candidate { id:format!("YP{:02}",i), axis:Axis::Y, polarity:Polarity::Positive, term:y.clone(), scope:"scope-alpha".into(), mass_milli:py, evidence:format!("positive evidence for {}",y) });
        out.push(Candidate { id:format!("XN{:02}",i), axis:Axis::X, polarity:Polarity::Negative, term:x.clone(), scope:"scope-alpha".into(), mass_milli:nx, evidence:format!("negative critique for {}",x) });
        out.push(Candidate { id:format!("YN{:02}",i), axis:Axis::Y, polarity:Polarity::Negative, term:y.clone(), scope:"scope-alpha".into(), mass_milli:ny, evidence:format!("negative critique for {}",y) });
    }
    out
}

pub fn canonical_cases() -> Vec<I0Case> {
    vec![
        I0Case { id:"I0-A-FAIL-CLOSED".into(), xi_term:"x11".into(), yi_term:"y02".into(), scope:"scope-alpha".into(), question:"Find exact x11/y02 with both polarities".into() },
        I0Case { id:"I0-B-NEGATIVE-SUPERIOR".into(), xi_term:"x03".into(), yi_term:"y03".into(), scope:"scope-alpha".into(), question:"Evaluate x03/y03 with exact negative critique".into() },
        I0Case { id:"I0-C-POSITIVE-SUPERIOR".into(), xi_term:"x07".into(), yi_term:"y07".into(), scope:"scope-alpha".into(), question:"Evaluate x07/y07 with exact negative critique".into() },
    ]
}

// Minimal dependency-free SHA-256 used only for capsule integrity/binding.
// It is content integrity, not signer identity and not truth proof.
pub fn sha256_hex(input: &[u8]) -> String {
    const K:[u32;64]=[
        0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
        0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
        0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
        0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
        0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
        0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
        0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
        0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2];
    let mut h:[u32;8]=[0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19];
    let bit_len=(input.len() as u64)*8;
    let mut msg=input.to_vec(); msg.push(0x80);
    while (msg.len()%64)!=56 { msg.push(0); }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w=[0u32;64];
        for i in 0..16 { let j=i*4; w[i]=u32::from_be_bytes([chunk[j],chunk[j+1],chunk[j+2],chunk[j+3]]); }
        for i in 16..64 {
            let s0=w[i-15].rotate_right(7)^w[i-15].rotate_right(18)^(w[i-15]>>3);
            let s1=w[i-2].rotate_right(17)^w[i-2].rotate_right(19)^(w[i-2]>>10);
            w[i]=w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
        }
        let (mut a,mut b,mut c,mut d,mut e,mut f,mut g,mut hh)=(h[0],h[1],h[2],h[3],h[4],h[5],h[6],h[7]);
        for i in 0..64 {
            let s1=e.rotate_right(6)^e.rotate_right(11)^e.rotate_right(25);
            let ch=(e&f)^((!e)&g);
            let t1=hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0=a.rotate_right(2)^a.rotate_right(13)^a.rotate_right(22);
            let maj=(a&b)^(a&c)^(b&c);
            let t2=s0.wrapping_add(maj);
            hh=g; g=f; f=e; e=d.wrapping_add(t1); d=c; c=b; b=a; a=t1.wrapping_add(t2);
        }
        h[0]=h[0].wrapping_add(a); h[1]=h[1].wrapping_add(b); h[2]=h[2].wrapping_add(c); h[3]=h[3].wrapping_add(d);
        h[4]=h[4].wrapping_add(e); h[5]=h[5].wrapping_add(f); h[6]=h[6].wrapping_add(g); h[7]=h[7].wrapping_add(hh);
    }
    h.iter().map(|x| format!("{:08x}",x)).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn sha256_known_vector() {
        assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }
    #[test] fn fixture_is_exactly_40_with_10_per_component() {
        let v=vault_fixture(); assert_eq!(v.len(),40);
        assert_eq!(v.iter().filter(|c|c.axis==Axis::X&&c.polarity==Polarity::Positive).count(),10);
        assert_eq!(v.iter().filter(|c|c.axis==Axis::Y&&c.polarity==Polarity::Positive).count(),10);
        assert_eq!(v.iter().filter(|c|c.axis==Axis::X&&c.polarity==Polarity::Negative).count(),10);
        assert_eq!(v.iter().filter(|c|c.axis==Axis::Y&&c.polarity==Polarity::Negative).count(),10);
    }
    #[test] fn zero_noi_is_trigger_without_power() { assert!(ZeroNoiVector::canonical().valid()); }
    #[test] fn fail_closed_case_yields_scoped_next_i0_question() {
        let r=run_case(&vault_fixture(),canonical_cases().remove(0)).unwrap();
        assert_eq!(r.shadow.final_state,EpistemicOutcome::FailClosed); assert!(r.shadow.verified);
        assert!(r.shadow.iz.next_i0_question.contains("scope-alpha")); assert!(r.shadow.iz.critique.contains("x11"));
    }
    #[test] fn negative_superior_is_verified_advance_not_erasure() {
        let r=run_case(&vault_fixture(),canonical_cases().remove(1)).unwrap();
        assert_eq!(r.shadow.final_state,EpistemicOutcome::RigorousNegative); assert!(r.shadow.verified);
        assert!(r.shadow.iz.achieved.contains("positive support")); assert!(r.shadow.iz.critique.contains("negative mass"));
        assert!(r.nbes.as_ref().unwrap().promise_kept);
    }
    #[test] fn positive_superior_retains_negative_critique() {
        let r=run_case(&vault_fixture(),canonical_cases().remove(2)).unwrap();
        assert_eq!(r.shadow.final_state,EpistemicOutcome::Constructive); assert!(r.shadow.verified);
        assert!(r.shadow.iz.critique.contains("negative critique")); assert!(r.nbes.as_ref().unwrap().promise_kept);
    }
    #[test] fn missing_negative_blocks_even_when_positive_exists() {
        let mut v=vault_fixture(); v.retain(|c| !(c.id=="XN07"));
        let r=run_case(&v,canonical_cases().remove(2)).unwrap();
        assert_eq!(r.shadow.final_state,EpistemicOutcome::FailClosed);
        assert!(r.shadow.iz.critique.contains("no exact negative"));
    }
    #[test] fn scope_drift_fails_closed() {
        let mut c=canonical_cases().remove(2); c.scope="scope-beta".into();
        let r=run_case(&vault_fixture(),c).unwrap(); assert_eq!(r.shadow.final_state,EpistemicOutcome::FailClosed);
    }
    #[test] fn besa_rejects_dbab_case_drift() {
        let c=canonical_cases().remove(2); let o=activate_collision_obligation(&c,ZeroNoiVector::canonical()).unwrap();
        let mut d=DbabLawWitness::from_obligation(&c,&o); d.case_id="other".into();
        assert!(BesaPromise::give(&c,&d).is_err());
    }
    #[test] fn nbes_rejects_trace_break() {
        let v=vault_fixture(); let c=canonical_cases().remove(2); let l=light_trace(&c).unwrap(); let mut a=asht_reason(&v,&c,&l);
        a.parent_light_digest="00".repeat(32); assert!(nbes_claim(&c,&l,&a).is_err());
    }
}
