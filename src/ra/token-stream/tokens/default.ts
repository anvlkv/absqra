import { Token, TokenType } from '../token';
import { LineColumnAddress } from '../../line-column-address';


export class DefaultToken extends Token {
    constructor(start: LineColumnAddress, end: LineColumnAddress){
        super(
            TokenType.KW,
            'default',
            start,
            end
        );
    }
}