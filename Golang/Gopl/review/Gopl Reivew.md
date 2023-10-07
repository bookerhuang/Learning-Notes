# Gopl Reivew

## 8️⃣ Goroutines与Channels

Go语言中的并发程序可以用两种手段来实现：goroutine 和 channel，其支持顺序通信进程，或被简称为`CSP`，`CSP`是一种并发编程模型，在这种并发编程模型中，值会在不同运行实例中传递，第二个手段便是多线程共享内存。

### 8.1 Goroutines

在Go语言中，每一个并发的执行单元叫作一个`goroutine`。当一个程序启动时，**其主函数在一个单独的goroutine中运行，我们叫它main goroutine**。新的goroutine会用go语句创建，即在函数或者方法前加上`go`关键字，go语句会使其语句中的函数在一个新建的goroutine中运行，而go语句本身也会迅速完成：

```go
go f()
```

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	go spinner(100 * time.Millisecond)
	const n = 45
	fibN := fib(n)
	fmt.Printf("\rFibonacci(%d) = %d\n", n, fibN)
}

func spinner(delay time.Duration) {
	for {
		for _, r := range "-/\\|" {
			fmt.Printf("\r%c", r)
			time.Sleep(delay)
		}
	}
}
func fib(x int) int {
	if x < 2 {
		return x
	}
	return fib(x-1) + fib(x-2)
}
```

函数运行会在几秒后出现结果：

```shell
Fibonacci(45) = 1134903170
```

然后主函数返回，**除了从主函数退出或者直接终止程序外，没有其它编程方法能够让一个goroutine打断另一个的执行**，但是之后可以看到一种方式来实现这个目的，**通过goroutine之间的通信来让一个goroutine请求其他的goroutine，并让被请求的goroutine自行结束执行**。

注意，`fib`函数和`spinner`函数是同时执行的，如果spinner调用前不加上`go`，则会一直执行spinner函数，阻塞fib函数执行。


### 8.2 示例：并发的Clock服务

网络编程是并发最适合的领域之一，最典型的是服务器要同时处理很多连接程序。

这段代码是一个简单的Go语言TCP服务器程序，用于监听`localhost:8000`端口，并向连接到该端口的客户端发送当前时间。具体来说，该程序使用`net.Listen`函数创建一个TCP监听器，监听`localhost:8000`端口。然后，使用`listener.Accept`函数接受客户端的连接请求，并将连接对象传递给`handleConn`函数进行处理。`handleConn`函数中，使用`io.WriteString`函数向客户端发送当前时间，并使用`time.Sleep`函数暂停1秒钟，以模拟实时时间的流逝。如果客户端断开连接，则函数会返回：

```go
package main

import (
	"io"
	"log"
	"net"
	"time"
)

func main() {
	listener, err := net.Listen("tcp", "localhost:8000")
	if err != nil {
		log.Fatal(err)
	}
	for {
		conn, err := listener.Accept()
		if err != nil {
			log.Print(err)
			continue
		}
		handleConn(conn)
	}
}

func handleConn(c net.Conn) {
	defer c.Close()
	for {
		_, err := io.WriteString(c, time.Now().Format("15:04:05\n"))
		if err != nil {
			return // e.g., client disconnected
		}
		time.Sleep(1 * time.Second)
	}
}
```

创建了一个`listener`来监听网络端口到来的连接，然后用`handleConn`来处理这个完整的客户端连接：

![image-20231004154023721](imgs/image-20231004154023721.png)

客户端将服务器发来的时间显示了出来。

这段代码是一个简单的Go语言TCP客户端程序，用于连接`localhost:8000`端口，并将从服务器接收到的数据输出到标准输出。具体来说，该程序使用`net.Dial`函数创建一个TCP连接，连接到`localhost:8000`端口。然后，使用`defer`语句在函数结束时关闭连接。接着，调用`mustCopy`函数将从服务器接收到的数据输出到标准输出。`mustCopy`函数中，使用`io.Copy`函数将从`src`中读取的数据写入到`dst`中，直到`src`中的数据全部读取完毕。如果在复制过程中发生错误，则会输出一个错误信息并退出程序：

```go
package main

import (
	"io"
	"log"
	"net"
	"os"
)

