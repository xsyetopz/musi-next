# Checklist and Agent Guardrails

Status: reference for design review and AI-agent behavior.

This note imports pressure from Colin McMillen, Jason Reed, and Elly Fong-Jones, "Programming Language Checklist" (2011-10-10), and from the 2026 r/ProgrammingLanguages discussion "List of known problems in design of existing languages?".

Sources:

```text
https://www.mcmillen.dev/language_checklist.html
https://www.reddit.com/r/ProgrammingLanguages/comments/1thycym/list_of_known_problems_in_design_of_existing/
```

## Relevant checklist pressure

The checklist is not a feature list for Musi. It is used as a hostile audit against shortcuts.

The important pressure points for this source pack are:

```text
No language spec.
"The implementation is the spec."
Your type system is unsound.
Your language cannot be unambiguously parsed.
Shift-reduce conflicts in parsing seem to be resolved using rand().
You require the compiler to be present at runtime.
You require the language runtime to be present at compile-time.
Your compiler errors are completely inscrutable.
Dangerous behavior is only a warning.
Unsupported claims of increased productivity.
Unsupported claims of greater ease of use.
Rejection of orthodox programming-language theory without justification.
Rejection of orthodox systems programming without justification.
```

## Reddit-thread interpretation

The Reddit discussion is used for framing, not authority over Musi syntax.

Relevant takeaways:

```text
Design flaws are often tradeoffs relative to a language's baseline.
A checklist is not itself a design.
Ambiguous grammar is a real problem, but not every disliked feature is objectively a flaw.
Unicode/text behavior should not be assumed to be solved by ASCII-era defaults.
Powerful type/value systems need explicit restrictions rather than vague confidence.
```

## Agent guardrails

An AI agent working on Musi must not fill design space from familiar languages.

Reject or stop any change that does one of these:

```text
invents syntax not present in the docs or explicitly requested by the developer;
reserves tokens or operators without an explicit Musi decision;
turns RHS forms into separate declaration families;
imports Rust, Swift, Zig, C++, JavaScript, Lisp, Python, or ANTLR behavior as Musi law;
uses parser-generator acceptance as proof of the parser contract;
requires more than one parser lookahead token;
uses parser backtracking or semantic predicates for syntax;
uses name/type resolution to choose a grammar alternative;
conflates known with fixed;
derives namespace syntax from known, fixed, or import;
changes dot selection into a second lookup system;
changes API/stdlib examples into syntax/language rules;
turns runtime/bytecode/VM, FFI, or attribute questions into source syntax by convenience.
```

Accepted behavior:

```text
check the relevant spec chapter;
check locked decisions;
if the docs do not say a construct exists, do not add it;
if a gap exists, report the gap instead of proposing syntax;
if the developer explicitly asks for options, mark options as options.
```
