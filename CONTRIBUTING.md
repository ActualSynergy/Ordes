# Contributing to Ordes

¡Gracias por tu interés en contribuir a Ordes! Este documento te guía en cómo participar en el proyecto.

## Cómo Empezar

### 1. Fork y Clone
```bash
git clone https://github.com/ActualSynergy/Ordes.git
cd Ordes
```

### 2. Crear rama para tu feature
```bash
git checkout -b feature/nombre-descriptivo
```

### 3. Hacer cambios
- Sigue el estilo de código del proyecto
- Escribe tests para nuevas funcionalidades
- Actualiza la documentación si es necesario

### 4. Commit
```bash
git commit -m "tipo: descripción"
```

**Tipos de commit:**
- `feat:` Nueva funcionalidad
- `fix:` Corrección de bug
- `docs:` Cambios en documentación
- `refactor:` Refactorización sin cambios funcionales
- `test:` Adición o cambios en tests
- `chore:` Cambios en herramientas, dependencias, etc.

### 5. Push y Pull Request
```bash
git push origin feature/nombre-descriptivo
```

Abre un PR con descripción clara de los cambios.

## Guías de Desarrollo

### Backend (Rust)
- Usa `cargo fmt` para formatear
- Usa `cargo clippy` para linting
- Escribe tests con `#[test]`

### Frontend (TypeScript/React)
- Usa `eslint` y `prettier`
- Componentes funcionales con hooks
- Tests con Vitest o Jest

### Documentación
- Markdown con máximo 80 caracteres por línea
- Actualiza `docs/` si modificas features
- Mantén READMEs actualizados

## Reportar Issues

Abre un issue con:
- Descripción clara del problema
- Pasos para reproducir
- Comportamiento esperado vs actual
- Tu entorno (OS, versión, etc.)

## Preguntas?

Abre una discusión o contacta al equipo. ¡Bienvenido! 🚀