func main() {
	conn, err := net.Dial("tcp", "localhost:8000")
	if err != nil {
		log.Fatal(err)
	}
	defer conn.Close()
	mustCopy(os.Stdout, conn)
}

func mustCopy(dst io.Writer, src io.Reader) {

	if _, err := io.Copy(dst, src); err != nil {
		log.Fatal(err)
	}
}
```

将这个代码同时在两个终端中运行测试，可以看到只能在一个终端中执行，也就是说第二个客户端必须等待第一个客户端完成才能进行。

![image-20231004154502767](imgs/image-20231004154502767.png)

---

![image-20231004154525828](imgs/image-20231004154525828.png)

现在把程序使用go关键字改成并发支持：

```go
for {
	conn,err := listener.Accept()
	if err != nil {
		log.Print(err)
		continue
	}	
	go handleConn(conn)
}
```

就可以将这个代码同时在两个终端中运行。

![image-20231004154620651](imgs/image-20231004154620651.png)

---

![image-20231004154646975](imgs/image-20231004154646975.png)

### 8.3 示例：并发的Echo服务

上一节中`clock`服务器中，每一个连接都会创建一个`goroutine`。现在我们创建一个`echo`服务器，它的每个连接会有多个Goroutine。大多数echo会返回它们读取到的内容，就像下面这个函数一样：

```go
func handleConn(c net.Conn) {
	io.Copy(c,c)
	c.Close()
}
```

现在让echo服务器模拟一个真实的`回响`，也就是随着时间推移，返回值由大写变为小写再到消失：

```go
package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
	"strings"
	"time"
)

func main() {
	listener, err := net.Listen("tcp", "localhost:8000")
	if err != nil {
		log.Fatal(err)
	}
	for {
		conn, err := listener.Accept()
		if err != nil {
			log.Print(err)
			continue
		}
		go handleConn(conn)
	}
}

func echo(c net.Conn, shout string, delay time.Duration) {
	fmt.Fprintln(c, "\t", strings.ToUpper(shout))
	time.Sleep(delay)
	fmt.Fprintln(c, "\t", shout)
	time.Sleep(delay)
	fmt.Fprintln(c, "\t", strings.ToLower(shout))
}

func handleConn(c net.Conn) {
	input := bufio.NewScanner(c)
	for input.Scan() {
		echo(c, input.Text(), 1*time.Second)
	}
	c.Close()
}

```

升级一下客户端程序：

```go
package main

import (
	"io"
	"log"
	"net"
	"os"
)

func main() {
	conn, err := net.Dial("tcp", "localhost:8000")
	if err != nil {
		log.Fatal(err)
	}
	defer conn.Close()
	go mustCopy(os.Stdout, conn)
	mustCopy(conn, os.Stdin)
}

func mustCopy(dst io.Writer, src io.Reader) {

	if _, err := io.Copy(dst, src); err != nil {
		log.Fatal(err)
	}
}
```

程序使用`mustCopy`函数将从连接读取的数据复制到标准输出，同时将从标准输入读取的数据复制到连接中。运行当main goroutine 从标准输入获取内容并将其发送给服务器时，另一个goroutine 会读取并答应服务端的响应。当main goroutine结束时，例如用户在终端输入`Control+D`，其它goroutine也会结束。

下面这个会话中，客户端的输入是左对齐的，服务端的响应会用缩进来表示，客户端会向服务端喊三次话：

![image-20231004160034859](imgs/image-20231004160034859.png)

**注意**：第三次的呼叫只有在第二次的回声衰竭时才会进行发出第三次的回声，真实的回声会由三个独立的回升`叠加`组成。所以为了模仿真实，需要在调用`echo`时加入`go`关键字：

```go
// ...
func handleConn(c net.Conn) {
	input := bufio.NewScanner(c)
	for input.Scan() {
		go echo(c, input.Text(), 1*time.Second)
	}
	c.Close()
}
// ...
```

解决：

![image-20231004160956076](imgs/image-20231004160956076.png)

不仅在处理多个客户的请求时可以使用并发，在处理单个用户请求时也可以，就像上面的用法，但是可能存在安全隐患，所以需要慎重考虑。



### 8.4 Channels

简单来说，可以把`goroutine`看成`并发体`，把`channel`看成它们之间的`通信机制`，有了这个，独立的goroutine可以通过它来发送数据，channel根据具体的数据类型不同也不同比如` channel int `和` channel string `是两个发送不同类型数据的channel。

使用内置的`make`函数可以创建channel：

```go
ch = make(chan int)
```

和map类似，channel也对应一个make创建的底层数据结构的调用。channel可以与`==`或`nil`来比较。

一个channel主要有两个操作，都是通信行为：接收和发送。另外还有一个是close，除了这些之外的其它操作都会导致panic！

```go
ch <- x        //把x发送进通道
x  <- ch       //把接受通道里的值
   <- ch       //把通道里的值丢弃
