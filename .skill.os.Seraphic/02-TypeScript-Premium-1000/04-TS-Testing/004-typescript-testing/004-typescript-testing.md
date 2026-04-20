# SKILL TS-004: TYPESCRIPT TESTING

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        TYPESCRIPT TESTING
                     Jest, Vitest, Testing Library
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Testing in TypeScript with Jest and React Testing Library.

---

## JEST

### 1.1 Typed Tests

```typescript
describe('Calculator', () => {
    it('should add two numbers', () => {
        const result = add(2, 3);
        expect(result).toBe(5);
    });

    it('should multiply correctly', () => {
        expect(multiply(4, 5)).toBe(20);
    });
});

describe('UserService', () => {
    it('should create user', async () => {
        const user = await userService.create({
            email: 'test@example.com',
            name: 'Test User',
        });

        expect(user.id).toBeDefined();
        expect(user.email).toBe('test@example.com');
    });
});
```

---

## REACT TESTING

### 2.1 Component Tests

```typescript
import { render, screen, fireEvent } from '@testing-library/react';

describe('Button', () => {
    it('renders correctly', () => {
        render(<Button label="Click me" onClick={jest.fn()} />);
        expect(screen.getByText('Click me')).toBeInTheDocument();
    });

    it('handles click', () => {
        const onClick = jest.fn();
        render(<Button label="Click me" onClick={onClick} />);

        fireEvent.click(screen.getByText('Click me'));
        expect(onClick).toHaveBeenCalledTimes(1);
    });
});
```

---

## MOCKING

### 3.1 Module Mocking

```typescript
jest.mock('./api', () => ({
    fetchUsers: jest.fn().mockResolvedValue([
        { id: '1', name: 'User 1' },
    ]),
}));

jest.mock('react-router', () => ({
    useNavigate: () => jest.fn(),
}));
```

---

*Skill TS-004 | Category: TypeScript | Complexity: Expert*