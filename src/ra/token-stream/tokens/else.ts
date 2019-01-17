import { Token, TokenType } from '../token';
import { LineColumnAddress } from '../../line-column-address';


export class ElseToken extends Token {
    constructor(start: LineColumnAddress, end: LineColumnAddress){
        super(
            TokenType.KW,
            'else',
            start,
            end
        );
    }
}