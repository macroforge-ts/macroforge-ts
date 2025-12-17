# Your First Macro
 *Let's create a class that uses Macroforge's derive macros to automatically generate useful methods.*
 ## Creating a Class with Derive Macros
 Start by creating a simple <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">User</code> class. We'll use the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorator to automatically generate methods.
 <div><div class="flex items-center justify-between gap-2 px-4 py-2 bg-muted rounded-t-lg border border-b-0 border-border"> 
**Before:**
```
/** @derive(Debug, Clone, PartialEq) */
export class User &#123;
    name: string;
    age: number;
    email: string;

    constructor(name: string, age: number, email: string) &#123;
        this.name = name;
        this.age = age;
        this.email = email;
    &#125;
&#125;
``` <div class="flex items-center justify-between gap-2 px-4 py-2 bg-muted rounded-t-lg border border-b-0 border-border"> 
**After:**
```
export class User &#123;
    name: string;
    age: number;
    email: string;

    constructor(name: string, age: number, email: string) &#123;
        this.name = name;
        this.age = age;
        this.email = email;
    &#125;

    static toString(value: User): string &#123;
        return userToString(value);
    &#125;

    static clone(value: User): User &#123;
        return userClone(value);
    &#125;

    static equals(a: User, b: User): boolean &#123;
        return userEquals(a, b);
    &#125;
&#125;

export function userToString(value: User): string &#123;
    const parts: string[] = [];
    parts.push('name: ' + value.name);
    parts.push('age: ' + value.age);
    parts.push('email: ' + value.email);
    return 'User &#123; ' + parts.join(', ') + ' &#125;';
&#125;

export function userClone(value: User): User &#123;
    const cloned = Object.create(Object.getPrototypeOf(value));
    cloned.name = value.name;
    cloned.age = value.age;
    cloned.email = value.email;
    return cloned;
&#125;

export function userEquals(a: User, b: User): boolean &#123;
    if (a === b) return true;
    return a.name === b.name &#x26;&#x26; a.age === b.age &#x26;&#x26; a.email === b.email;
&#125;
``` ## Using the Generated Methods
 ```
const&nbsp;user&nbsp;=&nbsp;new&nbsp;User("Alice",&nbsp;30,&nbsp;"alice@example.com");

//&nbsp;Debug:&nbsp;toString()
console.log(user.toString());
//&nbsp;Output:&nbsp;User&nbsp;&#123;&nbsp;name:&nbsp;Alice,&nbsp;age:&nbsp;30,&nbsp;email:&nbsp;alice@example.com&nbsp;&#125;

//&nbsp;Clone:&nbsp;clone()
const&nbsp;copy&nbsp;=&nbsp;user.clone();
console.log(copy.name);&nbsp;//&nbsp;"Alice"

//&nbsp;Eq:&nbsp;equals()
console.log(user.equals(copy));&nbsp;//&nbsp;true

const&nbsp;different&nbsp;=&nbsp;new&nbsp;User("Bob",&nbsp;25,&nbsp;"bob@example.com");
console.log(user.equals(different));&nbsp;//&nbsp;false
``` ## Customizing Behavior
 You can customize how macros work using field-level decorators. For example, with the Debug macro:
 <div><div class="flex items-center justify-between gap-2 px-4 py-2 bg-muted rounded-t-lg border border-b-0 border-border"> 
**Before:**
```
/** @derive(Debug) */
export class User &#123;
    /** @debug(&#123; rename: "userId" &#125;) */
    id: number;

    name: string;

    /** @debug(&#123; skip: true &#125;) */
    password: string;

    constructor(id: number, name: string, password: string) &#123;
        this.id = id;
        this.name = name;
        this.password = password;
    &#125;
&#125;
``` <div class="flex items-center justify-between gap-2 px-4 py-2 bg-muted rounded-t-lg border border-b-0 border-border"> 
**After:**
```
export class User &#123;
    id: number;

    name: string;

    password: string;

    constructor(id: number, name: string, password: string) &#123;
        this.id = id;
        this.name = name;
        this.password = password;
    &#125;

    static toString(value: User): string &#123;
        return userToString(value);
    &#125;
&#125;

export function userToString(value: User): string &#123;
    const parts: string[] = [];
    parts.push('userId: ' + value.id);
    parts.push('name: ' + value.name);
    return 'User &#123; ' + parts.join(', ') + ' &#125;';
&#125;
``` ```
const&nbsp;user&nbsp;=&nbsp;new&nbsp;User(42,&nbsp;"Alice",&nbsp;"secret123");
console.log(user.toString());
//&nbsp;Output:&nbsp;User&nbsp;&#123;&nbsp;userId:&nbsp;42,&nbsp;name:&nbsp;Alice&nbsp;&#125;
//&nbsp;Note:&nbsp;'id'&nbsp;is&nbsp;renamed&nbsp;to&nbsp;'userId',&nbsp;'password'&nbsp;is&nbsp;skipped
```  **Field-level decorators Field-level decorators let you control exactly how each field is handled by the macro. ## Next Steps
 - [Learn how macros work under the hood](../../docs/concepts)
 - [Explore all Debug options](../../docs/builtin-macros/debug)
 - [Create your own custom macros](../../docs/custom-macros)
**