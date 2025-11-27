## 📋 VALIDATION CHECKLIST - PHASE 2

### Después de crear `src/tracking.rs`:

```bash
# 1. Verificar sintaxis
cargo check

# 2. Compilar módulo
cargo build --lib

# 3. Test unitarios (si existen)
cargo test tracking

# 4. Verificar integración con database
cargo run -- track --project "test" --task "validation"

# 5. Verificar comando finish
cargo run -- finish

# 6. Limpiar entry de prueba
cargo run -- erase [UUID generado]
```

**✅ Criterios de éxito:**

- Zero warnings en `cargo check`
- Compila sin errores
- Crea entry en SQLite correctamente
- Git integration funciona (si hay repo configurado)
- Time offset parsing (-0:15) funciona

---

### Después de crear `src/stats.rs`:

```bash
# 1. Verificar sintaxis
cargo check

# 2. Compilar
cargo build --lib

# 3. Test de estadísticas
cargo test stats

# 4. Generar entries de prueba
cargo run -- track --project "Project A" --task "Task 1"
sleep 10
cargo run -- finish
cargo run -- track --project "Project A" --task "Task 2"
sleep 15
cargo run -- finish

# 5. Verificar output con colores
cargo run -- list --total

# 6. Verificar stats por proyecto
cargo run -- stats --project "Project A"

# 7. Test de export
cargo run -- export --format json --output test_export.json
cat test_export.json | jq .
```

**✅ Criterios de éxito:**

- Colores Materia se muestran correctamente
- Totales calculados con precisión
- Export JSON válido y parseable
- Stats agregadas por proyecto/task correctas

---

### Después de crear `src/ui.rs`:

```bash
# 1. Verificar sintaxis
cargo check

# 2. Verificar dependencias Ratatui
cargo tree | grep ratatui

# 3. Compilar con feature TUI
cargo build --features tui

# 4. Lanzar dashboard interactivo
cargo run -- dashboard

# 5. Test de temas
cargo run -- dashboard --theme fire
cargo run -- dashboard --theme ice
cargo run -- dashboard --theme lightning

# 6. Verificar shortcuts de teclado
# (dentro del dashboard: q=quit, t=track, f=finish, l=list)
```

**✅ Criterios de éxito:**

- TUI se renderiza sin glitches
- Nerd Font icons visibles (💎⚔️✨🏆⭐)
- Temas cambian colores correctamente
- Keyboard navigation responsive
- Exit limpio sin panic

---

## 🔍 VALIDACIÓN COMPLETA POST-PHASE 2:

```bash
# Full compilation test
cargo build --release

# Ejecutar test suite completa
cargo test --all

# Verificar size del binario
ls -lh target/release/materiatrack

# Test de integración end-to-end
./target/release/materiatrack track --project "Integration" --task "E2E Test"
sleep 5
./target/release/materiatrack finish
./target/release/materiatrack list --total
./target/release/materiatrack stats

# Benchmark performance (opcional)
cargo bench
```

**✅ Criterios finales Phase 2:**

- Binario < 10MB (optimizado)
- Zero panics en usage normal
- All tests passing
- Memory leaks = 0 (valgrind/heaptrack)
- CLI responsivo < 100ms startup

---

## 📝 LOGGING DE VALIDACIÓN

Crea este archivo: `validation_log.md`

```markdown
# MatteriaTrack Phase 2 Validation Log

## src/tracking.rs

- [ ] cargo check: PASS/FAIL
- [ ] cargo build: PASS/FAIL
- [ ] track command: PASS/FAIL
- [ ] finish command: PASS/FAIL
- [ ] git integration: PASS/FAIL
- Notes: \_\_\_

## src/stats.rs

- [ ] cargo check: PASS/FAIL
- [ ] list --total: PASS/FAIL
- [ ] export JSON: PASS/FAIL
- [ ] color themes: PASS/FAIL
- Notes: \_\_\_

## src/ui.rs

- [ ] TUI launch: PASS/FAIL
- [ ] theme switching: PASS/FAIL
- [ ] keyboard nav: PASS/FAIL
- [ ] icons render: PASS/FAIL
- Notes: \_\_\_

## Integration

- [ ] Full compile: PASS/FAIL
- [ ] E2E test: PASS/FAIL
- [ ] Performance: \_\_\_ms
- [ ] Binary size: \_\_\_MB
```

---

**Usa este checklist después de cada archivo generado por Codex CLI para asegurar calidad producción-ready antes de avanzar al siguiente.**
