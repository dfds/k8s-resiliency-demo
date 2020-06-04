package main

import (
	"fmt"
	"github.com/hpcloud/tail"
	"log"
	"net/http"
	"os"
	"time"
	"github.com/urfave/negroni"
	"github.com/gorilla/mux"
	"github.com/gorilla/websocket"
)

var (
	upgrader  = websocket.Upgrader{
		ReadBufferSize:  2048,
		WriteBufferSize: 2048,
	}
	state = &State{
		clients: make(map[int]*websocket.Conn),
	}
	counter = 1
)

type State struct {
	clients map[int]*websocket.Conn
}

const (
	// Time allowed to read the next pong message from the client.
	pongWait = 60 * time.Second

	// Send pings to client with this period. Must be less than pongWait.
	pingPeriod = (pongWait * 9) / 10
)

func main() {
	// Background stats
	go watcher()
	//go testWriter() // Test data

	// Server
	router := mux.NewRouter()
	router.HandleFunc("/", func (w http.ResponseWriter, req *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		fmt.Fprintf(w, "HoooRay")
	})
	router.HandleFunc("/ws", func (w http.ResponseWriter, req *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		ws, err := upgrader.Upgrade(w, req, nil)
		if err != nil {
			if _, ok := err.(websocket.HandshakeError); !ok {
				log.Println(err)
			}
			return
		}

		err = ws.WriteMessage(1, []byte("It's alive!"))
		if err != nil {
			log.Fatal(err)
		}
		id := wsWriter(ws)
		wsReader(ws, id)
	})

	n := negroni.Classic()
	n.UseHandler(router)

	err := http.ListenAndServe(":3000", n)
	if err != nil {
		log.Fatal(err)
	}
}

// Data race all over the place
func wsWriter(ws *websocket.Conn) int{
	id := counter
	counter = counter + 1
	state.clients[id] = ws
	return id
}

func wsReader(ws *websocket.Conn, id int) {
	defer ws.Close()
	ws.SetCloseHandler(func(code int, text string) error {
		delete(state.clients, id)
		return nil
	})
	ws.SetReadLimit(512)
	ws.SetReadDeadline(time.Now().Add(pongWait))
	ws.SetPongHandler(func(string) error { ws.SetReadDeadline(time.Now().Add(pongWait)); return nil })
	for {
		_, _, err := ws.ReadMessage()
		if err != nil {
			break
		}
	}

	delete(state.clients, id)
}


func watcher() {
	for {
		time.Sleep(1 * time.Second)
		t, err := tail.TailFile("access.log", tail.Config{Follow: true})
		if err != nil {
			log.Println(err)
			return
		}

		for line := range t.Lines {
			fmt.Println(line.Text)
			for _, v := range state.clients {
				err = v.WriteMessage(1, []byte(line.Text))
				if err != nil {
					log.Fatal(err)
				}
			}
		}
	}
}

func testWriter() {
	for {
		time.Sleep(1 * time.Second)
		file, err := os.OpenFile("access.log", os.O_APPEND|os.O_RDONLY, 0644)
		if err != nil {
			log.Fatal(err)
		}

		defer file.Close()

		fWrite, err := file.WriteString("It's working")
		if err != nil {
			log.Fatal(err)
		}

		fmt.Printf("Wrote %d bytes\n", fWrite)
	}
}