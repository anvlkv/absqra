import { RaLine } from './line-stream/line';


export const exampleLineOutput = [
    {indent: 0, start: [1, 1], end: [1, 2], _value:'`\n'},
    {indent: 1, start: [2, 1], end: [2, 23], _value:'\tthis is one string\n'},
    {indent: 1, start: [3, 1], end: [3, 22], _value:'\tstill same string\n'},
    {indent: 1, start: [4, 1], end: [4, 15], _value:'\t`even now`\n'},
    {indent: 2, start: [5, 1], end: [5, 22], _value:'\t\tand `now` too\n'},
    {indent: 1, start: [6, 1], end: [6, 26], _value:'\t        and `now` too\n'},
    {indent: 0, start: [7, 1], end: [7, 2], _value:'`\n'},
    {indent: 0, start: [8, 1], end: [8, 54], _value:'`this is [another string](http://www.internet.earth)`'}
];
