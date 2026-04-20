# SKILL 009: MATHEMATICS & PHYSICS SIMULATION

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        MATHEMATICS & PHYSICS SIMULATION
                     Scientific Computing in Rust
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive mastery of mathematical modeling and physics simulation in Rust.
Covers numerical methods, differential equations, particle systems, rigid body dynamics,
fluid dynamics, and real-time simulation frameworks.

## TABLE OF CONTENTS

1. [Numerical Methods](#numerical-methods)
2. [Linear Algebra](#linear-algebra)
3. [Differential Equations](#differential-equations)
4. [Particle Systems](#particle-systems)
5. [Rigid Body Dynamics](#rigid-body-dynamics)
6. [Fluid Dynamics](#fluid-dynamics)
7. [Collision Detection](#collision-detection)
8. [Optimization](#optimization)
9. [Statistics & Probability](#statistics--probability)
10. [Simulation Frameworks](#simulation-frameworks)

---

## NUMERICAL METHODS

### 1.1 Root Finding

```rust
pub struct RootFinder;

impl RootFinder {
    pub fn bisection<F>(f: F, a: f64, b: f64, tol: f64, max_iter: usize) -> Option<f64>
    where
        F: Fn(f64) -> f64,
    {
        let mut lo = a;
        let mut hi = b;
        
        if f(lo) * f(hi) > 0.0 {
            return None;
        }
        
        for _ in 0..max_iter {
            let mid = (lo + hi) / 2.0;
            let f_mid = f(mid);
            
            if f_mid.abs() < tol || (hi - lo) / 2.0 < tol {
                return Some(mid);
            }
            
            if f(lo) * f_mid < 0.0 {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        
        Some((lo + hi) / 2.0)
    }

    pub fn newton<F, G>(f: F, df: G, x0: f64, tol: f64, max_iter: usize) -> Option<f64>
    where
        F: Fn(f64) -> f64,
        G: Fn(f64) -> f64,
    {
        let mut x = x0;
        
        for _ in 0..max_iter {
            let fx = f(x);
            let dfx = df(x);
            
            if dfx.abs() < 1e-10 {
                return None;
            }
            
            let x_new = x - fx / dfx;
            
            if (x_new - x).abs() < tol {
                return Some(x_new);
            }
            
            x = x_new;
        }
        
        None
    }

    pub fn secant<F>(f: F, x0: f64, x1: f64, tol: f64, max_iter: usize) -> Option<f64>
    where
        F: Fn(f64) -> f64,
    {
        let mut x_prev = x0;
        let mut x = x1;
        
        for _ in 0..max_iter {
            let fx = f(x);
            let fx_prev = f(x_prev);
            
            if (fx - fx_prev).abs() < 1e-10 {
                return None;
            }
            
            let x_new = x - fx * (x - x_prev) / (fx - fx_prev);
            
            if (x_new - x).abs() < tol {
                return Some(x_new);
            }
            
            x_prev = x;
            x = x_new;
        }
        
        Some(x)
    }
}
```

### 1.2 Numerical Integration

```rust
pub struct Integrator;

impl Integrator {
    pub fn trapezoid<F>(f: F, a: f64, b: f64, n: usize) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let h = (b - a) / n as f64;
        let mut sum = (f(a) + f(b)) / 2.0;
        
        for i in 1..n {
            sum += f(a + i as f64 * h);
        }
        
        sum * h
    }

    pub fn simpson<F>(f: F, a: f64, b: f64, n: usize) -> f64
    where
        F: Fn(f64) -> f64,
    {
        if n % 2 != 0 {
            return Self::simpson(f, a, b, n + 1);
        }
        
        let h = (b - a) / n as f64;
        let mut sum = f(a) + f(b);
        
        for i in 1..n {
            let x = a + i as f64 * h;
            if i % 2 == 0 {
                sum += 2.0 * f(x);
            } else {
                sum += 4.0 * f(x);
            }
        }
        
        sum * h / 3.0
    }

    pub fn romberg<F>(f: F, a: f64, b: f64, max_order: usize) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let mut table: Vec<Vec<f64>> = Vec::new();
        
        for k in 0..max_order {
            let n = 1 << k;
            let row: Vec<f64> = (0..=k)
                .map(|j| {
                    if j == 0 {
                        Self::trapezoid(f, a, b, n)
                    } else {
                        let h = j as f64;
                        4.0_f64.powf(h) / (4.0_f64.powf(h) - 1.0) * table[j-1][k]
                            - 1.0 / (4.0_f64.powf(h) - 1.0) * table[j-1][k-1]
                    }
                })
                .collect();
            table.push(row);
        }
        
        table[max_order-1][max_order-1]
    }

    pub fn gauss_legendre<F>(f: F, a: f64, b: f64, n: usize) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let (nodes, weights) = Self::gauss_nodes_weights(n);
        let midpoint = (a + b) / 2.0;
        let half_width = (b - a) / 2.0;
        
        nodes
            .iter()
            .zip(weights.iter())
            .map(|(&x, &w)| w * f(midpoint + half_width * x))
            .sum::<f64>() * half_width
    }

    fn gauss_nodes_weights(n: usize) -> (Vec<f64>, Vec<f64>) {
        let nodes = match n {
            1 => vec![0.0],
            2 => vec![-1.0_f64 / 3.0_f64.sqrt(), 1.0_f64 / 3.0_f64.sqrt()],
            3 => vec![0.0, -6.0_f64.sqrt() / 6.0, 6.0_f64.sqrt() / 6.0],
            _ => vec![],
        };
        
        let weights = match n {
            1 => vec![2.0],
            2 => vec![1.0, 1.0],
            3 => vec![8.0 / 9.0, 5.0 / 9.0, 5.0 / 9.0],
            _ => vec![],
        };
        
        (nodes, weights)
    }
}
```

---

## LINEAR ALGEBRA

### 2.1 Matrix Operations

```rust
use std::ops::{Add, Sub, Mul};

pub struct Matrix {
    pub data: Vec<f64>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    pub fn identity(n: usize) -> Self {
        let mut m = Matrix::new(n, n);
        for i in 0..n {
            m.set(i, i, 1.0);
        }
        m
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    pub fn add(&self, other: &Matrix) -> Option<Matrix> {
        if self.rows != other.rows || self.cols != other.cols {
            return None;
        }
        
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] + other.data[i];
        }
        Some(result)
    }

    pub fn mul(&self, other: &Matrix) -> Option<Matrix> {
        if self.cols != other.rows {
            return None;
        }
        
        let mut result = Matrix::new(self.rows, other.cols);
        
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        
        Some(result)
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j));
            }
        }
        
        result
    }

    pub fn determinant(&self) -> f64 {
        if self.rows != self.cols {
            return 0.0;
        }
        
        let n = self.rows;
        
        if n == 1 {
            self.get(0, 0)
        } else if n == 2 {
            self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0)
        } else {
            // LU decomposition for larger matrices
            self.lu_decomposition().0.determinant()
        }
    }

    pub fn inverse(&self) -> Option<Matrix> {
        if self.rows != self.cols {
            return None;
        }
        
        let n = self.rows;
        let mut augmented = Matrix::new(n, 2 * n);
        
        // Create augmented matrix [A | I]
        for i in 0..n {
            for j in 0..n {
                augmented.set(i, j, self.get(i, j));
                augmented.set(i, j + n, if i == j { 1.0 } else { 0.0 });
            }
        }
        
        // Gauss-Jordan elimination
        for col in 0..n {
            // Find pivot
            let mut max_row = col;
            for row in (col + 1)..n {
                if augmented.get(row, col).abs() > augmented.get(max_row, col).abs() {
                    max_row = row;
                }
            }
            
            // Swap rows
            for j in 0..2 * n {
                let temp = augmented.get(col, j);
                augmented.set(col, j, augmented.get(max_row, j));
                augmented.set(max_row, j, temp);
            }
            
            // Normalize pivot row
            let pivot = augmented.get(col, col);
            if pivot.abs() < 1e-10 {
                return None;
            }
            
            for j in 0..2 * n {
                augmented.set(col, j, augmented.get(col, j) / pivot);
            }
            
            // Eliminate column
            for row in 0..n {
                if row != col {
                    let factor = augmented.get(row, col);
                    for j in 0..2 * n {
                        augmented.set(
                            row,
                            j,
                            augmented.get(row, j) - factor * augmented.get(col, j),
                        );
                    }
                }
            }
        }
        
        // Extract inverse
        let mut inverse = Matrix::new(n, n);
        for i in 0..n {
            for j in 0..n {
                inverse.set(i, j, augmented.get(i, j + n));
            }
        }
        
        Some(inverse)
    }

    fn lu_decomposition(&self) -> (Matrix, Matrix) {
        let n = self.rows;
        let mut l = Matrix::new(n, n);
        let mut u = Matrix::new(n, n);
        
        for i in 0..n {
            for k in i..n {
                let mut sum = 0.0;
                for j in 0..i {
                    sum += u.get(j, i) * l.get(k, j);
                }
                u.set(k, i, self.get(k, i) - sum);
            }
            
            for k in i..n {
                if i == k {
                    l.set(i, i, 1.0);
                } else {
                    let mut sum = 0.0;
                    for j in 0..i {
                        sum += u.get(j, i) * l.get(k, j);
                    }
                    if u.get(i, i).abs() < 1e-10 {
                        l.set(k, i, 0.0);
                    } else {
                        l.set(k, i, (self.get(k, i) - sum) / u.get(i, i));
                    }
                }
            }
        }
        
        (l, u)
    }
}
```

### 2.2 Eigenvalue Computation

```rust
impl Matrix {
    pub fn power_iteration(&self, num_iter: usize) -> f64 {
        let n = self.rows;
        let mut v = vec![1.0_f64 / (n as f64).sqrt(); n];
        
        for _ in 0..num_iter {
            let mut av = vec![0.0_f64; n];
            
            for i in 0..n {
                for j in 0..n {
                    av[i] += self.get(i, j) * v[j];
                }
            }
            
            let norm: f64 = av.iter().map(|x| x * x).sum::<f64>().sqrt();
            v = av.iter().map(|x| x / norm).collect();
        }
        
        // Rayleigh quotient
        let mut av = vec![0.0_f64; n];
        for i in 0..n {
            for j in 0..n {
                av[i] += self.get(i, j) * v[j];
            }
        }
        
        av.iter().zip(v.iter()).map(|(av_i, v_i)| av_i * v_i).sum::<f64>()
            / v.iter().map(|v_i| v_i * v_i).sum::<f64>()
    }

    pub fn jacobi_eigen(&self, max_iter: usize) -> (Matrix, Matrix) {
        let n = self.rows;
        let mut v = Matrix::identity(n);
        let mut a = self.clone();
        
        for _ in 0..max_iter {
            // Find largest off-diagonal element
            let mut max_val = 0.0;
            let mut p = 0;
            let mut q = 0;
            
            for i in 0..n {
                for j in (i + 1)..n {
                    if a.get(i, j).abs() > max_val {
                        max_val = a.get(i, j).abs();
                        p = i;
                        q = j;
                    }
                }
            }
            
            if max_val < 1e-10 {
                break;
            }
            
            // Compute rotation
            let mut theta = 0.0;
            let diff = a.get(q, q) - a.get(p, p);
            
            if diff.abs() < 1e-10 {
                theta = std::f64::consts::PI / 4.0;
            } else {
                theta = 0.5 * (a.get(p, q) / diff).atan();
            }
            
            // Apply rotation
            let c = theta.cos();
            let s = theta.sin();
            
            for i in 0..n {
                if i != p && i != q {
                    let api = a.get(i, p);
                    let aqi = a.get(i, q);
                    a.set(i, p, c * api - s * aqi);
                    a.set(p, i, a.get(i, p));
                    a.set(i, q, s * api + c * aqi);
                    a.set(q, i, a.get(i, q));
                }
            }
            
            let app = a.get(p, p);
            let aqq = a.get(q, q);
            let apq = a.get(p, q);
            
            a.set(p, p, c * c * app - 2.0 * s * c * apq + s * s * aqq);
            a.set(q, q, s * s * app + 2.0 * s * c * apq + c * c * aqq);
            a.set(p, q, 0.0);
            a.set(q, p, 0.0);
            
            // Update eigenvectors
            for i in 0..n {
                let vip = v.get(i, p);
                let viq = v.get(i, q);
                v.set(i, p, c * vip - s * viq);
                v.set(i, q, s * vip + c * viq);
            }
        }
        
        (a, v)
    }
}
```

---

## DIFFERENTIAL EQUATIONS

### 3.1 ODE Solvers

```rust
pub struct ODESolver;

pub type ODEFunction = fn(f64, &Vec<f64>) -> Vec<f64>;

impl ODESolver {
    pub fn euler<F>(f: F, y0: &Vec<f64>, t0: f64, h: f64, n: usize) -> Vec<Vec<f64>>
    where
        F: Fn(f64, &Vec<f64>) -> Vec<f64>,
    {
        let mut y = y0.clone();
        let mut t = t0;
        let mut results = vec![y.clone()];
        
        for _ in 0..n {
            let dy = f(t, &y);
            for i in 0..y.len() {
                y[i] += h * dy[i];
            }
            t += h;
            results.push(y.clone());
        }
        
        results
    }

    pub fn rk4<F>(f: F, y0: &Vec<f64>, t0: f64, h: f64, n: usize) -> Vec<Vec<f64>>
    where
        F: Fn(f64, &Vec<f64>) -> Vec<f64>,
    {
        let mut y = y0.clone();
        let mut t = t0;
        let mut results = vec![y.clone()];
        
        for _ in 0..n {
            let k1 = f(t, &y);
            
            let y2: Vec<f64> = y.iter().enumerate()
                .map(|(i, &yi)| yi + h / 2.0 * k1[i])
                .collect();
            let k2 = f(t + h / 2.0, &y2);
            
            let y3: Vec<f64> = y.iter().enumerate()
                .map(|(i, &yi)| yi + h / 2.0 * k2[i])
                .collect();
            let k3 = f(t + h / 2.0, &y3);
            
            let y4: Vec<f64> = y.iter().enumerate()
                .map(|(i, &yi)| yi + h * k3[i])
                .collect();
            let k4 = f(t + h, &y4);
            
            for i in 0..y.len() {
                y[i] += h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
            }
            
            t += h;
            results.push(y.clone());
        }
        
        results
    }

    pub fn rk45<F>(f: F, y0: &Vec<f64>, t0: f64, tf: f64, tol: f64) -> Vec<Vec<f64>>
    where
        F: Fn(f64, &Vec<f64>) -> Vec<f64>,
    {
        let mut y = y0.clone();
        let mut t = t0;
        let mut h = (tf - t0) / 100.0;
        let mut results = vec![y.clone()];
        
        while t < tf {
            if t + h > tf {
                h = tf - t;
            }
            
            // RK4 step
            let k1 = f(t, &y);
            let k2 = f(t + h / 2.0, &y.iter().enumerate().map(|(i, &yi)| yi + h / 2.0 * k1[i]).collect());
            let k3 = f(t + h / 2.0, &y.iter().enumerate().map(|(i, &yi)| yi + h / 2.0 * k2[i]).collect());
            let k4 = f(t + h, &y.iter().enumerate().map(|(i, &yi)| yi + h * k3[i]).collect());
            
            let y_new: Vec<f64> = y.iter().enumerate()
                .map(|(i, &yi)| yi + h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]))
                .collect();
            
            // Error estimation (simplified)
            let error: f64 = y_new.iter().zip(y.iter()).map(|(yn, y)| (yn - y).abs()).sum();
            
            if error < tol {
                y = y_new;
                t += h;
                results.push(y.clone());
                
                // Adjust step size
                h = (h * tol / (error + 1e-10)).min(h * 2.0).max(h / 2.0);
            } else {
                h /= 2.0;
            }
        }
        
        results
    }
}
```

### 3.2 PDE Solvers

```rust
pub struct PDESolver;

impl PDESolver {
    pub fn heat_equation_1d(
        u0: &[f64],
        alpha: f64,
        dx: f64,
        dt: f64,
        n_steps: usize,
    ) -> Vec<Vec<f64>> {
        let nx = u0.len();
        let r = alpha * dt / (dx * dx);
        let mut u = u0.to_vec();
        let mut results = vec![u.clone()];
        
        for _ in 0..n_steps {
            let u_next = u.clone();
            
            for i in 1..nx - 1 {
                u_next[i] = u[i] + r * (u[i - 1.0 - 2.0 * u[i] + u[i + 1.0]);
            }
            
            u = u_next;
            results.push(u.clone());
        }
        
        results
    }

    pub fn wave_equation_1d(
        u0: &[f64],
        v0: &[f64],
        c: f64,
        dx: f64,
        dt: f64,
        n_steps: usize,
    ) -> Vec<Vec<f64>> {
        let nx = u0.len();
        let r = c * dt / dx;
        let r2 = r * r;
        
        let mut u_prev = u0.to_vec();
        let mut u_curr = v0.iter().enumerate()
            .map(|(i, &vi)| u0[i] + dt * vi)
            .collect();
        
        let mut results = vec![u_prev.clone(), u_curr.clone()];
        
        for _ in 0..n_steps {
            let mut u_next = u_curr.clone();
            
            for i in 1..nx - 1 {
                u_next[i] = 2.0 * u_curr[i] - u_prev[i]
                    + r2 * (u_curr[i - 1] - 2.0 * u_curr[i] + u_curr[i + 1]);
            }
            
            u_prev = u_curr;
            u_curr = u_next;
            results.push(u_curr.clone());
        }
        
        results
    }
}
```

---

## PARTICLE SYSTEMS

### 4.1 Particle

```rust
#[derive(Clone, Copy)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: f64,
    pub radius: f64,
    pub lifetime: f64,
    pub age: f64,
    pub active: bool,
}

impl Particle {
    pub fn new(position: Vec3) -> Self {
        Particle {
            position,
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            mass: 1.0,
            radius: 0.1,
            lifetime: f64::INFINITY,
            age: 0.0,
            active: true,
        }
    }

    pub fn update(&mut self, dt: f64) {
        if !self.active { return; }
        
        self.velocity = self.velocity + self.acceleration * dt;
        self.position = self.position + self.velocity * dt;
        self.age += dt;
        
        if self.age >= self.lifetime {
            self.active = false;
        }
    }

    pub fn apply_force(&mut self, force: Vec3) {
        self.acceleration = self.acceleration + force / self.mass;
    }
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    capacity: usize,
}

impl ParticleSystem {
    pub fn new(capacity: usize) -> Self {
        ParticleSystem {
            particles: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn emit(&mut self, position: Vec3, velocity: Vec3, count: usize) {
        for _ in 0..count {
            if self.particles.len() < self.capacity {
                let mut p = Particle::new(position);
                p.velocity = velocity;
                self.particles.push(p);
            }
        }
    }

    pub fn update(&mut self, dt: f64, gravity: Vec3) {
        for p in &mut self.particles {
            p.apply_force(gravity * p.mass);
            p.update(dt);
        }
        
        self.particles.retain(|p| p.active);
    }

    pub fn render(&self, graphics: &mut Graphics) {
        for p in &self.particles {
            graphics.draw_sphere(p.position, p.radius);
        }
    }
}
```

---

## RIGID BODY DYNAMICS

### 5.1 Rigid Body

```rust
#[derive(Clone)]
pub struct RigidBody {
    pub position: Vec3,
    pub orientation: Quaternion,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub mass: f64,
    pub inertia: Mat3,
    pub inverse_inertia: Mat3,
    pub forces: Vec3,
    pub torques: Vec3,
}

impl RigidBody {
    pub fn new(mass: f64) -> Self {
        let inertia = Mat3::identity() * mass / 12.0;
        
        RigidBody {
            position: Vec3::zero(),
            orientation: Quaternion::identity(),
            velocity: Vec3::zero(),
            angular_velocity: Vec3::zero(),
            mass,
            inertia,
            inverse_inertia: inertia.inverse(),
            forces: Vec3::zero(),
            torques: Vec3::zero(),
        }
    }

    pub fn apply_force(&mut self, force: Vec3, point: Vec3) {
        self.forces = self.forces + force;
        self.torques = self.torques + (point - self.position).cross(force);
    }

    pub fn integrate(&mut self, dt: f64) {
        // Linear
        let acceleration = self.forces / self.mass;
        self.velocity = self.velocity + acceleration * dt;
        self.position = self.position + self.velocity * dt;
        
        // Angular
        let angular_momentum = self.inertia * self.angular_velocity;
        let torque = self.torques - self.angular_velocity.cross(angular_momentum);
        self.angular_velocity = self.inverse_inertia * angular_momentum;
        
        // Update orientation
        let q_dot = Quaternion::from_xyz(self.angular_velocity * dt / 2.0);
        self.orientation = (self.orientation + q_dot * self.orientation).normalize();
        
        // Reset forces
        self.forces = Vec3::zero();
        self.torques = Vec3::zero();
    }
}

/// Collision between rigid bodies
pub fn resolve_collision(a: &mut RigidBody, b: &mut RigidBody, normal: Vec3, penetration: f64) {
    let rel_vel = b.velocity - a.velocity;
    let vel_along_normal = rel_vel.dot(normal);
    
    if vel_along_normal > 0.0 {
        return;
    }
    
    let e = 0.8; // Restitution
    
    let j = -(1.0 + e) * vel_along_normal
        / (1.0 / a.mass + 1.0 / b.mass);
    
    let impulse = normal * j;
    
    a.velocity = a.velocity - impulse / a.mass;
    b.velocity = b.velocity + impulse / b.mass;
    
    // Positional correction
    let percent = 0.2;
    let slop = 0.01;
    let correction = normal * (penetration / (1.0 / a.mass + 1.0 / b.mass) * percent).max(0.0);
    
    a.position = a.position - correction * (1.0 / a.mass);
    b.position = b.position + correction * (1.0 / b.mass);
}
```

---

## COLLISION DETECTION

### 7.1 Collision Shapes

```rust
pub trait CollisionShape {
    fn intersect(&self, other: &dyn CollisionShape) -> bool;
    fn contact(&self, other: &dyn CollisionShape) -> Option<Contact>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl CollisionShape for Sphere {
    fn intersect(&self, other: &dyn CollisionShape) -> bool {
        other.intersect_sphere(self)
    }

    fn contact(&self, other: &dyn CollisionShape) -> Option<Contact> {
        other.contact_sphere(self)
    }

    fn intersect_sphere(&self, other: &Sphere) -> bool {
        (self.center - other.center).magnitude() < self.radius + other.radius
    }

    fn contact_sphere(&self, other: &Sphere) -> Option<Contact> {
        let delta = other.center - self.center;
        let dist = delta.magnitude();
        
        if dist < self.radius + other.radius {
            let normal = if dist > 1e-10 { delta / dist } else { Vec3::unit_y() };
            let penetration = self.radius + other.radius - dist;
            
            Some(Contact {
                point: self.center + normal * self.radius,
                normal,
                penetration,
            })
        } else {
            None
        }
    }

    fn intersect_box(&self, other: &AABB) -> bool {
        other.intersect_sphere(self)
    }

    fn contact_box(&self, other: &AABB) -> Option<Contact> {
        other.contact_sphere(self)
    }
}

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) / 2.0
    }

    pub fn extents(&self) -> Vec3 {
        (self.max - self.min) / 2.0
    }

    fn intersect_sphere(&self, other: &Sphere) -> bool {
        let closest = self.clamp_point(other.center);
        (closest - other.center).magnitude() < other.radius
    }

    fn clamp_point(&self, p: Vec3) -> Vec3 {
        Vec3::new(
            p.x.max(self.min.x).min(self.max.x),
            p.y.max(self.min.y).min(self.max.y),
            p.z.max(self.min.z).min(self.max.z),
        )
    }
}

#[derive(Clone, Copy)]
pub struct Contact {
    pub point: Vec3,
    pub normal: Vec3,
    pub penetration: f64,
}
```

---

## OPTIMIZATION

### 8.1 Gradient Descent

```rust
pub struct Optimizer;

impl Optimizer {
    pub fn gradient_descent<F, G>(
        f: F,
        gradient: G,
        x0: &[f64],
        lr: f64,
        tol: f64,
        max_iter: usize,
    ) -> Vec<f64>
    where
        F: Fn(&[f64]) -> f64,
        G: Fn(&[f64]) -> Vec<f64>,
    {
        let mut x = x0.to_vec();
        
        for _ in 0..max_iter {
            let grad = gradient(&x);
            let grad_norm: f64 = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
            
            if grad_norm < tol {
                break;
            }
            
            for i in 0..x.len() {
                x[i] -= lr * grad[i];
            }
        }
        
        x
    }

    pub fn conjugate_gradient<F, G>(
        f: F,
        gradient: G,
        x0: &[f64],
        max_iter: usize,
    ) -> Vec<f64>
    where
        F: Fn(&[f64]) -> f64,
        G: Fn(&[f64]) -> Vec<f64>,
    {
        let mut x = x0.to_vec();
        let mut g = gradient(&x);
        let mut d = g.iter().map(|gi| -gi).collect::<Vec<_>>();
        let mut rsold: f64 = g.iter().map(|gi| gi * gi).sum();
        
        for _ in 0..max_iter {
            // Line search
            let alpha = 0.01;
            let x_new: Vec<f64> = x.iter().enumerate()
                .map(|(i, &xi)| xi + alpha * d[i])
                .collect();
            
            let g_new = gradient(&x_new);
            let rsnew: f64 = g_new.iter().map(|gi| gi * gi).sum();
            
            if rsnew.sqrt() < 1e-10 {
                x = x_new;
                break;
            }
            
            let beta = rsnew / rsold;
            d = g_new.iter().enumerate()
                .map(|(i, &gi)| -gi + beta * d[i])
                .collect();
            
            g = g_new;
            rsold = rsnew;
            x = x_new;
        }
        
        x
    }
}
```

---

## STATISTICS & PROBABILITY

### 9.1 Statistical Functions

```rust
pub struct Statistics;

impl Statistics {
    pub fn mean(data: &[f64]) -> f64 {
        data.iter().sum::<f64>() / data.len() as f64
    }

    pub fn variance(data: &[f64]) -> f64 {
        let m = Self::mean(data);
        data.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / data.len() as f64
    }

    pub fn std_dev(data: &[f64]) -> f64 {
        Self::variance(data).sqrt()
    }

    pub fn covariance(x: &[f64], y: &[f64]) -> f64 {
        let mx = Self::mean(x);
        let my = Self::mean(y);
        
        x.iter()
            .zip(y.iter())
            .map(|(xi, yi)| (xi - mx) * (yi - my))
            .sum::<f64>() / x.len() as f64
    }

    pub fn correlation(x: &[f64], y: &[f64]) -> f64 {
        let cov = Self::covariance(x, y);
        let sx = Self::std_dev(x);
        let sy = Self::std_dev(y);
        
        cov / (sx * sy)
    }

    pub fn normal_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
        let z = (x - mu) / sigma;
        (-0.5 * z * z).exp() / (sigma * (2.0 * std::f64::consts::PI).sqrt())
    }

    pub fn normal_cdf(x: f64, mu: f64, sigma: f64) -> f64 {
        let z = (x - mu) / sigma;
        
        if z > 6.0 {
            1.0
        } else if z < -6.0 {
            0.0
        } else {
            // Approximation
            1.0 / (1.0 + (-0.07056 * z * z - 1.59782).exp()).powf(4.0)
        }
    }
}
```

---

## SIMULATION FRAMEWORKS

### 10.1 Physics Engine

```rust
pub struct PhysicsEngine {
    pub gravity: Vec3,
    pub bodies: Vec<RigidBody>,
    pub particles: ParticleSystem,
}

impl PhysicsEngine {
    pub fn new() -> Self {
        PhysicsEngine {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            bodies: Vec::new(),
            particles: ParticleSystem::new(10000),
        }
    }

    pub fn step(&mut self, dt: f64) {
        // Integrate rigid bodies
        for body in &mut self.bodies {
            body.apply_force(self.gravity * body.mass, body.position);
            body.integrate(dt);
        }
        
        // Update particles
        self.particles.update(dt, self.gravity);
        
        // Collision detection and response
        self.resolve_collisions();
    }

    fn resolve_collisions(&mut self) {
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                // Check collision
            }
        }
    }
}
```

---

## RECAP

1. **Numerical methods matter** - Choose right algorithm
2. **Symplectic integrators** - Better energy conservation
3. **Broad-phase collision** - Pruning is essential
4. **GJK for convex** - Separating axis for boxes
5. **Spatial hashing** - Fast neighbor queries
6. **Time step fixed** - Stable simulation

---

*Skill ID: 009 | Category: Mathematics-Simulation | Complexity: Expert*
*Version: 1.0.0 | Last Updated: 2024*