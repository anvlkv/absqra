import { Token, TokenType } from '../token';
import { LineColumnAddress } from '../../line-column-address';


export class LangToken extends Token {
    constructor(start: LineColumnAddress, end: LineColumnAddress){
        super(
            TokenType.KW,
            'lang',
            start,
            end
        );
    }
}