close(ch)      //关闭通道，但是仍然可以接收到值
```

channel是有容量的，如果容量大于零就是有缓存的channel：

```go
ch = make(chan int)    // 无缓存的channel
ch = make(chan int, 0) // 无缓存的channel
ch = make(chan int, 3) // 有缓存的channel，容量为3
```

#### 8.4.1 不带缓存的Channels

在使用无缓存channel的情况下会导致发送和接收goroutine同步化，也被称为goroutine同步化。

**当一个值在无缓冲的通道上传递时，接受值后发送方的goroutine才会被唤醒。**

> 在讨论并发的时候，并不意味着同时发生。当两个goroutine并发地访问同一个变量时，其间需要有顺序，避免程序的执行发生问题。

8.3中的客户端程序在主goroutine中将输入复制到服务器中，这样客户端在输入接收后立即退出，即使后台的goroutine还在继续。为了让程序等待后台的goroutine在完成后退出，使用一个通道来同步两个goroutine：

```go
package main

import (
	"io"
	"log"
	"net"
	"os"
)

func main() {
	conn, err := net.Dial("tcp", "localhost:8000")
	if err != nil {
		log.Fatal(err)
	}
	done := make(chan struct{})
	go func() { 
		io.Copy(os.Stdout, conn)
		log.Println("done")
		done <- struct{}{} // 指示主goroutine
	}()
	mustCopy(conn, os.Stdin)
	conn.Close()
    <-done // 等待后台放入 struct{}{}
}

func mustCopy(dst io.Writer, src io.Reader) {

	if _, err := io.Copy(dst, src); err != nil {
		log.Fatal(err)
	}
}
```

这里的go语句中调用了匿名函数，这是go语言中启用goroutine常见的方法。

当用户关闭标准输入流后，mustCopy返回，主goroutine调用conn.Close()来关闭两端的网络连接。关闭写半边的连接会导致服务器看到EOF，关闭读半边的连接会导致后台goroutine调用io.Copy返回`read from closed connection`错误，这也是去掉错误日志的原因。

#### 8.4.2 管道（串联的channels）

channel可以用来连接goroutine，使得一个goroutine的输出是另一个的输入，此为管道（pipeline）。

下面的程序由三个goroutine（counter、squarer、printer）组成，两个通道连接：

```go
package main

import "fmt"

func main() {
	naturals := make(chan int)
	squares := make(chan int)

    // counter
	go func() {
		for x := 0; ; x++ {
			naturals <- x
		}
	}()

    // squarer
	go func() {
		for {
			x := <-squares
			squares <- x * x
		}
	}()
	
    // 在主goroutine中
	for {
		fmt.Println(<-squares)
	}
}
```

该程序输出无限的平方序列：0，1，4，9，......。

如果发送方没有更多的数据要发送，告诉接收者所在的goroutine可以停止等待是很有用的。这可以使用内置的close函数来关闭通道：close(naturals)。通道关闭后，后续的发送操作将会导致应用崩溃。当关闭的通道被读完，所有的后续操作的接收能获取到的只有零值。

没有一个直接的方式来判断是否通道已经关闭，但是有一个接收操作的变种，它将产生两个结果：接收到的通道元素以及一个布尔值（通常称为ok），它为true的时候表示接收成功，fasle表示当前的接收操作在一个关闭且读完的通道上，这样就可以关闭squares通道

```go
go func() {
		x, ok := <-naturals
		if !ok {
			squares <- x * x
		}
		close(squares)
	}()
```

上面处理是很笨拙的，其实不需要关闭每一个channel，因为不管有没有被关闭，当它没有引用时，将会被go语言的垃圾自动回收器回收：

```go
package main

import "fmt"

