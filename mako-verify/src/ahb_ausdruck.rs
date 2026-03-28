//! Parser and evaluator for AHB condition expressions.
//!
//! AHB (Anwendungshandbuch) rules attach condition expressions to fields, e.g.:
//! - `"Muss [556] ∧ [559]"` — mandatory if conditions 556 AND 559 hold
//! - `"Kann [931] ∨ [932]"` — optional if 931 OR 932
//! - `"X [931] [494]"` — space-separated refs mean AND (legacy notation)
//!
//! Operators (Unicode): `∧` (and), `∨` (or), `⊻` (xor)
//! Operators (legacy):  `U` (und=and), `O` (oder=or), `X` (xor)
//! Special: `UB` means "unless" = A ∧ ¬B

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A parsed AHB expression: qualifier + optional condition tree.
///
/// Multi-qualifier expressions like `"Muss [106] Soll [248]"` are split into
/// the first qualifier with its conditions; any trailing qualifiers are stored
/// as `weitere`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AhbAusdruck {
	Muss,
	Soll,
	Kann,
	X,
	Bedingt {
		basis: Box<AhbAusdruck>,
		bedingung: Bedingung,
	},
	/// Expression could not be parsed or is free text.
	Unbekannt(String),
}

/// A boolean condition tree over numbered/named AHB references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bedingung {
	/// A single reference, e.g. `[556]` → `Ref("556")`.
	/// Stored as string because some refs are alphanumeric (e.g. `[UB1]`, `[1P0..1]`).
	Ref(String),
	/// Logical AND (`∧`, `U`, or implicit space-separated refs).
	Und(Box<Bedingung>, Box<Bedingung>),
	/// Logical OR (`∨`, `O`).
	Oder(Box<Bedingung>, Box<Bedingung>),
	/// Logical XOR (`⊻`, or legacy `X` as operator).
	XOder(Box<Bedingung>, Box<Bedingung>),
	/// Negation.
	Nicht(Box<Bedingung>),
}

/// Three-valued logic result for condition evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BedingungsZustand {
	Wahr,
	Falsch,
	Unbestimmt,
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

/// Parse an AHB expression string into a structured `AhbAusdruck`.
pub fn parse_ahb_ausdruck(input: &str) -> AhbAusdruck {
	let trimmed = input.trim();
	if trimmed.is_empty() {
		return AhbAusdruck::Unbekannt(String::new());
	}

	// Try to match a qualifier prefix, then parse the rest as a condition.
	if let Some(rest) = strip_qualifier(trimmed, "Muss") {
		parse_nach_qualifier(AhbAusdruck::Muss, rest)
	} else if let Some(rest) = strip_qualifier(trimmed, "Soll") {
		parse_nach_qualifier(AhbAusdruck::Soll, rest)
	} else if let Some(rest) = strip_qualifier(trimmed, "Kann") {
		parse_nach_qualifier(AhbAusdruck::Kann, rest)
	} else if let Some(rest) = strip_short_qualifier(trimmed, "M") {
		parse_nach_qualifier(AhbAusdruck::Muss, rest)
	} else if let Some(rest) = strip_short_qualifier(trimmed, "S") {
		parse_nach_qualifier(AhbAusdruck::Soll, rest)
	} else if let Some(rest) = strip_short_qualifier(trimmed, "K") {
		parse_nach_qualifier(AhbAusdruck::Kann, rest)
	} else if let Some(rest) = strip_short_qualifier(trimmed, "X") {
		parse_nach_qualifier(AhbAusdruck::X, rest)
	} else {
		AhbAusdruck::Unbekannt(trimmed.to_string())
	}
}

/// Strip a long qualifier word (must be followed by whitespace, `[`, or end of string).
fn strip_qualifier<'a>(input: &'a str, qualifier: &str) -> Option<&'a str> {
	if !input.starts_with(qualifier) {
		return None;
	}
	let rest = &input[qualifier.len()..];
	if rest.is_empty() || rest.starts_with(char::is_whitespace) || rest.starts_with('[') {
		Some(rest.trim_start())
	} else {
		None
	}
}

