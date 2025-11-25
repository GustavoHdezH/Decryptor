# Fuzz Testing Guide

Este documento explica cómo usar fuzz testing en el proyecto Decryptor para descubrir bugs y casos edge mediante pruebas automatizadas con entradas aleatorias.

## ¿Qué es Fuzz Testing?

El fuzz testing (o fuzzing) es una técnica de testing automatizada que alimenta un programa con datos aleatorios o malformados para encontrar bugs, crashes, memory leaks, y otros problemas que las pruebas tradicionales podrían no descubrir.

## Requisitos Previos

1. **Rust Nightly**: El fuzzing requiere Rust nightly
   ```bash
   rustup install nightly
   ```

2. **cargo-fuzz**: Herramienta de fuzzing para Rust
   ```bash
   cargo install cargo-fuzz
   ```

## Fuzz Targets Disponibles

### 1. `fuzz_file_type`

Prueba el parsing de tipos de archivo (`FileType::from_path`).

**Qué testea:**
- Manejo de strings arbitrarios como nombres de archivo
- Detección correcta de extensiones `.gpg` y `.age`
- Que solo retorne `Some` para extensiones válidas
- Que nunca cause panic con entradas malformadas

**Ejecutar:**
```bash
cargo +nightly fuzz run fuzz_file_type
```

**Ejecutar por tiempo limitado (ej: 60 segundos):**
```bash
cargo +nightly fuzz run fuzz_file_type -- -max_total_time=60
```

### 2. `fuzz_paths`

Prueba el manejo de rutas (`PathBuf`) con strings arbitrarios.

**Qué testea:**
- Construcción de PathBuf con strings aleatorios
- Operaciones comunes (exists, is_file, extension, etc.)
- Manejo de caracteres especiales y unicode
- Path traversal y componentes inválidos

**Ejecutar:**
```bash
cargo +nightly fuzz run fuzz_paths
```

**Ejecutar por tiempo limitado:**
```bash
cargo +nightly fuzz run fuzz_paths -- -max_total_time=60
```

## Opciones Comunes

### Limitar tiempo de ejecución
```bash
cargo +nightly fuzz run <target> -- -max_total_time=<segundos>
```

### Limitar número de iteraciones
```bash
cargo +nightly fuzz run <target> -- -runs=<número>
```

### Usar múltiples threads
```bash
cargo +nightly fuzz run <target> -- -workers=<número>
```

### Ver todas las opciones
```bash
cargo +nightly fuzz run <target> -- -help=1
```

## Interpretación de Resultados

### ✅ Ejecución Exitosa

Si el fuzzer corre sin encontrar crashes, verás algo como:
```
#1000	NEW    cov: 42 corp: 15 exec/s: 100
#2000	NEW    cov: 45 corp: 18 exec/s: 105
...
```

- `NEW`: Nueva entrada interesante encontrada
- `cov`: Cobertura de código alcanzada
- `corp`: Tamaño del corpus (casos de prueba guardados)
- `exec/s`: Ejecuciones por segundo

### ❌ Crash Encontrado

Si se encuentra un crash:
```
==12345== ERROR: libFuzzer: deadly signal
```

El fuzzer guardará:
1. **Artifact**: El input que causó el crash en `fuzz/artifacts/<target>/`
2. **Corpus**: Casos de prueba interesantes en `fuzz/corpus/<target>/`

## Reproducir un Crash

Si se encuentra un crash, el artifact se guarda automáticamente.

**Reproducir el crash:**
```bash
cargo +nightly fuzz run <target> fuzz/artifacts/<target>/<archivo>
```

**Ejemplo:**
```bash
cargo +nightly fuzz run fuzz_file_type fuzz/artifacts/fuzz_file_type/crash-1234567890abcdef
```

## Mejores Prácticas

### 1. Ejecutar Regularmente

Ejecuta el fuzzer periódicamente, especialmente:
- Antes de releases importantes
- Después de cambios significativos en el código
- Como parte de CI/CD (si es factible)

### 2. Duración Recomendada

- **Desarrollo rápido**: 1-5 minutos por target
- **Testing regular**: 10-30 minutos por target
- **Testing exhaustivo**: Varias horas o días

### 3. Mantener el Corpus

El directorio `fuzz/corpus/` contiene casos de prueba valiosos:
- Considera hacer commit del corpus para testing regression
- El corpus mejora con el tiempo, encontrando más edge cases

### 4. Agregar Nuevos Targets

Para agregar un nuevo fuzz target:

```bash
cargo +nightly fuzz add <nombre_del_target>
```

Luego edita `fuzz/fuzz_targets/<nombre_del_target>.rs` con tu lógica.

## Integración con CI

### GitHub Actions (ejemplo)

```yaml
- name: Run Fuzz Tests
  run: |
    rustup install nightly
    cargo install cargo-fuzz
    cargo +nightly fuzz run fuzz_file_type -- -max_total_time=300
    cargo +nightly fuzz run fuzz_paths -- -max_total_time=300
```

## Limitaciones

- **Requiere Nightly**: No funciona con Rust stable
- **Solo Unix-like**: No soportado en Windows nativamente
- **Tiempo**: Puede tomar mucho tiempo encontrar bugs profundos
- **No determinista**: Resultados varían entre ejecuciones

## Recursos Adicionales

- [Libro oficial de cargo-fuzz](https://rust-fuzz.github.io/book/cargo-fuzz.html)
- [Documentación de libFuzzer](https://llvm.org/docs/LibFuzzer.html)
- [Rust Fuzz Trophy Case](https://github.com/rust-fuzz/trophy-case) - Bugs reales encontrados con fuzzing

## Solución de Problemas

### Error: "requires a nightly compiler"

Asegúrate de usar `cargo +nightly fuzz` en lugar de solo `cargo fuzz`.

### Error: "cargo-fuzz not found"

Instala cargo-fuzz:
```bash
cargo install cargo-fuzz
```

### Fuzzer corre muy lento

Prueba compilar en modo release del fuzzer editando `fuzz/Cargo.toml`:
```toml
[profile.release]
opt-level = 3
```

### Out of Memory

Reduce el número de workers:
```bash
cargo +nightly fuzz run <target> -- -workers=1
```
