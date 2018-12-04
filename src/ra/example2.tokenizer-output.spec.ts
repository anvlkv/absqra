import { RaToken, TokenType } from './token-stream/token';
import { RaTypes } from './types.enum';


export const example2TokenizerOutput: RaToken[] = [
    {
        start: [
            1,
            1
        ],
        end:[
            1,
            13
        ],
        tokenType:TokenType.VAR,
        value: 'declaredName',
        type: RaTypes.TOKEN
    },
    {
        start: [
            1,
            13
        ],
        end:[
            1,
            14
        ],
        tokenType:TokenType.OP,
        value: ':',
        type: RaTypes.TOKEN
    },
    {
        start: [
            1,
            15
        ],
        end:[
            1,
            23
        ],
        tokenType:TokenType.VAR,
        value: 'Sequence',
        type: RaTypes.TOKEN
    },
    {
        start: [
            2,
            5
        ],
        end:[
            2,
            13
        ],
        tokenType:TokenType.KW,
        value: 'question',
        type: RaTypes.TOKEN
    },
    {
        start: [
            2,
            14
        ],
        end:[
            2,
            24
        ],
        tokenType:TokenType.VAR,
        value: 'outputName',
        type: RaTypes.TOKEN
    },
    {
        start: [
            4,
            9
        ],
        end:[
            4,
            10
        ],
        tokenType:TokenType.OP,
        value: '>',
        type: RaTypes.TOKEN
    },
    {
        start: [
            4,
            10
        ],
        end:[
            4,
            14
        ],
        tokenType:TokenType.VAR,
        value: 'text',
        type: RaTypes.TOKEN
    },
    {
        end: [
            5,
            8
        ],
        start: [
            5,
            1
        ],
        tokenType: TokenType.VAR,
        type: RaTypes.TOKEN,
        value: 'content',
    },
    {
        end: [
            5,
            20
        ],
        start: [
            5,
            9
        ],
        tokenType: TokenType.INLINE_CONTENT,
        type: RaTypes.TOKEN,
        value: '`some text`',
    },
    {
        end: [
            5,
            34
        ],
        start: [
            5,
            21
        ],
        tokenType: TokenType.VAR,
        type: RaTypes.TOKEN,
        value: 'unexpectedVar',
    },
    {
        end: [
            6,
            8
        ],
        start: [
            6,
            1
        ],
        tokenType: TokenType.VAR,
        type: RaTypes.TOKEN,
        value: 'textVar',
    },
    {
        end: [
            6,
            10
        ],
        start: [
            6,
            9
        ],
        tokenType: TokenType.OP,
        type: RaTypes.TOKEN,
        value:'=',
    },
    {
        end: [
            6,
            28
        ],
        start: [
            6,
            11
        ],
        tokenType: TokenType.INLINE_CONTENT,
        type: RaTypes.TOKEN,
        value: '`some other text`',
    }
];
