# API Specification - Ordes

## Base URL
```
https://api.ordes.dev/v1
```

## Authentication

Todos los endpoints (excepto login y register) requieren header:
```
Authorization: Bearer <jwt_token>
```

---

## Auth Endpoints

### POST /auth/register
Crear nueva cuenta de usuario

**Request:**
```json
{
  "email": "user@example.com",
  "name": "John Doe",
  "password": "securePassword123"
}
```

**Response (201):**
```json
{
  "id": "user_123",
  "email": "user@example.com",
  "name": "John Doe",
  "token": "eyJhbGc...",
  "refreshToken": "refresh_..."
}
```

### POST /auth/login
Autenticar usuario

**Request:**
```json
{
  "email": "user@example.com",
  "password": "securePassword123"
}
```

**Response (200):**
```json
{
  "token": "eyJhbGc...",
  "refreshToken": "refresh_...",
  "expiresIn": 3600
}
```

### POST /auth/refresh
Refrescar token JWT

**Request:**
```json
{
  "refreshToken": "refresh_..."
}
```

**Response (200):**
```json
{
  "token": "eyJhbGc...",
  "expiresIn": 3600
}
```

---

## Projects Endpoints

### GET /projects
Listar proyectos del usuario

**Query Params:**
- `teamId` (opcional) - Filtrar por equipo
- `page` - Página (default: 1)
- `limit` - Items por página (default: 20, max: 100)

**Response (200):**
```json
{
  "data": [
    {
      "id": "proj_123",
      "name": "CubeSat V1",
      "description": "Mi primer satélite",
      "teamId": "team_456",
      "createdAt": "2026-08-12T10:30:00Z",
      "updatedAt": "2026-08-12T15:45:00Z",
      "currentVersion": "v1.2.3"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 5
  }
}
```

### POST /projects
Crear nuevo proyecto

**Request:**
```json
{
  "name": "CubeSat V1",
  "description": "Mi primer satélite",
  "teamId": "team_456" (opcional)
}
```

**Response (201):**
```json
{
  "id": "proj_123",
  "name": "CubeSat V1",
  "description": "Mi primer satélite",
  "teamId": "team_456",
  "createdAt": "2026-08-12T10:30:00Z"
}
```

### GET /projects/:id
Obtener detalles de un proyecto

**Response (200):**
```json
{
  "id": "proj_123",
  "name": "CubeSat V1",
  "description": "Mi primer satélite",
  "teamId": "team_456",
  "satellite": {
    "id": "sat_789",
    "components": [...]
  },
  "currentVersion": "v1.2.3",
  "createdAt": "2026-08-12T10:30:00Z",
  "updatedAt": "2026-08-12T15:45:00Z"
}
```

### PATCH /projects/:id
Actualizar proyecto

**Request:**
```json
{
  "name": "CubeSat V2",
  "description": "Versión mejorada"
}
```

**Response (200):**
```json
{
  "id": "proj_123",
  "name": "CubeSat V2",
  "description": "Versión mejorada",
  "updatedAt": "2026-08-12T16:00:00Z"
}
```

### DELETE /projects/:id
Eliminar proyecto

**Response (204):** Sin contenido

---

## Versions Endpoints

### GET /projects/:id/versions
Listar versiones de un proyecto

**Response (200):**
```json
{
  "data": [
    {
      "id": "v1.2.3",
      "projectId": "proj_123",
      "author": "user_123",
      "message": "Añadido panel solar",
      "createdAt": "2026-08-12T15:45:00Z",
      "parentVersion": "v1.2.2"
    }
  ]
}
```

### POST /projects/:id/versions
Crear nueva versión (commit)

**Request:**
```json
{
  "message": "Añadido panel solar",
  "satellite": {
    "components": [...]
  }
}
```

**Response (201):**
```json
{
  "id": "v1.2.3",
  "projectId": "proj_123",
  "message": "Añadido panel solar",
  "author": "user_123",
  "createdAt": "2026-08-12T15:45:00Z"
}
```

### GET /projects/:id/versions/:versionId
Obtener detalles de una versión

**Response (200):**
```json
{
  "id": "v1.2.3",
  "projectId": "proj_123",
  "satellite": {
    "id": "sat_789",
    "components": [...]
  },
  "author": "user_123",
  "message": "Añadido panel solar",
  "createdAt": "2026-08-12T15:45:00Z",
  "diff": {
    "added": [...],
    "removed": [...],
    "modified": [...]
  }
}
```

---

## Components Endpoints

### GET /components
Listar componentes disponibles

**Query Params:**
- `type` - Filtrar por tipo (sensor, processor, battery, solar_panel, etc.)
- `search` - Búsqueda por nombre
- `libraryId` - Filtrar por librería

