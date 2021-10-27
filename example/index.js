import { Matcher } from 'wasm-fuzzy'

const matcher = Matcher.from_documents([
    {content: "hello world",key:"2"},
    {content: "goodbye world",key:"3"},
])
console.log(matcher.indices("hell"))
console.log(matcher.indices("world"))
