package main

import (
	"flag"
	"fmt"
	"log"
	"net"
	"os"
	"os/exec"
	"strconv"
	"strings"
	"time"
)

func hash(input string) int32 {
	var hash int32
	hash = 5381
	for _, char := range input {
		hash = ((hash << 5) + hash + int32(char))
	}
	if hash < 0 {
		hash = 0 - hash
	}
	return hash
}

func runCmd(cmdstring string) {
	parts := strings.Split(cmdstring, " ")
	cmd := exec.Command(parts[0], parts[1:len(parts)]...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		log.Fatalf("The following command failed: \"%v\"\n", cmdstring)
	}
}

func outputCmd(argv []string) string {
	cmd := exec.Command(argv[0], argv[1:len(argv)]...)
	cmd.Stderr = os.Stderr
	output, err := cmd.Output()
	if err != nil {
		log.Fatalf("The following command failed: \"%v\"\n", argv)
	}
	return string(output)
}

func startCmd(cmdstring string) {
	parts := strings.Split(cmdstring, " ")
	cmd := exec.Command(parts[0], parts[1:len(parts)]...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Stdin = os.Stdin
	err := cmd.Start()
	if err != nil {
		log.Fatalf("The following command failed: \"%v\"\n", cmdstring)
	}
}

type Mode interface {
	getEntities() []string
	deleteEntity(string)
}

type podmode struct {
}

var podsMap = make(map[string]PodSpec)

type PodSpec struct {
	Name string
	Namespace string
}

func (m podmode) getEntities() []string {
	args := []string{"kubectl", "get", "pods", "-A", "-l", "kubedoom=y1", "-o", "go-template", "--template={{range .items}}{{.metadata.namespace}}/{{.metadata.name}} {{end}}"}
	output := outputCmd(args)
	outputstr := strings.TrimSpace(output)
	pods := strings.Split(outputstr, " ")

	podsRenamed := []string{}
	podsMap = make(map[string]PodSpec)
	for _, pod := range pods {
		podAndNamespace := strings.Split(pod, "/")
		podNames := strings.Split(podAndNamespace[1], "-")
		identifier := podNames[2][0:4]
		name := fmt.Sprintf("%s-%s", podNames[0], identifier)

		fullPodSpec := PodSpec{
			Name:      podAndNamespace[1],
			Namespace: podAndNamespace[0],
		}
		podsMap[name] = fullPodSpec
		podsRenamed = append(podsRenamed, name)
	}

	return podsRenamed
}

func (m podmode) deleteEntity(entity string) {
	log.Printf("Pod to kill: %v", entity)
	pod := podsMap[entity]
	fmt.Printf("Pod: %s, Namespace: %s\n", pod.Name, pod.Namespace)
	cmd := exec.Command("/usr/bin/kubectl", "delete", "pod", "-n", pod.Namespace, pod.Name)
	go cmd.Run()
}

type nsmode struct {
}

func (m nsmode) getEntities() []string {
	args := []string{"kubectl", "get", "namespaces", "-o", "go-template", "--template={{range .items}}{{.metadata.name}} {{end}}"}
	output := outputCmd(args)
	outputstr := strings.TrimSpace(output)
	namespaces := strings.Split(outputstr, " ")
	return namespaces
}

func (m nsmode) deleteEntity(entity string) {
	log.Printf("Namespace to kill: %v", entity)
	cmd := exec.Command("/usr/bin/kubectl", "delete", "namespace", entity)
	go cmd.Run()
}

func socketLoop(listener net.Listener, mode Mode) {
	for true {
		conn, err := listener.Accept()
		if err != nil {
			panic(err)
		}
		stop := false
		for !stop {
			bytes := make([]byte, 40960)
			n, err := conn.Read(bytes)
			if err != nil {
				stop = true
			}
			bytes = bytes[0:n]
			strbytes := strings.TrimSpace(string(bytes))
			entities := mode.getEntities()
			if strbytes == "list" {
				for _, entity := range entities {
					padding := strings.Repeat("\n", 255-len(entity))
					_, err = conn.Write([]byte(entity + padding))
					if err != nil {
						log.Fatal("Could not write to socker file")
					}
				}
				conn.Close()
				stop = true
			} else if strings.HasPrefix(strbytes, "kill ") {
				parts := strings.Split(strbytes, " ")
				killhash, err := strconv.ParseInt(parts[1], 10, 32)
				if err != nil {
					log.Fatal("Could not parse kill hash")
				}
				for _, entity := range entities {
					if hash(entity) == int32(killhash) {
						mode.deleteEntity(entity)
						break
					}
				}
				conn.Close()
				stop = true
			}
		}
	}
}

func main() {
	var modeFlag string
	flag.StringVar(&modeFlag, "mode", "pods", "What to kill pods|namespaces")

	flag.Parse()

	var mode Mode
	switch modeFlag {
	case "pods":
		mode = podmode{}
	case "namespaces":
		mode = nsmode{}
	default:
		log.Fatalf("Mode should be pods or namespaces")
	}

	listener, err := net.Listen("unix", "/dockerdoom.socket")
	if err != nil {
		log.Fatalf("Could not create socket file")
	}

	log.Print("Create virtual display")
	startCmd("/usr/bin/Xvfb :99 -ac -screen 0 640x480x24")
	time.Sleep(time.Duration(2) * time.Second)
	startCmd("x11vnc -geometry 640x480 -forever -usepw -display :99")
	log.Print("You can now connect to it with a VNC viewer at port 5900")

	log.Print("Trying to start DOOM ...")
	startCmd("/usr/bin/env DISPLAY=:99 /usr/local/games/psdoom -warp -E1M1")
	socketLoop(listener, mode)
}
