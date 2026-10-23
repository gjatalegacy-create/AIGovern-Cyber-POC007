#!/usr/bin/env python3
import hashlib, json, pathlib, sys
ROOT=pathlib.Path(__file__).resolve().parents[1]

def h(s): return hashlib.sha256(s.encode()).hexdigest()

def load():
    v=json.loads((ROOT/'fixtures/vault_40_candidates.json').read_text())['candidates']
    c=json.loads((ROOT/'fixtures/i0_cases.json').read_text())['cases']
    return v,c

def exact(v,axis,pol,term,scope):
    return next((x for x in v if x['axis']==axis and x['polarity']==pol and x['term']==term and x['scope']==scope),None)

def run(v,c):
    z={'state':0,'knowledge_creation':0,'verdict':0,'authority':0,'persistence':0,'trigger':1}
    obligation={'case_id':c['id'],'scope':c['scope'],'xi_term':c['xi_term'],'yi_term':c['yi_term'],
                'require_exact_positive':True,'require_exact_negative':True,'same_case_required':True,'same_scope_required':True}
    od=h('OPLS/COLLISION|'+'|'.join(map(str,[c['id'],c['scope'],c['xi_term'],c['yi_term'],True,True,True,True])))
    field='ESS-MAI/request-bound-epistemic'; space=c['scope']
    condition='exact positive AND exact negative must be request-bound to the same Xi/Yi case'
    consequence='if unmet: fail-closed with scoped gap evidence and next-i0 question'
    law=h('OPLS/DBAB|'+'|'.join([c['id'],field,space,c['xi_term'],c['yi_term'],condition,consequence,od]))
    promise=h('OPLS/BESA/PROMISE|'+c['id']+'|'+law)
    light=h('OPLS/LIGHT/TRACE|'+'|'.join([c['id'],c['scope'],law,promise,'1']))
    xp=exact(v,'X','positive',c['xi_term'],c['scope']); yp=exact(v,'Y','positive',c['yi_term'],c['scope'])
    xn=exact(v,'X','negative',c['xi_term'],c['scope']); yn=exact(v,'Y','negative',c['yi_term'],c['scope'])
    missing=[]
    if not xp: missing.append(f'no exact positive Xi={c["xi_term"]}')
    if not yp: missing.append(f'no exact positive Yi={c["yi_term"]}')
    if not xn: missing.append(f'no exact negative -Xi={c["xi_term"]}')
    if not yn: missing.append(f'no exact negative -Yi={c["yi_term"]}')
    if missing:
        outcome='FAIL_CLOSED'; pm=nm=0; reason='; '.join(missing)
        asht=h('OPLS/ASHT/FAIL|'+c['id']+'|'+light+'|'+reason); nbes=None
        achieved='request/obligation/promise trace preserved; no fabricated candidate'
        critique=reason
        q=f"Which exact additional evidence in scope '{c['scope']}' supplies the missing request-bound Xi/Yi positive+negative pair for case '{c['id']}' so a new i0 can lawfully continue?"
    else:
        pm=(xp['mass_milli']+yp['mass_milli'])//2; nm=(xn['mass_milli']+yn['mass_milli'])//2
        outcome='RIGOROUS_NEGATIVE' if nm>=pm else 'CONSTRUCTIVE'
        reason=f'exact pair collision completed: +mass={pm} -mass={nm} => {outcome}'
        asht=h('OPLS/ASHT/TRACE|'+'|'.join(map(str,[c['id'],light,xp['id'],yp['id'],xn['id'],yn['id'],pm,nm,outcome])))
        fulfil='PROMISE_KEPT_RIGOROUS_NEGATIVE_ADVANCE' if outcome=='RIGOROUS_NEGATIVE' else 'PROMISE_KEPT_CONSTRUCTIVE_ADVANCE'
        nbes=h('OPLS/NBES|'+'|'.join([c['id'],asht,promise,fulfil]))
        if outcome=='RIGOROUS_NEGATIVE':
            achieved=f'positive support mass={pm} retained as context'; critique=f'negative mass={nm} is superior and must guide next advancement'
            q=f"What new evidence can answer the verified negative critique for case '{c['id']}' without changing scope '{c['scope']}' silently?"
        else:
            achieved=f'constructive support mass={pm} > negative mass={nm}'; critique=f'negative critique mass={nm} retained as advancement boundary'
            q=f"What additional indicia would stress-test the constructive result for case '{c['id']}' inside scope '{c['scope']}'?"
    shadow=h('OPLS/SHADOW|'+'|'.join([c['id'],light,asht,nbes or 'NO_NBES','True',outcome]))
    return {'case_id':c['id'],'outcome':outcome,'verified':True,'positive_mass_milli':pm,'negative_mass_milli':nm,
            'light_trace':light,'asht_trace':asht,'nbes_claim':nbes,'shadow_receipt':shadow,'reason':reason,
            'iZ':{'achieved':achieved,'critique':critique,'next_i0_question':q},'zero_noi':z,'collision_obligation':obligation,
            'dbab_law_digest':law,'besa_promise_digest':promise}

def main():
    v,cases=load(); assert len(v)==40
    results=[run(v,c) for c in cases]
    assert [r['outcome'] for r in results]==['FAIL_CLOSED','RIGOROUS_NEGATIVE','CONSTRUCTIVE']
    assert 'x11' in results[0]['reason'] and 'scope-alpha' in results[0]['iZ']['next_i0_question']
    assert results[1]['positive_mass_milli'] < results[1]['negative_mass_milli']
    assert 'positive support' in results[1]['iZ']['achieved'] and 'negative mass' in results[1]['iZ']['critique']
    assert results[2]['positive_mass_milli'] > results[2]['negative_mass_milli']
    assert 'negative critique' in results[2]['iZ']['critique']
    # Falsifier controls
    v_missing=[x for x in v if x['id']!='XN07']
    assert run(v_missing,cases[2])['outcome']=='FAIL_CLOSED'
    c_scope=dict(cases[2]); c_scope['scope']='scope-beta'
    assert run(v,c_scope)['outcome']=='FAIL_CLOSED'
    out={'schema':'ess-mai.poc007.oracle-results.v1','fixture_candidates':len(v),'results':results,
         'falsifier_controls':{'missing_negative_XN07':'PASS_FAIL_CLOSED','scope_drift':'PASS_FAIL_CLOSED'}}
    (ROOT/'evidence/reference_oracle_results.json').write_text(json.dumps(out,indent=2))
    print(json.dumps(out,indent=2))
if __name__=='__main__': main()
