package main

import (
	"fmt"
	"sort"
	"strings"
)

func main() {
	var n, l int

	var sarray []string

	fmt.Scan(&n, &l)

	for i := 0; i < n; i++ {
		var s string
		fmt.Scan(&s)

		sarray = append(sarray, s)
	}

	sort.Strings(sarray)

	ss := strings.Join(sarray[:], "")

	fmt.Println(ss)

}