func main() {
	naturals := make(chan int)
	squares := make(chan int)

	go func() {
		for x := 0; x < 100; x++ {
			naturals <- x
		}
		close(naturals)
	}()

	go func() {
		for x := range naturals{
			squares <- x * x
		}
		close(squares)
	}()

	for x := range squares {
		fmt.Println(x)
	}
}
```

#### 8.4.3 单向通道类型

单方向的channel表示为 `通道名 chan<- type`，如 `channel_name chan<- int`，这是一个只能用来发送的通道。相反，`channel_name <-chan int`是一个只能用来接收的通道。

把上述程序拆分一下并且使用单通道发送数据，让其中夹在中间的函数既要发送也要接收：

```go
package main

import "fmt"

func main() {
	naturals := make(chan int)
	squares := make(chan int)

	go counter(naturals)
	go squarer(squares, naturals)
	printer(squares)
}

func counter(out chan<- int) {
	for x := 0; x < 100; x++ {
		out <- x
	}
}

func squarer(out chan<- int, in chan<- int) {
	for v := range in {
		out <- v * v
	}
	close(out)
}

func printer(in <-chan int) {
	for v := range in {
		fmt.Println(v)
	}
}
```

counter(naturals)的调用隐式地将`chan int`转化为参数所要求的`chan<- int`类型。

**在任何赋值操作中将双向通道转换为单向通道都是被允许的，但是反过来不行。**

#### 8.4.4 缓冲通道

带缓存的channel创建如下，3是缓存大小：

```go
ch = make(chan string,3)
```

如果缓存满则不能发送值，缓存空则不能接收值，缓存中的值消耗遵循队列原则，先进先出，可以使用`cap(ch)`来获取缓管道的容量，用`len(ch)`来获取当前通道中的数据个数：

```go
ch <- "A"
ch <- "B"
ch <- "C"
fmt.Println(<-ch) // "A"
fmt.Println(cap(<-ch)) // 3
```

len函数在并发程序中获得到的信息会随着检索操作很快过时，所以价值很低，但是在错误诊断和性能优化的时候很好用。

---

不能将缓冲通道作为队列在单个goroutine中使用，这是个错误。通道和goroutine深度关联，如果没有另一个goroutine从通道中进行接收，可能会有永久阻塞的风险。如果只需要一个简单的队列，使用slice创建即可。

以下代码是并发向三个站点发出请求，三个镜像站点分散在不同的地理位置，站点响应后会把响应发送至带缓存的channel，接收者只接收最快的响应（多个goroutines同时并发的向同一个channel发送或者从同一个channel接收数据，都非常常见）：

```go
func mirroredQuery() string {
	responses := make(chan string,3)
	go func() { responses <- request("asia.gopl.io")}()
	go func() { responses <- request("europe.gopl.io")}()
	go func() { responses <- request("americas.gopl.io")}()
	return <- responses
}
func request(hostname string) (response string) { /*...*/}
```

在上述代码中，使用了有缓存的channel，但是如果是用了无缓存的channel，另外两个goroutines将会永远卡住，这种goroutines泄漏是BUG，不会被垃圾回收机制回收，因此确保goroutine正常退出是重要的。

### 8.5 并行循环

以下举几个伪代码说明在并行时循环迭代的常见并发模型，把全尺寸图片生成一些缩略图，假设下面这个函数能帮我们拉伸图片：

```go
package thumbnail

