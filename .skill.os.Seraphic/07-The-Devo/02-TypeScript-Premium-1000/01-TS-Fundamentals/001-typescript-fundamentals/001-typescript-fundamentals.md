# SKILL TS-001: TYPESCRIPT FUNDAMENTALS

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        TYPESCRIPT FUNDAMENTALS
                     Type System Mastery
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive TypeScript mastery including advanced type system,
generics, conditional types, mapped types, and utility types.

---

## ADVANCED TYPES

### 1.1 Conditional Types

```typescript
type IsString<T> = T extends string ? true : false;

type A = IsString<string>;  // true
type B = IsString<number>;  // false

type ReturnType<T> = T extends (...args: any[]) => infer R ? R : never;
```

### 1.2 Mapped Types

```typescript
type Readonly<T> = {
    readonly [P in keyof T]: T[P];
};

type Nullable<T> = {
    [P in keyof T]: T[P] | null;
};

type Optional<T> = {
    [P in keyof T]?: T[P];
};
```

### 1.3 Template Literal Types

```typescript
type EventName = `on${Capitalize<string>}`;
type CSSProperty = `${string}-${string}`;
```

---

## GENERICS

### 2.1 Advanced Generics

```typescript
type Flatten<T extends any[]> = T[number];

type Result = Flatten<[string, number]>;  // string | number
```

---

## UTILITY TYPES

### 3.1 Custom Utilities

```typescript
type Nullable<T> = T | null;
type NonNullable<T> = T extends null | undefined ? never : T;
type Return<T> = T extends (...args: any[]) => infer R ? R : never;
```

---

*Skill TS-001 | Category: TypeScript | Complexity: Expert*