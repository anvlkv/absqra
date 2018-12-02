import { RaTypes } from '../types.enum';
import { LineColumnAddress } from '../block-stream/block';


export enum TokenType {
   INDENT = 'indent',
   INLINE_CONTENT = 'inline',
   NUM = 'num',
   PUNCT = 'punct',
   KW = 'kw',
   VAR = 'var',
   OP = 'op',
   BLK = 'block'
}

export class RaToken {
    type = RaTypes.TOKEN;

    constructor(
        public tokenType: TokenType,
        public value: any,
        public start: LineColumnAddress,
        public end: LineColumnAddress,
    ){}
}