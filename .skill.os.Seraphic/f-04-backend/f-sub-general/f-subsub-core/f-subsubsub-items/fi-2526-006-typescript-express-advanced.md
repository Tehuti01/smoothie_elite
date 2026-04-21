---
id: fi-2526-006-typescript-express-advanced.md
category: f-04-backend
---

# SKILL TS-006: TYPESCRIPT EXPRESS ADVANCED

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        EXPRESS ADVANCED
                     Authentication, Middleware, Real-time
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Advanced Express.js patterns including JWT auth, WebSockets, and middleware.

---

## JWT AUTHENTICATION

### 1.1 Auth Middleware

```typescript
import jwt from 'jsonwebtoken';
import { Request, Response, NextFunction } from 'express';

interface AuthRequest extends Request {
    user?: { id: string; email: string };
}

export const authenticate = (
    req: AuthRequest,
    res: Response,
    next: NextFunction
) => {
    const token = req.headers.authorization?.split(' ')[1];
    
    if (!token) {
        return res.status(401).json({ error: 'No token provided' });
    }
    
    try {
        const decoded = jwt.verify(token, process.env.JWT_SECRET!);
        req.user = decoded as { id: string; email: string };
        next();
    } catch (error) {
        return res.status(401).json({ error: 'Invalid token' });
    }
};
```

---

## WEBSOCKETS

### 2.1 Socket.io

```typescript
import { Server as SocketIO } from 'socket.io';

interface ServerToClientEvents {
    'chat:message': (data: ChatMessage) => void;
    'user:joined': (data: { userId: string }) => void;
}

interface ClientToServerEvents {
    'chat:send': (data: { message: string }) => void;
}

export function setupSocketIO(httpServer: http.Server) {
    const io = new SocketIO<ClientToServerEvents, ServerToClientEvents>(httpServer);
    
    io.on('connection', (socket) => {
        console.log('User connected:', socket.id);
        
        socket.on('chat:send', (data) => {
            io.emit('chat:message', {
                id: crypto.randomUUID(),
                message: data.message,
                senderId: socket.id,
            });
        });
    });
    
    return io;
}
```

---

## MIDDLEWARE

### 3.1 Rate Limiter

```typescript
import rateLimit from 'express-rate-limit';

export const apiLimiter = rateLimit({
    windowMs: 15 * 60 * 1000, // 15 minutes
    max: 100, // limit each IP to 100 requests per windowMs
    message: 'Too many requests',
});

export const authLimiter = rateLimit({
    windowMs: 15 * 60 * 1000,
    max: 5,
    message: 'Too many login attempts',
});
```

---

*Skill TS-006 | Category: TypeScript | Complexity: Expert*