package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
)

type node struct {
	prnt *node
	left *node
	rite *node
	line *[]byte
}

func main() {

	root := &node{}
	head := root

	// first node
	scan := bufio.NewScanner(os.Stdin)
	if scan.Scan() {
		bytes := scan.Bytes()
		root.line = &bytes
	} else {
		os.Exit(1)
	}

	// populate nodes
	for scan.Scan() {

		line := scan.Bytes()

		// traverse tree
		for {
			rc := bytes.Compare(line, *head.line)

			var next *node
			var next_p **node

			// determine direction
			if rc <= 0 {
				next = head.left
				next_p = &head.left
			} else {
				next = head.rite
				next_p = &head.rite
			}

			// write out if unoccupied
			if next == nil {
				next = &node{
					prnt: head,
					line: &line,
				}
				*next_p = next
				head = root
				break
			}

			head = next
		}

		head = root
	}

	// get to first sorted
	for head.left != nil {
		head = head.left
	}

	// climb and print
	for root != nil {
		// traverse left
		if head.left != nil {
			head = head.left
			continue
		}

		// print self
		if head.line != nil {
			fmt.Println(string(*head.line))
			head.line = nil
			continue
		} else if head.rite != nil {
			head = head.rite
			continue
		}

		if head == root {
			break
		}

		prnt := head.prnt
		if prnt.left == head {
			prnt.left = nil
		} else {
			prnt.rite = nil
		}

		head = nil
		head = prnt
	}
}