func ImageFile(infile string) (string, error)
```

构建`makeThumbnais`函数：

```go
func makeThumbnails(filenames []string) (string, error) {
	for _, f := range filenames {
		if _, err := thumbnail.ImageFile(f); err != nil {
			log.Println(err)
		}
	}
}
```

上面每一个file都是独立的，我们再使用go 关键字把每次处理放入goroutine中，注意下面并没有处理错误（伪代码）：

```go
func makeThumbnails(filenames []string) (string, error) {
	for _, f := range filenames {
		go thumbnail.ImageFile(f)
	}
}
```

这样所有上述的file就是并行的了，**但是其中一个goroutine先完成时，makeThumnails就会返回，不管其他goroutine有没有结束**，现在使用channel的通信机制来让所有的goroutine执行结束：

```go
func makeThumbnails(filenames []string) (string, error) {
	for _, f := range filenames {
		ch := make(chan struct{})   //创建一个通道
		go func(f string) {         //把遍历到的作为goroutine的参数给里面的函数
			thumbnail.ImageFile(f)  //goroutine函数执行结束时给channel中发送一个信息（这里是结构体）
			ch <- struct{}{}
		}(f)
		for range filenames {
			<- ch                   //接收端在主函数结尾部分，这就可以保证所有goroutine结束，因为channel没有接收到值时，程序是不会停止的
		}
	}
}
```

在上面这个函数中我们在goroutine中调用了匿名函数，是函数就会存在调用失败的情况，所以要使用通道技术将可能的错误从goroutine中传递出来：

```go
func makeThumbnails(filenames []string) (string, error) {
	for _, f := range filenames {
		errors := make(chan error)
		go func(f string) {
			_, err := thumbnail.ImageFile(f)    //保留返回的错误
			errors <- err                       //把错误发送出去
		}(f)
	}
	for range filenames {
		if err := <- errors;err != nil {        //接收错误，如果有的话就返回
			return err
		}
	}
	return nil                                  //如果没有的话就返回空值
}
```

现在可以将调用函数及将可能会发生的错误传递出来了，不过这个程序是有问题的，因为传递一个错误，并且这个channel的错误可能没有被接收（**这里的赋给新变量err的值可能是nil**），这导致channel会被堵塞，所以后续即使goroutine还有错误，也无法将错误成功传递。现在来新建一个有缓存的channel，并且让函数不管处理成功与否都返回相对应合适的信息：

```go
func makeThumbnails(filenames []string) (thumbfiles []string, err error) {

	type item struct {                       //先定义一个结构体，为了把返回值整合近一个变量里，结构体内部成员分别是两个返回值
		thumbfile string
		err       error
	}

	ch := make(chan item, len(filenames))    //定义一个有缓存的通道

	for _, f := range filenames {
		go func(f string) {
			var it item                      //定义一个结构体类型的变量，用来调用
			it.thumbfile, it.err = thumbnail.ImageFile(f)
			ch <- it
		}(f)
	}

	for range filenames {
		it := <- ch
		if it.err != nil {
			return nil,it.err
		}
		thumbfiles = append(thumbfiles,it.thumbfile)
	}
	return thumbfiles,nil
}
```

下面这段代码makeThumbnails的最终版本，返回新文件所占用的总字节数。不同于之前，它采用字符串通道来接收文件名。但这样我们不能预测迭代的次数，所以需要在每一个goroutine启动前递增计数，在结束时递减计数。使用`sync.waitGroup`来当作计数器类型，可以被多个goroutine安全地操作，也有方法能一直等到计数器至0：

```go
func makeThumbnails(filenames <-chan string) int64 {
	
	sizes := make(chan int64)
	var wg sync.WaitGroup // 工作goroutin的个数
	
	for f := range filenames {
		wg.Add(1)
		go func(f string) {
			defer wg.Done() // 使用defer来确保在发送错误的情况下计数器可以递减
			thumb, err := thumbnail.ImageFile(f)
			if err != nil {
				log.Println(err)
				return
			}
			info, _ := os.Stat(err)
			sizes <- info.Size()
		}(f)
	}
	
    // closer
	go func() {
		wg.Wait() // Wait方法会阻塞调用它的goroutine，直到WaitGroup中的计数器归零。
		close(sizes)
	}()

	var total int64
	
	for size := range sizes {
		total += size
	}
	
	return total
}
```

**在不知道迭代次数的情况下，上面的代码是通用的、符合习惯的并行循环模式。**

---

## 9️⃣ 使用共享变量实现并发

使用goroutine和channel这样直接而自然的方式来实现并发的方法有时候会出现一些问题，本章更详细地介绍并发机制，尤其在goroutine之间共享变量。

### 9.1 竞态

一般情况下，无法推断两个goroutine中的程序执行的顺序。一个函数在并发的程序中仍可以正常工作就说它是并发安全的，同理，对于某个类型，如果它的操作和访问它的方法都能够正常工作的话，那这个类型就是并发安全的。虽然类型是在程序中的，但是在使用非并发安全类型的情况下也可以保证程序的并发安全性，但是实际上会避免并发使用大多数类型，使用的方法就是将变量局限在一个goroutine中，或者使用互斥条件维持更高级的不变性。

包级别的导出函数一般是并发安全的，因为package级的变量没法限制在单一的goroutine，所以修改这些`变量`必须使用互斥条件。函数在并发调用的时候没法工作的原因有很多，比如死锁、活锁和饿死等。

竞争条件指的是多个goroutine在交叉执行操作时，无法给出正确的结果，下面这是一个简单的银行账户程序：
```go
package bank

