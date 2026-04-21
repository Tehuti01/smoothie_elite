---
id: fi-2521-003-typescript-nodejs.md
category: f-04-backend
---

# SKILL TS-003: TYPESCRIPT & NODE.JS

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        TYPESCRIPT & NODE.JS
                     Server-Side TypeScript
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive Node.js with TypeScript including Express, APIs, databases, and real-time systems.

---

## EXPRESS

### 1.1 Typed Express

```typescript
import express, { Request, Response, NextFunction } from 'express';

interface UserRequest extends Request {
    user?: User;
    authToken?: string;
}

interface ApiError extends Error {
    status?: number;
    code?: string;
}

const app = express();

app.use('/api', (req: UserRequest, res: Response, next: NextFunction) => {
    const token = req.headers.authorization;
    if (token) {
        req.authToken = token;
    }
    next();
});

app.get('/api/users', async (req: Request, res: Response) => {
    const users = await getUsers();
    res.json(users);
});

app.post('/api/users', async (req: Request, res: Response) => {
    const user = await createUser(req.body);
    res.status(201).json(user);
});
```

---

## DATABASE

### 2.1 TypeScript Prisma

```typescript
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

interface CreateUserInput {
    email: string;
    name: string;
}

async function createUser(data: CreateUserInput): Promise<User> {
    return prisma.user.create({
        data: {
            email: data.email,
            name: data.name,
        },
    });
}

async function getUserById(id: string): Promise<User | null> {
    return prisma.user.findUnique({
        where: { id },
    });
}
```

---

## RECAP

1. **Typed requests** - Extend Request interface
2. **Prisma for DB** - Type-safe queries
3. **Middleware** - Request augmentation

---

*Skill TS-003 | Category: TypeScript | Complexity: Expert*