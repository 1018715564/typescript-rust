import "main.js"
import { stan, stoo as superStoo } from "./stanFactory"
// import { print } from "./mymodule"

const an = 0
let bu = 10 + 8

print(an)
print(bu)

if (an < bu) {
    bu = 0
}
else if (an > bu) {
    bu = 3
}
else {
    bu = 1
}

print(bu)

interface TheInterface {
    needHelp(nice: boolean)

    superDuperHelper(): Promise<string>
}

class TheClass implements TheInterface {
    help: boolean = false

    needHelp(nice: boolean = true) {
        // return this.help
    }

    async superDuperHelper(): Promise<string> {
        return "it helped realy well"
    }
}

const dave = (): string => {
    return "name"
}

type MyType = {
    must: string
}

type MyTypeToo = {can: boolean} | MyType

function doStuff<T extends number | string>(one: T): number {
    print("stuff")

    return 5
}

const result = doStuff(1)
print(result)