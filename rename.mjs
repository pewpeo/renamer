import * as path from 'node:path';
import * as fs from 'node:fs';

const umlautList = [
    { letter: '\u00dc', base: 'Ue' },
    { letter: '\u00c4', base: 'Ae' },
    { letter: '\u00d6', base: 'Oe' },
    { letter: '\u00fc', base: 'ue' },
    { letter: '\u00e4', base: 'ae' },
    { letter: '\u00f6', base: 'oe' },
    { letter: '\u00df', base: 'ss' },
];

export function replaceUmlauts(str) {
    umlautList.forEach((umlaut) => {
        str = str.replaceAll(umlaut.letter, umlaut.base);
    });
    return str;
}

export function renameFileStr(filename, date = new Date()) {
    filename = replaceUmlauts(filename);
    // replace with '_':
    //   - all non-ascii
    //   - the listet ascii characters and spaces (\s)
    //   - more than one '1'
    filename = filename.replace(/[\u{0080}-\u{FFFF} \.,"!@#\$%\^&\*\(\)=\+;:<>\/\\\|\}\{\[\]`~\s]+|-{2,}/gu, "_");
    filename = date.toISOString().substring(0, 10) + '_' + filename; // prepend date

    return filename;
}

export function renameFilepathStr(filepath) {
    // check if files exit
    const stats = fs.lstatSync(filepath);
    if (!stats.isFile() && !stats.isDirectory()) {
        throw new Error(`Error: ${filepath} is not a file or directory.`);
    }

    const direcory = path.dirname(filepath);
    const extname = path.extname(filepath);
    let filename = path.basename(filepath, extname);
    filename = renameFileStr(filename)
    
    return path.join(direcory, filename + extname);
}