var balance int
func Deposit(amount int) {balance = balance + amount}
func Balance() int {return balance}
```

上述中包含一个存款函数和一个余额函数，因为他们之间是存在变动关系的，顺序调用不会出现任何问题，但是如果把它们都放入goroutine就会出现问题。

```go
//Alice
go func() {
		bank.Deposit(200)
		fmt.Println("=",bank.Balance())
	}()

//Bob
go bank.Deposit(100)
```

上述代码中Alice和Bob的行为是并行发生的，无法推断到底是谁先发生，甚至它们可能是交叉发生的，比如A先存了200块，接着Bob又存了100块，Alice再余额。现在有一种情况就是Alice所有的行为执行完了，Bob的才开始执行，这就会导致Bob的这100块Alice看不到。这个程序其实包含了一个特定的竞争条件，叫做`数据竞争`。**无论任何时候，只要又有两个goroutine并发访问同一变量，且至少其中一个是写操作的时候就会发生数据竞争。**如果数据竞争的是一个比机器字长更大的类型时就比较麻烦了，64位计算机机器字长是8，比如interface，string和slice类型。

并发更新同一个slice：

```go
var x []int
	go func() { x = make([]int, 10) }()
	go func() { x = make([]int, 1000000) }()

x[99999] = 1
```

上面代码中将`x[99999]`赋值为1，但是因为goroutine们是并发执行的，并不能保证在赋值的时候x的capacity是多少。事实上最后一个语句中x是未定义的，其**可能是nil，也可能是上面的两个x之一，当然也可能是个混合体：指针来自于第一个make，长度来自于第二个make**，这种定义雷区被称为`未定义行为`，所以并发程序的概念并不是简单的语句交叉执行。

数据竞争如何才能避免呢？有三种方式：

1. 不要修改变量
   考虑如下的map，它进行了延迟初始化，对于每个键，在第一次访问时才触发加载。如果Icon的调用是串行的，那么程序能进行正常工作，否则就会出现数据竞态：

   ```go
   var icons = make(map[string]image.Image) {}
   func loadIcon(name string) image.Image()
   
   func Icon(name string) image.Image [
   	icon,ok := icons[name]
   	if !ok {
   		icon = loadIcon(name)
   		icons[name] = icon
   	}
   	return icon
   }
   ```

   但是如果初始化了变量，只读情况下就不存在数据竞争，因为每个goroutine都只读取这个map：

   ```go
   var icons = make(map[string]image.Image) {
   		"spades.png": loadIcon("spades.png")
   		"hearts.png": loadIcon("hearts.png")
   		"diamonds.png": loadIcon("diamonds.png")
   		"clubs.png": loadIcon("clubs.png")
   }
   
   func Icon(name string) image.Image { return icons[name] }
   ```

2. 避免多个goroutine访问变量

   让其他goroutine无法直接访问相关变量，使用通道来向受限的goroutine发送查询请求或者更新变量。这也是这句Go箴言的含义：**不要通过共享内存来通信，而应该通过通信来共享内存**。

   使用通道请求来代理一个受限变量的所有访问的goroutine称为该变量的`监控goroutine`。

   下面就是重写的银行案例，其实前面使用的通道技术就是基于这个点来避免数据竞争的：

   > `select`函数是Go语言中的一个关键字，用于在多个通道上进行非阻塞的选择操作。`select`函数会等待多个通道中的一个通道准备就绪，然后执行相应的操作。如果多个通道都准备就绪，`select`函数会随机选择一个通道执行相应的操作。通过channel和select可以使得我们能够达到避免数据竞争的问题

   ```go
   package bank
   
   var deposits = make(chan int)
   var balances = make(chan int)
   
   func Deposit(amount int) { deposits <- amount}
   func Balance() int { return <- balances}
   
   func teller() {
   	var balance  int
   	for {
   		select {
   		case amount := <- deposits:
   			balance += amount
   		case balances <- balance:
   		}
   	}
   }
   
   func init() {
   	go teller()
   }
   ```

   通过channel和select可以使得我们能够达到避免数据竞争的问题

