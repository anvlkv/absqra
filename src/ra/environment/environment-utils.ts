import { Environment } from './ra.environment';

export class EnvironmentUtils {
    static get indentationRegExp () {
        return new RegExp(`^${Environment.indentationCharacter}${Environment.indentationCharacter}*`, 'gm');
    }
    static get indentationCharacterRegExp () {
        return new RegExp(`^${Environment.indentationCharacter}`);
    }
}