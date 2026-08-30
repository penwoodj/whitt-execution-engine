# chunk-02 — prompts 51-100 of 1000
# span 06-23-26-to-08-29-26

## [0051] 06-27 22:14

TASK: Update `docs/specs/operator-type-matrix.md` with Batch 7 + Batch 8 Q&A answers. Several answers CORRECT prior content (not just additions) — rewrite affected sections.

WORKDIR: /home/jon/code/left-right

CONTEXT: Matrix exists at 1403 lines. Two more batches of answers landed. Some invalidate prior content. Update in place.

MUST READ FIRST:
- `docs/specs/operator-type-matrix.md` (current state)
- `docs/qa-history/08-specific-operators.md` (reference, READ-ONLY)

BATCH 7 ANSWERS (10):

1. **isBoolean = `?!` AND `??` BOTH ALIASES.** Conflict resolved as dual aliases. Update Type Check section.

2. **isOperator = `?*` AUTHORITATIVE** (NOT `?%`). Batch 6 wins. qa-history outdated here. Update Type Check section, remove `?%` conflict flag, replace with `?*`. Note `?%` deprecated/removed.

3. **`$%` sort = DESC default across the board** (NOT type-dependent). Strings DESC, numbers DESC. Ascending via explicit strategy. Update Higher-Order section.

4. **`$>` groupBy = PRESERVE RETURN TYPE.** Closure returns Number → numeric keys. Returns String → string keys. VM likely correct; qa-history numeric-keys claim was oversimplified. Update Map section + Discrepancies (mark resolved).

5. **`+:` spread = DEEP RECURSIVE, MAPS ONLY.** NO `+:` for lists. Lists use `+` (concat). This is a CORRECTION — if matrix documented `+:` for lists, remove that. Update Map section.

6. **`!!!?` catch = TWO-CLOSURE form, returns UNEXECUTED OPERATOR.** Syntax: `leftInput ({ diadicTryBody } !!!? { errorMap: _<, errorCaseBody }) rightInput`. The `!!!?` wraps try+catch into a new operator; doesn't run until invoked with left input. Update Error section with this exact form. Remove simpler `{tryBody} !!!? {handler}` description if present — replace with diadic two-closure form.

7. **`=` and `==` on Operators = DEEP key-value comparison** (structural). Update Comparison section.

8. **`List $? String` POLYMORPHIC by list content:**
   - listOfMaps $? string → keep maps where map[string] is truthy (standard truthy rules)
   - listOfStrings $? string → keep strings containing arg as CASE-INSENSITIVE substring
   Update List section.

9. **`$@` pluck on non-Map elements = SKIP** (result list shorter). Update List section.

10. **`?:` GUARD DOES NOT EXIST.** Major correction. `?` = toBoolean only. `/?` = toBoolean alias (like `"` and `/"` both toString). REMOVE the entire `?:` Guard entry from Control Operators section. Document `?` as toBoolean with `/?` alias.

BATCH 8 ANSWERS (10):

11. **Conditional logic via EARLY-RETURN GUARD in map literals.** In `{keyExpr: valueExpr, ...}`:
    - If `keyExpr` is a simple alphanumeric single word → normal key-value pair
    - If `keyExpr` is NOT alphanumeric single word → evaluated as BOOLEAN EARLY RETURN expression
    - When map executes as operator: eval key expr with input. Truthy → return valueExpr immediately. Falsy → continue to next pair.
    - Presence of ANY such key-expr makes the map an operator (even without `_<`).
    - **Inline ternary pattern**: `boolExpr { _< ? : thenResult, elseResult }` — boolExpr feeds `_<`, `_< ?` (toBoolean) is the early-return test key, thenResult returned if truthy, elseResult is next pair's value.
    - This is NOT special syntax — it's inline operator declaration + immediate execution.
    ADD this as a new subsection (e.g. "## Early-Return Guards in Map Literals") near Closure Syntax.

12. **`:` outside map literal = parse error.** Map separator ONLY. Add note.

13. **List clone = `list + []`.** (Concat with empty list = shallow copy.) Add to List section.

14. **`!!!?` walkthrough for `5 ({_< + 1} !!!? {_< `fallback`}) 10`:**
    - Left-to-right: `5`, then `(...)` opens new expression scope
    - Inside parens: `unexecutedOp !!!? unexecutedOp2` → produces NEW unexecuted operator (try/catch wrapped)
    - New operator receives `5` as left arg, runs try body: `5 + 1` = 6
    - Then `6 10` → ERROR (two numbers adjacent without operator)
    - So diadic form: the `(...)` produces an operator, which consumes the left value; any right value must connect via an operator, not adjacency.
    Add this example to Error section.

15. **Truthy = standard** (`false`, `undefined`, `""`, `0`, `[]`, `{}` falsy; rest truthy). Confirm in Boolean section.

16. **`$?` on listOfStrings = CASE-INSENSITIVE substring.** Confirm. `["Apple","banana"] $? "ap"` → `["Apple"]`.

17. **`_` on Number = isUndefined check** (alias for `?_`). On String = lowercase. Type-overloaded. Update String section note: `_` postfix overloaded — String→lowercase, Number→isUndefined (returns false for non-undefined numbers). FLAG as needing re-verification (user phrasing ambiguous).

18. **`/` = divide + cast prefix ONLY.** No slice/split/remove aliases. Update Arithmetic section.

19. **`#` on Operator = ARITY** (1 = monadic, 2 = diadic). Update Other section.

20. **Imported closures callable?** User asked for reword. LEAVE Open Question, mark "needs rewording".

MUST DO:
- Read matrix first
- Update Type Check section: `?!` + `??` aliases; `?*` authoritative isOperator (remove `?%`)
- Update Control Operators section: REMOVE `?:` guard entirely; document `?`=toBoolean + `/?` alias
- Update Higher-Order section: `$%` DESC default; `$@` skip non-maps
- Update Map section: `+:` maps-only deep; `$>` preserve return type (mark discrepancy resolved)
- Update List section: `$?` polymorphic (maps vs strings); `list + []` clone
- Update Comparison section: `=`/`==` on operators deep kv
- Update Error section: `!!!?` diadic two-closure form returning unexecuted op + walkthrough
- Update Arithmetic: `/` divide+casts only
- Update String: `_` overloaded note
- Update Other: `#` on op = arity
- ADD new section "Early-Return Guards in Map Literals" with inline ternary example
- Move resolved items from Open to Resolved with [Batch 7]/[Batch 8] citations
- Add new Open Questions: `_` on Number semantics (re-verify); imported closures reword
- Update Canonical Operator List

MUST NOT DO:
- Do NOT edit qa-history files
- Do NOT edit impl-corrections.md
- Do NOT edit any source (.rs, .lr, .js)
- Do NOT reword the imported-closures question yourself
- Do NOT silently pick interpretation for `_` on Number — flag for re-ask
- Do NOT invent behaviors

VERIFY before finishing:
- `?:` fully removed?
- `?*` authoritative, `?%` removed?
- `?!` and `??` both documented as isBoolean?
- Early-return guard section added with inline ternary example?
- `+:` maps-only documented?
- All Open Questions updated?
- Caveman style consistent?

Take time.
<!-- OMO_INTERNAL_INITIATOR -->

## [0052] 06-27 22:44

TASK: Fix 2 confirmed VM semantic bugs in `compiler/crates/lr-vm/src/vm.rs`. Reconcile broken tests.

WORKDIR: /home/jon/code/left-right

CONTEXT: Left-Right language VM. 706 tests pass currently (440 rust + 105 CLI + 161 live). Two behaviors diverge from documented spec. Fix both.

BUG 1: `&` (AND) returns Boolean, spec requires VALUE return (JS `&&`-style).

Spec (verified with user Batch 4): `left & right`:
- If `left` is truthy → return `right` (the VALUE, not boolean)
- If `left` is falsy → return `left` (the VALUE, short-circuit)

Current buggy code locations in `compiler/crates/lr-vm/src/vm.rs`:
- Line 699: `"&" => Value::boolean(mc, *left_val && *right_val)` (Bool+Bool case) — WRONG
- Line 744: `"&" => Value::boolean(mc, partial.left_arg.is_truthy() && right.is_truthy())` (Num+Num case in diadic dispatch) — WRONG

Correct semantics:
- Line 699 fix: `"&" => if *left_val { right } else { partial.left_arg }`
- Line 744 fix: `"&" => if partial.left_arg.is_truthy() { right } else { partial.left_arg }`

ALSO: search ENTIRE vm.rs for any other `"&"` dispatch that returns Boolean. Check String+String, List+List, Map+Map, mixed-type `&`. Each must follow same value-return pattern. If a case doesn't exist, leave it (don't add new cases).

Compare with `|` (OR) which is ALREADY CORRECT:
- Line 700: `"|" => if *left_val { partial.left_arg } else { right }` ✓
- Line 710: `"|" => if partial.left_arg.is_truthy() { partial.left_arg } else { right }` ✓
- Line 1195: `(_, _, "|") => if partial.left_arg.is_truthy() { partial.left_arg } else { right }` ✓

Use `|` as reference pattern. Mirror it for `&` (inverted: `&` returns right when left truthy).

BUG 2: `+` and `_` with Undefined operand string-concats, spec requires SKIP undefined.

Spec (verified Batch 2): undefined is identity for `+`:
- `undefined + X` → X
- `X + undefined` → X
- `undefined + undefined` → undefined

Current buggy code: Line 1202-1203 fallback: `(_, _, "+" | "_") => Value::string(mc, format!("{}{}", partial.left_arg, right))`. This catches undefined operands and concatenates as string "undefinedX".

Fix approach: BEFORE the string-concat fallback, add explicit undefined-skip cases. Insert near line 1202:
- `(Value::Undefined, _, "+" | "_") => right` (left undefined → return right)
- `(_, Value::Undefined, "+" | "_") => partial.left_arg` (right undefined → return left)

Place these BEFORE the `(_, _, "+" | "_") =>` fallback so they match first. Match arms are evaluated top-down.

ALSO check: are there other `+` paths that could hit undefined? E.g. `(Value::Number, Value::Undefined, "+")` — does this exist? If not, the fallback at 1202 catches it. Your early-insert fixes it. Verify by tracing: `5 + undefined` — left=5 Number → partial (line 627) → partial applied to undefined → your new early arm returns 5. ✓

MUST DO:
1. Read vm.rs fully around lines 690-750 (bool dispatch), 1190-1215 (fallback dispatch)
2. Fix Bug 1 (`&` value-return) at lines 699, 744, and any other `&` sites
3. Fix Bug 2 (`+`/`_` undefined-skip) by inserting early arms before line 1202 fallback
4. Run `cargo test` from `compiler/` directory
5. If tests fail because they asserted OLD buggy behavior:
   - Read the failing test
   - Determine if it was testing the bug (e.g. `assert true & false == false` — still passes since false==false)
   - Update test assertion to match NEW correct behavior
   - Tests that explicitly checked boolean return for `&` MUST be updated to check value return
6. Run `cargo test` again — ALL must pass
7. Run CLI tests: `cd compiler/crates/lr-cli && cargo run -- test` — all must pass
8. Update or add tests verifying NEW behavior:
   - `5 & 3` → 3 (value, not boolean true)
   - `0 & 3` → 0 (falsy left returns left)
   - `undefined + 5` → 5
   - `5 + undefined` → 5
9. Run `cargo check -p lr-vm` — zero warnings

MUST NOT DO:
- Do NOT change `|` behavior (already correct)
- Do NOT change semantics of other operators
- Do NOT touch parser, lexer, AST, codegen
- Do NOT edit qa-history or implementation-corrections.md
- Do NOT delete tests to make them pass — UPDATE assertions to correct behavior
- Do NOT suppress warnings with `#[allow(...)]`
- Do NOT use `as any`-style hacks

VERIFY before reporting done:
- `cargo test` from compiler/ — ALL pass, zero failures
- `cargo run -- test` from crates/lr-cli/ — ALL pass
- `cargo check -p lr-vm` — zero warnings
- New tests added proving value-return `&` and undefined-skip `+`
- Show output of all three commands in your report

Report exact line numbers changed, tests added/modified, command outputs.
<!-- OMO_INTERNAL_INITIATOR -->

## [0053] 06-27 22:45

TASK: Build live-system-testing capability demos — real `.lr` files demonstrating documented Left-Right operators, run them, capture output, confirm behaviors.

WORKDIR: /home/jon/code/left-right

CONTEXT: Left-Right language. CLI: `lr run <file>` runs a .lr file, prints result. `lr test` runs CLI tests. Live system testing harness at `compiler/tests/live_runner.sh` runs .lr files comparing against `.lr.expected` files.

Directory `docs/live-system-testing/` exists with scaffold:
- README.md (workflow doc — read it first)
- sandbox/ (throwaway iteration)
- capabilities/ (verified demos — YOUR target)
- logs/ (output captures)

