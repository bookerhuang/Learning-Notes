// 写一个快速排序
package main

import (
	"fmt"
	"math/rand"
	"time"
)

func quicksort(a []int, left, right int) {
	if left >= right {
		return
	}
	i := left
	j := right
	key := a[left]
	for i < j {
		for i < j && a[j] >= key {
			j--
		}
		a[i] = a[j]
		for i < j && a[i] <= key {
			i++
		}
		a[j] = a[i]
	}
	a[i] = key
	quicksort(a, left, i-1)
	quicksort(a, i+1, right)
}

func main() {
