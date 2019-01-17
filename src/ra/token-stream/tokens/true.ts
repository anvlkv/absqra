import { Token, TokenType } from '../token';
import { LineColumnAddress } from '../../line-column-address';


export class TrueToken extends Token {
    constructor(start: LineColumnAddress, end: LineColumnAddress){
        super(
            TokenType.KW,
            true,
            start,
            end
        );
    }
}