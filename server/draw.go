package main

import (
    "fmt"
)

const banner string = `
 ┌───────────────────────────────────────────────────┐ 
 │               IST Simulator Server                │ 
 │            __     ______     ______               │ 
 │           /\ \   /\  ___\   /\  ___\              │ 
 │           \ \ \  \ \___  \  \ \___  \             │ 
 │            \ \_\  \/\_____\  \/\_____\            │ 
 │             \/_/   \/_____/   \/_____/            │ `

const col = 51

func DrawStart() {
    fmt.Println(banner)
}

func DrawLine(name string, value string) {
    numDots := col - (len(name) + len(value) + 4)
    if numDots <= 0 {
        fmt.Printf(" │ %*s │ \n", - col + 2, name)
        fmt.Printf(" │ %*s │ \n", col - 2, value)
    } else {
        fmt.Printf(" │ %s ", name)
        for range numDots {
            fmt.Print(".")
        }
        fmt.Printf(" %s │ \n", value)
    }
}

const sectionHead string = ` └───────────╮   ╭═══════════════════╮   ╭───────────┘ `

const sectionFoot string = ` ┌───────────╯   ╰═══════════════════╯   ╰───────────┐ `

func DrawSection(name string) {
    fmt.Printf(" │%*s│ \n", col, "")
    fmt.Println(sectionHead)
    w:=19
    name = fmt.Sprintf("%*s", -w, fmt.Sprintf("%*s", (w + len(name))/2, name))
    fmt.Printf("             ╞═══┤%s├═══╡\n", name)
    fmt.Println(sectionFoot)
    fmt.Printf(" │%*s│ \n", col, "")
}

func DrawEnd() {
    fmt.Printf(" │%*s│ \n", col, "")
     fmt.Print(" └───────────────────────────────────────────────────┘ ")
}
