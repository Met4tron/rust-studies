# Tipos

## Scalar Types

### Integer

- Podem ser:
  - Signed => Inteiros Positivos + Negativos
  - Unsigned => Inteiros Positivos
- 8, 16, 32, 64, 128 bits
- Sempre utilizar o type mais próximo ao número possivel

```rust

let decimal = 98_222;
let hex = 0xff;
let octal = 0o77;
let binary = 0b1111_0000;
let byte = b'A'

```
---

### Floats

- 32 e 64 bits
- Single Precision (f32)
- Double Precision (f64)

```rust
let x: f64 = 2.0;
let y: f32 = 3.0;
```

### Boolean

```rust
let truthy: bool = true;
let falsy: bool = false;
```

### Char

- Representa somente um character unicode com 4 bytes

```rust
let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation
let heart_eyed_cat = '😻';
```

---
## Compound Types

### Tuplas

- Tamanho fixo
- Multiplos tipos
---

### Arrays 

- Single type
- Tamanho dinamico
- Manipulado com interadores
- Vec!<T> é a melhor opção
