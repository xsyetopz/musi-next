# Marker Example Style

Markers are used throughout the formal spec to clarify which part of a source line is being discussed.

## Type and value positions

```musi
let text : Text := "hello";
          ----     -------
          type     value position
```

## Constraint mismatch

```musi
where (K, V) |= #(Ordering, Show)
      ------    -----------------
      tuple     datum requirement set
      subject   invalid here
```

## Valid tuple-shaped constraint

```musi
where (K, V) |= (#(Ordering, Show), Show)
      ------    -------------------------
      tuple     tuple-shaped satisfier
      subject
```

## Discard marker

```musi
"done";
      - discard marker; result becomes Unit
```

## Guarded effect

```musi
cleanup() when opened;
---------      ------
effect         guard checked before effect
```

## Pattern alias

```musi
.Some(x) as whole
--------    -----
inner       whole matched .Some value
binding
```
