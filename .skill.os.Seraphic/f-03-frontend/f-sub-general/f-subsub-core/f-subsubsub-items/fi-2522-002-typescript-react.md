---
id: fi-2522-002-typescript-react.md
category: f-03-frontend
---

# SKILL TS-002: TYPESCRIPT & REACT

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        TYPESCRIPT & REACT
                     Component Patterns & Hooks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive React with TypeScript including component patterns, hooks,
state management, and best practices.

---

## COMPONENT PATTERNS

### 1.1 Function Components

```typescript
interface ButtonProps {
    label: string;
    onClick: () => void;
    variant?: 'primary' | 'secondary';
    disabled?: boolean;
}

const Button: React.FC<ButtonProps> = ({ 
    label, 
    onClick, 
    variant = 'primary',
    disabled = false 
}) => (
    <button 
        className={variant}
        onClick={onClick}
        disabled={disabled}
    >
        {label}
    </button>
);
```

### 1.2 Generic Components

```typescript
interface ListProps<T> {
    items: T[];
    renderItem: (item: T) => React.ReactNode;
    keyExtractor: (item: T) => string;
}

function List<T>({ items, renderItem, keyExtractor }: ListProps<T>) {
    return (
        <ul>
            {items.map(item => (
                <li key={keyExtractor(item)}>
                    {renderItem(item)}
                </li>
            ))}
        </ul>
    );
}

// Usage
<List<User>
    items={users}
    renderItem={user => <span>{user.name}</span>}
    keyExtractor={user => user.id}
/>
```

---

## HOOKS

### 2.1 Custom Hooks

```typescript
function useDebounce<T>(value: T, delay: number): T {
    const [debounced, setDebounced] = useState(value);
    
    useEffect(() => {
        const timer = setTimeout(() => setDebounced(value), delay);
        return () => clearTimeout(timer);
    }, [value, delay]);
    
    return debounced;
}

function useLocalStorage<T>(key: string, initialValue: T): [T, (value: T) => void] {
    const [stored, setStored] = useState<T>(() => {
        const item = localStorage.getItem(key);
        return item ? JSON.parse(item) : initialValue;
    });
    
    const setValue = (value: T) => {
        setStored(value);
        localStorage.setItem(key, JSON.stringify(value));
    };
    
    return [stored, setValue];
}
```

---

## STATE MANAGEMENT

### 3.1 Context with TypeScript

```typescript
interface AuthState {
    user: User | null;
    isAuthenticated: boolean;
}

interface AuthContextType {
    state: AuthState;
    login: (credentials: Credentials) => Promise<void>;
    logout: () => void;
}

const AuthContext = createContext<AuthContextType | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
    const [state, setState] = useState<AuthState>({
        user: null,
        isAuthenticated: false,
    });
    
    const login = async (credentials: Credentials) => {
        const user = await authApi.login(credentials);
        setState({ user, isAuthenticated: true });
    };
    
    return (
        <AuthContext.Provider value={{ state, login, logout }}>
            {children}
        </AuthContext.Provider>
    );
}
```

---

## RECAP

1. **Type props** - Always define interfaces
2. **Generic components** - Reusable patterns
3. **Custom hooks** - Extract logic
4. **Context typed** - Type-safe state

---

*Skill TS-002 | Category: TypeScript | Complexity: Expert*