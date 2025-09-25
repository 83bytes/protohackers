package main

import (
	"fmt"
	"io"
	"log"
	"net"
)

const (
	// the RFC 862 specifies the port as 7
	port = "7777"
	ip   = "0.0.0.0"
)

func main() {
	listener, err := net.Listen("tcp", fmt.Sprintf("%s:%s", ip, port))
	if err != nil {
		log.Fatal(err)
	}
	defer listener.Close()

	for {
		conn, err := listener.Accept()
		if err != nil {
			log.Fatal(err)
		}
		go handleRequest(conn)

	}
}

func handleRequest(c net.Conn) {
	defer c.Close()

	io.Copy(c, c)
}
