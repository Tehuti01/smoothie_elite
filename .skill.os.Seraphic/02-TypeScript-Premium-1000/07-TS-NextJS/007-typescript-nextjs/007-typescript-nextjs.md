# SKILL TS-007: NEXT.JS & FULL-STACK

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        NEXT.JS & FULL-STACK TYPESCRIPT
                     SSR, API Routes, Server Actions
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Full-stack Next.js with TypeScript including App Router, Server Actions, and streaming.

---

## APP ROUTER

### 1.1 Page Types

```typescript
// app/page.tsx
export default function HomePage() {
    return (
        <main>
            <h1>Welcome</h1>
        </main>
    );
}

// app/users/[id]/page.tsx
interface PageProps {
    params: { id: string };
    searchParams: { [key: string]: string };
}

export default async function UserPage({ params, searchParams }: PageProps) {
    const user = await getUser(params.id);
    
    return (
        <div>
            <h1>{user.name}</h1>
            <p>Query: {searchParams.tab}</p>
        </div>
    );
}
```

---

## SERVER ACTIONS

### 2.1 Typed Actions

```typescript
'use server'

import { z } from 'zod';

const schema = z.object({
    name: z.string().min(1),
    email: z.string().email(),
});

export async function createUser(formData: FormData) {
    const data = {
        name: formData.get('name'),
        email: formData.get('email'),
    };
    
    const validated = schema.parse(data);
    
    await db.user.create({
        data: validated,
    });
    
    revalidatePath('/users');
}

export async function deleteUser(id: string) {
    await db.user.delete({
        where: { id },
    });
    
    revalidatePath('/users');
}
```

---

## STREAMING

### 3.1 Suspense Streaming

```typescript
import { Suspense } from 'react';

export default function Page() {
    return (
        <main>
            <Suspense fallback={<Skeleton />}>
                <UserList />
            </Suspense>
            
            <Suspense fallback={<PostSkeleton />}>
                <UserPosts />
            </Suspense>
        </main>
    );
}

// Streaming with loading.tsx
export default function UserLoading() {
    return (
        <div className="animate-pulse">
            <div className="h-4 bg-gray-200 rounded w-3/4"></div>
        </div>
    );
}
```

---

## SERVER COMPONENTS

### 4.1 Data Fetching

```typescript
async function getData(): Promise<Data> {
    const res = await fetch('https://api.example.com/data', {
        next: { revalidate: 60 }, // ISR: Revalidate every 60 seconds
    });
    
    if (!res.ok) {
        throw new Error('Failed to fetch');
    }
    
    return res.json();
}

export default async function Page() {
    const data = await getData();
    
    return (
        <ul>
            {data.items.map((item) => (
                <li key={item.id}>{item.name}</li>
            ))}
        </ul>
    );
}
```

---

*Skill TS-007 | Category: TypeScript | Complexity: Expert*