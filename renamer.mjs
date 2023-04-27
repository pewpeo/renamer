#!/usr/bin/env node

import { exec, execSync, spawn } from 'node:child_process';
import * as path from 'node:path';
import * as fs from 'node:fs';

// remove node executable and mjs filename
const args = process.argv.slice(2);

if (args.length < 1) {
    console.error('Please provide file argument(s).')
    process.exit(1);
}

const date = new Date(); // current date

const umlautList = [
    { letter: '\u00dc', base: 'Ue' },
    { letter: '\u00c4', base: 'Ae' },
    { letter: '\u00d6', base: 'Oe' },
    { letter: '\u00fc', base: 'ue' },
    { letter: '\u00e4', base: 'ae' },
    { letter: '\u00f6', base: 'oe' },
    { letter: '\u00df', base: 'ss' },
];
function replaceUmlauts(str) {
    umlautList.forEach((umlaut) => {
        str = str.replaceAll(umlaut.letter, umlaut.base);
    });
    return str;
}

try {
    args.forEach((filepath) => {
        // check if files exit
        const stats = fs.lstatSync(filepath);
        if (!stats.isFile() && !stats.isDirectory()) {
            throw new Error(`Error: ${filepath} is not a file or directory.`);
        }

        const direcory = path.dirname(filepath);
        const extname = path.extname(filepath);
        let filename = path.basename(filepath, extname);
        filename = replaceUmlauts(filename);
        filename = filename.replaceAll(' ', '_');
        filename = date.toISOString().substring(0, 10) + '_' + filename; // prepend date
        const newFilepath = path.join(direcory, filename + extname);
        
        fs.rename(filepath, newFilepath, (err) => {
            console.log(`mv "${filepath}" "${newFilepath}"`);
            if (err) {
                throw err;
            }
        });
    });
} catch (error) {
    console.error(error.message);
    process.exit(1);
}
