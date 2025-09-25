package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
)

type Server struct {
	host string
	port string
}

type Client struct {
	conn net.Conn
}

type Config struct {
	Host string
	Port string
}

func New(config *Config) *Server {
	return &Server{
		host: config.Host,
		port: config.Port,
	}
}

func (server *Server) Run() {
	listener, err := net.Listen("tcp", fmt.Sprintf("%s:%s", server.host, server.port))
	if err != nil {
		log.Fatal(err)
	}
	defer listener.Close()

	for {
		conn, err := listener.Accept()
		if err != nil {
			log.Fatal(err)
		}

		client := &Client{
			conn: conn,
		}

		// This is a blocking call. If we do not run this as a go-routine
		// then we will be processing this as a serial one-after-the-other
		go client.handleRequest()

	}
}

func (client *Client) handleRequest() {
	reader := bufio.NewReader(client.conn)
	defer client.conn.Close()

	connSrc := client.conn.RemoteAddr()
	fmt.Printf("handling new connection from : %s", connSrc)
	for {
		message, err := reader.ReadString('\n')
		if err != nil {
			fmt.Printf("closing connection from : %s", connSrc)
			return
		}

		fmt.Printf("Incoming from: %s, Msg:  %s", connSrc, string(message))
		client.conn.Write([]byte(fmt.Sprintf("message recvd from: %s", connSrc)))
	}
}

func main() {
	server := New(&Config{
		Host: "localhost",
		Port: "3333",
	})
	server.Run()

}
