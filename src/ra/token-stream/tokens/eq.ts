import { Token, TokenType } from '../token';
import { LineColumnAddress } from '../../line-column-address';


export class EQToken extends Token {
    constructor(start: LineColumnAddress, end: LineColumnAddress){
        super(
            TokenType.OP,
            '=',
            start,
            end
        );
    }
}