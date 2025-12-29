# API Documentation

## Endpoints

### Health Check

```
GET /health
```

Returns the health status of the API server.

**Response:**
```json
{
  "status": "ok",
  "timestamp": "1234567890"
}
```

### Get Users

```
GET /api/users
```

Returns a list of all users.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "1",
      "name": "John Doe",
      "email": "john@example.com"
    }
  ]
}
```

### Get User by ID

```
GET /api/users/:id
```

Returns a specific user by ID.

### Create User

```
POST /api/users
```

Creates a new user.

**Request Body:**
```json
{
  "name": "Jane Doe",
  "email": "jane@example.com"
}
```

