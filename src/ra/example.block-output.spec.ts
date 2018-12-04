import { RaTypes } from './types.enum';
import { BlockType } from './block-stream/block';


export const exampleBlockOutput = [
    {
        blockType: BlockType.CONTENT,
        level: 0,
        content: [
            {indent: 0, start: [1, 1], end: [1, 2], value:'`\n'},
            {indent: 1, start: [2, 1], end: [2, 20], value:'\tthis is one string\n'},
            {indent: 1, start: [3, 1], end: [3, 19], value:'\tstill same string\n'},
            {indent: 1, start: [4, 1], end: [4, 12], value:'\t`even now`\n'},
            {indent: 2, start: [5, 1], end: [5, 16], value:'\t\tand `now` too\n'},
            {indent: 1, start: [6, 1], end: [6, 23], value:'\t        and `now` too\n'},
            {indent: 0, start: [7, 1], end: [7, 2], value:'`\n'},
        ],
        start: [
            1,
            1
        ],
        end: [
            7,
            2
        ],
        type: RaTypes.BLOCK
    },
    {
        content: [
            {indent: 0, start: [8, 1], end: [8, 54], value:'`this is [another string](http://www.internet.earth)`'}

        ],
        start: [
            8,
            1
        ],
        end: [
            8,
            54
        ],
        type: RaTypes.BLOCK,
        blockType: BlockType.CONTENT,
        level: 0
    }
];
