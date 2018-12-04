import { RaTypes } from '../types.enum';
import { LineColumnAddress } from '../line-column-address';



export enum TokenType {
   ML_COMMENT_START = 'ml',
   INLINE_CONTENT = 'inline',
   NUM = 'num',
   PUNCT = 'punct',
   KW = 'kw',
   VAR = 'var',
   OP = 'op',
   // BLK = 'block'
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