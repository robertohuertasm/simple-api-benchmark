package main

import (
	"fmt"
	"net/http"
)

func main() {

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello Word")
	})
	fmt.Println("Server started in port 8080")
	http.ListenAndServe(":8080", nil)
}
