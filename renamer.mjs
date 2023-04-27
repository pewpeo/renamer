#!/usr/bin/env node

import { renameFilepathStr } from './rename.js';
import * as fs from 'node:fs';

// remove node executable and mjs filename
const args = process.argv.slice(2);

if (args.length < 1) {
    console.error('Please provide file argument(s).')
    process.exit(1);
}

try {
    args.forEach((filepath) => {
        const newFilepath = renameFilepathStr(filepath);

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
