# SKILL TS-005: TYPESCRIPT ADVANCED PATTERNS

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        TYPESCRIPT ADVANCED PATTERNS
                     Design Patterns in TypeScript
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Advanced TypeScript design patterns including factory, singleton, observer, and more.

---

## FACTORY PATTERN

### 1.1 Generic Factory

```typescript
interface Product {
    name: string;
    price: number;
}

interface ProductFactory<T extends Product> {
    create(data: Partial<T>): T;
}

class UserFactory implements ProductFactory<User> {
    create(data: Partial<User>): User {
        return {
            id: data.id ?? crypto.randomUUID(),
            name: data.name ?? 'Unknown',
            email: data.email ?? 'unknown@example.com',
            createdAt: data.createdAt ?? new Date(),
        };
    }
}
```

---

## OBSERVER PATTERN

### 2.1 Event Emitter

```typescript
type EventMap = Record<string, any>;
type EventKey<T extends EventMap> = string & keyof T;
type EventReceiver<T> = (params: T) => void;

interface Emitter<T extends EventMap> {
    on<K extends EventKey<T>>(eventName: K, fn: EventReceiver<T[K]>): void;
    off<K extends EventKey<T>>(eventName: K, fn: EventReceiver<T[K]>): void;
    emit<K extends EventKey<T>>(eventName: K, params: T[K]): void;
}

function createEmitter<T extends EventMap>(): Emitter<T> {
    const listeners: { [K in keyof T]?: Array<EventReceiver<T[K]>> } = {};
    
    return {
        on<K extends EventKey<T>>(eventName: K, fn: EventReceiver<T[K]>) {
            listeners[eventName] = (listeners[eventName] || []).concat(fn);
        },
        off<K extends EventKey<T>>(eventName: K, fn: EventReceiver<T[K]>) {
            listeners[eventName] = (listeners[eventName] || []).filter(f => f !== fn);
        },
        emit<K extends EventKey<T>>(eventName: K, params: T[K]) {
            (listeners[eventName] || []).forEach(fn => fn(params));
        },
    };
}
```

---

## SINGLETON PATTERN

### 3.1 Database Connection

```typescript
class DatabaseConnection {
    private static instance: DatabaseConnection;
    private connection: any;

    private constructor() {}

    static getInstance(): DatabaseConnection {
        if (!DatabaseConnection.instance) {
            DatabaseConnection.instance = new DatabaseConnection();
        }
        return DatabaseConnection.instance;
    }

    connect(): void {
        console.log('Connected to database');
    }
}
```

---

## DECORATOR PATTERN

### 4.1 Logging Decorator

```typescript
function logged<T extends (...args: any[]) => any>(
    fn: T,
    context: ClassMethodDecoratorContext
) {
    return function(this: any, ...args: Parameters<T>): ReturnType<T> {
        console.log(`Calling ${context.name} with`, args);
        const result = fn.apply(this, args);
        console.log(`Result:`, result);
        return result;
    };
}

class Calculator {
    @logged
    add(a: number, b: number): number {
        return a + b;
    }
}
```

---

*Skill TS-005 | Category: TypeScript | Complexity: Expert*