/// Strip a single-char qualifier (M/S/K/X). Must be followed by whitespace or `[`, not
/// another letter (so "Muss" doesn't match "M").
fn strip_short_qualifier<'a>(input: &'a str, qualifier: &str) -> Option<&'a str> {
	if !input.starts_with(qualifier) {
		return None;
	}
	let rest = &input[qualifier.len()..];
	if rest.is_empty() {
		return Some(rest);
	}
	let next = rest.chars().next().unwrap();
	if next.is_whitespace() || next == '[' {
		Some(rest.trim_start())
	} else {
		None
	}
}

/// After extracting a qualifier, parse the remaining text.
/// Handles trailing qualifiers like `"Muss [106] Soll [248]"`.
fn parse_nach_qualifier(basis: AhbAusdruck, rest: &str) -> AhbAusdruck {
	if rest.is_empty() {
		return basis;
	}

	// Look for a trailing qualifier boundary (another Muss/Soll/Kann/M/S/K/X)
	// that starts a new section.
	let (condition_part, _trailing) = split_at_next_qualifier(rest);

	let condition_part = condition_part.trim();
	if condition_part.is_empty() {
		return basis;
	}

	// Try to parse the condition part as a boolean expression.
	let mut tokens = tokenize(condition_part);
	if tokens.is_empty() {
		return basis;
	}

	match parse_bedingung_expr(&mut tokens) {
		Some(bedingung) => AhbAusdruck::Bedingt {
			basis: Box::new(basis),
			bedingung,
		},
		None => AhbAusdruck::Unbekannt(format!("{}", rest)),
	}
}

/// Split text at the first occurrence of a standalone qualifier (Muss, Soll, Kann, M, S, K, X).
/// Returns (before, from_qualifier_onwards).
fn split_at_next_qualifier(input: &str) -> (&str, &str) {
	let qualifiers = ["Muss", "Soll", "Kann"];

	// Scan character by character looking for qualifier keywords not inside brackets.
	let mut i = 0;
	let bytes = input.as_bytes();
	let len = bytes.len();
	let mut bracket_depth: i32 = 0;
	let mut paren_depth: i32 = 0;

	while i < len {
		let ch = input[i..].chars().next().unwrap();
		match ch {
			'[' => bracket_depth += 1,
			']' => bracket_depth -= 1,
			'(' => paren_depth += 1,
			')' => paren_depth -= 1,
			_ => {}
		}

		if bracket_depth == 0 && paren_depth == 0 {
			for q in &qualifiers {
				if input[i..].starts_with(q) {
					// Check that it's not in the middle of a word
					let before_ok = i == 0 || input[..i].ends_with(char::is_whitespace);
					let after_pos = i + q.len();
					let after_ok = after_pos >= len
						|| input[after_pos..].starts_with(char::is_whitespace)
						|| input[after_pos..].starts_with('[');
					if before_ok && after_ok {
						return (&input[..i], &input[i..]);
					}
				}
			}

			// Check single-char qualifiers M/S/K/X at word boundary
			if matches!(ch, 'M' | 'S' | 'K' | 'X') {
				let before_ok = i == 0 || input[..i].ends_with(char::is_whitespace);
				let after_pos = i + 1;
				let after_ok = after_pos >= len
					|| input[after_pos..].starts_with(char::is_whitespace)
					|| input[after_pos..].starts_with('[');
				// Don't match at position 0 (that's the primary qualifier already stripped)
				if i > 0 && before_ok && after_ok {
					// Make sure it's not the start of a longer qualifier word
					let rest = &input[i..];
					let is_long_q = qualifiers.iter().any(|q| rest.starts_with(q));
					if !is_long_q {
						return (&input[..i], &input[i..]);
					}
				}
			}
		}

		i += ch.len_utf8();
	}

	(input, "")
}

// ---------------------------------------------------------------------------
// Tokenizer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
	Ref(String),    // [123] or [UB1]
	Und,            // ∧ or U
	Oder,           // ∨ or O
	XOder,          // ⊻ or X (as operator)
	UndNicht,       // UB (A and not B)
	LParen,         // (
	RParen,         // )
}

