import { Token, TokenType } from '../token';
import { LineColumnAddress } from '../../line-column-address';


export class IfToken extends Token {
    constructor(start: LineColumnAddress, end: LineColumnAddress){
        super(
            TokenType.KW,
            'if',
            start,
            end
        );
    }
}