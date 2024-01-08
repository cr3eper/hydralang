# Hydra

### What is it and why should I care about it?

Honestly it's just a hobby project of mine for now, I think it's quite an intersting language in terms of it's reflective capability but it's still a prototype and Work In Progress.

#### Still Interested?

Hydra has built in reflective meta-programming and is designed to empower the user to define it's own behaviour

Let's jump into some examples:
Here is how you might define a derivative function in Hydra
```
df(x^n, x) = df(n * x^(n - 1)) where { not(contains(n, x)) }
```
The first thing to note is that this definition of df does not exist for all cases of it's arguments, only the cases we defined here.
In this case it's only relevant when we're dealing with something (x) to the power of something that does not contain x.

The where clause of this definition is what we call a *`Constraint`* and it allows us to bound particular function definitions to when they are defined.
The constraint defined above is `not(contains(n, x))`  
The defintion of `not(p)` is very simple:
```
not(0) = 1
not(1) = 0
```  
The definition of `contains(n, x)` is much more involved and I will not detail it here. In general what it does is it returns 1 if n contains the variable x and returns 0 if n does not contain x.  
A few illustrative examples:
```
contains(a, x)              # returns 0
contains(10 + x, x)         # returns 1
contains(2^(1 + x))         # returns 1
contains(100 / 2 + 4 * dx)  # returns 0 (dx != x)
```

You would be surprised just how much of a programming languages features you can describe by building upon these simple rules.

Under the hood there is a function called `eval(e)` which has lots of definitions, `eval` is automatically called on every expression recursively and is responsible for almost all automatic transformations in the language (with the exception of function evaluation itself). It can be dangerous to modify since you can easily end up in a recursive eval that never returns a result. So modify it with caution. I intend to implement a much safer version of operator overloading in the future.

If you do not include the standard library and forget to define defaults like `eval(a + b)` for integers then when you type `1 + 2` into the terminal, your result will be: `1 + 2`

## Consider the following problems

## 1. Calculate the Cross Product of the following Vectors:

u = (1, 2, 3)  
v = (1, 1, 4)

First we need to define our [Cross Product](https://www.khanacademy.org/math/multivariable-calculus/thinking-about-multivariable-function/x786f2022:vectors-and-matrices/a/cross-products-mvc):

```
cross((x1, y1, z1), (x2, y2, z2)) = (y1*z2 - y2*z1, 0 - (x1*z2 - z1*x2), x1*y2 - y1*x2)
eval((x1, y1, z1) * (x2, y2, z2)) = cross((x1, y1, z1), (x2, y2, z2)) # allows us to use `u * v` instead of `cross(u, v)`
```
Next we simply call this function with the above values, note that we still need to include the brackets to ensure we treat the vectors as vectors:
```
(1, 2, 3) * (1, 1, 4)
```
We get the result:
```
(5, -1, -1)
```

## 2. Calculate the derivative of the following equation

#### `f(x) = 3 * x^2 + 2 * (2 * x + 2) ^ 2`

We're going to do this from scratch, we're not going to use the built in calc library for this example (but you can if you want to save time)  

We start by defining a function called `df` which we use to denote all derivatives, later I'll probably implement overrides for `df/dx` but for now let's not make this more complicated than it
needs to be. 

`df` needs to be a function with two arguments, the first argument is the expression to derive, the second is what we're deriving against. For illustrative purposes (this doesn't work yet)

```
dy/dx = df(y, x)
```

Let's start with some of the simpler rules we're going to need:
```
df(a + b, x) = df(a, x) + df(b, x)                              # Derivative is distributive, so we need to account for this
df(x ^ n, x) = n * x ^ (n - 1) where { not(contains(n, x)) }    # Derivative of x^n
```

We're going to need the product rule for this solution so let's include that
```
df(a * b, x) = a * df(b, x) + b * df(a, x)  # Yes really, that's all. You never need to worry about doing chain rule again.
```

We need to account for the chain rule for the `df((2 * x + 2)^2, x)` component.
Sadly we cannot define the chain rule completely generically like it is often taught. But we can define it for `u^n`
```
df(u^n, x) = df(u, x) * n * u ^ (n - 1) where { contains(u, x), not(contains(n, x)) }
``` 


Now that's it right?! Not quite, there are still quite a few cases you typically wouldn't think about that we haven't accounted for here
what about `df(3, x)` ? We need to define how to handle this constant
```
df(c, x) = 0 where { not(contains(c, x)) } 
```

Now we're done? Hehe, let's see if you can guess the last one. Don't cheat!
```
df(x, x) = 1
```

Now we get our answer
```
3 * 2 * x + 2 * 4 * (2 * x + 2)
```

Yeah I don't know why it's not simplifying either :P it's technically correct though. This is a WIP project after all.


## 3. Compound Interest Calculations

I want to calculate how much money I would have from a 7% interest (p.a) investment with a 10,000 dollar deposit over 5 years.

Easy.

```
calculateInterest(amount, rate, 0) = amount
calculateInterest(amount, rate, times) = calculateInterest(amount + amount * rate, rate, times - 1)

calculateInterest(10000, 0.07, 5)
```

Answer
```
14025.5173070000
```


## 4. Consider the following lines in **R<sup>3</sup>**

- u(t) = (0, -1, -1) + t(4, 2, 0)
- v(s) = (7, -7, 4) + s(1, 1, 4)

### Find the minimum distance between the two lines:

This question will be a little more complicated to solve but this isn't a maths class so I'm not going to delve too deep here.

We know we can find the cross product the same way as before:

We're going to need to define some more pre-requisite functions here, these include:
- dot product 
- magnitude of a vector
- scalar projection

Note that all of these are in **R<sup>3</sup>**

### Dot product

```
dot((x1, y1, z1), (x2, y2, z2)) = x1 * x2 + y1 * y2 + z1 * z2
eval((x1, y1, z1) . (x2, y2, z2)) = dot((x1, y1, z1), (x2, y2, z2)) # allows us to use `u . v` instead of `dot(u, v)`
```

### Magnitude of a vector

```
magnitude((x, y, z)) = sqrt(x^2 + y^2 + z^2)    # we won't define this as |u| because I haven't added that capability to the grammar yet
```

### Scalar Projection

```
scalarProjection(u, v) = u . v / magnitude(v)

```

### Find distance between lines

For now lamda's unfortunately don't exist so we'll need to pass in the function expressions directly, this will change since lambdas and higher order functions are on the cards to be implemented at some point.
```
u(t) = (0, -1, -1) + t * (4, 2, 0)
v(s) = (7, -7, 4) + s * (1, 1, 4)

# Utility function for us to grab a lines direction, I recommend more detailed names for larger problems
direction(c + d*(x, y, z), d) = (x, y, z) 

# Utility function for us to find a lines starting position (so when d=0) and the direction component cancels out
start(c + d*(x, y, z), d) = c

distanceBetweenSkewLines(l1, l2, t) = scalarProjection(start(l1, t) - start(l2, t), direction(l1, t) * direction(l2, t))

distanceBetweenSkewLines(u(t), v(t), t)

```