**Response (200):**
```json
{
  "data": [
    {
      "id": "comp_001",
      "name": "STM32F4 MCU",
      "type": "processor",
      "specs": {
        "mass": 0.5,
        "power": 0.8,
        "dimensions": {
          "width": 10,
          "height": 10,
          "depth": 2
        }
      },
      "createdBy": "user_123",
      "isPublic": true
    }
  ]
}
```

### POST /components
Crear nuevo componente

**Request:**
```json
{
  "name": "STM32F4 MCU",
  "type": "processor",
  "specs": {
    "mass": 0.5,
    "power": 0.8,
    "dimensions": {
      "width": 10,
      "height": 10,
      "depth": 2
    }
  },
  "libraryId": "lib_123",
  "isPublic": false
}
```

**Response (201):**
```json
{
  "id": "comp_001",
  "name": "STM32F4 MCU",
  "type": "processor",
  "createdBy": "user_123"
}
```

---

## Simulation Endpoints

### POST /projects/:id/simulate
Ejecutar simulación

**Request:**
```json
{
  "versionId": "v1.2.3",
  "simulationType": "orbital",
  "config": {
    "altitude": 500,
    "inclination": 45,
    "duration": 86400
  }
}
```

**Response (202):**
```json
{
  "id": "sim_123",
  "projectId": "proj_123",
  "status": "running",
  "createdAt": "2026-08-12T16:00:00Z"
}
```

### GET /simulations/:id
Obtener resultados de simulación

**Response (200):**
```json
{
  "id": "sim_123",
  "projectId": "proj_123",
  "status": "completed",
  "results": {
    "orbital": {
      "apogee": 550,
      "perigee": 450,
      "period": 94.5
    },
    "thermal": {
      "maxTemp": 85,
      "minTemp": -40
    },
    "power": {
      "avgConsumption": 15,
      "maxConsumption": 25
    }
  },
  "createdAt": "2026-08-12T16:00:00Z",
  "completedAt": "2026-08-12T16:05:30Z"
}
```

---

## Teams Endpoints

### GET /teams
Listar equipos del usuario

**Response (200):**
```json
{
  "data": [
    {
      "id": "team_456",
      "name": "Space Innovators",
      "ownerId": "user_123",
      "members": [
        {
          "userId": "user_123",
          "role": "owner"
        },
        {
          "userId": "user_456",
          "role": "member"
        }
      ],
      "createdAt": "2026-08-01T10:00:00Z"
    }
  ]
}
```

### POST /teams
Crear equipo

**Request:**
```json
{
  "name": "Space Innovators"
}
```

**Response (201):**
```json
{
  "id": "team_456",
  "name": "Space Innovators",
  "ownerId": "user_123"
}
```

### POST /teams/:id/invite
Invitar miembro al equipo

**Request:**
```json
{
  "email": "newmember@example.com",
  "role": "member"
}
```

**Response (200):**
```json
{
  "inviteToken": "invite_...",
  "expiresIn": 604800
}
```

---

## Error Responses

### 400 Bad Request
```json
{
  "error": "VALIDATION_ERROR",
  "message": "El campo 'name' es requerido",
  "details": {
    "field": "name",
    "reason": "required"
  }
}
```

### 401 Unauthorized
```json
{
  "error": "UNAUTHORIZED",
  "message": "Token inválido o expirado"
}
```

### 403 Forbidden
```json
{
  "error": "FORBIDDEN",
  "message": "No tienes permiso para acceder a este recurso"
}
```

### 404 Not Found
```json
{
  "error": "NOT_FOUND",
  "message": "El proyecto no existe"
}
```

### 500 Internal Server Error
```json
{
  "error": "INTERNAL_ERROR",
  "message": "Ocurrió un error interno"
}
```

---

## WebSocket Events (Real-time Collaboration)

### Connect
```
ws://api.ordes.dev/ws?token=<jwt_token>&projectId=proj_123
```

### Events

**project:updated**
```json
{
  "type": "project:updated",
  "projectId": "proj_123",
  "changes": {
    "satellite": {
      "components": [...]
    }
  },
  "author": "user_456",
  "timestamp": "2026-08-12T16:05:00Z"
}
```

**component:added**
```json
{
  "type": "component:added",
  "projectId": "proj_123",
  "component": {...},
  "author": "user_456"
}
```

**cursor:moved**
```json
{
  "type": "cursor:moved",
  "projectId": "proj_123",
  "userId": "user_456",
  "position": {
    "x": 100,
    "y": 200
  }
}
```

---

## Rate Limiting

- **Limite general:** 1000 requests/hora
- **Simulaciones:** 10 ejecuciones/hora
- **Uploads:** 100 MB/hora

Respuesta cuando se excede:
```json
{
  "error": "RATE_LIMIT_EXCEEDED",
  "retryAfter": 3600
}
```

---

**Documentación actualizada:** 2026-08-12
