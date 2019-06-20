import { Token, TokenType } from './token';
import { RaTypes } from '../types.enum';


export const example2TokenizerOutput = [
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
    }, // 0
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
    }, // 1
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
    }, // 2
    {
        start: [
            2,
            5
        ],
        end:[
            2,
            13
        ],
        tokenType:TokenType.VAR,
        value: 'question',
        type: RaTypes.TOKEN
    }, // 3
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
    }, // 4
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
    }, // 5
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
    }, // 6
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
    }, // 7
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
        value: 'some text',
    }, // 8
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
    }, // 9
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
    }, // 10
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
    }, // 11
    {
        end: [
            6,
            30
        ],
        start: [
            6,
            11
        ],
        tokenType: TokenType.INLINE_CONTENT,
        type: RaTypes.TOKEN,
        value: 'some othe`r text',
    }, // 12
    {
        end: [
            7,
            33
        ],
        start: [
            7,
            1
        ],
        tokenType: TokenType.REG_EXP,
        type: RaTypes.TOKEN,
        value: '/[0-9][a-z].*^&?!Reg\\\\E\\/**\\/xp/'
    } //13
];