fn tokenize(input: &str) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut chars = input.chars().peekable();

	while let Some(&ch) = chars.peek() {
		match ch {
			' ' | '\t' | '\n' | '\r' | '\u{a0}' => {
				chars.next();
			}
			'[' => {
				chars.next();
				let mut ref_text = String::new();
				while let Some(&c) = chars.peek() {
					if c == ']' {
						chars.next();
						break;
					}
					ref_text.push(c);
					chars.next();
				}
				tokens.push(Token::Ref(ref_text));
			}
			'(' => {
				chars.next();
				tokens.push(Token::LParen);
			}
			')' => {
				chars.next();
				tokens.push(Token::RParen);
			}
			'\u{2227}' => {
				// ∧
				chars.next();
				tokens.push(Token::Und);
			}
			'\u{2228}' => {
				// ∨
				chars.next();
				tokens.push(Token::Oder);
			}
			'\u{22BB}' => {
				// ⊻
				chars.next();
				tokens.push(Token::XOder);
			}
			'U' => {
				chars.next();
				if chars.peek() == Some(&'B') {
					chars.next();
					tokens.push(Token::UndNicht);
				} else {
					tokens.push(Token::Und);
				}
			}
			'O' => {
				chars.next();
				tokens.push(Token::Oder);
			}
			_ => {
				// Skip unknown characters
				chars.next();
			}
		}
	}

	tokens
}

// ---------------------------------------------------------------------------
// Recursive descent parser for conditions
//
// Grammar (precedence low→high):
//   expr     = xor_expr
//   xor_expr = or_expr ( (⊻ | UB) or_expr )*
//   or_expr  = and_expr ( ∨ and_expr )*
//   and_expr = atom ( ∧ atom )*       ← implicit AND for adjacent refs
//   atom     = [ref] | ( expr )
//
// UB is treated as: left UB right → Und(left, Nicht(right))
// ---------------------------------------------------------------------------

fn parse_bedingung_expr(tokens: &mut Vec<Token>) -> Option<Bedingung> {
	// reverse so we can pop from the front via pop() from the back
	tokens.reverse();
	let result = parse_xor(tokens);
	tokens.reverse(); // restore order of remaining tokens
	result
}

fn parse_xor(tokens: &mut Vec<Token>) -> Option<Bedingung> {
	let mut left = parse_or(tokens)?;
	loop {
		match tokens.last() {
			Some(Token::XOder) => {
				tokens.pop();
				let right = parse_or(tokens)?;
				left = Bedingung::XOder(Box::new(left), Box::new(right));
			}
			Some(Token::UndNicht) => {
				tokens.pop();
				let right = parse_or(tokens)?;
				left = Bedingung::Und(Box::new(left), Box::new(Bedingung::Nicht(Box::new(right))));
			}
			_ => break,
		}
	}
	Some(left)
}

fn parse_or(tokens: &mut Vec<Token>) -> Option<Bedingung> {
	let mut left = parse_and(tokens)?;
	while tokens.last() == Some(&Token::Oder) {
		tokens.pop();
		let right = parse_and(tokens)?;
		left = Bedingung::Oder(Box::new(left), Box::new(right));
	}
	Some(left)
}

fn parse_and(tokens: &mut Vec<Token>) -> Option<Bedingung> {
	let mut left = parse_atom(tokens)?;
	loop {
		match tokens.last() {
			Some(Token::Und) => {
				tokens.pop();
				let right = parse_atom(tokens)?;
				left = Bedingung::Und(Box::new(left), Box::new(right));
			}
			// Implicit AND: adjacent ref or paren without operator
			Some(Token::Ref(_) | Token::LParen) => {
				let right = parse_atom(tokens)?;
				left = Bedingung::Und(Box::new(left), Box::new(right));
			}
			_ => break,
		}
	}
	Some(left)
}

fn parse_atom(tokens: &mut Vec<Token>) -> Option<Bedingung> {
	match tokens.last()? {
		Token::Ref(_) => {
			if let Some(Token::Ref(r)) = tokens.pop() {
				Some(Bedingung::Ref(r))
			} else {
				None
			}
		}
		Token::LParen => {
			tokens.pop(); // consume (
			let inner = parse_xor(tokens)?;
			// consume )
			if tokens.last() == Some(&Token::RParen) {
				tokens.pop();
			}
			Some(inner)
		}
		_ => None,
	}
}

// ---------------------------------------------------------------------------
// Evaluator — three-valued logic
// ---------------------------------------------------------------------------