Each .lr file is a SINGLE EXPRESSION (LR chains expressions across lines, so multi-statement files don't work as independent demos). One operator demo per file.

REFERENCE: `docs/specs/operator-type-matrix.md` is authoritative spec. Read relevant sections before writing each demo.

RUNNING: From repo root, `./compiler/crates/lr-cli/target/release/lr run <file>` OR build first: `cargo build --release --bin lr` then run. Check `compiler/tests/live_runner.sh` for exact invocation pattern.

CREATE these demo files in `docs/live-system-testing/capabilities/` (one per topic, named `NN-topic.lr`):

01-arithmetic.lr — `5 + 3` → 8
02-multiplication.lr — `4 * 6` → 24
03-division.lr — `20 / 4` → 5
04-modulo.lr — `17 % 5` → 2
05-power.lr — `2 ^ 10` → 1024
06-strict-equality.lr — `5 == 5` → true
07-loose-equality.lr — `5 = \`5\`` → true (type-coercing)
08-less-than.lr — `3 < 7` → true
09-concat.lr — `` `hello` + ` ` + `world` `` → `hello world`
10-uppercase.lr — `` `hello` ^ `` → `HELLO`
11-lowercase.lr — `` `HELLO` _ `` → `hello`
12-capitalize.lr — `` `hello` ^_ `` → `Hello`
13-replace.lr — `` `a1a1` ~ [\`a\`, \`b\`] `` → `b1b1`
14-split.lr — `` `a,b,c` <> \`,\` `` → `[a, b, c]`
15-join.lr — `[\`a\`, \`b\`, \`c\`] >< \`,\`` → `a,b,c`
16-string-length.lr — `` `hello` # `` → 5
17-list-index.lr — `[10, 20, 30] @ 1` → 20
18-list-size.lr — `[1, 2, 3] #` → 3
19-list-concat.lr — `[1, 2] + [3, 4]` → `[1, 2, 3, 4]`
20-list-map.lr — `[1, 2, 3] $ { _< * 2 }` → `[2, 4, 6]`
21-list-filter.lr — `[1, 2, 3, 4] $? { _< > 2 }` → `[3, 4]`
22-list-flatmap.lr — `[[1, 2], [3]] $_ { _< }` → `[1, 2, 3]`
23-list-some.lr — `[1, 2, 3] $| { _< > 2 }` → true
24-list-every.lr — `[2, 4, 6] $& { _< % 2 = 0 }` → true
25-map-get.lr — `{a: 1, b: 2} @ \`a\`` → 1
26-map-merge.lr — `{a: 1} + {b: 2}` → `{a: 1, b: 2}`
27-map-size.lr — `{a: 1, b: 2} #` → 2
28-closure-apply.lr — `5 { _< + 1 }` → 6
29-closure-diadic.lr — `10 { _< + _> } 5` → 15
30-to-boolean.lr — `0 ?` → false
31-negate.lr — `true !` → false
32-is-string.lr — `` `hello` ?" `` → true
33-is-number.lr — `5 ?#` → true
34-is-list.lr — `[1, 2] ?%` → true (NOTE: `?%` is authoritative isList per spec, NOT `?//`)
35-is-map.lr — `{a: 1} ?~` → true

NOTE on file naming: use NN- prefix for sort order, lowercase-kebab-case.

For each file:
1. Write the .lr file with the single expression
2. Run it: capture command + output into `docs/live-system-testing/logs/NN-topic.log`
3. If output matches expected → create `capabilities/NN-topic.lr.expected` with the output
4. If output does NOT match expected → record in `docs/live-system-testing/logs/discrepancies.md` with: file, expected, actual, possible cause

AVOID these operators (known VM bugs being fixed in parallel, would produce wrong output):
- `&` AND (returns Boolean instead of value)
- `|` OR on non-Boolean (may work, but skip to be safe)
- `+` with undefined operand (string-concats instead of skipping)
- `+:` spread (maps-only semantics being clarified)
- `!!!?` catch (complex two-closure form)
- `///` `\\\` async
- `$%` sort, `$>` groupBy, `$~` uniqueBy (complex)
- imports

MUST DO:
- Read `docs/live-system-testing/README.md` first for workflow conventions
- Read relevant matrix sections before writing each demo to confirm syntax
- Build the CLI first: `cargo build --release --bin lr` from `compiler/`
- Write all 35 demo files
- Run each, capture logs
- Create .expected files for passing demos
- Document discrepancies for failing ones
- Update `docs/live-system-testing/README.md` with results summary (count passed / count discrepancy)

MUST NOT DO:
- Do NOT edit any .rs source files
- Do NOT edit qa-history, specs, or matrix
- Do NOT touch compiler/ directory except running builds
- Do NOT use multi-line .lr files (single expression per file)
- Do NOT use operators in the AVOID list
- Do NOT use single/double quotes — only backticks for strings

VERIFY before reporting done:
- `ls docs/live-system-testing/capabilities/*.lr | wc -l` → 35
- All 35 have corresponding .lr.expected files OR discrepancy entries
- `docs/live-system-testing/logs/discrepancies.md` exists (even if empty)
- README.md updated with pass/fail counts
- Show counts + sample discrepancy in report

Report: files created, pass count, discrepancy count, sample outputs.
<!-- OMO_INTERNAL_INITIATOR -->

## [0054] 06-27 22:45

TASK: Apply surgical corrections to `docs/specs/operator-type-matrix.md` for Batch 9 + Batch 10 answers. Several entries are now WRONG and need fixing.

WORKDIR: /home/jon/code/left-right

FILE: `docs/specs/operator-type-matrix.md` (1488 lines)

CORRECTION 1: `?%` is isList (NOT isOperator, NOT deprecated).

Current WRONG state (from Batch 7 update):
- Line 745: `**RESOLVED [Batch 7]**: Authoritative isOperator. \`?%\` deprecated/removed.`
- Line 1348: conflict flag about `?%`
- Line 1407: `\`?*\`/\`?%\` isOperator [Batch 7]`

CORRECT state per Batch 9:
- `?%` = isList (AUTHORITATIVE). `?//` is the deprecated one.
- `?*` = isOperator (authoritative, separate from `?%`)
- Remove ALL text saying `?%` is deprecated or `?%` is isOperator
- Update Type Check section: list `?%` as isList, `?//` as deprecated/removed, `?*` as isOperator

CORRECTION 2: `?//` status = DEPRECATED (was isList in qa-history, now removed).

Find any `?//` isList references in matrix → replace with note that `?//` is deprecated, `?%` is the authoritative isList.

CORRECTION 3: `_` on Number = FLOOR (truncate decimals, keep integer).

Find any text saying `_` on Number is isUndefined or throws → replace with: `_` postfix is type-overloaded: String→lowercase, Number→floor (truncate decimals), List→flatten.

CORRECTION 4: Map `#` = non-undefined pairs (NOT truthy-value pairs).

Find map `#` entry → update: counts key-value pairs where value is NOT undefined. `{a:0, b:1, c:undefined}` → 2. Values of `0`, `false`, `""` still count.

CORRECTION 5: `^` on Number = DIADIC EXPONENT (returns partial operator waiting for right operand).

Find `^` entries → clarify: on String = unary uppercase; on Number = diadic power (`base ^ exp`), so `5 ^` returns unexecuted operator awaiting exponent.

CORRECTION 6: `///` makeAsync = OPERATOR ONLY.

Find `///` entry → update: input must be unexecuted operator. Returns async version, still unexecuted, same interface. Non-operator input errors.

CORRECTION 7: `\\\` await = ANY VALUE passthrough.

Find `\\\` entry → update: non-promise values pass through unchanged. Promises unwrap. Rejected promises THROW (catch with `!!!?`).

CORRECTION 8: `~` replace = BOTH 2-list AND map forms.

Find `~` entry → update: accepts `[old, new]` (single replacement, all occurrences) OR `{old1: new1, old2: new2}` (multiple replacements).

CORRECTION 9: `><` on non-string elements = COERCE to string.

Find `><` entry → update: each element coerced via toString. `[1, 2, 3] >< \`,\`` → `` `1,2,3` ``.

CORRECTION 10: `<>` empty string = STANDARD split → empty list.

Find `<>` entry → add: `` `` <> \`,\` `` → `[]` (empty list, standard split behavior).

CORRECTION 11: Update Canonical Operator List at end of matrix. Ensure:
- `?%` listed as isList
- `?//` removed or marked deprecated
- `?*` listed as isOperator

CORRECTION 12: Imported closures — clarify per Batch 9.

Find any "imported closures uncallable" entry → replace with: files return single evaluated value (Map/List/operator/etc). `files@\`path\`` returns that value. If file evaluates to operator, the operator IS callable. No "top-level keys" concept — files aren't modules with named exports, just expressions.

MUST DO:
- Read current matrix FIRST
- Apply each correction surgically (use Edit tool, not rewrite)
- Keep caveman style consistent
- Update Canonical Operator List
- Move any newly-resolved items from Open Questions to Resolved Questions with [Batch 9]/[Batch 10] citations

MUST NOT DO:
- Do NOT rewrite whole sections — surgical edits only
- Do NOT edit qa-history, impl-corrections, or any source
- Do NOT change `?*` isOperator (that stays)
- Do NOT touch the Early-Return Guards section (added correctly in Batch 8)

VERIFY before reporting done:
- `grep -n "?%.*deprecated\|?%.*removed" docs/specs/operator-type-matrix.md` → no matches
- `grep -n "?//.*isList" docs/specs/operator-type-matrix.md` → no matches (should say deprecated)
- `grep -n "isUndefined.*Number\|Number.*isUndefined" docs/specs/operator-type-matrix.md` → no matches (floor corrected)
- Canonical list has `?%` isList, `?*` isOperator

Report: line counts before/after, corrections applied.
<!-- OMO_INTERNAL_INITIATOR -->

## [0055] 06-28 11:44

status then continue

## [0056] 06-28 12:23

TASK: Fix 3 confirmed VM bugs in `compiler/crates/lr-vm/src/vm.rs` (and lexer/parser if needed for token recognition). Run tests.

WORKDIR: /home/jon/code/left-right

CONTEXT: Left-Right VM. 465 tests currently pass. Live system testing caught 3 divergences from documented spec. Fix all three.

BUG 1: `?%` should be isList (returns `true` for List, `false` for all other types). Currently returns `false` EVERYWHERE.

Spec (verified with user Batch 9): `?%` = isList, authoritative. `?//` is deprecated (was old isList).

Current buggy code in `compiler/crates/lr-vm/src/vm.rs`:
- Line 228: `"?%" => Ok(Value::boolean(mc, false))` — left=undefined postfix
- Line 550: `"?%" => Value::boolean(mc, false)` — String postfix
- Line 619: `"?%" => Value::boolean(mc, false)` — **List postfix (WRONG — should be `true`)**
- Line 651: `"?%" => Value::boolean(mc, false)` — Number postfix
- Line 687: `"?%" => Value::boolean(mc, false)` — Boolean postfix
- Line 1267: `"?%" => Value::boolean(mc, false)` — undefined postfix (another site)
- Line 1192: `(left, _, "?%") => { ... }` — diadic form, check what it does

Fix: Line 619 change `false` → `true`. This is the ONLY change for Bug 1 (List `?%` should be true; all other types stay false).

ALSO: Line 617 has `"?//" => Value::boolean(mc, true)` for List — this is the OLD isList. Per Batch 9 it's deprecated. KEEP it returning true (backward compat) but it's deprecated in docs. Do NOT remove.

BUG 2: `??` is not recognized by VM at all. Should be alias for `?!` (isBoolean).

Spec (verified Batch 11): `??` = canonical isBoolean. `?!` = legacy alias. Both must work.

Current state: `grep '"??"' compiler/crates/lr-vm/src/vm.rs` returns NOTHING. `??` is completely absent.

Fix approach:
1. FIRST check if lexer tokenizes `??`. Search `compiler/crates/lr-lexer/src/` for `??` or the token pattern that would match it. If lexer doesn't recognize `??`, add it.
2. THEN check parser: `compiler/crates/lr-parser/src/` — does it parse `??` as an operator?
3. THEN in vm.rs: everywhere `"?!"` appears as a match arm, add `"?" | "??"` OR add a separate `"??"` arm returning the same value. Use `"?!" | "??"` pattern.

The `?!` sites in vm.rs (from grep):
- Line 225: `"?!" => Ok(Value::boolean(mc, false))` — undefined postfix
- Line 547: `"?!" => Value::boolean(mc, false)` — String postfix
- Line 616: `"?!" => Value::boolean(mc, false)` — List postfix
- Line 648: `"?!" => Value::boolean(mc, false)` — Number postfix
- Line 684: `"?!" => Value::boolean(mc, true)` — Boolean postfix (TRUE)
- Line 1183: `(left, _, "?!") => { ... }` — diadic form
- Line 1262: `"?!" => Value::boolean(mc, false)` — undefined postfix

Each needs `"?!" | "??"` to accept both.

BUG 3: `*` on String × Number should produce List of String duplicates.

Spec (verified Batch 12): `` `ab` * 3 `` → `[`ab`, `ab`, `ab`]` (List with 3 duplicates of the string).

Current state: `*` not implemented for String left operand. Line 526 block (String postfix ops) does NOT include `*`. So `` `ab` * 3 `` likely errors.

Fix approach:
1. In the String postfix block (around line 526), add `"*"` to the list of operators that create a partial operator. Pattern: `"*" => Value::partial_operator(mc, "*".to_string(), Value::string(mc, s.to_string()))`
2. In the diadic dispatch, add a NEW arm: `(Value::String(s), Value::Number(n), "*") => Value::list(mc, std::iter::repeat_with(|| s.clone()).take(*n as usize).collect())`
   - Handle edge cases: `n <= 0` → empty list. `n` non-integer → truncate to int (`n as usize`).
   - Handle `n` is NaN/Infinity → return empty list or error (your call, document it).

MUST DO:
1. Read vm.rs fully around affected lines before editing
2. Fix Bug 1 (`?%` List → true, one-line change at line 619)
3. Fix Bug 2 (`??` alias — check lexer first, then parser, then vm.rs)
4. Fix Bug 3 (`*` String×Number → List of duplicates)
5. Run `cargo test` from `compiler/` — ALL must pass
6. Run CLI tests: `cd compiler/crates/lr-cli && cargo run --release -- test` — all pass
7. Add NEW tests verifying each fix:
   - `[1, 2] ?%` → true
   - `5 ?%` → false
   - `` `x` ?% `` → false
   - `true ??` → true (canonical)
   - `true ?!` → true (legacy still works)
   - `5 ??` → false
   - `` `ab` * 3 `` → `[ab, ab, ab]`
   - `` `x` * 0 `` → `[]`
8. Re-run live test 34: `lr run docs/live-system-testing/capabilities/34-is-list.lr` — should now output `true`
9. Update `docs/live-system-testing/logs/discrepancies.md` — mark 34-is-list as RESOLVED (was VM bug, now fixed)
10. `cargo check -p lr-vm` — zero warnings

MUST NOT DO:
- Do NOT remove `?//` (keep as deprecated-but-working backward compat)
- Do NOT change `?!` behavior (still works, just legacy)
- Do NOT touch qa-history, implementation-corrections, or matrix
- Do NOT suppress warnings with `#[allow]`
- Do NOT delete tests — update assertions if old behavior was being tested
- Do NOT use `as any`-style hacks

VERIFY before reporting done:
- `cargo test` from compiler/ — ALL pass (show count)
- `cargo run --release -- test` from crates/lr-cli/ — ALL pass
- `cargo check -p lr-vm` — zero warnings
- `lr run docs/live-system-testing/capabilities/34-is-list.lr` outputs `true`
- Show output of all verification commands
- Show exact line numbers changed in vm.rs (and lexer/parser if touched)

Report: bugs fixed, line numbers, test counts before/after, live test 34 result.
<!-- OMO_INTERNAL_INITIATOR -->

## [0057] 06-28 12:23

TASK: Update `docs/specs/operator-type-matrix.md` with Batch 11 + Batch 12 Q&A answers. Surgical edits.

WORKDIR: /home/jon/code/left-right

FILE: `docs/specs/operator-type-matrix.md` (~1570 lines)

BATCH 11 ANSWERS (apply):

1. `^` on List/Map/Boolean/Undefined = THROWS. `^` only works on String (unary uppercase) and Number (diadic exponent). Update `^` section.

2. `~` map form `{old1:new1, old2:new2}` = PARALLEL ATOMIC. All replacements computed from original string, applied simultaneously. No cascade. Update `~` section.

3. `><` elements = COERCE (Batch 10 wins, Batch 11 "strict" was user misremembering). `[1,2,3] >< \`,\`` → `` `1,2,3` ``. Separator MUST be String. Elements coerce to string. RESOLVE the conflict flag — remove any "strict" note, confirm coerce.

4. `$%` sort ALWAYS needs right argument (unexecuted operator returning string/number/zero). Bare `$%` without right = error or partial. Update `$%` section.

5. `$>` groupBy: closure returning Map or List = THROWS. Closure must return String or Number. Update `$>` section.

6. `??` = CANONICAL isBoolean. `?!` = legacy alias. Update Type Check section — `??` primary, `?!` secondary. Remove any "equal aliases" note, replace with canonical/legacy.

7. Map `#` = non-undefined pairs (CONFIRMED, already correct in matrix from Batch 9/10 correction). Verify entry is accurate.

8. `///` on already-async operator = IDEMPOTENT. Returns same async op, no double-wrap. Update `///` section.

9. `5 + true` = 6 (NUMERIC COERCE). Boolean+Number with `+` → Boolean coerced to Number (true→1, false→0), then numeric add. Update `+` section: add Boolean+Number case.

10. Nested closures: `_<` FRESH per level. Inner `_<` shadows outer. Use named var to pass outer values down. Update Closure section.

BATCH 12 ANSWERS (apply):

11. `><` elements COERCE (confirms Batch 10, resolves conflict from Batch 11). Already covered above.

12. toString `"` and `/"` = EQUAL aliases. No canonical. Update Cast section.

13. toBoolean `?` = canonical. `/?` = legacy alias. Update Cast / Boolean section.

14. `+` polymorphism table COMPLETE. Confirmed: String-triggers-concat, Boolean+Number=numeric, Undefined=identity, Map+Map=merge, List+List=concat. Update `+` section — mark as complete, no more TBDs.

15. `===` (triple eq) DOES NOT EXIST. Only `=` (loose) and `==` (strict). Update Comparison section — remove any `===` mention or note it doesn't exist.

16. `*` on String × Number = LIST of duplicates. `` `ab` * 3 `` → `[`ab`, `ab`, `ab`]`. Update `*` section — add String×Number case. (Previously TBD from Batch 4, now resolved.)

17. `<>` on non-String left = COERCE left to string first. `[1,2] <> \`,\`` → splits stringified `[1, 2]`. Update `<>` section.

18. Early-return guard key evaluation = LAZY PER-PAIR. Key expr only evaluated when execution reaches that pair. Earlier truthy pair returns first, later keys never evaluated. Update Early-Return Guards section.

19. `!!!?` handler error context = `{error, trace}` map. `error` = thrown value. `trace` = operator chain (call stack). Update `!!!?` section.

20. `???` (ternary) DOES NOT EXIST. Only early-return guard pattern. Update — remove any `???` mention or note non-existence.

MUST DO:
- Read current matrix first
- Apply each correction surgically via Edit tool
- Move newly-resolved items from Open Questions to Resolved Questions with [Batch 11]/[Batch 12] citations
- Update Canonical Operator List if needed
- Keep caveman style consistent

MUST NOT DO:
- Do NOT rewrite whole sections — surgical edits
- Do NOT edit qa-history, impl-corrections, source files
- Do NOT change `?%` isList or `?//` deprecated (already correct from Batch 9/10 correction)
- Do NOT touch the `_` floor entry (already correct)

VERIFY before reporting:
- `grep -n "strict.*><\|><.*strict" docs/specs/operator-type-matrix.md` → no conflict flags remain
- `grep -n "===" docs/specs/operator-type-matrix.md` → either absent or noted as non-existent
- `grep -n "\`\` \`ab\` \* 3" docs/specs/operator-type-matrix.md` → String×Number `*` documented
- `grep -n "\\?\\? canonical\|canonical isBoolean" docs/specs/operator-type-matrix.md` → `??` marked canonical

Report: corrections applied, line count before/after.
<!-- OMO_INTERNAL_INITIATOR -->

## [0058] 06-28 12:31

(2026-06-28T12:31:56)
[search-mode]
MAXIMIZE SEARCH EFFORT. Launch multiple background agents IN PARALLEL:
- explore agents (codebase patterns, file structures, ast-grep)
- librarian agents (remote repos, official docs, GitHub examples)
Plus direct tools: Grep, ripgrep (rg), ast-grep (sg)
NEVER stop at first result - be exhaustive.

---

look through and find me everything I've mention on AI compute hardware and tell me what SOCs chips I've mentioned along with SBCs and everything else related to making my own hardware for running local AI.  mainly going to be in chatgpt notes and don't leave any stone unturned and make a new folder in new-ideas called ./new-ideas/hardware/soc-and-sbc/ and make a consolidated report of everything mentioned and every model number or requirement or tier and everything else in a well formatted markdown summary in that folder.  don't stop until it's fully combed through and you've consolidated deduplicated and formatted well the final reprot in that folder

## [0059] 06-28 12:34

(2026-06-28T12:34:16)
CONTEXT: I'm building a consolidated report of all AI compute hardware the user has mentioned in their personal notes. The notes live under /home/jon/code/life-data/chatgpt/organized/chat-history/ — each subfolder is a conversation topic with chunk-NNN.md files inside.

GOAL: Extract EVERY mention of Single-Board Computers (SBCs) and their System-on-Chips (SoCs) across all chatgpt notes. I need specific model numbers, RAM tiers, requirements, and context.

DOWNSTREAM: I'll merge your findings into a consolidated hardware report. Be exhaustive — capture every chip model, every board, every spec mentioned.

REQUEST:
1. Search the ENTIRE tree under /home/jon/code/life-data/chatgpt/organized/chat-history/ for SBC and SoC references.
2. Specifically look for these folders (but don't limit yourself to them):
   - llms-local-setup-factors
   - cpu-ram-types-comparison
   - extensive-system-requirement-breakdown
   - llm-enabled-raspberry-pi-setup
   - raspberry-pi-media-os-list
   - hardware-setup-brainstorming
   - edge-computing-hardware-breakdown
   - e-ink-phones-with-ram
   - open-source-fire-stick-alternatives
3. Capture:
   - SBC brand + model (Raspberry Pi 4/5, Orange Pi 5, ODROID-M1, Rock PRO, Le Potato, etc.)
   - SoC model numbers (BCM2711, BCM2712, RK3588, RK3588S, RK3399, S905, Allwinner H6, etc.)
   - CPU cores/architecture (Cortex-A76, A78, A55, etc.)
   - RAM tiers mentioned (4GB, 8GB, 16GB, 32GB LPDDR4/LPDDR5)
   - User-stated requirements (min RAM for X model size, etc.)
   - Any "tier" classifications (entry/mid/high, etc.)
   - Price points or budget mentions
4. Use grep on patterns: `\bRK\d{4}\b`, `\bBCM2712?\b`, `Orange Pi`, `ODROID`, `Le Potato`, `Banana Pi`, `NanoPi`, `Radxa`, `Rock \d`, `Cortex-A\d{2}`, `LPDDR[45]`, `Raspberry Pi [45]`
5. Return findings as structured markdown with sections:
   ## SBCs Mentioned (with SoC + RAM + context)
   ## SoCs/Chips Mentioned (standalone)
   ## CPU Architectures
   ## RAM Tiers / Requirements
   ## Tier Classifications
   ## Price/Budget Mentions
   Each entry MUST cite source folder + chunk file.

Skip: generic AI discussion not tied to hardware. Skip: pure software/OS talk unless specs mentioned.

Be exhaustive. Better to over-collect than miss a chip mention.
<!-- OMO_INTERNAL_INITIATOR -->

## [0060] 06-28 12:34

(2026-06-28T12:34:16)
CONTEXT: I'm building a consolidated report of all AI compute hardware the user has mentioned in personal notes. Notes live under /home/jon/code/life-data/chatgpt/organized/chat-history/ — each subfolder has chunk-NNN.md files.

GOAL: Extract EVERY mention of AI accelerators, NPUs, TPUs, GPUs, and dedicated AI hardware across all chatgpt notes. Include specific product names and model numbers.

DOWNSTREAM: Merging into consolidated hardware report. Be exhaustive.

REQUEST:
1. Search ENTIRE tree under /home/jon/code/life-data/chatgpt/organized/chat-history/
2. Focus on these relevant folders (but check others too):
   - edge-computing-hardware-breakdown
   - llms-local-setup-factors
   - extensive-system-requirement-breakdown
   - llm-enabled-raspberry-pi-setup
   - hardware-setup-brainstorming
   - esp32-c6-ram-details
   - esp32-som-modules
3. Capture EVERY mention of:
   - NVIDIA Jetson family (Nano, Orin Nano, Orin NX, AGX Orin, Xavier NX, TX2, TX1) with specs
   - Google Coral (Edge TPU, Dev Board, USB Accelerator, SoC mt8167)
   - Hailo (Hailo-8, Hailo-15, Hailo-10H, etc.) with TOPS
   - Intel NUC / Intel Arc / Intel Movidius / NCS2
   - AMD Ryzen Embedded / AMD Radeon / AMD NUC alternatives
   - Apple Silicon (M1/M2/M3/M4, Ultra, Max) for local AI
   - NPUs (TOPS ratings, embedded NPUs)
   - Qualcomm AI engines / Hexagon DSP / Snapdragon X Elite
   - Any GPU mentioned for local inference (RTX 3060/4060/4070/4090, etc.)
   - Frameworks: CUDA, TensorRT, ROCm, OpenVINO, MLPerf
4. Use grep patterns: `Jetson`, `Coral`, `Hailo`, `TOPS`, `TFLOPS`, `NPU`, `TPU`, `Movidius`, `OpenVINO`, `CUDA`, `\bRTX\s?\d{4}\b`, `\bGTX\s?\d{4}\b`, `Hexagon`, `Apple M[1-4]`, `Tensor RT`
5. Return structured markdown:
   ## Dedicated AI Accelerators (Jetson, Coral, Hailo, etc.)
   ## GPUs for Local Inference
   ## Embedded NPUs / TPUs
   ## Apple Silicon mentions
   ## Frameworks/SDKs mentioned
   ## Performance specs (TOPS/TFLOPS cited)
   Each entry cites source folder + chunk file.

Skip: pure software discussions unless tied to hardware capability. Skip: cloud AI (AWS, GCP) unless compared against local.

Be exhaustive. Capture every model number, every TOPS rating, every requirement.
<!-- OMO_INTERNAL_INITIATOR -->

## [0061] 06-28 12:34

(2026-06-28T12:34:16)
CONTEXT: Building consolidated report of all AI compute hardware user mentioned in notes. Notes live under /home/jon/code/life-data/chatgpt/organized/chat-history/ — subfolders with chunk-NNN.md files.

GOAL: Extract EVERY mention of microcontrollers, ESP32 variants, TinyML hardware, and ultra-low-power edge AI devices.

DOWNSTREAM: Merging into consolidated hardware report. Be exhaustive.

REQUEST:
1. Search ENTIRE tree under /home/jon/code/life-data/chatgpt/organized/chat-history/
2. Focus on these folders (but check all):
   - esp32-c6-ram-details (9 chunks)
   - esp32-som-modules (4 chunks)
   - edge-computing-hardware-breakdown
   - hardware-setup-brainstorming
   - llms-local-setup-factors
3. Capture every mention of:
   - ESP32 variants (ESP32, ESP32-S2, S3, C3, C6, H2, P4) with specs
   - ESP32-SOM modules / ESP32 modules
   - STM32 family (STM32H7, F7, etc.)
   - RP2040 / RP2350 (Raspberry Pi Pico)
   - nRF52840 / nRF5340 (Nordic)
   - K210 / K230 (Kendryte / Sipeed Maix)
   - TinyML-class boards (Arduino Nano 33 BLE Sense, etc.)
   - Realtek RTL8xxx
   - MediaTek LinkIt / MT7697
   - microcontroller RAM/Flash tiers
   - Mentioned AI workloads on MCUs (keyword spotting, image classification, etc.)
4. Use grep patterns: `ESP32[-_]?(?:S2|S3|C3|C6|H2|P4)?`, `\bRP2\d{3}\b`, `\bSTM32\w+\b`, `\bnRF\d{4,5}\b`, `K210|K230|Kendryte|Maix`, `TinyML`, `Arduino Nano 33`, `Sony Spresense`, `\bM33\b|M4F?|FPU`
5. Return structured markdown:
   ## ESP32 Variants Mentioned (with specs)
   ## Other Microcontrollers (STM32, RP2040, nRF, etc.)
   ## TinyML Hardware (K210, Maix boards, etc.)
   ## MCU RAM/Flash Tiers Mentioned
   ## AI Workloads on MCU (what user wants to run)
   Each entry cites source folder + chunk file.

Be exhaustive — these are small files so don't skip anything.
<!-- OMO_INTERNAL_INITIATOR -->

## [0062] 06-28 12:34

(2026-06-28T12:34:16)
CONTEXT: Building consolidated report of all AI compute hardware user mentioned in notes. The user said "mainly in chatgpt notes but don't leave any stone unturned" — so I need to find EVERY folder/topic that mentions hardware.

GOAL: Index all chat folders that discuss AI compute hardware, so the other agents don't miss anything. Also check non-chatgpt note locations.

DOWNSTREAM: This becomes the master index for the report. Need to ensure nothing slips through.

REQUEST:
1. List ALL subfolders under /home/jon/code/life-data/chatgpt/organized/chat-history/ whose names suggest hardware/compute/AI/edge/SBC/SOC/embedded topics. (Folder names are descriptive — use that.)
2. Then grep each candidate folder to confirm it discusses relevant hardware.
3. ALSO check these other locations for hardware mentions:
   - /home/jon/code/life-data/chatgpt/organized/projects/ (esp. pcb-design-suggestions, e-ink-setup-plan, project-idea-brainstorming, designing-a-programming-language)
   - /home/jon/code/life-data/chatgpt/organized/archive/ (if exists)
   - /home/jon/code/life-data/keep-notes/
   - /home/jon/code/life-data/research/
   - /home/jon/code/life-data/books/
   - /home/jon/code/life-data/00-BUSINESS-IDEA-ANALYSIS-SUITE/
   - /home/jon/code/life-data/new-ideas/ (existing hardware subdirs)
4. For each location, run these greps:
   - `\b(?:RK35\d{2}|BCM2712?)\b` (SoCs)
   - `\b(?:Jetson|Coral|Hailo|AGX Orin|Edge TPU)\b` (AI accel)
   - `\b(?:ESP32|STM32|RP2040|RP2350)\b` (MCUs)
   - `\b(?:SBC|SoC|NPU|TPU|TOPS|TFLOPS)\b` (concept)
   - `local (?:AI|LLM|inference)` (use case)
   - `Raspberry Pi`
5. Return structured markdown:
   ## Hardware-Relevant Chat Folders (full path + brief topic + 1-line summary of hardware mentioned)
   ## Hardware-Relevant Project Folders
   ## Other Locations with Hardware Mentions
   ## Folders REVIEWED but Found Empty of Hardware

Be thorough. List every folder you checked even if empty so we know coverage is complete.
<!-- OMO_INTERNAL_INITIATOR -->

## [0063] 06-28 12:52

(2026-06-28T12:52:07)
computer restart continue

## [0064] 06-28 12:52

(2026-06-28T12:52:28)
CONTEXT: I'm building a consolidated report of all AI compute hardware the user has mentioned in their personal notes. The notes live under /home/jon/code/life-data/chatgpt/organized/chat-history/ — each subfolder is a conversation topic with chunk-NNN.md files inside.

GOAL: Extract EVERY mention of Single-Board Computers (SBCs) and their System-on-Chips (SoCs) across all chatgpt notes. I need specific model numbers, RAM tiers, requirements, and context.

DOWNSTREAM: I'll merge your findings into a consolidated hardware report. Be exhaustive — capture every chip model, every board, every spec mentioned.

REQUEST:
1. Search the ENTIRE tree under /home/jon/code/life-data/chatgpt/organized/chat-history/ for SBC and SoC references.
2. Specifically look for these folders (but don't limit yourself to them):
   - llms-local-setup-factors
   - cpu-ram-types-comparison
   - extensive-system-requirement-breakdown
   - llm-enabled-raspberry-pi-setup
   - raspberry-pi-media-os-list
   - hardware-setup-brainstorming
   - edge-computing-hardware-breakdown
   - e-ink-phones-with-ram
   - open-source-fire-stick-alternatives
3. Capture:
   - SBC brand + model (Raspberry Pi 4/5, Orange Pi 5, ODROID-M1, Rock PRO, Le Potato, Banana Pi, NanoPi, Radxa, etc.)
   - SoC model numbers (BCM2711, BCM2712, RK3588, RK3588S, RK3399, S905, Allwinner H6, etc.)
   - CPU cores/architecture (Cortex-A76, A78, A55, etc.)
   - RAM tiers mentioned (4GB, 8GB, 16GB, 32GB LPDDR4/LPDDR5)
   - User-stated requirements (min RAM for X model size, etc.)
   - Any "tier" classifications (entry/mid/high, etc.)
   - Price points or budget mentions
4. Use grep on patterns: `\bRK\d{4}\b`, `\bBCM2712?\b`, `Orange Pi`, `ODROID`, `Le Potato`, `Banana Pi`, `NanoPi`, `Radxa`, `Rock \d`, `Cortex-A\d{2}`, `LPDDR[45]`, `Raspberry Pi [45]`
5. Return findings as structured markdown with sections:
   ## SBCs Mentioned (with SoC + RAM + context)
   ## SoCs/Chips Mentioned (standalone)
   ## CPU Architectures
   ## RAM Tiers / Requirements
   ## Tier Classifications
   ## Price/Budget Mentions
   Each entry MUST cite source folder + chunk file.

Skip: generic AI discussion not tied to hardware. Skip: pure software/OS talk unless specs mentioned.

Be exhaustive. Better to over-collect than miss a chip mention.
<!-- OMO_INTERNAL_INITIATOR -->

## [0065] 06-28 12:52

(2026-06-28T12:52:34)
CONTEXT: I'm building a consolidated report of all AI compute hardware the user has mentioned in personal notes. Notes live under /home/jon/code/life-data/chatgpt/organized/chat-history/ — each subfolder has chunk-NNN.md files.

GOAL: Extract EVERY mention of AI accelerators, NPUs, TPUs, GPUs, and dedicated AI hardware across all chatgpt notes. Include specific product names and model numbers.

DOWNSTREAM: Merging into consolidated hardware report. Be exhaustive.

REQUEST:
1. Search ENTIRE tree under /home/jon/code/life-data/chatgpt/organized/chat-history/
2. Focus on these relevant folders (but check others too):
   - edge-computing-hardware-breakdown
   - llms-local-setup-factors
   - extensive-system-requirement-breakdown
   - llm-enabled-raspberry-pi-setup
   - hardware-setup-brainstorming
   - esp32-c6-ram-details
   - esp32-som-modules
3. Capture EVERY mention of:
   - NVIDIA Jetson family (Nano, Orin Nano, Orin NX, AGX Orin, Xavier NX, TX2, TX1) with specs
   - Google Coral (Edge TPU, Dev Board, USB Accelerator, SoC mt8167)
   - Hailo (Hailo-8, Hailo-15, Hailo-10H, etc.) with TOPS
   - Intel NUC / Intel Arc / Intel Movidius / NCS2
   - AMD Ryzen Embedded / AMD Radeon / AMD NUC alternatives
   - Apple Silicon (M1/M2/M3/M4, Ultra, Max) for local AI
   - NPUs (TOPS ratings, embedded NPUs)
   - Qualcomm AI engines / Hexagon DSP / Snapdragon X Elite
   - Any GPU mentioned for local inference (RTX 3060/4060/4070/4090, etc.)
   - Frameworks: CUDA, TensorRT, ROCm, OpenVINO, MLPerf
4. Use grep patterns: `Jetson`, `Coral`, `Hailo`, `TOPS`, `TFLOPS`, `NPU`, `TPU`, `Movidius`, `OpenVINO`, `CUDA`, `\bRTX\s?\d{4}\b`, `\bGTX\s?\d{4}\b`, `Hexagon`, `Apple M[1-4]`, `Tensor RT`
5. Return structured markdown:
   ## Dedicated AI Accelerators (Jetson, Coral, Hailo, etc.)
   ## GPUs for Local Inference
   ## Embedded NPUs / TPUs
   ## Apple Silicon mentions
   ## Frameworks/SDKs mentioned
   ## Performance specs (TOPS/TFLOPS cited)
   Each entry cites source folder + chunk file.

Skip: pure software discussions unless tied to hardware capability. Skip: cloud AI (AWS, GCP) unless compared against local.

Be exhaustive. Capture every model number, every TOPS rating, every requirement.
<!-- OMO_INTERNAL_INITIATOR -->

## [0066] 06-28 12:52

(2026-06-28T12:52:39)
CONTEXT: Building consolidated report of all AI compute hardware user mentioned in notes. Notes live under /home/jon/code/life-data/chatgpt/organized/chat-history/ — subfolders with chunk-NNN.md files.

GOAL: Extract EVERY mention of microcontrollers, ESP32 variants, TinyML hardware, and ultra-low-power edge AI devices.

DOWNSTREAM: Merging into consolidated hardware report. Be exhaustive.

REQUEST:
1. Search ENTIRE tree under /home/jon/code/life-data/chatgpt/organized/chat-history/
2. Focus on these folders (but check all):
   - esp32-c6-ram-details (9 chunks)
   - esp32-som-modules (4 chunks)
   - edge-computing-hardware-breakdown
   - hardware-setup-brainstorming
   - llms-local-setup-factors
3. Capture every mention of:
   - ESP32 variants (ESP32, ESP32-S2, S3, C3, C6, H2, P4) with specs
   - ESP32-SOM modules / ESP32 modules
   - STM32 family (STM32H7, F7, etc.)
   - RP2040 / RP2350 (Raspberry Pi Pico)
   - nRF52840 / nRF5340 (Nordic)
   - K210 / K230 (Kendryte / Sipeed Maix)
   - TinyML-class boards (Arduino Nano 33 BLE Sense, etc.)
   - Realtek RTL8xxx
   - MediaTek LinkIt / MT7697
   - microcontroller RAM/Flash tiers
   - Mentioned AI workloads on MCUs (keyword spotting, image classification, etc.)
4. Use grep patterns: `ESP32[-_]?(?:S2|S3|C3|C6|H2|P4)?`, `\bRP2\d{3}\b`, `\bSTM32\w+\b`, `\bnRF\d{4,5}\b`, `K210|K230|Kendryte|Maix`, `TinyML`, `Arduino Nano 33`, `Sony Spresense`, `\bM33\b|M4F?|FPU`
5. Return structured markdown:
   ## ESP32 Variants Mentioned (with specs)
   ## Other Microcontrollers (STM32, RP2040, nRF, etc.)
   ## TinyML Hardware (K210, Maix boards, etc.)
   ## MCU RAM/Flash Tiers Mentioned
   ## AI Workloads on MCU (what user wants to run)
   Each entry cites source folder + chunk file.

Be exhaustive — these are small files so don't skip anything.
<!-- OMO_INTERNAL_INITIATOR -->

## [0067] 06-28 12:52

(2026-06-28T12:52:43)
CONTEXT: Building consolidated report of all AI compute hardware user mentioned in notes. The user said "mainly in chatgpt notes but don't leave any stone unturned" — so I need to find EVERY folder/topic that mentions hardware.

GOAL: Index all chat folders that discuss AI compute hardware, plus check non-chatgpt note locations.

DOWNSTREAM: This becomes the master index for the report. Need to ensure nothing slips through.

REQUEST:
1. List ALL subfolders under /home/jon/code/life-data/chatgpt/organized/chat-history/ whose names suggest hardware/compute/AI/edge/SBC/SOC/embedded topics. (Folder names are descriptive — use that.)
2. Then grep each candidate folder to confirm it discusses relevant hardware.
3. ALSO check these other locations for hardware mentions:
   - /home/jon/code/life-data/chatgpt/organized/projects/ (esp. pcb-design-suggestions, e-ink-setup-plan, project-idea-brainstorming, designing-a-programming-language)
   - /home/jon/code/life-data/chatgpt/organized/archive/ (if exists)
   - /home/jon/code/life-data/keep-notes/
   - /home/jon/code/life-data/research/
   - /home/jon/code/life-data/books/
   - /home/jon/code/life-data/00-BUSINESS-IDEA-ANALYSIS-SUITE/
   - /home/jon/code/life-data/new-ideas/ (existing hardware subdirs)
4. For each location, run these greps:
   - `\b(?:RK35\d{2}|BCM2712?)\b` (SoCs)
   - `\b(?:Jetson|Coral|Hailo|AGX Orin|Edge TPU)\b` (AI accel)
   - `\b(?:ESP32|STM32|RP2040|RP2350)\b` (MCUs)
   - `\b(?:SBC|SoC|NPU|TPU|TOPS|TFLOPS)\b` (concept)
   - `local (?:AI|LLM|inference)` (use case)
   - `Raspberry Pi`
5. Return structured markdown:
   ## Hardware-Relevant Chat Folders (full path + brief topic + 1-line summary of hardware mentioned)
   ## Hardware-Relevant Project Folders
   ## Other Locations with Hardware Mentions
   ## Folders REVIEWED but Found Empty of Hardware

Be thorough. List every folder you checked even if empty so we know coverage is complete.
<!-- OMO_INTERNAL_INITIATOR -->

## [0068] 06-28 12:55

(2026-06-28T12:55:07)
[search-mode]
MAXIMIZE SEARCH EFFORT. Launch multiple background agents IN PARALLEL:
- explore agents (codebase patterns, file structures, ast-grep)
- librarian agents (remote repos, official docs, GitHub examples)
Plus direct tools: Grep, ripgrep (rg), ast-grep (sg)
NEVER stop at first result - be exhaustive.

[analyze-mode]
ANALYSIS MODE. Gather context before diving deep:
CONTEXT GATHERING (parallel):
- 1-2 explore agents (codebase patterns, implementations)
- 1-2 librarian agents (if external library involved)
- Direct tools: Grep, AST-grep, LSP for targeted searches

IF COMPLEX - DO NOT STRUGGLE ALONE. Consult specialists:
- **Oracle**: Conventional problems (architecture, debugging, complex logic)
- **Artistry**: Non-conventional problems (different approach needed)

SYNTHESIZE findings before proceeding.
---
MANDATORY delegate_task params: ALWAYS include load_skills=[] and run_in_background when calling delegate_task.
Example: delegate_task(subagent_type="explore", prompt="...", run_in_background=true, load_skills=[])

---

<auto-slash-command>
# /ralph-loop Command

**Description**: (builtin) Start self-referential development loop until completion

**User Arguments**: Do web search on what option for anything above the 8gb ddr5 ram sbcs and socs that exist in the orange pi and banana pi.  only ddr4 with 32-64gb ddr4 ram options.  do web researchn then respond with a detailed breakdown of tiers and options and costs on aliexpress.  don't miss any option and there are definitely more than 20 that exist so give me a list of 10 per tier or category in terms of specification options usability opensource ecosystem for running linux on them, and ram tier for ddr and amount.

**Scope**: builtin

---

## Command Instructions

<command-instruction>
You are starting a Ralph Loop - a self-referential development loop that runs until task completion.

## How Ralph Loop Works

1. You will work on the task continuously
2. When you believe the task is FULLY complete, output: `<promise>{{COMPLETION_PROMISE}}</promise>`
3. If you don't output the promise, the loop will automatically inject another prompt to continue
4. Maximum iterations: Configurable (default 100)

## Rules

- Focus on completing the task fully, not partially
- Don't output the completion promise until the task is truly done
- Each iteration should make meaningful progress toward the goal
- If stuck, try different approaches
- Use todos to track your progress

## Exit Conditions

1. **Completion**: Output your completion promise tag when fully complete
2. **Max Iterations**: Loop stops automatically at limit
3. **Cancel**: User runs `/cancel-ralph` command

## Your Task

Parse the arguments below and begin working on the task. The format is:
`"task description" [--completion-promise=TEXT] [--max-iterations=N] [--strategy=reset|continue]`

Default completion promise is "DONE" and default max iterations is 100.
</command-instruction>

<user-task>
Do web search on what option for anything above the 8gb ddr5 ram sbcs and socs that exist in the orange pi and banana pi.  only ddr4 with 32-64gb ddr4 ram options.  do web researchn then respond with a detailed breakdown of tiers and options and costs on aliexpress.  don't miss any option and there are definitely more than 20 that exist so give me a list of 10 per tier or category in terms of specification options usability opensource ecosystem for running linux on them, and ram tier for ddr and amount.
</user-task>


---

## User Request

Do web search on what option for anything above the 8gb ddr5 ram sbcs and socs that exist in the orange pi and banana pi.  only ddr4 with 32-64gb ddr4 ram options.  do web researchn then respond with a detailed breakdown of tiers and options and costs on aliexpress.  don't miss any option and there are definitely more than 20 that exist so give me a list of 10 per tier or category in terms of specification options usability opensource ecosystem for running linux on them, and ram tier for ddr and amount.
</auto-slash-command>

## [0069] 06-28 12:55

<command-instruction>
You are starting a Ralph Loop - a self-referential development loop that runs until task completion.

## How Ralph Loop Works

1. You will work on the task continuously
2. When you believe the task is FULLY complete, output: `<promise>{{COMPLETION_PROMISE}}</promise>`
3. If you don't output the promise, the loop will automatically inject another prompt to continue
4. Maximum iterations: Configurable (default 100)

## Rules

- Focus on completing the task fully, not partially
- Don't output the completion promise until the task is truly done
- Each iteration should make meaningful progress toward the goal
- If stuck, try different approaches
- Use todos to track your progress

## Exit Conditions

1. **Completion**: Output your completion promise tag when fully complete
2. **Max Iterations**: Loop stops automatically at limit
3. **Cancel**: User runs `/cancel-ralph` command

## Your Task

Parse the arguments below and begin working on the task. The format is:
`"task description" [--completion-promise=TEXT] [--max-iterations=N] [--strategy=reset|continue]`

Default completion promise is "DONE" and default max iterations is 100.
</command-instruction>

<user-task>
Do web search on what option for anything above the 8gb ddr5 ram sbcs and socs that exist in the orange pi and banana pi.  only ddr4 with 32-64gb ddr4 ram options.  do web researchn then respond with a detailed breakdown of tiers and options and costs on aliexpress.  don't miss any option and there are definitely more than 20 that exist so give me a list of 10 per tier or category in terms of specification options usability opensource ecosystem for running linux on them, and ram tier for ddr and amount.
</user-task>

## [0070] 06-28 12:55

(2026-06-28T12:55:20)
RESEARCH TASK: Find ALL Orange Pi SBC models that use DDR5 RAM with 8GB or more. Include the complete Orange Pi 5 series (5, 5 Plus, 5 Pro, 5 Max, 5 Ultra, 5B, 5 Super, 5X, 5G, 5A, 5 Plus V2.1, any new 2024-2025 releases) and any RK3588/RK3588S/RK3592 based boards.

For EACH model found:
- Exact model name
- SoC used (RK3588, RK3588S, RK3592, etc.)
- RAM type (DDR4 vs DDR5) and amount (8GB, 12GB, 16GB, 32GB, 64GB)
- Key specs (CPU cores, GPU, NPU, storage, networking)
- AliExpress price range (USD)

Focus on DDR5 boards specifically. Also note which Orange Pi boards support LPDDR5 vs standard DDR5.

Search AliExpress listings, orangepi.org, Xunlong official wiki, and community forums.
<!-- OMO_INTERNAL_INITIATOR -->

## [0071] 06-28 12:55

(2026-06-28T12:55:24)
RESEARCH TASK: Find ALL Orange Pi SBC models that use DDR4 RAM with 16GB or more, especially 32GB and 64GB options. These would typically be boards with DDR4 DIMM slots (not soldered RAM) that allow users to install their own RAM modules.

Focus on:
- Orange Pi RK3399 based boards (RK3399 supports up to 4GB LPDDR4, so check for any with DIMM slots)
- Orange Pi RK3588/RK3588S boards - some have DDR4 DIMM slots for SO-DIMM
- Orange Pi boards based on Intel/AMD or other x86 SoCs
- Orange Pi boards with DDR4 SO-DIMM slots (like mini PC form factors)
- Orange Pi 800, Orange Pi 700, Orange Pi Win, Orange Pi RKBoard series
- Any Orange Pi NAS/server boards with RAM slots

For EACH model:
- Exact model name
- SoC
- RAM type and max supported
- DIMM slots? (SO-DIMM vs soldered)
- AliExpress price
- Linux support status

Search: orangepi.org, AliExpress "orange pi 64gb", AliExpress "orange pi 32gb", AliExpress "orange pi ddr4 sodimm"
<!-- OMO_INTERNAL_INITIATOR -->

## [0072] 06-28 12:55

(2026-06-28T12:55:30)
RESEARCH TASK: Find ALL Banana Pi SBC models that support 32GB or 64GB DDR4 RAM. Banana Pi (by LeMaker/Sinovoip) has a large lineup including boards with DDR4 SO-DIMM slots.

Focus on:
- Banana Pi BPI-R4 (Mediatek MT7986, Wi-Fi 7 router board)
- Banana Pi BPI-M7 (RK3588 based, DDR4 SO-DIMM slot)
- Banana Pi BPI-M6 (RK3588S based)
- Banana Pi BPI-M5 Pro (RK3588S)
- Banana Pi BPI-F3 (SpacemiT K1)
- Banana Pi BPI-CM4 / CM5 (like RPi CM form factor)
- Banana Pi BPI-W3 (Allwinner T3)
- Banana Pi BPI-M2 Ultra, BPI-M2 Berry, BPI-M2 Plus
- Banana Pi BPI-M4 Berry (RK3399)
- Banana Pi BPI-R2 Pro, BPI-R3
- Banana Pi BPI-A20, BPI-M1, BPI-M1+
- Banana Pi BPI-Nano, BPI-R1, BPI-G1
- Banana Pi BPI-5B (RK3588S)
- Banana Pi BPI-M5S (RK3588S)
- Banana Pi BPI-M4V2 (RK3588S)
- Banana Pi PRO / Banana Pi Pro MAX
- Banana Pi BPI-T4 (RK3588)
- Any x86-based Banana Pi boards
- Banana Pi BPI-5200, BPI-5300, BPI-5500 series

For EACH: model, SoC, RAM type (DDR4 LPDDR4 DDR5), max RAM, DIMM slot or soldered, AliExpress price, Linux ecosystem support.

Search: bananapi.org, AliExpress "banana pi 64gb", AliExpress "banana pi 32gb", "banana pi ddr4"
<!-- OMO_INTERNAL_INITIATOR -->

## [0073] 06-28 12:55

(2026-06-28T12:55:34)
RESEARCH TASK: Search AliExpress for all Orange Pi and Banana Pi SBC listings with 32GB and 64GB RAM configurations. Find actual current prices.

Search queries to try:
- "orange pi 32gb" on AliExpress
- "orange pi 64gb" on AliExpress  
- "banana pi 32gb" on AliExpress
- "banana pi 64gb" on AliExpress
- "orange pi ddr4 16gb" AliExpress
- "banana pi rk3588 16gb" AliExpress
- "banana pi ddr5" AliExpress
- "orange pi ddr5" AliExpress
- "RK3588 board 32gb" AliExpress
- "RK3588 board 64gb" AliExpress

For each listing found, note:
- Exact product title
- Store name
- Price in USD
- RAM configuration
- Whether RAM is soldered or DIMM slot
- Shipping time/availability

Also search for: orangepi.aliexpress.com (official store) and sinovoip.aliexpress.com (Banana Pi official)
<!-- OMO_INTERNAL_INITIATOR -->

## [0074] 06-28 12:55

(2026-06-28T12:55:38)
RESEARCH TASK: Find ALL RK3588/RK3588S based SBCs from Orange Pi and Banana Pi that offer 8GB, 12GB, 16GB, or higher RAM options. The RK3588 family is the most common high-RAM ARM SoC.

For each board, research:
1. Mainline Linux kernel support (which kernel version works out of box)
2. Armbian support status (supported, in development, not supported)
3. Ubuntu/Debian official images availability
4. HDMI/Display outputs
5. NVMe SSD support
6. PCIe lanes
7. GPIO/pin header compatibility
8. Community size (GitHub stars on BSP, wiki activity)
9. Active maintenance (last BSP update, last community build)

Orange Pi RK3588 boards: OPi 5, 5 Plus, 5 Pro, 5 Max, 5B, 5 Super
Banana Pi RK3588 boards: BPI-M7, BPI-M5 Pro, BPI-5B, BPI-M5S, BPI-M4V2, BPI-T4, BPI-F3

Search: armbian.com "hardware" page for supported RK3588 boards, wiki.radxa.com, rockchip official docs, GitHub armbian/build
<!-- OMO_INTERNAL_INITIATOR -->

## [0075] 06-28 12:56

(2026-06-28T12:56:59)
RESEARCH TASK: Get the COMPLETE current product catalog for both Orange Pi and Banana Pi. I need every single model they sell, not just the popular ones.

For Orange Pi (orangepi.org):
- Navigate to orangepi.org and find their full product listing
- Get all current production models
- Note: Orange Pi 5 series, Orange Pi 3 series, Orange Pi Zero series, Orange Pi PC series, Orange Pi Win series, Orange Pi RK series, Orange Pi 800/700, Orange Pi AI, Orange Pi Neo, Orange Pi 800G, Orange Pi One, Orange Pi Lite, Orange Pi Plus, Orange Pi 2, Orange Pi Prime, Orange Pi R1, Orange Pi i96, Orange Pi 96, etc.

For Banana Pi (bananapi.org):
- Navigate to bananapi.org and get full product listing  
- Get all current production models
- Note: BPI-M series, BPI-R series, BPI-F series, BPI-CM series, BPI-Pro series, BPI-Nano series, BPI-G1, BPI-A1, etc.

For EACH model: Name, SoC, RAM (type + amount), form factor, status (current production / discontinued), price.

Also search for any 2024 or 2025 new releases from both brands.
<!-- OMO_INTERNAL_INITIATOR -->

## [0076] 06-28 12:56

(2026-06-28T12:56:59)
RESEARCH TASK: Find ALL Banana Pi SBC models that support 32GB or 64GB DDR4 RAM. Banana Pi (by LeMaker/Sinovoip) has a large lineup including boards with DDR4 SO-DIMM slots.

Focus on:
- Banana Pi BPI-R4 (Mediatek MT7986, Wi-Fi 7 router board)
- Banana Pi BPI-M7 (RK3588 based, DDR4 SO-DIMM slot)
- Banana Pi BPI-M6 (RK3588S based)
- Banana Pi BPI-M5 Pro (RK3588S)
- Banana Pi BPI-F3 (SpacemiT K1)
- Banana Pi BPI-CM4 / CM5 (like RPi CM form factor)
- Banana Pi BPI-W3 (Allwinner T3)
- Banana Pi BPI-M2 Ultra, BPI-M2 Berry, BPI-M2 Plus
- Banana Pi BPI-M4 Berry (RK3399)
- Banana Pi BPI-R2 Pro, BPI-R3
- Banana Pi BPI-A20, BPI-M1, BPI-M1+
- Banana Pi BPI-Nano, BPI-R1, BPI-G1
- Banana Pi BPI-5B (RK3588S)
- Banana Pi BPI-M5S (RK3588S)
- Banana Pi BPI-M4V2 (RK3588S)
- Banana Pi PRO / Banana Pi Pro MAX
- Banana Pi BPI-T4 (RK3588)
- Any x86-based Banana Pi boards
- Banana Pi BPI-5200, BPI-5300, BPI-5500 series

For EACH: model, SoC, RAM type (DDR4 LPDDR4 DDR5), max RAM, DIMM slot or soldered, AliExpress price, Linux ecosystem support.

Search: bananapi.org, AliExpress "banana pi 64gb", AliExpress "banana pi 32gb", "banana pi ddr4"
<!-- OMO_INTERNAL_INITIATOR -->

## [0077] 06-28 12:56

(2026-06-28T12:56:59)
RESEARCH TASK: Find ALL Banana Pi SBC models that support 32GB or 64GB DDR4 RAM. Banana Pi (by LeMaker/Sinovoip) has a large lineup including boards with DDR4 SO-DIMM slots.

Focus on:
- Banana Pi BPI-R4 (Mediatek MT7986, Wi-Fi 7 router board)
- Banana Pi BPI-M7 (RK3588 based, DDR4 SO-DIMM slot)
- Banana Pi BPI-M6 (RK3588S based)
- Banana Pi BPI-M5 Pro (RK3588S)
- Banana Pi BPI-F3 (SpacemiT K1)
- Banana Pi BPI-CM4 / CM5 (like RPi CM form factor)
- Banana Pi BPI-W3 (Allwinner T3)
- Banana Pi BPI-M2 Ultra, BPI-M2 Berry, BPI-M2 Plus
- Banana Pi BPI-M4 Berry (RK3399)
- Banana Pi BPI-R2 Pro, BPI-R3
- Banana Pi BPI-A20, BPI-M1, BPI-M1+
- Banana Pi BPI-Nano, BPI-R1, BPI-G1
- Banana Pi BPI-5B (RK3588S)
- Banana Pi BPI-M5S (RK3588S)
- Banana Pi BPI-M4V2 (RK3588S)
- Banana Pi PRO / Banana Pi Pro MAX
- Banana Pi BPI-T4 (RK3588)
- Any x86-based Banana Pi boards
- Banana Pi BPI-5200, BPI-5300, BPI-5500 series

For EACH: model, SoC, RAM type (DDR4 LPDDR4 DDR5), max RAM, DIMM slot or soldered, AliExpress price, Linux ecosystem support.

Search: bananapi.org, AliExpress "banana pi 64gb", AliExpress "banana pi 32gb", "banana pi ddr4"
<!-- OMO_INTERNAL_INITIATOR -->

## [0078] 06-28 12:56

(2026-06-28T12:56:59)
RESEARCH TASK: Find ALL Banana Pi SBC models that support 32GB or 64GB DDR4 RAM. Banana Pi (by LeMaker/Sinovoip) has a large lineup including boards with DDR4 SO-DIMM slots.

Focus on:
- Banana Pi BPI-R4 (Mediatek MT7986, Wi-Fi 7 router board)
- Banana Pi BPI-M7 (RK3588 based, DDR4 SO-DIMM slot)
- Banana Pi BPI-M6 (RK3588S based)
- Banana Pi BPI-M5 Pro (RK3588S)
- Banana Pi BPI-F3 (SpacemiT K1)
- Banana Pi BPI-CM4 / CM5 (like RPi CM form factor)
- Banana Pi BPI-W3 (Allwinner T3)
- Banana Pi BPI-M2 Ultra, BPI-M2 Berry, BPI-M2 Plus
- Banana Pi BPI-M4 Berry (RK3399)
- Banana Pi BPI-R2 Pro, BPI-R3
- Banana Pi BPI-A20, BPI-M1, BPI-M1+
- Banana Pi BPI-Nano, BPI-R1, BPI-G1
- Banana Pi BPI-5B (RK3588S)
- Banana Pi BPI-M5S (RK3588S)
- Banana Pi BPI-M4V2 (RK3588S)
- Banana Pi PRO / Banana Pi Pro MAX
- Banana Pi BPI-T4 (RK3588)
- Any x86-based Banana Pi boards
- Banana Pi BPI-5200, BPI-5300, BPI-5500 series

For EACH: model, SoC, RAM type (DDR4 LPDDR4 DDR5), max RAM, DIMM slot or soldered, AliExpress price, Linux ecosystem support.

Search: bananapi.org, AliExpress "banana pi 64gb", AliExpress "banana pi 32gb", "banana pi ddr4"
<!-- OMO_INTERNAL_INITIATOR -->

## [0079] 06-28 12:56

(2026-06-28T12:56:59)
RESEARCH TASK: Find ALL Banana Pi SBC models that support 32GB or 64GB DDR4 RAM. Banana Pi (by LeMaker/Sinovoip) has a large lineup including boards with DDR4 SO-DIMM slots.

Focus on:
- Banana Pi BPI-R4 (Mediatek MT7986, Wi-Fi 7 router board)
- Banana Pi BPI-M7 (RK3588 based, DDR4 SO-DIMM slot)
- Banana Pi BPI-M6 (RK3588S based)
- Banana Pi BPI-M5 Pro (RK3588S)
- Banana Pi BPI-F3 (SpacemiT K1)
- Banana Pi BPI-CM4 / CM5 (like RPi CM form factor)
- Banana Pi BPI-W3 (Allwinner T3)
- Banana Pi BPI-M2 Ultra, BPI-M2 Berry, BPI-M2 Plus
- Banana Pi BPI-M4 Berry (RK3399)
- Banana Pi BPI-R2 Pro, BPI-R3
- Banana Pi BPI-A20, BPI-M1, BPI-M1+
- Banana Pi BPI-Nano, BPI-R1, BPI-G1
- Banana Pi BPI-5B (RK3588S)
- Banana Pi BPI-M5S (RK3588S)
- Banana Pi BPI-M4V2 (RK3588S)
- Banana Pi PRO / Banana Pi Pro MAX
- Banana Pi BPI-T4 (RK3588)
- Any x86-based Banana Pi boards
- Banana Pi BPI-5200, BPI-5300, BPI-5500 series

For EACH: model, SoC, RAM type (DDR4 LPDDR4 DDR5), max RAM, DIMM slot or soldered, AliExpress price, Linux ecosystem support.

Search: bananapi.org, AliExpress "banana pi 64gb", AliExpress "banana pi 32gb", "banana pi ddr4"
<!-- OMO_INTERNAL_INITIATOR -->

## [0080] 06-28 12:57

(2026-06-28T12:57:01)
RESEARCH TASK: Get the COMPLETE current product catalog for both Orange Pi and Banana Pi. I need every single model they sell, not just the popular ones.

For Orange Pi (orangepi.org):
- Navigate to orangepi.org and find their full product listing
- Get all current production models
- Note: Orange Pi 5 series, Orange Pi 3 series, Orange Pi Zero series, Orange Pi PC series, Orange Pi Win series, Orange Pi RK series, Orange Pi 800/700, Orange Pi AI, Orange Pi Neo, Orange Pi 800G, Orange Pi One, Orange Pi Lite, Orange Pi Plus, Orange Pi 2, Orange Pi Prime, Orange Pi R1, Orange Pi i96, Orange Pi 96, etc.

For Banana Pi (bananapi.org):
- Navigate to bananapi.org and get full product listing  
- Get all current production models
- Note: BPI-M series, BPI-R series, BPI-F series, BPI-CM series, BPI-Pro series, BPI-Nano series, BPI-G1, BPI-A1, etc.

For EACH model: Name, SoC, RAM (type + amount), form factor, status (current production / discontinued), price.

Also search for any 2024 or 2025 new releases from both brands.
<!-- OMO_INTERNAL_INITIATOR -->

## [0081] 06-28 12:57

(2026-06-28T12:57:01)
RESEARCH TASK: Get the COMPLETE current product catalog for both Orange Pi and Banana Pi. I need every single model they sell, not just the popular ones.

For Orange Pi (orangepi.org):
- Navigate to orangepi.org and find their full product listing
- Get all current production models
- Note: Orange Pi 5 series, Orange Pi 3 series, Orange Pi Zero series, Orange Pi PC series, Orange Pi Win series, Orange Pi RK series, Orange Pi 800/700, Orange Pi AI, Orange Pi Neo, Orange Pi 800G, Orange Pi One, Orange Pi Lite, Orange Pi Plus, Orange Pi 2, Orange Pi Prime, Orange Pi R1, Orange Pi i96, Orange Pi 96, etc.

For Banana Pi (bananapi.org):
- Navigate to bananapi.org and get full product listing  
- Get all current production models
- Note: BPI-M series, BPI-R series, BPI-F series, BPI-CM series, BPI-Pro series, BPI-Nano series, BPI-G1, BPI-A1, etc.

For EACH model: Name, SoC, RAM (type + amount), form factor, status (current production / discontinued), price.

Also search for any 2024 or 2025 new releases from both brands.
<!-- OMO_INTERNAL_INITIATOR -->

## [0082] 06-28 12:57

(2026-06-28T12:57:01)
RESEARCH TASK: Get the COMPLETE current product catalog for both Orange Pi and Banana Pi. I need every single model they sell, not just the popular ones.

For Orange Pi (orangepi.org):
- Navigate to orangepi.org and find their full product listing
- Get all current production models
- Note: Orange Pi 5 series, Orange Pi 3 series, Orange Pi Zero series, Orange Pi PC series, Orange Pi Win series, Orange Pi RK series, Orange Pi 800/700, Orange Pi AI, Orange Pi Neo, Orange Pi 800G, Orange Pi One, Orange Pi Lite, Orange Pi Plus, Orange Pi 2, Orange Pi Prime, Orange Pi R1, Orange Pi i96, Orange Pi 96, etc.

For Banana Pi (bananapi.org):
- Navigate to bananapi.org and get full product listing  
- Get all current production models
- Note: BPI-M series, BPI-R series, BPI-F series, BPI-CM series, BPI-Pro series, BPI-Nano series, BPI-G1, BPI-A1, etc.

For EACH model: Name, SoC, RAM (type + amount), form factor, status (current production / discontinued), price.

Also search for any 2024 or 2025 new releases from both brands.
<!-- OMO_INTERNAL_INITIATOR -->

## [0083] 06-28 12:57

(2026-06-28T12:57:01)
RESEARCH TASK: Get the COMPLETE current product catalog for both Orange Pi and Banana Pi. I need every single model they sell, not just the popular ones.

For Orange Pi (orangepi.org):
- Navigate to orangepi.org and find their full product listing
- Get all current production models
- Note: Orange Pi 5 series, Orange Pi 3 series, Orange Pi Zero series, Orange Pi PC series, Orange Pi Win series, Orange Pi RK series, Orange Pi 800/700, Orange Pi AI, Orange Pi Neo, Orange Pi 800G, Orange Pi One, Orange Pi Lite, Orange Pi Plus, Orange Pi 2, Orange Pi Prime, Orange Pi R1, Orange Pi i96, Orange Pi 96, etc.

For Banana Pi (bananapi.org):
- Navigate to bananapi.org and get full product listing  
- Get all current production models
- Note: BPI-M series, BPI-R series, BPI-F series, BPI-CM series, BPI-Pro series, BPI-Nano series, BPI-G1, BPI-A1, etc.

For EACH model: Name, SoC, RAM (type + amount), form factor, status (current production / discontinued), price.

Also search for any 2024 or 2025 new releases from both brands.
<!-- OMO_INTERNAL_INITIATOR -->

## [0084] 06-28 13:00

Continue if you have next steps, or stop and ask for clarification if you are unsure how to proceed.

## [0085] 06-28 13:00

(2026-06-28T13:00:57)
[restore checkpointed session agent configuration after compaction]
<!-- OMO_INTERNAL_INITIATOR -->

## [0086] 06-28 13:26

(2026-06-28T13:26:11)
[search-mode]
MAXIMIZE SEARCH EFFORT. Launch multiple background agents IN PARALLEL:
- explore agents (codebase patterns, file structures, ast-grep)
- librarian agents (remote repos, official docs, GitHub examples)
Plus direct tools: Grep, ripgrep (rg), ast-grep (sg)
NEVER stop at first result - be exhaustive.

[analyze-mode]
ANALYSIS MODE. Gather context before diving deep:
CONTEXT GATHERING (parallel):
- 1-2 explore agents (codebase patterns, implementations)
- 1-2 librarian agents (if external library involved)
- Direct tools: Grep, AST-grep, LSP for targeted searches

IF COMPLEX - DO NOT STRUGGLE ALONE. Consult specialists:
- **Oracle**: Conventional problems (architecture, debugging, complex logic)
- **Artistry**: Non-conventional problems (different approach needed)

SYNTHESIZE findings before proceeding.
---
MANDATORY delegate_task params: ALWAYS include load_skills=[] and run_in_background when calling delegate_task.
Example: delegate_task(subagent_type="explore", prompt="...", run_in_background=true, load_skills=[])

---

# Consolidated AI Compute Hardware Report

**Generated**: 2026-06-28
**Scope**: Every mention of SOCs, SBCs, AI accelerators, GPUs, microcontrollers, and related hardware for running local AI — extracted from personal notes (primarily ChatGPT exports, plus project/research folders).

---

## Methodology

Sources combed (exhaustively):

- `chatgpt/organized/chat-history/` — 330+ topic folders
- `chatgpt/organized/projects/` — 20 project folders (3 hardware-relevant)
- `chatgpt/organized/archive/` — 23 folders (no AI hardware)
- `keep-notes/` — 6 folders (no AI hardware)
- `research/esp-idf/` — ESP32 SDK docs
- `books/` — 2 folders (no AI hardware)
- `00-BUSINESS-IDEA-ANALYSIS-SUITE/` — 4 files mention SBCs
- `new-ideas/hardware/peripherals/` — scroll-ball speech-to-text peripheral project

35+ chat folders contained AI compute hardware mentions. All entries below are deduplicated and consolidated by category.

---

## Table of Contents

1. [User's Current Setup](#1-users-current-setup)
2. [Workload Tier Classification (user's framework)](#2-workload-tier-classification-users-framework)
3. [Single-Board Computers (SBCs)](#3-single-board-computers-sbcs)
4. [System-on-Chips (SoCs)](#4-system-on-chips-socs)
5. [AI Accelerators (Jetson, Coral, Hailo, etc.)](#5-ai-accelerators-jetson-coral-hailo-etc)
6. [GPUs for Local Inference](#6-gpus-for-local-inference)
7. [Data Center / 128GB+ VRAM Hardware](#7-data-center--128gb-vram-hardware)
8. [Mini PCs / Workstations](#8-mini-pcs--workstations)
9. [x86 CPUs Mentioned](#9-x86-cpus-mentioned)
10. [Apple Silicon](#10-apple-silicon)
11. [Microcontrollers (ESP32 family etc.)](#11-microcontrollers-esp32-family-etc)
12. [TinyML Hardware](#12-tinyml-hardware)
13. [Custom PCB Components (WatchyAI project)](#13-custom-pcb-components-watchyai-project)
14. [DIY GPU Components (128GB VRAM project)](#14-diy-gpu-components-128gb-vram-project)
15. [Memory Types & Tiers](#15-memory-types--tiers)
16. [Software Stack / Frameworks Mentioned](#16-software-stack--frameworks-mentioned)
17. [Performance Specs Reference](#17-performance-specs-reference)
18. [Price Bands](#18-price-bands)
19. [Source Folder Index](#19-source-folder-index)

---

## 1. User's Current Setup

From `gpu-offloading-for-llms/`:

- **GPU**: 8 GB VRAM (moderate gaming hardware; almost certainly AMD Radeon RX 580 8GB per `backup-and-os-recommendation/` and `localforge-ai-ide/`)
- **System RAM**: 16 GB
- **Software stack**: LM Studio + llama.cpp + Vulkan backend
- **Pain point**: RX 580 (Polaris/gfx803) NOT supported by modern ROCm — must fall back to Vulkan/llama.cpp

---

## 2. Workload Tier Classification (user's framework)

From `edge-computing-hardware-breakdown/_summary.md` and chunk-014/018/020.

### Tier A — Micro LLM Node (4–8 GB RAM)
**Goal**: Competent scoped summaries + extraction; speed doesn't matter.
- **Examples**: Raspberry Pi 5 (8 GB), Orange Pi 5 (8 GB), Radxa ROCK 5 (8 GB), Intel N100 mini PC (16 GB)
- **Realistic workload**: 200–500 lines per call; one-shot 500 lines depends on line length + context size

### Tier B — Medium Orchestrator (16–32 GB RAM)
**Goal**: Task orchestration + light reasoning; still power-efficient.
- **Examples**: Orange Pi 5 Plus (16/32 GB), Radxa ROCK 5B (up to 32 GB), Ryzen 7840/8845HS mini PCs
- **Realistic workload**: ~1,000 lines one-shot at 16k context; 2,000 lines needs 32k context

### Tier C — Best Under $1k (Multi-Project + High Speed)
**Goal**: Fast multi-project concurrency; GPU wins.
- **Examples**: RTX 4060 Ti 16GB (~48 tok/s on Llama 3.1 8B Q4)
- **Alternative**: High-RAM CPU orchestrator for fitting bigger models slowly (1–3 tok/s)

### RAM → Model Size Mapping
| RAM | Achievable Model Size |
|---|---|
| 2 GB | 1B class (TinyLlama), 400–500 lines max |
| 4 GB | 1B–3B class, 500–1000 lines |
| 8 GB | 3B class for better summarization; 7–8B quant with usable context |
| 16 GB | 7B–9B quant, ~1000 lines one-shot at 60 chars/line; CPU-only ARM "agentic sweet spot" |
| 32 GB | 8B–14B models with longer context |
| 48 GB | ODROID-H4 Ultra ceiling |
| 64 GB | Mini-ITX boards, ODROID-H3+ max |
| 128 GB | Data-center class (MI250X, H200) |
| 192 GB | Orange Pi AI Studio Pro claimed ceiling (with caveats) |

### Three Power/Workload Bands (from edge-computing-hardware-breakdown/_summary.md)
- **2–4 GB / 0 W (lowest/slowest)** — Radxa ZERO 3W class
- **Mid** — typical RK3588 boards
- **High** — mini-PC / GPU

---

## 3. Single-Board Computers (SBCs)

### Raspberry Pi Family
| Board | SoC | RAM | Notes | Price | Source |
|---|---|---|---|---|---|
| Raspberry Pi 4 | BCM2711 | up to 8 GB | Suggested for LLM workloads | ~$75 (8 GB Amazon) | llm-enabled-raspberry-pi-setup/, hardware-setup-brainstorming/ |
| Raspberry Pi 5 (8 GB) | BCM2712 | 8 GB LPDDR4X | ~4–6 tok/s on 7B quant; ~2.0 tok/s on 8B | ~$95–103 | edge-computing-hardware-breakdown/chunk-018 |
| Raspberry Pi 5 (16 GB) | BCM2712 | 16 GB LPDDR4X | MSRP $120 | $120 | edge-computing-hardware-breakdown/chunk-020 |
| Raspberry Pi Compute Module 5 | BCM2712 | up to 16 GB | | | esp32-c6-ram-details/chunk-006, 009 |
| Raspberry Pi Zero 2 W | BCM2710A1 | 512 MB | Ultra-low-power tier | | edge-computing-hardware-breakdown |

### Radxa
| Board | SoC | RAM | Notes | Price | Source |
|---|---|---|---|---|---|
| Radxa ZERO 3W (2 GB) | RK3566 (4× Cortex-A55 @ 1.6 GHz) | 2/4/8 GB | 65×30 mm; "0 W lowest/slowest" tier | $22–50 | edge-computing-hardware-breakdown/_summary, chunk-014/015 |
| Radxa ROCK 3A | RK356x | 2/4/8 GB | | $45–65 | edge-computing-hardware-breakdown/chunk-014 |
| Radxa ROCK 3C | RK356x | 8 GB | | $54.99 | edge-computing-hardware-breakdown/chunk-014 |
| Radxa ROCK 5B | RK3588 | 8/16/32 GB | Broad IO; Pi 5 alternative | $119–230 (32 GB $215–230) | llms-local-setup-factors/chunk-005/006, edge-computing-hardware-breakdown/chunk-018 |
| Radxa ROCK 5B+ | RK3588 | 16/24 GB | Newer revision | $119–159 | llms-local-setup-factors/chunk-006 |
| Radxa ROCK 5 ITX | RK3588 | up to 32 GB | ITX form factor | | llms-local-setup-factors/chunk-005 |
| Radxa CM5 | RK3588 | LPDDR4X + eMMC | Compute Module (CPU+GPU+NPU+PMU integrated) | | edge-computing-hardware-breakdown/chunk-008 |

### Orange Pi
| Board | SoC | RAM | Notes | Price | Source |
|---|---|---|---|---|---|
| Orange Pi 5 | RK3588 | 4 GB | Better $/compute than Pi | ~$60–80 | hardware-setup-brainstorming/chunk-003 |
| Orange Pi 5 Plus (32 GB) | RK3588 | 32 GB LPDDR4X-2133 | 34.1 GB/s bandwidth, 6 TOPS NPU | ~$320 | llms-local-setup-factors/chunk-011, esp32-c6-ram-details/chunk-009 |
| Orange Pi 5 Pro (32 GB) | RK3588 | 32 GB LPDDR5 | Newer memory tech | | llms-local-setup-factors/chunk-005 |
| Orange Pi AI-Pro | (20 TOPS NPU) | 12–24 GB | MindSpore-native | $150–200 | tattoo-idea-brainstorming/chunk-006 |
| Orange Pi AI Studio Pro | | up to 192 GB (claimed, with limitations) | | | esp32-c6-ram-details/chunk-006 |

### Khadas
| Board | SoC | RAM | Notes | Price | Source |
|---|---|---|---|---|---|
| Khadas Edge2 Pro | RK3588S | 16 GB LPDDR4X | Compact premium | ~$299 | llms-local-setup-factors/chunk-006 |

### NanoPi / FriendlyELEC
| Board | SoC | RAM | Notes | Source |
|---|---|---|---|---|
| NanoPC T6 | RK3588 | 32 GB variant | | llms-local-setup-factors/chunk-006 |

### Banana Pi
| Board | SoC | RAM | Notes | Source |
|---|---|---|---|---|
| Banana Pi BPI-M7 | RK3588 | 16 GB (potential 32 GB) | | llms-local-setup-factors/chunk-005 |
| Banana Pi BPI-W3 | RK3588 | up to 32 GB | | llms-local-setup-factors/chunk-005 |
| Banana Pi LM7 | RK3588 | — | LGA "core board" for custom baseboards | edge-computing-hardware-breakdown/chunk-008 |

### Firefly
| Board | SoC | RAM | Price | Source |
|---|---|---|---|---|
| Firefly ITX-3588J | RK3588 | 32 GB | $439 | llms-local-setup-factors/chunk-005 |
| Firefly AIO-3588Q | RK3588 | 32 GB option | Base $359; 32 GB $592.52 | llms-local-setup-factors/chunk-005 |
| Firefly ROC-RK3588S-PC | RK3588S | 32 GB options | | llms-local-setup-factors/chunk-010 |

### ODROID (Hardkernel)
| Board | CPU | RAM | Notes | Price | Source |
|---|---|---|---|---|---|
| ODROID-H3+ | Intel (x86) | up to 64 GB DDR4 | DDR4-2933 dual-channel @ 46.9 GB/s | Board ~$165 (or $220) | llms-local-setup-factors/chunk-011 |
| ODROID-H4 Ultra | Intel (x86) | up to 48 GB DDR5 | DDR5-4800 @ 38.4 GB/s | Board $220; total $285–325 | llms-local-setup-factors/chunk-005/006 |

### BeagleBoard
| Board | SoC | RAM | TOPS | Price | Source |
|---|---|---|---|---|---|
| BeagleBone AI-64 | Dual Cortex-A15 + C66x DSPs + 4× EVE | 4 GB LPDDR4 (soldered) | ~8 TOPS | ~$187.50 | cpu-ram-types-comparison/chunk-010, tattoo-idea-brainstorming/chunk-006 |
| BeagleY-AI | TI AM67A (4× Cortex-A53) | — | 4 TOPS | ~$72 | tattoo-idea-brainstorming/chunk-006 |

### LattePanda
| Board | CPU | RAM | Price | Source |
|---|---|---|---|---|
| LattePanda 3 Delta | Intel | 8 GB LPDDR4-2933 (soldered) | | cpu-ram-types-comparison/chunk-010 |
| LattePanda Sigma | Intel | 16 GB ($648) or 32 GB LPDDR5 ($698; supports up to 64 GB DDR5) | 32 GB exceeds $600 budget | cpu-ram-types-comparison/chunk-010, llms-local-setup-factors/chunk-008 |

### Other SBCs
| Board | SoC | RAM | Notes | Source |
|---|---|---|---|---|
| UDOO Bolt V3/V8 | AMD Ryzen Embedded | up to 32 GB DDR4 via 2× SO-DIMM | Upgradeable; ~$414 + RAM $60–80 = $474–495 | cpu-ram-types-comparison/chunk-010 |
| UP Squared Pro 7000 | Intel | up to 16 GB LPDDR5 (soldered) | | cpu-ram-types-comparison/chunk-010 |
| PRO Portenta X8 (module) | | 2 GB LPDDR4 (soldered) | Industrial Arduino | cpu-ram-types-comparison/chunk-010 |
| Libre Computer Alta | AML-A311D | — | | llms-local-setup-factors/chunk-005 |
| Milk-V Mars CM | RISC-V | — | | llms-local-setup-factors/chunk-005 |
| Lichee Pi 4A | RISC-V | 16 GB | | llms-local-setup-factors/chunk-005 |
| Pine64 QuartzPro64 | RK3588 | 32 GB | | llms-local-setup-factors/chunk-006 |
| VisionFive 2 | RISC-V (JH7110) | — | Memory-limited | llms-local-setup-factors/chunk-010 |

---

## 4. System-on-Chips (SoCs)

### Broadcom
| SoC | CPU | Used In | Source |
|---|---|---|---|
| BCM2711 | Cortex-A72 | Raspberry Pi 4 | cpu-ram-types-comparison/chunk-010 |
| BCM2712 | Cortex-A76 | Raspberry Pi 5 / CM5 | cpu-ram-types-comparison/chunk-010 |

### Rockchip
| SoC | CPU | GPU | NPU | Memory | Source |
|---|---|---|---|---|---|
| RK3566 | 4× Cortex-A55 @ 1.6 GHz | | | | edge-computing-hardware-breakdown/_summary |
| RK3588 | 4× Cortex-A76 + 4× Cortex-A55 | Mali-G610 MP4 | 6 TOPS INT8 | LPDDR4/LPDDR5; LPDDR4X-4266 64-bit → 34.1 GB/s; LPDDR5 → 51.2 GB/s; up to 32 GB | llms-local-setup-factors/chunk-005/011/012, edge-computing-hardware-breakdown/chunk-018 |
| RK3588S | (cut-down RK3588) | | | | hardware-setup-brainstorming/chunk-003 |

### Allwinner
| SoC | Used In | Source |
|---|---|---|
| AML-A311D | Libre Computer Alta | llms-local-setup-factors/chunk-005 |

### Texas Instruments
| SoC | CPU | NPU | Used In | Source |
|---|---|---|---|---|
| AM67A | 4× Cortex-A53 | 4 TOPS | BeagleY-AI | tattoo-idea-brainstorming/chunk-006 |

### MediaTek
| SoC | Used In | Source |
|---|---|---|
| MT8167 | Google Coral Dev Board | (Coral context) |

### ARM Cortex Cores Referenced
- Cortex-A15 (BeagleBone AI-64)
- Cortex-A53 (BeagleY-AI)
- Cortex-A55 (RK3566/3588 little cores)
- Cortex-A72 (BCM2711)
- Cortex-A76 (BCM2712, RK3588 big cores)
- Cortex-A78 (higher-end ARM, Jetson Orin)
- Cortex-A78AE (Jetson Orin Nano Super, 6-core @ 1.7 GHz)
- Cortex-A57 (older Jetson)
- Cortex-M0+ (Google Coral Edge TPU host MCU @ 32 MHz)

---

## 5. AI Accelerators (Jetson, Coral, Hailo, etc.)

### NVIDIA Jetson Family — Full Lineup
*Source: `llms-local-setup-factors/chunk-003` (full spec breakdown), `chunk-012`, `tattoo-idea-brainstorming/chunk-006`*

| Module | RAM | Memory BW | AI Perf | GPU | Cores | Power | Price |
|---|---|---|---|---|---|---|---|
| Jetson Nano (B01) | 4 GB LPDDR4 | 25.6 GB/s | ~0.5 TFLOPS FP16 | 128-core Maxwell | — | 5–10 W | ~$129 |
| Jetson Nano 2GB | 2 GB LPDDR4 | — | — | Maxwell | — | — | — |
| Jetson Xavier NX | 8 GB LPDDR4x | 51.2 GB/s | 21 TOPS INT8 | 384-core Volta + 48 Tensor Cores | — | — | ~$399 |
| Jetson Orin Nano (4/8 GB) | LPDDR5 | 68 GB/s | 40 TOPS INT8 | 1024-core Ampere + 32 Tensor Cores | — | 7–25 W | $249–299 |
| Jetson Orin Nano Super Dev Kit | 8 GB LPDDR5 | 102 GB/s | **67 TOPS** Tensor-accelerated | 1024-core Ampere + 32 TC | 6-core ARM Cortex-A78AE @ 1.7 GHz | 7–25 W | **$249** |
| Jetson Orin NX (8/16 GB) | LPDDR5 | 102 GB/s | 70 TOPS INT8 | 1024-core Ampere + 32 TC | — | — | $599–699 (module only; needs carrier) |
| Jetson AGX Orin (32/64 GB) | LPDDR5 | **204 GB/s** | **275 TOPS INT8** | 2048-core Ampere + 64 Tensor Cores | — | 15–60 W | $1,499–1,999 |

### Google Coral
*Source: `tattoo-idea-brainstorming/chunk-005/006`, `edge-computing-hardware-breakdown/chunk-020`*

| Product | Specs | Price | Notes |
|---|---|---|---|
| Coral USB Accelerator (Edge TPU) | 4 TOPS @ 2 W; host MCU 32 MHz Cortex-M0+; USB 3.0 Type-C | $83–100 | NOT for LLMs (no DRAM bandwidth) |
| Coral M.2 / PCIe variants | Edge TPU | $25–40 | Same limitation |
| Coral Dev Board | Edge TPU + MT8167 SoC | — | Single-board ML computer |

### Hailo
*Source: `edge-computing-hardware-breakdown/chunk-019/020`*

| Product | TOPS | Interface | Notes | Price |
|---|---|---|---|---|
| Hailo-8 | **26 TOPS** | M.2 PCIe | DRAM-less; not for LLMs | $199–239 |
| Hailo-8L | (similar) | M.2 | Commonly paired with Raspberry Pi AI kit | — |
| Hailo-10H | — | — | "Gen-AI" accelerator with **direct DDR interface** (scales to LLMs) | — |

**Raspberry Pi + 4× Hailo-8 rig concept**: Pi 5 (8 GB ~$103 or 16 GB $120) + Geekworm X1011 (~$45) + 4× Hailo-8 ($796–956) = **~$941–1,101 total**

### Other AI Accelerators Mentioned
*Source: `tattoo-idea-brainstorming/chunk-005/006`, `edge-computing-hardware-breakdown/chunk-016/017/019`*

| Product | Specs | Source |
|---|---|---|
| Intel Neural Compute Stick 2 (NCS2) | USB accelerator for Pi 4 | tattoo-idea-brainstorming/chunk-005 |
| Intel Movidius | M.2 accelerator | edge-computing-hardware-breakdown/chunk-019 |
| Kneron | M.2 NPU for LLM use cases | edge-computing-hardware-breakdown/chunk-017 |
| Kinara Ara-2 | M.2/PCIe NPU | edge-computing-hardware-breakdown/chunk-016 |
| Tenstorrent Grayskull e75 | M.2 AI accelerator | edge-computing-hardware-breakdown/chunk-016 |

---

## 6. GPUs for Local Inference

### NVIDIA GPUs
*Source: `llms-local-setup-factors/chunk-003`, `edge-computing-hardware-breakdown/chunk-018`, `extensive-system-requirement-breakdown/chunk-007`*

| GPU | VRAM | Bandwidth | Perf | Price | Notes |
|---|---|---|---|---|---|
| RTX 3060 | 12 GB | — | — | Older, <$600 | llms-local-setup-factors/chunk-003 |
| RTX 4060 Ti | 16 GB | — | ~48 tok/s on Llama 3.1 8B Q4 | $400–500 | Tier C winner; edge-computing-hardware-breakdown/chunk-018 |
| RTX 3090 | 24 GB | — | ~30 tok/s on 32B; ~160 tok/s on 70B (historical) | ~$250 used | Best <$1k VRAM/$ ratio |
| RTX 5070 (Laptop) | — | — | — | — | extensive-system-requirement-breakdown/chunk-007 |
| Tesla P40 | 24 GB | **346 GB/s** | — | ~$250 used | Data-center recycle |
| Tesla T4 | 16 GB | 320 GB/s | — | — | Data-center recycle |

### AMD GPUs
*Source: `llms-local-setup-factors/chunk-003`, `localforge-ai-ide/chunk-006/007`, `backup-and-os-recommendation/chunk-001`*

| GPU | VRAM | Bandwidth | Notes | Price |
|---|---|---|---|---|
| **Radeon RX 580** (USER'S CURRENT) | 8 GB | — | **NOT supported by ROCm** (Polaris/gfx803 dropped ~ROCm 4.0); use Vulkan/llama.cpp | — |
| Radeon VII | 16 GB HBM2 | **1 TB/s** | <$600 candidate | — |
| Instinct MI25 | 16 GB HBM2 | 484 GB/s | Used data-center | — |
| Radeon RX 6800 | 16 GB | — | Consumer | — |
| Radeon RX 7600 XT | 16 GB | — | Budget | — |
| Radeon RX 7900 GRE | 16 GB | — | $549–579 | — |
| Radeon 7600M XT (eGPU dock) | — | — | $469.99 | External GPU solution |

### AMD Integrated GPUs (iGPUs)
*Source: `llms-local-setup-factors/chunk-012/014`*

| iGPU | Cores/Freq | Pairs With | Bandwidth |
|---|---|---|---|
| Radeon 780M | 12 CUs @ up to 2.8 GHz | Ryzen 7 7840HS/8845HS, 32 GB LPDDR5 shared | ~100 GB/s class |
| Radeon 680M | RDNA2 | Ryzen 9 6900HX | — |
| Radeon 760M | — | Ryzen 7 5825U | — |
| Radeon 8060S | — | Beelink GTR9 Pro (Ryzen AI Max+ 395) | up to 96 GB dynamic VRAM |

### Intel GPUs
*Source: `llms-local-setup-factors/chunk-003`, `localforge-ai-ide/chunk-006`, `edge-computing-hardware-breakdown/chunk-003`*

| GPU | VRAM | Bandwidth | Notes |
|---|---|---|---|
| Arc A770 | 16 GB | **560 GB/s** | <$600 candidate |
| Arc A380 | — | — | Example model |
| Arc 140V | — | — | MSI Intel Ultra 7 mini PC |

---

## 7. Data Center / 128GB+ VRAM Hardware

From `gpu-components-for-128gb-vram/` — user explored building a custom 128GB+ VRAM GPU for LLM workloads.

### Pre-Built HBM Accelerators (the realistic path)
| Module | Capacity | Bandwidth | Form Factor | Price | Source |
|---|---|---|---|---|---|
| **AMD Instinct MI250X** | **128 GB HBM2e** | ~3.2 TB/s | OAM (needs baseboard/cooling) | $3–4k used | chunk-003/007 |
| **AMD Instinct MI210** | 64 GB HBM2e | **1.6 TB/s** | PCIe 4.0 FHFL passive | — | chunk-007 |
| **NVIDIA A100 80 GB** | 80 GB HBM2e | ~1.94–2.04 TB/s | PCIe 4.0 or SXM | mid-high 4/5 figures | chunk-003 |
| **NVIDIA H200** | **141 GB HBM3e** | **~4.8 TB/s** | PCIe 5.0 / NVL | — | chunk-007 |
| NVIDIA H100 | (HBM3e) | — | PCIe/SXM | mid-high 4/5 figures | chunk-003 |

### DIY GPU Build Path (FPGA + GDDR6)
- **Achronix Speedster7t FPGA** — up to 8 GDDR6 controllers, ~4 Tb/s memory I/O
- **BittWare VectorPath S7t-VG6** — 16× GDDR6, ~448 GB/s; PCIe Gen5; real-world reference design
- **Verdict**: DIY 128GB+ VRAM impossible (HBM3E locked behind CoWoS/EMIB/InFO packaging + NDA). FPGA+GDDR6 path caps at ~32 GB.

### Advanced Packaging Tech Mentioned
- TSMC CoWoS / CoWoS-L
- TSMC InFO
- Intel EMIB

---

## 8. Mini PCs / Workstations

*Source: `llms-local-setup-factors/chunk-002/011/014/016/021`, `edge-computing-hardware-breakdown/chunk-002`*

### AMD Ryzen Mini PCs (LPDDR5X 128 GB ceiling)
| Model | CPU | RAM | Bandwidth | Tok/s | Price |
|---|---|---|---|---|---|
| **GMKtec EVO-X2** | Ryzen AI Max+ 395 | up to **128 GB LPDDR5X** | — | ~8.5 tps | ~$2,000 |
| **Beelink GTR9 Pro** | Ryzen AI Max+ 395 | up to 128 GB LPDDR5X | — | ~8.3 tps | ~$1,985 |
| **Seaviv AIdeaStation R1** | Ryzen AI Max+ 395 | 128 GB LPDDR5X | — | ~8.0 tps | $1,799 |
| **MINISFORUM AI X1 Pro** | Ryzen AI 9 HX 370 | 96–128 GB DDR5 | — | ~4.5 tps | $927 (32 GB SKU) |
| **MINISFORUM UM790 Pro** | Ryzen 9 7940HS | 64 GB DDR5 | — | — | — |
| **AOOSTAR GEM10** | Ryzen 7 7840HS + Radeon 780M | **32 GB LPDDR5-6400** | **102.4 GB/s** | — | $399–439 |

### Intel Mini PCs
| Model | CPU | RAM | Notes | Price |
|---|---|---|---|---|
| GEEKOM IT12 | i7-1280P | up to 64 GB DDR4 | DDR4-3200 dual-channel @ 51.2 GB/s | $419–432 |
| GEEKOM XT12 | Intel Core Ultra 7 155H | — | — | $700–900 |
| HP ProDesk 4 G1i AI PC | Intel Core Ultra | — | **13 TOPS NPU** | — |
| Intel NUC 12 / Compute Elements | up to 16 GB | mini-ITX | — |

### Mini-ITX / Motherboards
| Model | CPU Support | RAM | Notes | Source |
|---|---|---|---|---|
| ASRock N100M | Intel N100 | — | PCIe x16 (x2 electrical limitation); $119.99 | edge-computing-hardware-breakdown/chunk-016 |
| ASRock H810M-X WIFI | Intel Core Ultra Series 2 | DDR5 dual-channel | | esp32-c6-ram-details/chunk-007 |
| ASRock 4X4 BOX | — | — | | esp32-c6-ram-details/chunk-006 |
| ASRock H810TM-ITX | Intel Core Ultra Series 2 | DDR5 | | esp32-c6-ram-details |

### Other Workstations / Mini PC Concepts
- **ASUS ROG Flow Z13** — AMD Ryzen AI Max+ processor
- **LattePanda Sigma** — 16 GB ($648) or 32 GB LPDDR5 ($698)

---

## 9. x86 CPUs Mentioned

### AMD Ryzen
| CPU | NPU | Notes | Source |
|---|---|---|---|
| Ryzen 7 7840HS | XDNA ~10 TOPS INT8 | Radeon 780M iGPU; 32 GB LPDDR5-6400 @ 102.4 GB/s | llms-local-setup-factors/chunk-011/012 |
| Ryzen 7 8845HS | XDNA 10–16 TOPS | Similar to 7840HS | llms-local-setup-factors/chunk-011 |
| Ryzen 9 7940HS | — | — | llms-local-setup-factors/chunk-021 |
| Ryzen AI 9 HX 370 | — | MINISFORUM AI X1 Pro | llms-local-setup-factors/chunk-021 |
| Ryzen AI Max+ 395 | — | Beelink/GMKtec/Seaviv 128 GB LPDDR5X mini PCs | llms-local-setup-factors/chunk-002/021 |

### Intel
| CPU | Notes | Source |
|---|---|---|
| Intel N100 | Entry mini-PC; edge-computing Tier A example | edge-computing-hardware-breakdown/chunk-016/018 |
| Intel N305 | Entry mini-PC | edge-computing-hardware-breakdown/chunk-018 |
| Intel Core Ultra (Series 2) | 13 TOPS NPU variant in HP ProDesk | edge-computing-hardware-breakdown/chunk-002 |
| Intel Core Ultra 7 155H | GEEKOM XT12 | llms-local-setup-factors/chunk-013 |
| Intel i9-13900H | High-end mobile | llms-local-setup-factors/chunk-021 |
| Intel i7-1280P | GEEKOM IT12 | llms-local-setup-factors/chunk-011 |

---

## 10. Apple Silicon

*Source: `llms-local-setup-factors/chunk-008/011`, `edge-computing-hardware-breakdown/chunk-003`*

| Model | Unified Memory | Bandwidth | Notes | Price |
|---|---|---|---|---|
| Apple M1 Mini | 16 GB option | ~68 GB/s | — | — |
| Apple M4 Mac mini | 16 GB unified | **120 GB/s** | Handles 8B–14B; 24–32 GB needed for larger contexts | $509 refurb mentioned |
| Apple M4 Pro/Max | (implied high-end) | — | Not fully priced in notes | — |

**Apple Silicon model**: Unified memory approach; VRAM is shared system memory; good efficiency but capacity limits.

---

## 11. Microcontrollers (ESP32 family etc.)

### ESP32 Variants Mentioned
*Source: `esp32-c6-ram-details/`, `esp32-som-modules/`, `research/esp-idf/`, `projects/pcb-design-suggestions/`*

| Variant | RAM | PSRAM | Notes | Source |
|---|---|---|---|---|
| ESP32 (original) | — | external via SPI/OPI | Baseline | esp32-som-modules |
| ESP32-S2 | — | — | Security-focused | research/esp-idf |
| **ESP32-S3** (WROOM-2-N32R16V) | — | **16 MB PSRAM + 32 MB flash** | **User's chosen MCU for WatchyAI PCB** | projects/pcb-design-suggestions/chunk-011 |
| ESP32-C3 | — | — | RISC-V baseline | research/esp-idf |
| **ESP32-C6** | **512 KB SRAM + 16 KB LP SRAM** (up to 400 KB usable) | **NO PSRAM** | Wi-Fi 6 | esp32-c6-ram-details |
| **ESP32-C61** | — | **Supports PSRAM** (unlike C6) | | esp32-c6-ram-details |
| ESP32-H2 | — | — | BLE / Thread | research/esp-idf |
| ESP32-P4 | — | **Up to 64 MB PSRAM** | Max PSRAM tier | esp32-som-modules |

### ESP32-SOM Module PSRAM Tiers Mentioned
- Typical: 8 MB or 16 MB
- Max (P4): 64 MB
- Some chips support external PSRAM via SPI/OPI; modules typically come with PSRAM

### Other Microcontrollers
| MCU | Mentions | Source |
|---|---|---|
| RP2040 | 3 files matched grep; no detailed specs in read content | (grep hits) |
| STM32 family | **0 mentions found** (grep returned no results) | — |
| Nordic nRF5xxx | **0 mentions found** | — |

### ARM Cortex-M Cores Referenced
- **Cortex-M0+** — Google Coral Edge TPU host MCU @ 32 MHz

---

## 12. TinyML Hardware

*Source: `tattoo-idea-brainstorming/chunk-006` (primary TinyML reference)*

| Device | SoC / Chip | RAM | TOPS | Price | Use Case |
|---|---|---|---|---|---|
| **Sipeed Maix-II / MaixSense** | Kendryte K210 RISC-V dual-core 64-bit @ 400 MHz | **8 MB SRAM** | Neural KPU built-in | Maix-II Dock ~$40.90; Maix-I WiFi ~$18.98 | Voice/image recognition, wearable AI, robotics, IoT |
| Google Coral USB Accelerator | Edge TPU + 32 MHz Cortex-M0+ | — | 4 TOPS @ 2 W | $83–100 | Vision, audio, lightweight NLP |
| BeagleY-AI | TI AM67A (4× Cortex-A53) | — | 4 TOPS | ~$72 | Real-time vision, voice control, TinyLLM |
| BeagleBone AI-64 | Dual Cortex-A15 + C66x DSPs + 4× EVE | 4 GB LPDDR4 | ~8 TOPS | ~$187.50 | Industrial monitoring, embedded vision, robotics |

### TinyML AI Workloads Mentioned
- Vision processing (camera inference)
- Audio processing (voice recognition, keyword spotting)
- Lightweight NLP inference (TensorFlow Lite)
- TinyLLM experimentation
- IoT sensors (robotics sensors, wearable AI)
- Real-time low-latency edge AI

### TinyML Frameworks
- TensorFlow Lite (for Coral Edge TPU)
- Arduino-compatible (MaixDuino variant)

---

## 13. Custom PCB Components (WatchyAI project)

User designed a custom ESP32-S3 smartwatch PCB. Project: `WatchyAI_JLCPCB.kicad_pro`. Full BOM from `chatgpt/organized/projects/pcb-design-suggestions/`.

### Main Compute
| Part | Description | Source |
|---|---|---|
| **ESP32-S3-WROOM-2-N32R16V** (JLC LCSC C42417293) | Main compute module; Wi-Fi/BLE + dual-core MCU + **16 MB PSRAM + 32 MB flash** | chunk-011 |

### Display Stack
| Part | Description | Source |
|---|---|---|
| Good Display **GDEY027T91-FT11** | E-ink display module (24-pin FPC for EPD + 6-pin FPC for touch) | chunk-011 |
| **SSD1680Z8** | EPD driver controller (SPI + control pins) | chunk-011 |
| **FT6336U** | Capacitive touch controller (I²C) | chunk-011 |

### Audio
| Part | Description | Source |
|---|---|---|
| **ICS-43434** (×2) | I2S MEMS microphones | chunk-011 |
| **MAX98357A** | I2S Class-D speaker amplifier | chunk-011 |

### Sensors
| Part | Description | Source |
|---|---|---|
| **LIS2DU12TR** | Ultra-low-power 3-axis accelerometer (I²C/SPI) | chunk-011 |

### Power
| Part | Description | Source |
|---|---|---|
| **TP4056** | Linear LiPo charger (USB 5 V → LiPo) | chunk-011 |
| **MT3608** | 1.2 MHz boost converter (for frontlight LEDs) | chunk-011 |
| LDO (low-Iq) | Generates 3V3 rail for ESP32 + digital logic | chunk-011 |

### Connectors / Protection
| Part | Description | Source |
|---|---|---|
| **TYPE-C-31-M-12** (C110613) | USB-C receptacle (USB 2.0, top-edge 2 mm overhang) | chunk-010/011 |
| **USBLC6-2SC6** | USB ESD protector | chunk-011 |
| **JST-PH 2.0 mm** | LiPo battery connector (J6) | chunk-011 |
| **Hirose DM3D-SF** | microSD connector (mechanical reference) | chunk-011 |

### Power Architecture
```
USB-C 5V (VBUS) → TP4056 charger → LiPo (VBAT) → 3V3 LDO → ESP32 + sensors + touch
                                                ↘ MT3608 boost → frontlight LEDs
```

### Data Flow
```
ESP32-S3 ↔ microSD (SPI)
ESP32-S3 ↔ SSD1680Z8 EPD driver (SPI + control pins) [24-pin FPC]
ESP32-S3 ↔ FT6336U touch (I²C) [6-pin FPC]
ESP32-S3 ↔ 2× ICS-43434 mics (I²S)
ESP32-S3 → MAX98357A amp (I²S) → speaker
ESP32-S3 ↔ LIS2DU12 accelerometer (I²C/SPI)
ESP32-S3 → haptic driver (GPIO/PWM) → vibration motor
```

---

## 14. DIY GPU Components (128GB VRAM project)

From `gpu-components-for-128gb-vram/` — user explored sourcing bare memory chips and support silicon for a hypothetical 128 GB+ VRAM GPU. Verdict: not feasible for DIY, but full bill of materials was compiled.

### Memory ICs (purchasable)
| Memory | Part Number | Density | Speed | Package | Source |
|---|---|---|---|---|---|
| GDDR6 16 Gb | **Micron MT61K512M32KPA-16** | 16 Gb (2 GB) | 8 GHz I/O per-pin | 180-FBGA (12×14 mm) | chunk-007 |
| GDDR6 16 Gb (high-speed grade) | **Samsung K4ZAF325BC-SC20** | 16 Gb | **20 Gb/s per pin** | 180-FBGA | chunk-007 |
| GDDR7 24 Gb | Samsung 24 Gb GDDR7 (family) | 24 Gb (3 GB) | **>40 Gb/s per pin** | — | chunk-007 (sampling only) |
| HBM3E (reference) | Micron 8-Hi 24 GB / 12-Hi 36 GB | 24/36 GB per stack | **>1.2 TB/s per stack** | TSV-bonded, not DIY-solderable | chunk-007 |

### Capacity Math for 128 GB
| DRAM Density | Packages Needed | Reality |
|---|---|---|
| 16 Gb (2 GB) GDDR6 | **64** | Typical GPUs use 8–24 packages → infeasible |
| 24 Gb (3 GB) GDDR7 | **≈43** | Same problem |
| HBM3E 4× 36 GB | 4 stacks (144 GB) | Only feasible via CoWoS/EMIB at manufacture time |

### PCIe / Clock Support ICs
| Component | Part Numbers | Purpose | Source |
|---|---|---|---|
| PCIe retimer (Gen4 x8) | **TI DS160PT801** | Extends PCIe links at 16 GT/s | chunk-007 |
| PCIe redriver (Gen5 x16) | **TI DS320PR1601** | 32 GT/s | chunk-007 |
| PCIe clock generator | **Renesas 9FGV1002C** (Gen1–5) | 100 MHz differential reference | chunk-007 |
| PCIe clock fanout buffer | **Renesas/IDT 9DBL0851** (Gen1–5 zero-delay) | Low-jitter fanout | chunk-007 |
| 100 MHz differential oscillator | **SiTime SiT9121** (LVDS/LVPECL); **Abracon ClearClock LP-HCSL** | Low-jitter PCIe ref clock | chunk-007 |

### Power / VRM
| Component | Part Number | Specs | Source |
|---|---|---|---|
| Multiphase PWM controller | **TI TPS53681** | Configurable multiphase | chunk-004/007 |
| Smart power stage (DrMOS) | **Infineon TDA21472** | **70 A** per stage | chunk-004/007 |
| Alternative PWM controller | Renesas ISL69269 | (mentioned as alternative) | chunk-004 |

### Boot / Firmware
| Component | Part Number | Purpose | Source |
|---|---|---|---|
| SPI-NOR flash | **Winbond W25Q256JV** | Boot/ROM storage | chunk-007 |

### PCB Materials
- **Megtron 6** (Panasonic) — high-speed PCB material
- **ITEQ IT-180A** — high-speed PCB material

### FPGA Development Boards Referenced
| Board | Specs | Source |
|---|---|---|
| **BittWare VectorPath S7t-VG6** (Achronix Speedster7t FPGA) | 16× GDDR6, ~448 GB/s, PCIe Gen5 | chunk-007 |
| Achronix Speedster7t devices | Up to 8 GDDR6 controllers, ~4 Tb/s memory I/O | chunk-007 |

---

## 15. Memory Types & Tiers

### Memory Technologies Mentioned
| Type | Used In | Source Examples |
|---|---|---|
| **LPDDR4** | LattePanda 3 Delta, PRO Portenta X8 | cpu-ram-types-comparison/chunk-010 |
| **LPDDR4X** | Raspberry Pi 5, Orange Pi 5 Plus (34.1 GB/s @ LPDDR4X-4266 64-bit), BeagleBone AI-64 | cpu-ram-types-comparison/chunk-010 |
| **LPDDR4X** (Radxa CM5) | Radxa CM5 SoM | edge-computing-hardware-breakdown/chunk-008 |
| **LPDDR5** | NVIDIA Jetson Orin Nano (68–102 GB/s), AOOSTAR GEM10 (32 GB @ 6400 MT/s → 102.4 GB/s) | cpu-ram-types-comparison/chunk-010, llms-local-setup-factors/chunk-011 |
| **LPDDR5X** | High-end mini PCs (GMKtec EVO-X2, Beelink GTR9 Pro, Seaviv) up to 128 GB | llms-local-setup-factors/chunk-021 |
| **DDR4** | UDOO Bolt (2× SO-DIMM, up to 32 GB, 2400–2933), GEEKOM IT12 (up to 64 GB) | cpu-ram-types-comparison/chunk-010 |
| **DDR5** | ODROID-H4/H3+ (DDR5-4800 @ 38.4 GB/s), ASRock H810M-X WIFI, ASRock H810TM-ITX | llms-local-setup-factors/chunk-011 |
| **GDDR5** | (legacy GPU reference) | gpu-components-for-128gb-vram |
| **GDDR6** | DIY GPU project; Achronix FPGA boards; Micron MT61K512M32KPA-16, Samsung K4ZAF325BC-SC20 | gpu-components-for-128gb-vram |
| **GDDR6X** | (mentioned, scarce retail) | gpu-components-for-128gb-vram |
| **GDDR7** | Samsung 24 Gb (>40 Gb/s/pin); not retail | gpu-components-for-128gb-vram |
| **HBM** | (legacy reference) | gpu-components-for-128gb-vram |
| **HBM2** | AMD Radeon VII (16 GB @ 1 TB/s), AMD Instinct MI25 (16 GB @ 484 GB/s) | llms-local-setup-factors/chunk-003 |
| **HBM2e** | AMD Instinct MI250X (128 GB @ 3.2 TB/s), MI210 (64 GB @ 1.6 TB/s), NVIDIA A100 (80 GB @ ~2 TB/s) | gpu-components-for-128gb-vram |
| **HBM3e** | NVIDIA H200 (141 GB @ 4.8 TB/s) | gpu-components-for-128gb-vram |

### MCU Memory Tiers
| Tier | Hardware | Use |
|---|---|---|
| KB-class SRAM | ESP32-C6 (512 KB SRAM + 16 KB LP SRAM) | Bare MCU, no model inference |
| 8 MB SRAM | Kendryte K210 | TinyML vision/audio |
| 8–16 MB PSRAM | ESP32-SOM typical | Audio buffering, light ML |
| 64 MB PSRAM | ESP32-P4 (max) | Larger model weights |

---

## 16. Software Stack / Frameworks Mentioned

### Local LLM Runners
| Tool | Backend | Use Case | Source |
|---|---|---|---|
| **LM Studio** | llama.cpp-based | User's primary frontend | gpu-offloading-for-llms |
| **llama.cpp** | CPU/Vulkan/CUDA/ROCm/Metal/SYCL | Multi-platform runner | gpu-offloading-for-llms, localforge-ai-ide |
| **vLLM** | CUDA/ROCm | NVIDIA/AMD high-throughput, long-context | localforge-ai-ide/chunk-006 |

### GPU Compute Backends
| Backend | Vendor | Notes | Source |
|---|---|---|---|
| **CUDA** | NVIDIA | Required for NVIDIA GPUs | throughout |
| **TensorRT** | NVIDIA | Inference optimizer (JetPack) | llms-local-setup-factors/chunk-012 |
| **cuDNN** | NVIDIA | Part of JetPack | llms-local-setup-factors/chunk-012 |
| **JetPack SDK** | NVIDIA | Ubuntu-based; CUDA+cuDNN+TensorRT for Jetson | llms-local-setup-factors/chunk-012 |
| **ROCm** | AMD | **NOT supported on RX 580 (Polaris/gfx803) since ~ROCm 4.0** | localforge-ai-ide/chunk-007 |
| **Vulkan** | Khronos | User's fallback (RX 580); also llama.cpp backend | gpu-offloading-for-llms, localforge-ai-ide |
| **OpenCL** | Khronos | MLC on Adreno | react-native-device-control/chunk-006 |
| **SYCL** | Intel | Intel Arc/Xe backend | extensive-system-requirement-breakdown/chunk-007 |
| **OpenVINO** | Intel | Optional ONNX Runtime EP | model-specialization-search/chunk-009 |

### NPU / TPU SDKs
| SDK | Vendor | Hardware | Source |
|---|---|---|---|
| **RKNN** | Rockchip | RK3588 NPU | llms-local-setup-factors/chunk-012 |
| **TensorFlow Lite** | Google | Coral Edge TPU, microcontrollers | tattoo-idea-brainstorming/chunk-006 |
| **MindSpore** | Huawei | Orange Pi AI-Pro native | tattoo-idea-brainstorming/chunk-006 |
| **ONNX Runtime** | Microsoft | Broad; optional OpenVINO EP | model-specialization-search/chunk-009 |
| **MLC (Machine Learning Compilation)** | TVM-based | Compile to device GPUs/NPUs; OpenCL/Vulkan on Adreno | react-native-device-control/chunk-006 |
| **Lemonade** | AMD | Ryzen AI-optimized local LLM server; uses NPU/iGPU | extensive-system-requirement-breakdown/chunk-017 |

### Key llama.cpp Tuning Knobs (from user's research)
- `--ctx-size`, `--gpu-layers`, `--no-kv-offload`, `--flash-attn`, `--cache-type-k`, `--cache-type-v`
- LM Studio: `offload_kv_cache_to_gpu`, `eval_batch_size`, `flash_attention`

---

## 17. Performance Specs Reference

### TOPS Ratings (AI Inference Throughput)
| Hardware | TOPS | Tier |
|---|---|---|
| Jetson AGX Orin | **275 TOPS INT8** | Top edge |
| Jetson Orin NX | 70 TOPS INT8 | |
| Jetson Orin Nano Super | **67 TOPS** Tensor-accelerated | Sweet spot |
| Jetson Orin Nano | 40 TOPS INT8 | |
| Hailo-8 | 26 TOPS | M.2 PCIe |
| Jetson Xavier NX | 21 TOPS INT8 | Legacy |
| Orange Pi AI-Pro | 20 TOPS NPU | SBC |
| BeagleBone AI-64 | ~8 TOPS | SBC |
| RK3588 NPU | ~6 TOPS INT8 | SBC SoC |
| Coral Edge TPU | 4 TOPS @ 2 W | USB/M.2 |
| BeagleY-AI | 4 TOPS | SBC |
| Ryzen 7 7840HS XDNA NPU | ~10 TOPS INT8 | x86 iGPU |
| HP ProDesk Intel Core Ultra NPU | 13 TOPS | x86 mini PC |

### Memory Bandwidth Specs
| Hardware | Bandwidth |
|---|---|
| NVIDIA H200 (141 GB HBM3e) | **~4.8 TB/s** |
| AMD Instinct MI250X (128 GB HBM2e) | ~3.2 TB/s |
| NVIDIA A100 (80 GB HBM2e) | ~1.94–2.04 TB/s |
| AMD Instinct MI210 (64 GB HBM2e) | 1.6 TB/s |
| AMD Radeon VII (16 GB HBM2) | **1 TB/s** |
| Jetson AGX Orin | 204 GB/s |
| Apple M4 Mac mini | 120 GB/s |
| Intel Arc A770 | 560 GB/s |
| Jetson Orin NX / Orin Nano Super / Apple M4 | 102 GB/s |
| AOOSTAR GEM10 (LPDDR5-6400) | 102.4 GB/s |
| Jetson Orin Nano | 68 GB/s |
| Apple M1 Mini | 68 GB/s |
| Jetson Xavier NX | 51.2 GB/s |
| GEEKOM IT12 (DDR4-3200 dual) | 51.2 GB/s |
| ODROID-H3+ (DDR4-2933 dual) | 46.9 GB/s |
| RK3588 LPDDR4X-4266 64-bit | 34.1 GB/s theoretical |
| ODROID-H4 Ultra (DDR5-4800) | 38.4 GB/s |
| NVIDIA Tesla P40 | 346 GB/s |
| NVIDIA Tesla T4 | 320 GB/s |
| AMD Instinct MI25 (16 GB HBM2) | 484 GB/s |
| Jetson Nano | 25.6 GB/s |

### Token/s Benchmarks Captured
| Hardware | Model | Tok/s | Source |
|---|---|---|---|
| RTX 4060 Ti 16GB | Llama 3.1 8B Q4 | ~48 tok/s | edge-computing-hardware-breakdown/chunk-018 |
| RTX 3090 24GB | 32B model | ~30 tok/s | edge-computing-hardware-breakdown/chunk-003 |
| RTX 3090 | 70B (historical) | ~160 tok/s | yaml-to-rust-transpiler/chunk-008 |
| Raspberry Pi 5 8GB | 7B quant | 4–6 tok/s | edge-computing-hardware-breakdown/chunk-018 |
| Raspberry Pi 5 8GB | 8B | ~2.0 tok/s | edge-computing-hardware-breakdown/chunk-018 |
| Orange Pi 5 Plus (CPU) | 8B | 13.13 tok/s | edge-computing-hardware-breakdown/chunk-012 |
| Orange Pi 5 Plus (NPU) | 8B | 10.47 tok/s | edge-computing-hardware-breakdown/chunk-012 |
| RK3588 SBC (~$130) + NPU | 8B | ~4 tok/s | edge-computing-hardware-breakdown/chunk-015 |
| High-RAM CPU orchestrator | Bigger models | 1–3 tok/s | edge-computing-hardware-breakdown/chunk-018 |
| GMKtec EVO-X2 (Ryzen AI Max+ 395, 128 GB LPDDR5X) | — | ~8.5 tps | llms-local-setup-factors/chunk-021 |
| Beelink GTR9 Pro (Ryzen AI Max+ 395) | — | ~8.3 tps | llms-local-setup-factors/chunk-021 |
| Seaviv AIdeaStation R1 | — | ~8.0 tps | llms-local-setup-factors/chunk-021 |
| MINISFORUM AI X1 Pro (Ryzen AI 9 HX 370) | — | ~4.5 tps | llms-local-setup-factors/chunk-021 |

### Power Draw Specs
| Hardware | Power |
|---|---|
| Jetson Nano | 5–10 W |
| Jetson Orin Nano / Super | 7–25 W configurable |
| Jetson AGX Orin | 15–60 W configurable |
| Coral Edge TPU | 2 W |
| Radxa ZERO 3W class | ~0 W (lowest tier) |

---

## 18. Price Bands

### Sub-$100 (Entry SBCs)
- Raspberry Pi Zero 2 W
- Radxa ZERO 3W 2 GB: $22–50
- Orange Pi 5 4 GB: ~$60 (Etsy) / ~$80 (Amazon)
- Raspberry Pi 4 8 GB: ~$75

### $100–250 (Mid SBCs)
- Raspberry Pi 5 8 GB: ~$95–103
- Radxa ROCK 3C 8 GB: $54.99
- Radxa ROCK 3A: $45–65
- Raspberry Pi 5 16 GB: $120
- BeagleY-AI: ~$72
- Jetson Nano (B01): ~$129
- Radxa ROCK 5B+ 16/24 GB: $119–159

### $250–600 (Sweet Spot for Local LLMs)
- **Jetson Orin Nano Super Dev Kit 8 GB**: **$249** ⭐
- RTX 3090 24 GB used: ~$250
- Tesla P40 24 GB used: ~$250
- Coral USB Accelerator: $83–100
- Hailo-8 M.2: $199–239
- BeagleBone AI-64: ~$187.50
- Radxa ROCK 5B 32 GB: $215–230
- Khadas Edge2 Pro 16 GB: ~$299
- Orange Pi 5 Plus 32 GB: ~$320
- RTX 4060 Ti 16 GB: $400–500
- Firefly ITX-3588J 32 GB: $439
- ODROID-H4 Ultra (board + RAM): $285–325
- UDOO Bolt V3 + 32 GB DDR4: $474–495
- Jetson Xavier NX: ~$399
- AOOSTAR GEM10 32 GB: $399–439
- GEEKOM IT12: $419–432
- Firefly AIO-3588Q 32 GB: $592.52
- Jetson Orin NX 16 GB module: $599 (needs carrier)

### $600–1,100 (High-End)
- LattePanda Sigma 16 GB: $648
- LattePanda Sigma 32 GB: $698
- Jetson AGX Orin 32 GB: $1,499
- Jetson AGX Orin 64 GB: $1,999
- **Raspberry Pi 5 + 4× Hailo-8 rig**: ~$941–1,101
- AMD Radeon 7600M XT eGPU dock: $469.99
- AMD RX 7900 GRE 16 GB: $549–579

### $1,800+ (Workstation Class)
- Seaviv AIdeaStation R1 (128 GB LPDDR5X): $1,799
- Beelink GTR9 Pro (128 GB): ~$1,985
- GMKtec EVO-X2 (128 GB): ~$2,000
- AMD Instinct MI250X 128 GB used: $3,000–4,000

---

## 19. Source Folder Index

### Primary Chat Folders (Heavy Hardware Content)
| Folder | Topic | Why It Matters |
|---|---|---|
| `edge-computing-hardware-breakdown/` (20 chunks, 385 msgs) | Tier classification, RK3588 ecosystem, Jetson vs SBC | Richest source; user's own tier framework |
| `llms-local-setup-factors/` (21 chunks) | Full Jetson lineup, mini PCs, GPUs under $600 | Price/spec comparisons |
| `cpu-ram-types-comparison/` (10 chunks) | RAM types across SBCs (LPDDR4/5, DDR4/5) | Memory architecture reference |
| `gpu-components-for-128gb-vram/` (7 chunks) | DIY GPU BOM, HBM3E, GDDR6/7, PCIe retimers | Deep dive into building custom AI GPU |
| `esp32-c6-ram-details/` (9 chunks) | ESP32-C6 SRAM, PSRAM, C61 variant | MCU memory deep dive |
| `esp32-som-modules/` (4 chunks) | ESP32 SOM PSRAM tiers up to 64 MB (P4) | MCU module landscape |
| `gpu-offloading-for-llms/` (8 chunks) | llama.cpp tuning, user's 8 GB VRAM / 16 GB RAM rig | User's actual hardware + software |
| `hardware-setup-brainstorming/` (3+ chunks) | Pi 4/Orange Pi 5 pricing, external RAM feasibility | Early ideation |
| `tattoo-idea-brainstorming/` chunks 5–7 | TinyML boards (Coral, Maix, BeagleY, BeagleBone) | Best TinyML reference |
| `extensive-system-requirement-breakdown/` | Ryzen AI, Snapdragon X Elite, Intel Arc | System-level specs |

### Secondary Chat Folders (Hardware-Adjacent)
- `llm-enabled-raspberry-pi-setup/` — LLM on Pi
- `raspberry-pi-media-os-list/` — Pi media OS
- `backup-and-os-recommendation/` — AMD RX 580 / ROCm context
- `localforge-ai-ide/` — vLLM, CUDA, ROCm, Vulkan, Intel Arc
- `open-source-fire-stick-alternatives/` — TV streaming hardware
- `e-ink-phones-with-ram/` — Hisense A9 e-ink phone specs
- `hisense-a9-os-options/` — Snapdragon 662, Adreno 610
- `react-native-device-control/` — Hexagon DSP, MLC framework
- `screen-mirroring-devices/` — Wireless display hardware
- `model-specialization-search/` — OpenVINO EP
- `yaml-to-rust-transpiler/` & `agentic-yaml-to-rust-transpiler/` — RTX 3090 perf reference

### Project Folders
- `chatgpt/organized/projects/pcb-design-suggestions/` (11+ chunks) — WatchyAI ESP32-S3 smartwatch PCB, full BOM ⭐
- `chatgpt/organized/projects/e-ink-setup-plan/` — Hisense A9 setup
- `chatgpt/organized/projects/discussing-spacemouse-features/` — SpaceMouse 6DOF input

### Other Locations
- `research/esp-idf/` — Complete ESP32 SDK docs
- `new-ideas/hardware/peripherals/text-input/scroll-ball-speech-to-text/` — ESP32 + local Whisper peripheral
- `00-BUSINESS-IDEA-ANALYSIS-SUITE/reports/11-IDEA-Custom-PCB-Maker-Platform.md` — Custom PCB business idea
- `00-BUSINESS-IDEA-ANALYSIS-SUITE/reports/04-IDEA-Crowdsourcing-Competition-Platform.md` — ESP32 mention
- `00-BUSINESS-IDEA-ANALYSIS-SUITE/reports/28-SUMMARY-AGGREGATE.md` & `01-STATUS-REPORT.md` — SBC mentions

### Folders Reviewed, No AI Compute Hardware Found
- `chatgpt/organized/archive/` (23 folders) — empty of AI hardware
- `keep-notes/` (6 folders) — empty of AI hardware
- `books/` (2 folders) — empty of AI hardware
- Non-compute hardware folders (keyboards, trackballs, solenoids, batteries, QMK switches, 3D printers) — excluded from this report; they live under `new-ideas/hardware/peripherals/` or are not AI-compute relevant
- `chipset-brainstorming/` — only system messages, no real content
- Pure-software AI folders (quantization, agentic UI, model specialization, etc.) — excluded

---

## Appendix A: User's Hardware Philosophy (inferred)

From the breadth of notes, the user's intent is:

1. **Cost-conscious local AI** — extensive <$600 budget analysis; sub-$1,000 ceiling for "Tier C"
2. **Power-aware edge compute** — explicit "0 W to 60 W" tier framework
3. **Custom PCB capability** — designed full ESP32-S3 watch BOM; explored DIY GPU BOM down to component level
4. **Realistic about limits** — understands HBM3E is locked behind CoWoS; understands Coral TPU can't do LLMs
5. **Vulkan/llama.cpp fallback** — current RX 580 forces this; user knows the stack deeply
6. **Tiered orchestration** — wants micro nodes (Tier A), orchestrators (Tier B), and fast rigs (Tier C) working together
7. **PSRAM-on-MCU curiosity** — explored ESP32-C61/P4 PSRAM tiers for TinyLLM on microcontroller

---

## Appendix B: Gaps Noted

These patterns returned **zero matches** in the entire chatgpt notes tree (user has not discussed):

- **STM32 family** (STM32H7, F7, etc.)
- **Nordic nRF52840 / nRF5340**
- **Realtek RTL8xxx**
- **MediaTek LinkIt / MT7697**
- **Arduino Nano 33 BLE Sense**
- **Sony Spresense**
- **Cortex-M33 / M4F / FPU** (as primary topics)
- **Kendryte K230** (only K210 mentioned)

If building a truly comprehensive local AI hardware program, these are unstudied gaps.

---

**End of report.**

## [0087] 06-28 13:36

(2026-06-28T13:36:55)
distill the reports content in terms of specific model options and model options from this other chats response and give me a deduplicated table and I'll tell you real prices as aliexpress can be a bit deceptive with thier titles and descriptions using different options to obscure the real price for the ram your getting.

TIER 0: LPDDR5 32GB+ (Highest RAM, Newest Gen)
# 	Board 	SoC 	RAM 	AI 	Networking 	Display 	Storage 	Size 	AliExpress Price 	Linux Ecosystem 	Notes
1 	Orange Pi 6 Plus 	CIX CD8180/CD8160 (12-core ARMv9: 4xA720@2.6G + 4xA720@2.4G + 4xA520@1.8G) 	16/32/64GB LPDDR5 128-bit @5500MT/s 	45 TOPS (30 NPU + CPU+GPU) 	Dual 5GbE 	DP1.4 4K@120 + HDMI1.4 4K@60 + 2x USB-C DP + eDP 	2x M.2 NVMe PCIe4 x4, uSD 	115x100mm 	224(16GB),224 (16GB),224(16GB),269 (32GB), 64GB N/A yet 	Debian, Ubuntu, Android, OpenHarmony, Windows, ROS2. Very new — limited community. 	Highest RAM potential. 64GB announced but not shippable. Needs active cooling. 100W PD.
2 	Orange Pi 6 	CIX CD8180 (12-core ARMv9) 	8/16/24GB LPDDR5 	45 TOPS 	Dual 2.5GbE 	DP1.4 + HDMI1.4 + USB-C DP + eDP 	2x M.2 NVMe PCIe4, uSD 	115x100mm 	TBD (new 2025) 	Debian, Ubuntu, Android. Very new. 	Lower spec sibling of 6 Plus. 24GB max (not 64GB).
3 	Orange Pi 5 Max 	RK3588 (4xA76@2.4G + 4xA55@1.8G) 	4/8/16GB LPDDR5 	6 TOPS 	1x 2.5GbE 	2x HDMI 2.1 8K@60 	M.2 NVMe PCIe3 x4, eMMC socket, uSD 	89x57mm 	95(8GB),95 (8GB),95(8GB),125 (16GB) 	Excellent. Armbian, Ubuntu, Debian, Android 12/13, OpenWRT, DietPi. Mainline kernel 6.x+. Massive community. 	Best value DDR5 RK3588. WiFi 6E+BT5.3 onboard. eARC.
4 	Orange Pi 5 Ultra 	RK3588 (same as 5 Max) 	4/8/16GB LPDDR5 	6 TOPS 	1x 2.5GbE 	1x HDMI 2.1 out 8K + 1x HDMI 2.0 in 4K@60 	M.2 NVMe PCIe3 x4, eMMC socket, uSD 	89x57mm 	$125 (16GB) 	Same as OPi5 Max — excellent. 	Unique HDMI input for capture card use. WiFi 6E+BT5.3.
5 	Orange Pi 5 Pro 	RK3588S 	4/8/16GB LPDDR5 	6 TOPS 	1x GbE 	1x HDMI 2.1 + 1x HDMI 2.0 	M.2 NVMe PCIe2 x1 (2242), eMMC socket, uSD 	89x56mm 	60(4GB),60 (4GB),60(4GB),109 (16GB) 	Excellent. Same ecosystem as OPi5 series. 	Cheapest LPDDR5 option. WiFi 5+BT5. Slower M.2 (PCIe 2.0 x1).
6 	Orange Pi Zero 3W 	Allwinner A733 	Up to 16GB LPDDR5 	3 TOPS NPU 	WiFi6+BT5.4 	MIPI DSI 	uSD 	Small 	TBD (new 2025) 	Limited — very new SoC. 	Budget LPDDR5 with NPU. RISC-V coprocessor.
7 	Orange Pi 4 Pro 	Allwinner A733 	Up to 16GB LPDDR5 	3 TOPS NPU 	WiFi6+BT5.4 	MIPI DSI 	uSD 	Small 	TBD (new 2025) 	Limited — very new. 	Similar to Zero 3W but different form factor.

Ecosystem Rating: ★★★★★ (RK3588), ★★☆☆☆ (CIX CD8180, Allwinner A733)
TIER 1: LPDDR4X 32GB (Maximum Available DDR4 — Ships Today)
# 	Board 	SoC 	RAM 	AI 	Networking 	Display 	Storage 	Size 	AliExpress Price 	Linux Ecosystem 	Notes
1 	Orange Pi 5 Plus (32GB) 	RK3588 full (4xA76@2.4G + 4xA55@1.8G) 	32GB LPDDR4X 	6 TOPS 	2x 2.5GbE 	2x HDMI 2.1 8K + HDMI in 	M.2 NVMe PCIe3 x4, eMMC socket, uSD 	100x75mm 	275−275-275−499 	Best. Armbian, Ubuntu, Debian, Android, OpenWRT. Mainline 6.x+. 	King of DDR4 ARM SBCs. Dual 2.5GbE, HDMI in, PCIe 3.0. M.2 2230 for WiFi.
2 	Orange Pi 5 (32GB) 	RK3588S 	32GB LPDDR4X 	6 TOPS 	1x GbE 	1x HDMI 2.1 4K 	M.2 NVMe PCIe2 x1 (2242), uSD 	100x46mm 	$159 	Excellent. Same ecosystem. 	Smallest 32GB RK3588. Single GbE, slower M.2.
3 	Orange Pi 5B (32GB) 	RK3588S 	32GB LPDDR4X 	6 TOPS 	1x GbE 	1x HDMI 2.1 	M.2 NVMe PCIe2 x1 (2242), uSD 	100x56mm 	$179 	Excellent. Same ecosystem. 	Slightly larger than OPi5, better cooling.
4 	Banana Pi M7 (32GB) 	RK3588 	32GB LPDDR4X + 128GB eMMC 	6 TOPS 	2x 2.5GbE 	HDMI 2.1 8K + USB-C DP 8K@30 + MIPI DSI 	M.2 NVMe PCIe3 x4, eMMC, uSD 	92x62mm 	165−165-165−292 	Good. Official: Debian11, Android12, Buildroot. 3rd: Armbian, Ubuntu 20/22. 	Best value 32GB BPI. WiFi 6+BT5 onboard. Compact.
5 	Banana Pi AIM7 	RK3588 core module (260-pin SO-DIMM) 	32GB LPDDR4X + 128GB eMMC 	6 TOPS 	Via carrier 	Via carrier 	eMMC onboard, carrier-dependent 	Module 	~$200+ (module only) 	Same as RK3588 ecosystem. 	Jetson TX2 NX compatible carrier. Core module only — needs carrier board.
6 	Orange Pi AI Station 	Huawei Ascend 310 	48/96GB LPDDR4X 	176 TOPS 	WiFi+BT4.2 	N/A (headless AI) 	NVMe slot 	Large 	$800+ (specialized) 	Huawei ecosystem. Limited Linux distro support. 	Highest RAM of any OPi/BPI. Specialized AI inference board, not general-purpose.

Ecosystem Rating: ★★★★★ (RK3588), ★★☆☆☆ (Ascend 310)
TIER 2: LPDDR4X/LPDDR5 16GB (Sweet Spot — Performance/Price)
# 	Board 	SoC 	RAM Type 	AI 	Networking 	Display 	Storage 	Size 	AliExpress Price 	Linux Ecosystem 	Notes
1 	Orange Pi 5 Plus (16GB) 	RK3588 	16GB LPDDR4X 	6 TOPS 	2x 2.5GbE 	2x HDMI 2.1 + HDMI in 	M.2 NVMe PCIe3 x4, eMMC 	100x75mm 	135−135-135−294 	★★★★★ 	Dual 2.5GbE, PCIe 3.0. Best OPi5+ value.
2 	Orange Pi 5 (16GB) 	RK3588S 	16GB LPDDR4X 	6 TOPS 	1x GbE 	HDMI 2.1 	M.2 NVMe 2242 	100x46mm 	$120 	★★★★★ 	Smallest RK3588. Great value.
3 	Orange Pi 5B (16GB) 	RK3588S 	16GB LPDDR4X 	6 TOPS 	1x GbE 	HDMI 2.1 	M.2 NVMe 2242 	100x56mm 	$120 	★★★★★ 	Same as OPi5, bigger PCB.
4 	Orange Pi 5 Plus (8GB) 	RK3588 	8GB LPDDR4X 	6 TOPS 	2x 2.5GbE 	2x HDMI 2.1 + HDMI in 	M.2 NVMe PCIe3 x4 	100x75mm 	$135 	★★★★★ 	Cheapest dual-2.5GbE RK3588.
5 	Orange Pi 5 (8GB) 	RK3588S 	8GB LPDDR4X 	6 TOPS 	1x GbE 	HDMI 2.1 	M.2 NVMe 2242 	100x46mm 	75−75-75−89 	★★★★★ 	Budget RK3588 entry.
6 	Orange Pi 5B (8GB) 	RK3588S 	8GB LPDDR4X 	6 TOPS 	1x GbE 	HDMI 2.1 	M.2 NVMe 2242 	100x56mm 	$75 	★★★★★ 	
7 	Orange Pi 5 Max (8GB) 	RK3588 	8GB LPDDR5 	6 TOPS 	2.5GbE 	2x HDMI 2.1 	M.2 NVMe PCIe3 x4, eMMC 	89x57mm 	$95 	★★★★★ 	DDR5 at 8GB price. WiFi 6E.
8 	Banana Pi M7 (16GB) 	RK3588 	16GB LPDDR4X + 128GB eMMC 	6 TOPS 	2x 2.5GbE 	HDMI 2.1 + USB-C DP + DSI 	M.2 NVMe PCIe3 x4, eMMC 	92x62mm 	165−165-165−200 	★★★★☆ 	WiFi 6 onboard. RPi 40-pin compatible.
9 	Banana Pi M7 (8GB) 	RK3588 	8GB LPDDR4X + 64GB eMMC 	6 TOPS 	2x 2.5GbE 	HDMI 2.1 + USB-C DP + DSI 	M.2 NVMe, eMMC 	92x62mm 	$165 	★★★★☆ 	Same board, 8GB config.
10 	Banana Pi M5 Pro 	RK3576 	8/16GB LPDDR4x + 128GB eMMC 	Lower (no strong NPU) 	2x GbE 	HDMI + MIPI DSI 	M.2, eMMC 	92x62mm 	~$120-180 	★★★☆☆ 	Newer SoC, growing support.

Ecosystem Rating: ★★★★★ (RK3588), ★★★☆☆ (RK3576)
TIER 3: LPDDR5/LPDDR4 8-16GB Budget & Specialty
# 	Board 	SoC 	RAM 	AI 	Networking 	Storage 	Size 	AliExpress Price 	Linux Ecosystem 	Notes
1 	Orange Pi 3B 	RK3566 	2/4/8GB LPDDR4X 	None 	WiFi5 + 1x GbE 	HDMI 2.0 	eMMC, uSD 	92x62mm 	$45 	★★★★☆
2 	Orange Pi Zero 3 	Allwinner H618 	1/2/4GB LPDDR4 	None 	WiFi5 	MIPI DSI 	uSD 	Tiny 	$40 	★★★★☆
3 	Orange Pi RV2 	SpacemiT X1 RISC-V 	8GB LPDDR4X 	2 TOPS 	WiFi5 + 2x GbE 	HDMI 	M.2, uSD 	Compact 	46−46-46−64 	★★☆☆☆
4 	Orange Pi RV 	SpacemiT X1 RISC-V 	4GB LPDDR4 	2 TOPS 	WiFi5 	HDMI 	M.2, uSD 	Tiny 	$50 	★★☆☆☆
5 	Orange Pi CM4 	RK3566 	1/2/4/8GB LPDDR4X 	None 	Via carrier 	Via carrier 	eMMC, uSD 	CM module 	~$30-60 	★★★★☆
6 	Orange Pi CM5 	RK3588S 	4/8/16GB (LPDDR5 configs) 	6 TOPS 	Via carrier 	Via carrier 	eMMC, uSD 	CM module 	~$70-150 	★★★★★
7 	Banana Pi F3 (16GB) 	SpacemiT K1 RISC-V 	16GB LPDDR4 	2 TOPS 	WiFi5 + 2x GbE 	HDMI 1.4 	M.2 NVMe/SATA, eMMC, uSD 	148x100mm 	61−61-61−90 	★★☆☆☆
8 	Banana Pi F3 (8GB) 	SpacemiT K1 RISC-V 	8GB LPDDR4 	2 TOPS 	WiFi5 + 2x GbE 	HDMI 1.4 	M.2, eMMC, uSD 	148x100mm 	61−61-61−74 	★★☆☆☆
9 	Banana Pi CM6 	SpacemiT K1 RISC-V 	4/8/16GB LPDDR4 	2 TOPS 	WiFi6+BT5.2 	Via carrier 	eMMC 	CM module 	TBD 	★★☆☆☆
10 	Banana Pi CM5 Pro 	RK3576 	8/16GB LPDDR5/LPDDR4x 	Lower 	Via carrier 	Via carrier 	eMMC 	CM module 	TBD 	★★★☆☆

Ecosystem Rating: ★★★★☆ (RK3566/RK3588 CM), ★★☆☆☆ (RISC-V)
TIER 4: Router / Specialty (8GB)
# 	Board 	SoC 	RAM 	Special Feature 	AliExpress Price 	Linux Ecosystem
1 	Banana Pi R4 Pro 	MT7988A 	8GB 	WiFi 7, 2x 10G SFP+ 	~$150-200 	OpenWrt. Excellent router OS.
2 	Banana Pi W3 	RK3588 	8GB 	eMMC, RK3588 core board 	~$180 	RK3588 ecosystem
3 	Banana Pi SM10 	SpacemiT K3 RISC-V 	8/16/32GB LPDDR5 6400MT/s 	60 TOPS AI, Jetson Orin Nano carrier 	TBD (new 2026) 	Very new.
4 	Banana Pi M4 Super 	RK3568B2 	2/4GB 	Budget GbE 	~$40-60 	Armbian.
5 	Banana Pi M1 Super 	RK3528 	2/4GB 	Budget GbE 	~$30-40 	Armbian.
ECOSYSTEM RANKING (Linux/OpenSource Support)
Rank 	SoC Family 	Score 	Details
🥇 	RK3588/RK3588S 	★★★★★ 	Mainline kernel 6.x+, Armbian (official), Ubuntu, Debian, Android 12/13, OpenWRT, DietPi, ROS2. Massive community. Most documentation.
🥈 	RK3576 	★★★☆☆ 	Armbian available. Rockchip BSP. Growing community. Newer = less mature.
🥉 	RK3566/RK3568 	★★★★☆ 	Very mature. Armbian official. Excellent budget option.
4 	SpacemiT K3 	★★☆☆☆ 	Brand new (April 2026). Jetson carrier compatible. LPDDR5 32GB potential.
5 	SpacemiT K1 RISC-V 	★★☆☆☆ 	RISC-V ecosystem immature. Limited distros. 2 TOPS only.
6 	CIX CD8180/CD8160 	★★☆☆☆ 	Brand new SoC. Debian/Ubuntu/Android listed. No Armbian yet. Closed BSP.
7 	Ascend 310 	★★☆☆☆ 	Huawei specialized. Limited to Huawei AI stack. Not general-purpose.
8 	Allwinner A733 	★★☆☆☆ 	New 2025. Unknown community support level.
KEY TAKEAWAYS

    32GB LPDDR4X is the max you can BUY today — on RK3588 boards (OPi 5/5+/5B 32GB, BPI-M7 32GB)
    64GB LPDDR5 announced but NOT shipping — Orange Pi 6 Plus. Blocked by LPDDR5 chip cost.
    No DIMM slots exist on any Orange Pi or Banana Pi ARM board. All RAM is soldered.
    RK3588 is king for ecosystem. If you want Linux that works out of the box, buy RK3588.
    Best value 32GB: Orange Pi 5 (32GB) at $159 — cheapest 32GB ARM SBC available.
    Best featured 32GB: Orange Pi 5 Plus (32GB) at $275-499 — dual 2.5GbE, PCIe 3.0, HDMI in.
    Best 32GB Banana Pi: BPI-M7 (32GB) at $165-292 — dual 2.5GbE, WiFi 6, M.2 NVMe.
    Best DDR5: Orange Pi 5 Max/Ultra (16GB LPDDR5) at $125 — excellent ecosystem, WiFi 6E.
    RISC-V options exist (BPI-F3 16GB, OPi RV2 8GB) but ecosystem is immature.
    The Orange Pi AI Station has 48/96GB LPDDR4X but it's a specialized AI board, not general-purpose. ~$800+.

## [0088] 06-28 13:38

(2026-06-28T13:38:14)
distill the reports content in terms of specific model options and model options from this other chats response and give me a deduplicated table and I'll tell you real prices as aliexpress can be a bit deceptive with thier titles and descriptions using different options to obscure the real price for the ram your getting.

TIER 0: LPDDR5 32GB+ (Highest RAM, Newest Gen)
# 	Board 	SoC 	RAM 	AI 	Networking 	Display 	Storage 	Size 	AliExpress Price 	Linux Ecosystem 	Notes
1 	Orange Pi 6 Plus 	CIX CD8180/CD8160 (12-core ARMv9: 4xA720@2.6G + 4xA720@2.4G + 4xA520@1.8G) 	16/32/64GB LPDDR5 128-bit @5500MT/s 	45 TOPS (30 NPU + CPU+GPU) 	Dual 5GbE 	DP1.4 4K@120 + HDMI1.4 4K@60 + 2x USB-C DP + eDP 	2x M.2 NVMe PCIe4 x4, uSD 	115x100mm 	224(16GB),224 (16GB),224(16GB),269 (32GB), 64GB N/A yet 	Debian, Ubuntu, Android, OpenHarmony, Windows, ROS2. Very new — limited community. 	Highest RAM potential. 64GB announced but not shippable. Needs active cooling. 100W PD.
2 	Orange Pi 6 	CIX CD8180 (12-core ARMv9) 	8/16/24GB LPDDR5 	45 TOPS 	Dual 2.5GbE 	DP1.4 + HDMI1.4 + USB-C DP + eDP 	2x M.2 NVMe PCIe4, uSD 	115x100mm 	TBD (new 2025) 	Debian, Ubuntu, Android. Very new. 	Lower spec sibling of 6 Plus. 24GB max (not 64GB).
3 	Orange Pi 5 Max 	RK3588 (4xA76@2.4G + 4xA55@1.8G) 	4/8/16GB LPDDR5 	6 TOPS 	1x 2.5GbE 	2x HDMI 2.1 8K@60 	M.2 NVMe PCIe3 x4, eMMC socket, uSD 	89x57mm 	95(8GB),95 (8GB),95(8GB),125 (16GB) 	Excellent. Armbian, Ubuntu, Debian, Android 12/13, OpenWRT, DietPi. Mainline kernel 6.x+. Massive community. 	Best value DDR5 RK3588. WiFi 6E+BT5.3 onboard. eARC.
4 	Orange Pi 5 Ultra 	RK3588 (same as 5 Max) 	4/8/16GB LPDDR5 	6 TOPS 	1x 2.5GbE 	1x HDMI 2.1 out 8K + 1x HDMI 2.0 in 4K@60 	M.2 NVMe PCIe3 x4, eMMC socket, uSD 	89x57mm 	$125 (16GB) 	Same as OPi5 Max — excellent. 	Unique HDMI input for capture card use. WiFi 6E+BT5.3.
5 	Orange Pi 5 Pro 	RK3588S 	4/8/16GB LPDDR5 	6 TOPS 	1x GbE 	1x HDMI 2.1 + 1x HDMI 2.0 	M.2 NVMe PCIe2 x1 (2242), eMMC socket, uSD 	89x56mm 	60(4GB),60 (4GB),60(4GB),109 (16GB) 	Excellent. Same ecosystem as OPi5 series. 	Cheapest LPDDR5 option. WiFi 5+BT5. Slower M.2 (PCIe 2.0 x1).
6 	Orange Pi Zero 3W 	Allwinner A733 	Up to 16GB LPDDR5 	3 TOPS NPU 	WiFi6+BT5.4 	MIPI DSI 	uSD 	Small 	TBD (new 2025) 	Limited — very new SoC. 	Budget LPDDR5 with NPU. RISC-V coprocessor.
7 	Orange Pi 4 Pro 	Allwinner A733 	Up to 16GB LPDDR5 	3 TOPS NPU 	WiFi6+BT5.4 	MIPI DSI 	uSD 	Small 	TBD (new 2025) 	Limited — very new. 	Similar to Zero 3W but different form factor.

Ecosystem Rating: ★★★★★ (RK3588), ★★☆☆☆ (CIX CD8180, Allwinner A733)
TIER 1: LPDDR4X 32GB (Maximum Available DDR4 — Ships Today)
# 	Board 	SoC 	RAM 	AI 	Networking 	Display 	Storage 	Size 	AliExpress Price 	Linux Ecosystem 	Notes
1 	Orange Pi 5 Plus (32GB) 	RK3588 full (4xA76@2.4G + 4xA55@1.8G) 	32GB LPDDR4X 	6 TOPS 	2x 2.5GbE 	2x HDMI 2.1 8K + HDMI in 	M.2 NVMe PCIe3 x4, eMMC socket, uSD 	100x75mm 	275−275-275−499 	Best. Armbian, Ubuntu, Debian, Android, OpenWRT. Mainline 6.x+. 	King of DDR4 ARM SBCs. Dual 2.5GbE, HDMI in, PCIe 3.0. M.2 2230 for WiFi.
2 	Orange Pi 5 (32GB) 	RK3588S 	32GB LPDDR4X 	6 TOPS 	1x GbE 	1x HDMI 2.1 4K 	M.2 NVMe PCIe2 x1 (2242), uSD 	100x46mm 	$159 	Excellent. Same ecosystem. 	Smallest 32GB RK3588. Single GbE, slower M.2.
3 	Orange Pi 5B (32GB) 	RK3588S 	32GB LPDDR4X 	6 TOPS 	1x GbE 	1x HDMI 2.1 	M.2 NVMe PCIe2 x1 (2242), uSD 	100x56mm 	$179 	Excellent. Same ecosystem. 	Slightly larger than OPi5, better cooling.
4 	Banana Pi M7 (32GB) 	RK3588 	32GB LPDDR4X + 128GB eMMC 	6 TOPS 	2x 2.5GbE 	HDMI 2.1 8K + USB-C DP 8K@30 + MIPI DSI 	M.2 NVMe PCIe3 x4, eMMC, uSD 	92x62mm 	165−165-165−292 	Good. Official: Debian11, Android12, Buildroot. 3rd: Armbian, Ubuntu 20/22. 	Best value 32GB BPI. WiFi 6+BT5 onboard. Compact.
5 	Banana Pi AIM7 	RK3588 core module (260-pin SO-DIMM) 	32GB LPDDR4X + 128GB eMMC 	6 TOPS 	Via carrier 	Via carrier 	eMMC onboard, carrier-dependent 	Module 	~$200+ (module only) 	Same as RK3588 ecosystem. 	Jetson TX2 NX compatible carrier. Core module only — needs carrier board.
6 	Orange Pi AI Station 	Huawei Ascend 310 	48/96GB LPDDR4X 	176 TOPS 	WiFi+BT4.2 	N/A (headless AI) 	NVMe slot 	Large 	$800+ (specialized) 	Huawei ecosystem. Limited Linux distro support. 	Highest RAM of any OPi/BPI. Specialized AI inference board, not general-purpose.

Ecosystem Rating: ★★★★★ (RK3588), ★★☆☆☆ (Ascend 310)
TIER 2: LPDDR4X/LPDDR5 16GB (Sweet Spot — Performance/Price)
# 	Board 	SoC 	RAM Type 	AI 	Networking 	Display 	Storage 	Size 	AliExpress Price 	Linux Ecosystem 	Notes
1 	Orange Pi 5 Plus (16GB) 	RK3588 	16GB LPDDR4X 	6 TOPS 	2x 2.5GbE 	2x HDMI 2.1 + HDMI in 	M.2 NVMe PCIe3 x4, eMMC 	100x75mm 	135−135-135−294 	★★★★★ 	Dual 2.5GbE, PCIe 3.0. Best OPi5+ value.
2 	Orange Pi 5 (16GB) 	RK3588S 	16GB LPDDR4X 	6 TOPS 	1x GbE 	HDMI 2.1 	M.2 NVMe 2242 	100x46mm 	$120 	★★★★★ 	Smallest RK3588. Great value.
3 	Orange Pi 5B (16GB) 	RK3588S 	16GB LPDDR4X 	6 TOPS 	1x GbE 	HDMI 2.1 	M.2 NVMe 2242 	100x56mm 	$120 	★★★★★ 	Same as OPi5, bigger PCB.
4 	Orange Pi 5 Plus (8GB) 	RK3588 	8GB LPDDR4X 	6 TOPS 	2x 2.5GbE 	2x HDMI 2.1 + HDMI in 	M.2 NVMe PCIe3 x4 	100x75mm 	$135 	★★★★★ 	Cheapest dual-2.5GbE RK3588.
5 	Orange Pi 5 (8GB) 	RK3588S 	8GB LPDDR4X 	6 TOPS 	1x GbE 	HDMI 2.1 	M.2 NVMe 2242 	100x46mm 	75−75-75−89 	★★★★★ 	Budget RK3588 entry.
6 	Orange Pi 5B (8GB) 	RK3588S 	8GB LPDDR4X 	6 TOPS 	1x GbE 	HDMI 2.1 	M.2 NVMe 2242 	100x56mm 	$75 	★★★★★ 	
7 	Orange Pi 5 Max (8GB) 	RK3588 	8GB LPDDR5 	6 TOPS 	2.5GbE 	2x HDMI 2.1 	M.2 NVMe PCIe3 x4, eMMC 	89x57mm 	$95 	★★★★★ 	DDR5 at 8GB price. WiFi 6E.
8 	Banana Pi M7 (16GB) 	RK3588 	16GB LPDDR4X + 128GB eMMC 	6 TOPS 	2x 2.5GbE 	HDMI 2.1 + USB-C DP + DSI 	M.2 NVMe PCIe3 x4, eMMC 	92x62mm 	165−165-165−200 	★★★★☆ 	WiFi 6 onboard. RPi 40-pin compatible.
9 	Banana Pi M7 (8GB) 	RK3588 	8GB LPDDR4X + 64GB eMMC 	6 TOPS 	2x 2.5GbE 	HDMI 2.1 + USB-C DP + DSI 	M.2 NVMe, eMMC 	92x62mm 	$165 	★★★★☆ 	Same board, 8GB config.
10 	Banana Pi M5 Pro 	RK3576 	8/16GB LPDDR4x + 128GB eMMC 	Lower (no strong NPU) 	2x GbE 	HDMI + MIPI DSI 	M.2, eMMC 	92x62mm 	~$120-180 	★★★☆☆ 	Newer SoC, growing support.

Ecosystem Rating: ★★★★★ (RK3588), ★★★☆☆ (RK3576)
TIER 3: LPDDR5/LPDDR4 8-16GB Budget & Specialty
# 	Board 	SoC 	RAM 	AI 	Networking 	Storage 	Size 	AliExpress Price 	Linux Ecosystem 	Notes
1 	Orange Pi 3B 	RK3566 	2/4/8GB LPDDR4X 	None 	WiFi5 + 1x GbE 	HDMI 2.0 	eMMC, uSD 	92x62mm 	$45 	★★★★☆
2 	Orange Pi Zero 3 	Allwinner H618 	1/2/4GB LPDDR4 	None 	WiFi5 	MIPI DSI 	uSD 	Tiny 	$40 	★★★★☆
3 	Orange Pi RV2 	SpacemiT X1 RISC-V 	8GB LPDDR4X 	2 TOPS 	WiFi5 + 2x GbE 	HDMI 	M.2, uSD 	Compact 	46−46-46−64 	★★☆☆☆
4 	Orange Pi RV 	SpacemiT X1 RISC-V 	4GB LPDDR4 	2 TOPS 	WiFi5 	HDMI 	M.2, uSD 	Tiny 	$50 	★★☆☆☆
5 	Orange Pi CM4 	RK3566 	1/2/4/8GB LPDDR4X 	None 	Via carrier 	Via carrier 	eMMC, uSD 	CM module 	~$30-60 	★★★★☆
6 	Orange Pi CM5 	RK3588S 	4/8/16GB (LPDDR5 configs) 	6 TOPS 	Via carrier 	Via carrier 	eMMC, uSD 	CM module 	~$70-150 	★★★★★
7 	Banana Pi F3 (16GB) 	SpacemiT K1 RISC-V 	16GB LPDDR4 	2 TOPS 	WiFi5 + 2x GbE 	HDMI 1.4 	M.2 NVMe/SATA, eMMC, uSD 	148x100mm 	61−61-61−90 	★★☆☆☆
8 	Banana Pi F3 (8GB) 	SpacemiT K1 RISC-V 	8GB LPDDR4 	2 TOPS 	WiFi5 + 2x GbE 	HDMI 1.4 	M.2, eMMC, uSD 	148x100mm 	61−61-61−74 	★★☆☆☆
9 	Banana Pi CM6 	SpacemiT K1 RISC-V 	4/8/16GB LPDDR4 	2 TOPS 	WiFi6+BT5.2 	Via carrier 	eMMC 	CM module 	TBD 	★★☆☆☆
10 	Banana Pi CM5 Pro 	RK3576 	8/16GB LPDDR5/LPDDR4x 	Lower 	Via carrier 	Via carrier 	eMMC 	CM module 	TBD 	★★★☆☆

Ecosystem Rating: ★★★★☆ (RK3566/RK3588 CM), ★★☆☆☆ (RISC-V)
TIER 4: Router / Specialty (8GB)
# 	Board 	SoC 	RAM 	Special Feature 	AliExpress Price 	Linux Ecosystem
1 	Banana Pi R4 Pro 	MT7988A 	8GB 	WiFi 7, 2x 10G SFP+ 	~$150-200 	OpenWrt. Excellent router OS.
2 	Banana Pi W3 	RK3588 	8GB 	eMMC, RK3588 core board 	~$180 	RK3588 ecosystem
3 	Banana Pi SM10 	SpacemiT K3 RISC-V 	8/16/32GB LPDDR5 6400MT/s 	60 TOPS AI, Jetson Orin Nano carrier 	TBD (new 2026) 	Very new.
4 	Banana Pi M4 Super 	RK3568B2 	2/4GB 	Budget GbE 	~$40-60 	Armbian.
5 	Banana Pi M1 Super 	RK3528 	2/4GB 	Budget GbE 	~$30-40 	Armbian.
ECOSYSTEM RANKING (Linux/OpenSource Support)
Rank 	SoC Family 	Score 	Details
🥇 	RK3588/RK3588S 	★★★★★ 	Mainline kernel 6.x+, Armbian (official), Ubuntu, Debian, Android 12/13, OpenWRT, DietPi, ROS2. Massive community. Most documentation.
🥈 	RK3576 	★★★☆☆ 	Armbian available. Rockchip BSP. Growing community. Newer = less mature.
🥉 	RK3566/RK3568 	★★★★☆ 	Very mature. Armbian official. Excellent budget option.
4 	SpacemiT K3 	★★☆☆☆ 	Brand new (April 2026). Jetson carrier compatible. LPDDR5 32GB potential.
5 	SpacemiT K1 RISC-V 	★★☆☆☆ 	RISC-V ecosystem immature. Limited distros. 2 TOPS only.
6 	CIX CD8180/CD8160 	★★☆☆☆ 	Brand new SoC. Debian/Ubuntu/Android listed. No Armbian yet. Closed BSP.
7 	Ascend 310 	★★☆☆☆ 	Huawei specialized. Limited to Huawei AI stack. Not general-purpose.
8 	Allwinner A733 	★★☆☆☆ 	New 2025. Unknown community support level.
KEY TAKEAWAYS

    32GB LPDDR4X is the max you can BUY today — on RK3588 boards (OPi 5/5+/5B 32GB, BPI-M7 32GB)
    64GB LPDDR5 announced but NOT shipping — Orange Pi 6 Plus. Blocked by LPDDR5 chip cost.
    No DIMM slots exist on any Orange Pi or Banana Pi ARM board. All RAM is soldered.
    RK3588 is king for ecosystem. If you want Linux that works out of the box, buy RK3588.
    Best value 32GB: Orange Pi 5 (32GB) at $159 — cheapest 32GB ARM SBC available.
    Best featured 32GB: Orange Pi 5 Plus (32GB) at $275-499 — dual 2.5GbE, PCIe 3.0, HDMI in.
    Best 32GB Banana Pi: BPI-M7 (32GB) at $165-292 — dual 2.5GbE, WiFi 6, M.2 NVMe.
    Best DDR5: Orange Pi 5 Max/Ultra (16GB LPDDR5) at $125 — excellent ecosystem, WiFi 6E.
    RISC-V options exist (BPI-F3 16GB, OPi RV2 8GB) but ecosystem is immature.
    The Orange Pi AI Station has 48/96GB LPDDR4X but it's a specialized AI board, not general-purpose. ~$800+.

## [0089] 06-28 14:43

(2026-06-28T14:43:59)
continue

## [0090] 06-28 14:44

(2026-06-28T14:44:18)
continue

## [0091] 06-28 16:50

(2026-06-28T16:50:15)
no changes just search and answer in details, what hooks and tools are we missing that opencode has that if we had in our execution engine expecting similar performance if not much much slower but just as if not more reliably?  list in a well formatted organized way what we are missing.

## [0092] 06-28 16:57

(2026-06-28T16:57:34)
what from the unified schema .yml file related to tool usage and skill usage and script usage is unimplemented in the engine or in the meta workflow generator .yml ?  no changes just explore and answer exhaustively

## [0093] 06-28 17:27

(2026-06-28T17:27:56)
look into what remaining gaps we have that would enable us to surpass opencode reloability

## [0094] 06-28 18:10

(2026-06-28T18:10:23)
what else could we add to help?

## [0095] 06-28 18:41

(2026-06-28T18:41:04)
 Allwinner A733  seems like the most approachable from a price to ram ratio having 12gb of lpddr5 ram for only $150.  tell me about it

## [0096] 06-28 18:41

(2026-06-28T18:41:28)
Orange Pi Zero 3W 12GB Ram + Fan Development Board LPDDR5 Allwinner A733 Chip 3Tops NPU Wi-Fi6.0+BT5.4 Single Board Computer

## [0097] 06-28 18:41

(2026-06-28T18:41:29)
[CONTEXT]: Researching Allwinner A733 SoC for SBC buying decision. User saw AliExpress listing claiming "12GB LPDDR5 for $150" — likely deceptive RAM-for-price. My internal notes only know A733 is "new 2025, used in Orange Pi Zero 3W and Orange Pi 4 Pro, up to 16GB LPDDR5, 3 TOPS NPU, WiFi6+BT5.4, RISC-V coprocessor, ecosystem unknown."

[GOAL]: Authoritative specs on Allwinner A733 — CPU cores, process node, GPU, NPU, memory controller (type/speed/channels/bits), ISP, video codecs, launch date, who's actually shipping it.

[DOWNSTREAM]: Will use to verify user's $150/12GB claim and assess whether A733 is a serious Tier B candidate (16-32GB orchestrator class) or budget-tier trap.

[REQUEST]: 
- Find official Allwinner A733 datasheet or product page (allwinnertech.com)
- Cross-reference Linux-Sunxi wiki (linux-sunxi.org) for A733 page
- Find board announcements: Orange Pi Zero 3W, Orange Pi 4 Pro, any others using A733
- Spec out: CPU cluster config, NPU TOPS confirmed, max RAM config officially supported, memory bus width
- Real ship status: is anyone actually shipping A733 boards in 2026, or paper launch?
- Ecosystem: any mainline Linux kernel support? Armbian? BSP only?

Return: concise spec sheet + ship status + ecosystem verdict. Skip generic SoC tutorials.
<!-- OMO_INTERNAL_INITIATOR -->

## [0098] 06-28 18:41

(2026-06-28T18:41:37)
[CONTEXT]: User evaluating Orange Pi boards using Allwinner A733 SoC. Saw AliExpress claim "12GB LPDDR5 for $150" — known deceptive pattern where sellers list multiple RAM options under one listing to obscure per-RAM real price.

[GOAL]: Find ACTUAL street prices per RAM configuration for A733 boards (Orange Pi Zero 3W, Orange Pi 4 Pro, any others). Verify whether 12GB LPDDR5 / 16GB LPDDR5 is real shipping or paper launch.

[DOWNSTREAM]: User will compare A733 boards against RK3588 alternatives (OPi 5 Max 16GB LPDDR5 ~$125, OPi 5 Pro 16GB LPDDR5 ~$109). Need apples-to-apples price-per-GB and price-per-usable-feature comparison.

[REQUEST]:
- AliExpress Orange Pi official store: orangepiworld, orange-pi-official — list SKUs and current prices for A733-based boards
- Orange Pi official site (orangepi.org) pricing for Zero 3W and 4 Pro
- Amazon / Banggood / Geekbuying / AliExpress alternate sellers
- Per-RAM-tier prices: 1GB / 2GB / 4GB / 8GB / 12GB / 16GB if they exist
- Note which RAM tiers ACTUALLY SHIP vs "announced"
- Flag any listing where price shown is for lowest RAM config and 12GB/16GB requires selecting higher option
- Compare shipping time/cost from China

Return: table of board × RAM-tier × real price × ship status. Skip marketing fluff.
<!-- OMO_INTERNAL_INITIATOR -->

## [0099] 06-28 18:44

(2026-06-28T18:44:01)
no  opi 5 pro is currently $350

## [0100] 06-28 18:50

(2026-06-28T18:50:10)
radxa cubie models don't have 16gb models availabe and the 8gb are over $300

