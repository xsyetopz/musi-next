> This agent-indexable topic view is extracted from [`../MSR1.md`](../MSR1.md). `MSR1.md` remains the sole normative authority.

# Annex C (normative) — Core semantic binding closure

The following source-visible identities are fixed for MSR1 where already named by Part I and shall not be replaced by implementation-private alternatives:

```text
Type[N]     Index
Bool        Unit        Never       Rune
Bits[N]     Bytes[N]    Signed[N]   Unsigned[N]   Floating[F]
Storage[N,A]            Atomic[T]   Unknown      Address
target      sizeOf[T]   alignOf[T]
```

The remaining irreducible operation families in Part I section 10 shall be provided as ordinary lowerCamelCase core bindings with these canonical identities:

```text
integerExact[T]         guaranteed/exact integer conversion
integerChecked[T]       checked integer conversion returning the established Fallible result form
integerTruncate[T]      explicit low-order integer truncation
accessAddress[T]        safe Access -> raw Address exposure
addressOffset           raw Address arithmetic in target-defined address units
rawLoad[T]              represented raw load
rawStore[T]             represented raw store
rawVolatileLoad[T]      represented volatile raw load
rawVolatileStore[T]     represented volatile raw store
storageBegin[T]         establish one live T in suitable Storage and return safe Access
storageEnd[T]           end the live T lifetime in Storage
unknownErase[T]         construct Unknown from a permitted live T designation
unknownIs[T]            exact runtime type-identity test
atomicLoad[T]
atomicStore[T]
atomicExchange[T]
atomicCompareExchange[T]
```

Their parameter/result/effect semantics are exactly those defined by the corresponding Part I semantic clauses. These names are bindings, not grammar. An implementation may implement them directly, lower them to CPC operations, or replace calls internally after semantic analysis; it shall expose the MSR1 meaning to portable source.

Target-specific irreducible execution leaves are not added to this global namespace. They are bindings of reserved `musi:` modules whose existence and meaning are fixed by the selected target contract. Thus a target can add capability without creating a Musi dialect or changing MSR1 semantics.