/// Evaluate a condition tree with three-valued logic.
///
/// The `zustaende` function returns the truth value for a given reference ID.
pub fn auswerten(
	bedingung: &Bedingung,
	zustaende: &dyn Fn(&str) -> BedingungsZustand,
) -> BedingungsZustand {
	use BedingungsZustand::*;

	match bedingung {
		Bedingung::Ref(id) => zustaende(id),

		Bedingung::Und(a, b) => {
			let va = auswerten(a, zustaende);
			let vb = auswerten(b, zustaende);
			match (va, vb) {
				(Wahr, Wahr) => Wahr,
				(Falsch, _) | (_, Falsch) => Falsch,
				_ => Unbestimmt,
			}
		}

		Bedingung::Oder(a, b) => {
			let va = auswerten(a, zustaende);
			let vb = auswerten(b, zustaende);
			match (va, vb) {
				(Wahr, _) | (_, Wahr) => Wahr,
				(Falsch, Falsch) => Falsch,
				_ => Unbestimmt,
			}
		}

		Bedingung::XOder(a, b) => {
			let va = auswerten(a, zustaende);
			let vb = auswerten(b, zustaende);
			match (va, vb) {
				(Wahr, Falsch) | (Falsch, Wahr) => Wahr,
				(Wahr, Wahr) | (Falsch, Falsch) => Falsch,
				_ => Unbestimmt,
			}
		}

		Bedingung::Nicht(inner) => match auswerten(inner, zustaende) {
			Wahr => Falsch,
			Falsch => Wahr,
			Unbestimmt => Unbestimmt,
		},
	}
}

// ---------------------------------------------------------------------------
// Convenience: extract all references from a condition tree
// ---------------------------------------------------------------------------

/// Collect all unique reference IDs from a condition tree.
pub fn sammle_referenzen(bedingung: &Bedingung) -> Vec<String> {
	let mut refs = Vec::new();
	sammle_referenzen_inner(bedingung, &mut refs);
	refs.sort();
	refs.dedup();
	refs
}

