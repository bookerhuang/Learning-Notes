package main

import (
	"fmt"
	"syscall"
)

var err error = syscall.Errno(2)

func main() {
	fmt.Println(err.Error()) // "no such file or directory"
	fmt.Println(err)         // "no such file or directory"
}
