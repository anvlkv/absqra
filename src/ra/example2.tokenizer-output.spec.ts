import { RaToken, TokenType } from './token-stream/token';
import { RaTypes } from './types.enum';


export const example2TokenizerOutput: RaToken[] = [
    {
        start: [
            1,
            0
        ],
        end:[
            1,
            12
        ],
        tokenType:TokenType.VAR,
        value: 'declaredName',
        type: RaTypes.TOKEN
    },
    {
        start: [
            1,
            12
        ],
        end:[
            1,
            13
        ],
        tokenType:TokenType.OP,
        value: ':',
        type: RaTypes.TOKEN
    },
    {
        start: [
            1,
            14
        ],
        end:[
            1,
            22
        ],
        tokenType:TokenType.VAR,
        value: 'Sequence',
        type: RaTypes.TOKEN
    },
    {
        start: [
            2,
            4
        ],
        end:[
            2,
            12
        ],
        tokenType:TokenType.KW,
        value: 'question',
        type: RaTypes.TOKEN
    },
    {
        start: [
            2,
            13
        ],
        end:[
            2,
            23
        ],
        tokenType:TokenType.VAR,
        value: 'outputName',
        type: RaTypes.TOKEN
    },
    {
        start: [
            4,
            8
        ],
        end:[
            4,
            9
        ],
        tokenType:TokenType.OP,
        value: '>',
        type: RaTypes.TOKEN
    },
    {
        start: [
            4,
            9
        ],
        end:[
            4,
            13
        ],
        tokenType:TokenType.VAR,
        value: 'text',
        type: RaTypes.TOKEN
    },
    {
        end: [
            5,
            7
        ],
        start: [
            5,
            0
        ],
        tokenType: TokenType.VAR,
        type: RaTypes.TOKEN,
        value: 'content',
    },
    {
        end: [
            5,
            19
        ],
        start: [
            5,
            8
        ],
        tokenType: TokenType.INLINE_CONTENT,
        type: RaTypes.TOKEN,
        value: '`some text`',
    },
    {
        end: [
            5,
            33
        ],
        start: [
            5,
            20
        ],
        tokenType: TokenType.VAR,
        type: RaTypes.TOKEN,
        value: 'unexpectedVar',
    },
    {
        end: [
            6,
            7
        ],
        start: [
            6,
            0
        ],
        tokenType: TokenType.VAR,
        type: RaTypes.TOKEN,
        value: 'textVar',
    },
    {
        end: [
            6,
            9
        ],
        start: [
            6,
            8
        ],
        tokenType: TokenType.OP,
        type: RaTypes.TOKEN,
        value:'=',
    },
    {
        end: [
            6,
            27
        ],
        start: [
            6,
            10
        ],
        tokenType: TokenType.INLINE_CONTENT,
        type: RaTypes.TOKEN,
        value: '`some other text`',
    }
];