fn sammle_referenzen_inner(bedingung: &Bedingung, acc: &mut Vec<String>) {
	match bedingung {
		Bedingung::Ref(id) => acc.push(id.clone()),
		Bedingung::Und(a, b) | Bedingung::Oder(a, b) | Bedingung::XOder(a, b) => {
			sammle_referenzen_inner(a, acc);
			sammle_referenzen_inner(b, acc);
		}
		Bedingung::Nicht(inner) => sammle_referenzen_inner(inner, acc),
	}
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	// -- Parse simple qualifiers --

	#[test]
	fn parse_muss() {
		assert_eq!(parse_ahb_ausdruck("Muss"), AhbAusdruck::Muss);
	}

	#[test]
	fn parse_soll() {
		assert_eq!(parse_ahb_ausdruck("Soll"), AhbAusdruck::Soll);
	}

	#[test]
	fn parse_kann() {
		assert_eq!(parse_ahb_ausdruck("Kann"), AhbAusdruck::Kann);
	}

	#[test]
	fn parse_x() {
		assert_eq!(parse_ahb_ausdruck("X"), AhbAusdruck::X);
	}

	#[test]
	fn parse_short_m() {
		assert_eq!(parse_ahb_ausdruck("M"), AhbAusdruck::Muss);
	}

	#[test]
	fn parse_short_k() {
		assert_eq!(parse_ahb_ausdruck("K"), AhbAusdruck::Kann);
	}

	// -- Parse qualifier + condition --

	#[test]
	fn parse_muss_und() {
		let result = parse_ahb_ausdruck("Muss [556] ∧ [559]");
		assert_eq!(
			result,
			AhbAusdruck::Bedingt {
				basis: Box::new(AhbAusdruck::Muss),
				bedingung: Bedingung::Und(
					Box::new(Bedingung::Ref("556".into())),
					Box::new(Bedingung::Ref("559".into())),
				),
			}
		);
	}

	#[test]
	fn parse_kann_oder() {
		let result = parse_ahb_ausdruck("Kann [931] ∨ [932]");
		assert_eq!(
			result,
			AhbAusdruck::Bedingt {
				basis: Box::new(AhbAusdruck::Kann),
				bedingung: Bedingung::Oder(
					Box::new(Bedingung::Ref("931".into())),
					Box::new(Bedingung::Ref("932".into())),
				),
			}
		);
	}

	#[test]
	fn parse_x_implicit_and() {
		// Space-separated refs without operator = implicit AND
		let result = parse_ahb_ausdruck("X [931] [494]");
		assert_eq!(
			result,
			AhbAusdruck::Bedingt {
				basis: Box::new(AhbAusdruck::X),
				bedingung: Bedingung::Und(
					Box::new(Bedingung::Ref("931".into())),
					Box::new(Bedingung::Ref("494".into())),
				),
			}
		);
	}

	#[test]
	fn parse_soll_single_ref() {
		let result = parse_ahb_ausdruck("Soll [127]");
		assert_eq!(
			result,
			AhbAusdruck::Bedingt {
				basis: Box::new(AhbAusdruck::Soll),
				bedingung: Bedingung::Ref("127".into()),
			}
		);
	}

	#[test]
	fn parse_ub_operator() {
		// UB means "unless": A UB B → Und(A, Nicht(B))
		let result = parse_ahb_ausdruck("Muss [100] UB [200]");
		assert_eq!(
			result,
			AhbAusdruck::Bedingt {
				basis: Box::new(AhbAusdruck::Muss),
				bedingung: Bedingung::Und(
					Box::new(Bedingung::Ref("100".into())),
					Box::new(Bedingung::Nicht(Box::new(Bedingung::Ref("200".into())))),
				),
			}
		);
	}

	#[test]
	fn parse_nested_parens() {
		// Real expression from data: Soll [165] ∧ (([2061] ∧ [583]) ∨ [584])
		let result = parse_ahb_ausdruck("Soll [165] ∧ (([2061] ∧ [583]) ∨ [584])");
		let expected = AhbAusdruck::Bedingt {
			basis: Box::new(AhbAusdruck::Soll),
			bedingung: Bedingung::Und(
				Box::new(Bedingung::Ref("165".into())),
				Box::new(Bedingung::Oder(
					Box::new(Bedingung::Und(
						Box::new(Bedingung::Ref("2061".into())),
						Box::new(Bedingung::Ref("583".into())),
					)),
					Box::new(Bedingung::Ref("584".into())),
				)),
			),
		};
		assert_eq!(result, expected);
	}

	#[test]
	fn parse_xor_expression() {
		let result = parse_ahb_ausdruck("Muss [129] ⊻ [130]");
		assert_eq!(
			result,
			AhbAusdruck::Bedingt {
				basis: Box::new(AhbAusdruck::Muss),
				bedingung: Bedingung::XOder(
					Box::new(Bedingung::Ref("129".into())),
					Box::new(Bedingung::Ref("130".into())),
				),
			}
		);
	}

	#[test]
	fn parse_alphanumeric_ref() {
		// Real data has refs like [UB1], [1P0..1]
		let result = parse_ahb_ausdruck("X [UB1]");
		assert_eq!(
			result,
			AhbAusdruck::Bedingt {
				basis: Box::new(AhbAusdruck::X),
				bedingung: Bedingung::Ref("UB1".into()),
			}
		);
	}

	#[test]
	fn parse_complex_real_expression() {
		// Real: Muss ([6] ∧ [7]) ∨ [8]
		let result = parse_ahb_ausdruck("Muss ([6] ∧ [7]) ∨ [8]");
		assert_eq!(
			result,
			AhbAusdruck::Bedingt {
				basis: Box::new(AhbAusdruck::Muss),
				bedingung: Bedingung::Oder(
					Box::new(Bedingung::Und(
						Box::new(Bedingung::Ref("6".into())),
						Box::new(Bedingung::Ref("7".into())),
					)),
					Box::new(Bedingung::Ref("8".into())),
				),
			}
		);
	}

	#[test]
	fn parse_chained_and() {
		// Real: Muss [10] ∧ [17]
		let result = parse_ahb_ausdruck("Muss [10] ∧ [17]");
		assert_eq!(
			result,
			AhbAusdruck::Bedingt {
				basis: Box::new(AhbAusdruck::Muss),
				bedingung: Bedingung::Und(
					Box::new(Bedingung::Ref("10".into())),
					Box::new(Bedingung::Ref("17".into())),
				),
			}
		);
	}

	#[test]
	fn parse_unbekannt() {
		let result = parse_ahb_ausdruck("bei zugeordnetem Drittlieferant");
		assert!(matches!(result, AhbAusdruck::Unbekannt(_)));
	}

	#[test]
	fn parse_trailing_qualifier_split() {
		// Real: "Muss [13] Soll [9]" — only first qualifier's condition is parsed.
		// The trailing "Soll [9]" is a separate qualifier section.
		let result = parse_ahb_ausdruck("Muss [13] Soll [9]");
		assert_eq!(
			result,
			AhbAusdruck::Bedingt {
				basis: Box::new(AhbAusdruck::Muss),
				bedingung: Bedingung::Ref("13".into()),
			}
		);
	}

	// -- Evaluator tests --

	#[test]
	fn eval_und_wahr_wahr() {
		let bed = Bedingung::Und(
			Box::new(Bedingung::Ref("1".into())),
			Box::new(Bedingung::Ref("2".into())),
		);
		let result = auswerten(&bed, &|_| BedingungsZustand::Wahr);
		assert_eq!(result, BedingungsZustand::Wahr);
	}

	#[test]
	fn eval_und_wahr_falsch() {
		let bed = Bedingung::Und(
			Box::new(Bedingung::Ref("1".into())),
			Box::new(Bedingung::Ref("2".into())),
		);
		let result = auswerten(&bed, &|id| {
			if id == "1" {
				BedingungsZustand::Wahr
			} else {
				BedingungsZustand::Falsch
			}
		});
		assert_eq!(result, BedingungsZustand::Falsch);
	}

	#[test]
	fn eval_und_wahr_unbestimmt() {
		let bed = Bedingung::Und(
			Box::new(Bedingung::Ref("1".into())),
			Box::new(Bedingung::Ref("2".into())),
		);
		let result = auswerten(&bed, &|id| {
			if id == "1" {
				BedingungsZustand::Wahr
			} else {
				BedingungsZustand::Unbestimmt
			}
		});
		assert_eq!(result, BedingungsZustand::Unbestimmt);
	}

	#[test]
	fn eval_oder_wahr_falsch() {
		let bed = Bedingung::Oder(
			Box::new(Bedingung::Ref("1".into())),
			Box::new(Bedingung::Ref("2".into())),
		);
		let result = auswerten(&bed, &|id| {
			if id == "1" {
				BedingungsZustand::Wahr
			} else {
				BedingungsZustand::Falsch
			}
		});
		assert_eq!(result, BedingungsZustand::Wahr);
	}

	#[test]
	fn eval_oder_falsch_falsch() {
		let bed = Bedingung::Oder(
			Box::new(Bedingung::Ref("1".into())),
			Box::new(Bedingung::Ref("2".into())),
		);
		let result = auswerten(&bed, &|_| BedingungsZustand::Falsch);
		assert_eq!(result, BedingungsZustand::Falsch);
	}

	#[test]
	fn eval_nicht() {
		let bed = Bedingung::Nicht(Box::new(Bedingung::Ref("1".into())));
		assert_eq!(
			auswerten(&bed, &|_| BedingungsZustand::Wahr),
			BedingungsZustand::Falsch
		);
		assert_eq!(
			auswerten(&bed, &|_| BedingungsZustand::Falsch),
			BedingungsZustand::Wahr
		);
		assert_eq!(
			auswerten(&bed, &|_| BedingungsZustand::Unbestimmt),
			BedingungsZustand::Unbestimmt
		);
	}

	#[test]
	fn eval_xor() {
		let bed = Bedingung::XOder(
			Box::new(Bedingung::Ref("1".into())),
			Box::new(Bedingung::Ref("2".into())),
		);
		assert_eq!(
			auswerten(&bed, &|id| if id == "1" {
				BedingungsZustand::Wahr
			} else {
				BedingungsZustand::Falsch
			}),
			BedingungsZustand::Wahr
		);
		assert_eq!(
			auswerten(&bed, &|_| BedingungsZustand::Wahr),
			BedingungsZustand::Falsch
		);
	}

	// -- sammle_referenzen --

	#[test]
	fn sammle_refs() {
		let bed = Bedingung::Und(
			Box::new(Bedingung::Ref("556".into())),
			Box::new(Bedingung::Oder(
				Box::new(Bedingung::Ref("559".into())),
				Box::new(Bedingung::Ref("556".into())),
			)),
		);
		let refs = sammle_referenzen(&bed);
		assert_eq!(refs, vec!["556", "559"]);
	}
}
