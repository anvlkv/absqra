import { Token, TokenType } from '../token';
import { LineColumnAddress } from '../../line-column-address';


export class FalseToken extends Token {
    constructor(start: LineColumnAddress, end: LineColumnAddress){
        super(
            TokenType.KW,
            false,
            start,
            end
        );
    }
}