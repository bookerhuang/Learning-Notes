# Review

## 变量

`var name type = expression`

类型和表达式部分可以省略一个，但是不能都省略。如果类型省略，它的类型将由初始化表达式决定；如果表达式省略，其初始值对应于类型的零值。

>对于接口类型和引用类型（slice，指针，map，通道，函数）是nil。

---

### 短变量声明

在代码块的包围内，一种称为短变量声明的可选方式可以用来声明和初始化局部变量。

var 声明通常是为了那些跟初始化表达式类型不一致的局部变量保留的，或者用于后面才对变量赋值以及变量初始化不重要的情况。

```go
i := 100 // 一个int类型的变量
var boiling float64 = 100 // 一个float64类型的变量
```

> := 表示声明，而 = 表示赋值。一个多变量的声明不能和多重赋值搞混

短变量声明至少声明一个新变量：

```go
f, err := os.Open(infile)
// ...
f, err := os.Create(outfile) // 编译错误：没有新的变量
```

---

### 指针

指针类型的零值是nil，两个指针当且仅当指向同一个变量或者两者都是nil的情况才会相等

---

### new函数

每一次调用new都会返回一个具有唯一地址的不同变量：

```go
p := new(int)
q := new(int)
fmt.Println(p == q) // "false"
```

这个规则有一个例外：两个变量的类型不携带任何信息且是零值，例如`struct{}`或`[0]int`，当前的实现里面，它们有相同的地址

> fmt.Println ( new ( struct ) == new ( struct ) ) // false

new是一个预声明的函数，不是一个关键字：

```go
func delta(old, new int) int { return new - old }
```

自然，在delta函数内，内置的new函数是不可用，语法检测没问题，但编译会出错

---

### 变量的生存周期

```go
var global *int

func f() {
    var x int
    x = 1
    global = &x
}
```

这里， x 一定使用堆空间，因为它在 f 函数返回以后还可以从global变量访问，尽管它被声明为一个局部变量。不要用长生命周期的指针去指向短生命周期的变量。

---

### 类型声明

```go
package tempconv
// ...

type Celsius float64
type Fahrenheit float64

// ...
```

这个包里定义了两个类型，即使使用相同的底层类型float64，它们也不是相同的类型，所以它们不能用算是表达式进行比较和合并。 

---

### 包初始化

包的初始化从初始化包级别的变量开始，这些变量按照声明顺序初始化，在依赖已解析完毕的情况下，根据依赖的顺序进行。

```go
var a = b + c
var b = f()
var c = 1

func f() int { return c + 1 }

func main() {
    fmt.Println(a, b, c) // 3,2,1
}
```

```go
func init() { /* ... */}
```

init 函数：会在程序开始时自动执行，不能被调用和被引用。

