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

```shell
hsl% nc localhost 8000 
21:27:58
21:27:59
21:28:00
21:28:01
21:28:02
21:28:03
```

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