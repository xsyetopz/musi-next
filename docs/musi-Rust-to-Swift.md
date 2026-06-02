## Dependencies

- **thiserror** → Built-in: `enum MyError: Error, LocalizedError`
- **anyhow** → Built-in: `any Error` (implicit error type erasure)
- **ariadne** → Package: `SwiftDiagnostics` (from Apple)
- **memchr** → Built-in: `UnsafeRawBufferPointer.firstIndex(where:)`
- **bstr** → Built-in: `String.UTF8View` or `[UInt8]`
- **phf** → Built-in: Swift 6.3+ Macros generating switches
- **strum** → Built-in: `CaseIterable` / `RawRepresentable`
- **enum-map** → Built-in: `Dictionary` / `Bitset` (Swift `Collections`)
- **enumset** → Package: `Bitset` (from Swift `Collections`)
- **bitflags** → Built-in: `OptionSet` protocol
- **smallvec / arrayvec** → Package: `InlineArray` / `FixedArray` (from Swift `Collections`)
- **bumpalo** → Built-in: `UnsafeMutableRawPointer` arithmetic (Immix)
- **string-interner** → Built-in: Custom `[String: ID]` array/map wrapper
- **slotmap** → Built-in: Custom `[Slot]` array using stable indices
- **clap** → Package: `ArgumentParser` (from Apple)
- **rustyline** → Built-in: Zero-overhead C Interop with '`readline`'
- **serde** → Built-in: `Codable` protocol
- **postcard / ron** → Package: `Yams` (for text) / Custom binary writers

## Dev-Dependencies

- **insta** → Package: `SnapshotTesting` (by Point-Free)
- **proptest / arbitrary** → Package: `SwiftCheck`
- **criterion** → Package: package-benchmark (by Ordo One)
