use linked_hash_set::LinkedHashSet;

use crate::{parse_state::ParseState, rule::Rule, symbol::Symbol};

#[derive(Debug)]
pub enum Forest {
    Leaf { kind: String },
    Node { kind: String, leaves: Vec<Forest> },
    Nodes { nodes: Vec<Forest> },
}

// 0
//   "P"[0] ::=  • "S" @1-0
//   "S"[0] ::=  • "S" Plus "S" @1-0
//   "S"[1] ::=  • "T" @1-0
//   "T"[0] ::=  • Int @1-0
// 1
//   "T"[0] ::= Int •  @1-1
//   "S"[1] ::= "T" •  @1-1
//   "P"[0] ::= "S" •  @1-1
//   "S"[0] ::= "S" • Plus "S" @1-1
// 2
//   "S"[0] ::= "S" Plus • "S" @1-2
//   "S"[0] ::=  • "S" Plus "S" @3-2
//   "S"[1] ::=  • "T" @3-2
//   "T"[0] ::=  • Int @3-2
// 3
//   "T"[0] ::= Int •  @3-3
//   "S"[1] ::= "T" •  @3-3
//   "S"[0] ::= "S" Plus "S" •  @1-3
//   "S"[0] ::= "S" • Plus "S" @3-3
//   "P"[0] ::= "S" •  @1-3
//   "S"[0] ::= "S" • Plus "S" @1-3
// 4
//   "S"[0] ::= "S" Plus • "S" @3-4
//   "S"[0] ::= "S" Plus • "S" @1-4
//   "S"[0] ::=  • "S" Plus "S" @5-4
//   "S"[1] ::=  • "T" @5-4
//   "T"[0] ::=  • Int @5-4
// 5
//   "T"[0] ::= Int •  @5-5
//   "S"[1] ::= "T" •  @5-5
//   "S"[0] ::= "S" Plus "S" •  @3-5
//   "S"[0] ::= "S" Plus "S" •  @1-5
//   "S"[0] ::= "S" • Plus "S" @5-5
//   "S"[0] ::= "S" • Plus "S" @3-5
//   "P"[0] ::= "S" •  @1-5
//   "S"[0] ::= "S" • Plus "S" @1-5
pub(crate) fn build_forest(
    grammar: &[Rule],
    states_vector: &Vec<LinkedHashSet<ParseState>>,
) -> Forest {
    eprintln!();
    eprintln!("Elizabeth");
    let mut E: Vec<LinkedHashSet<ParseState>> =
        states_vector.iter().map(|_| LinkedHashSet::new()).collect();

    // Set E0 to be the items (S ::= ·α, 0).
    for state in &states_vector[0] {
        if state.dot_index == 0 && state.end_lexeme_index == 0 {
            E[0].insert(state.clone());
        }
    }

    // For i > 0 initialise Ei by adding the item
    // p = (A ::= αai · β, j) for each q = (A ::= α · aiβ, j) ∈ Ei−1
    for i in 1..2
    // states_vector.len()
    {
        for q in &E[i - 1].clone() {
            for p in &states_vector[i] {
                if p.rule == q.rule && p.dot_index == q.dot_index + 1 {
                    // TODO: and, if α != empty, creating a
                    // predecessor pointer labelled i − 1 from q to p 
                    E[i].insert(ParseState {
                        dot_index: q.dot_index + 1,
                        ..q.clone()
                    });
                }
            }
        }

        // Before initialising Ei+1 complete Ei as follows
        if i + 1 < states_vector.len() {
            // For each item q = (B ::= γ · Dδ, k) ∈ Ei
            // and each rule p = D ::= ρ,
            // (D ::= ·ρ, i) is added to Ei
            for q in &E[i].clone() {
                if let Some(symbol) = q.rule.to.get(q.dot_index) {
                    if let Symbol::NonTerminal(_) = symbol {
                        for p in grammar.iter() {
                            if p.from == q.rule.to[q.dot_index] {
                                E[i].insert(ParseState {
                                    rule:             p.clone(),
                                    dot_index:        0,
                                    end_lexeme_index: i,
                                });
                            }
                        }
                    }
                }
            }
            // For each item t = (B ::= τ ·, k) ∈ Ei
            // and each corresponding item q = (D ::= τ · Bμ, h) ∈ Ek,
            // if there is no item p = (D ::= τB · μ, h) ∈ Ei create one
            for t in &E[i].clone() {
                if t.completed() {
                    for q in &E[t.end_lexeme_index] {
                        if q.dot_index == t.dot_index
                        // && q.rule.to.starts_with(t.rule.to)
                        {}
                    }
                }
            }
        }
    }

    for (index, states) in E.iter().enumerate() {
        eprintln!("{index}");
        for state in states {
            eprintln!("  {state}");
        }
    }

    // and, if α = , creating a
    // predecessor pointer labelled i − 1 from q to p 
    // for states in states_vector {
    //     eprintln!("--");
    //     for state in states {
    //         eprintln!("{state}");
    //     }
    // }
    // println!("build_forest");
    // println!("  state: {}", state);

    // let state = simplify(states, state);
    // println!("  simplify(state): {}", state);

    // let mut leaves: Vec<Forest> = Vec::new();
    // let mut end_lexeme_index = state.end_lexeme_index;

    // for symbol in state.rule.to.iter().rev() {
    //     println!(
    //         "  alternatives({symbol} @ {}-{end_lexeme_index}):",
    //         state.start_lexeme_index
    //     );

    //     match symbol {
    //         Symbol::NonTerminal(_) => {
    //             for alternative in alternatives(
    //                 states,
    //                 &symbol,
    //                 state.start_lexeme_index,
    //                 end_lexeme_index,
    //             ) {
    //                 println!("    {}", alternative);
    //                 leaves.push(build_forest(states, &alternative));
    //                 end_lexeme_index = alternative.start_lexeme_index;
    //                 break;
    //             }
    //         }
    //         Symbol::Terminal(lexeme_kind) => {
    //             println!("    {:?}", lexeme_kind);
    //             leaves.push(Forest::Leaf { kind: format!("{lexeme_kind:?}") });
    //             end_lexeme_index -= 1;
    //         }
    //     }
    // }

    Forest::Leaf { kind: "X".to_string() }
}

// fn alternatives(
//     states: &Vec<LinkedHashSet<ParseState>>,
//     symbol: &Symbol,
//     start_lexeme_index: usize,
//     end_lexeme_index: usize,
// ) -> Vec<ParseState>
// {
//     states[end_lexeme_index]
//         .iter()
//         .filter(|state| {
//             state.start_lexeme_index >= start_lexeme_index
//                 && state.end_lexeme_index <= end_lexeme_index
//                 && (state.end_lexeme_index - state.start_lexeme_index
//                     < end_lexeme_index - start_lexeme_index)
//                 && state.rule.from == *symbol
//         })
//         .map(|state| simplify(states, state))
//         .collect()
// }

// pub(crate) fn simplify(
//     states: &Vec<LinkedHashSet<ParseState>>,
//     state: &ParseState,
// ) -> ParseState
// {
//     if state.rule.to.len() == 1 {
//         for st in &states[state.end_lexeme_index] {
//             if st != state
//                 && st.start_lexeme_index == state.start_lexeme_index
//                 && st.end_lexeme_index == state.end_lexeme_index
//                 && st.rule.from == state.rule.to[0]
//             {
//                 return simplify(states, st);
//             }
//         }
//     }

//     state.clone()
// }
