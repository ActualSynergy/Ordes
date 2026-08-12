# Arquitectura Técnica de Ordes

## Principios de Diseño

1. **Modularidad:** Cada módulo es independiente y reutilizable
2. **Escalabilidad:** Soportar desde startups hasta grandes agencias
3. **Performance:** Cálculos rápidos y renderizado fluido
4. **Seguridad:** Autenticación, autorización y validación
5. **Extensibilidad:** Fácil agregar nuevos componentes y simulaciones

## Arquitectura de Capas

### 1. Presentación (Frontend)

**Tecnología:** React + TypeScript

**Módulos:**
- `ui/` - Componentes reutilizables (botones, modales, etc.)
- `pages/` - Páginas principales (dashboard, editor, proyectos)
- `3d/` - Renderizado 3D (Three.js/Babylon.js)
- `editor/` - Editor visual de satélites
- `collaboration/` - UI para versionado y colaboración

**Características:**
- SPA (Single Page Application)
- WebSockets para colaboración real-time
- Local storage para cache

### 2. API Gateway (Backend)

**Tecnología:** Rust (Actix-web / Axum) + Go (opcional para microservicios)

**Endpoints principales:**
```
POST   /auth/login
POST   /auth/register
POST   /auth/refresh

GET    /projects
POST   /projects
GET    /projects/:id
PATCH  /projects/:id
DELETE /projects/:id

GET    /projects/:id/versions
POST   /projects/:id/versions
GET    /projects/:id/versions/:versionId

GET    /components
POST   /components
GET    /components/:id

POST   /projects/:id/simulate
GET    /simulations/:id/results

GET    /teams
POST   /teams/invite
```

### 3. Lógica de Negocio (Core)

**Ubicación:** `src/modules/`

**Módulos:**

#### `satellite/`
- Definiciones de satélites
- Estructuras de datos
- Validación de componentes

#### `components/`
- Gestión de librería de componentes
- Propiedades, especificaciones
- Cálculos de masa, volumen

#### `simulation/`
- Motor de física orbital
- Cálculos de potencia/térmica
- Validaciones de compatibilidad

#### `collaboration/`
- Sistema de versionado tipo Git
- Diffs y merges
- Conflictos y resolución

#### `rendering/`
- Exportación a formatos 3D
- Generación de modelos
- Métricas visuales

#### `shared/`
- Utilidades comunes
- Tipos y estructuras compartidas
- Funciones matemáticas

### 4. Persistencia

**Base de Datos:** PostgreSQL

**Tablas principales:**
```sql
-- Usuarios y autenticación
users (id, email, name, passwordHash, createdAt)
teams (id, name, ownerId)
team_members (teamId, userId, role)

-- Proyectos y versiones
projects (id, name, teamId, description, createdAt, updatedAt)
versions (id, projectId, satellite_data, author, createdAt)
version_tree (parentVersionId, childVersionId)

-- Componentes
components (id, name, type, specs, mass, power, createdBy)
component_libraries (id, name, ownerId, isPublic)

-- Simulaciones
simulations (id, projectId, versionId, config, results, status)
```

**Cache:** Redis
- Sesiones de usuario
- Proyectos recientes
- Resultados de simulaciones

### 5. Computación Intensiva

**Lenguaje:** Rust + C/C++ (compilado a WASM)

**Componentes:**
- `orbital_mechanics.rs` - Cálculos orbitales
- `thermal_analysis.cpp` - Análisis térmico
- `power_budget.rs` - Gestión de potencia
- `collision_detection.cpp` - Detección de colisiones

**Ejecución:**
- Rust: Actix-web o tareas asincrónicas
- WASM: En el navegador para cálculos ligeros

### 6. Desktop App (Tauri)

**Stack:** Rust backend + React frontend

**Ventajas:**
- Funciona offline
- Acceso a sistema de archivos
- Mejor performance que web pura
- Misma interfaz en todas las plataformas

## Flujo de Datos

```
Usuario → Frontend (React)
   ↓
WebSocket / REST API
   ↓
Backend (Rust/Go)
   ↓
Validación → Negocio → Persistencia
   ↓
PostgreSQL / Redis
```

## Módulos Acoplados

```
src/
├── modules/
│   ├── satellite/
│   │   ├── mod.rs
│   │   ├── domain.rs          # Entidades
│   │   ├── repository.rs      # Acceso a datos
│   │   └── service.rs         # Lógica de negocio
│   │
│   ├── components/
│   │   ├── mod.rs
│   │   ├── domain.rs
│   │   ├── repository.rs
│   │   └── service.rs
│   │
│   ├── simulation/
│   │   ├── mod.rs
│   │   ├── engine.rs          # Motor de simulación
│   │   ├── physics.rs         # Cálculos físicos
│   │   └── validator.rs       # Validación
│   │
│   ├── collaboration/
│   │   ├── mod.rs
│   │   ├── versioning.rs      # Git-like versionado
│   │   ├── diff.rs            # Cálculo de diffs
│   │   └── merge.rs           # Estrategia de merge
│   │
│   ├── rendering/
│   │   ├── mod.rs
│   │   ├── exporter.rs        # Exportación 3D
│   │   └── metrics.rs         # Cálculo de métricas
│   │
│   └── shared/
│       ├── mod.rs
│       ├── math.rs            # Funciones matemáticas
│       ├── types.rs           # Tipos comunes
│       └── utils.rs           # Utilidades
│
└── config/
    └── mod.rs                 # Configuración global
```

## Patrones de Comunicación

### Síncrona
- REST API para operaciones CRUD
- Validaciones inmediatas

### Asíncrona
- WebSockets para colaboración real-time
- Message queues para simulaciones largas (RabbitMQ/Kafka)
- Event sourcing para auditoría

## Seguridad

- **Autenticación:** JWT con refresh tokens
- **Autorización:** RBAC (Role-Based Access Control)
- **Validación:** Input sanitization y schema validation
- **Encriptación:** TLS/SSL para comunicaciones
- **Auditoría:** Event logging de todas las operaciones

## Deployment

**Desarrollo:**
```bash
docker-compose up  # Backend + DB + Cache
npm start          # Frontend
```

**Producción:**
- Backend: Kubernetes (Railway, Render, etc.)
- Frontend: Vercel, Netlify, o CDN estático
- DB: Managed PostgreSQL (AWS RDS, Heroku, etc.)
- Cache: Redis Cloud

---

**Próximos pasos:** Iniciar con Phase 1 (Fundamentos) implementando autenticación y CRUD básico de proyectos.
