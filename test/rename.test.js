import { equal } from 'assert';
import { renameFileStr } from '../rename.js';


const testData = [
    { filename: 'test 123 ü `',    date: new Date('2023-04-25'), result: '2023-04-25_test_123_ue_' },
    { filename: 'test    123 ü `', date: new Date('2023-04-25'), result: '2023-04-25_test_123_ue_' },
];

describe('Rename', function () {
    describe('#renameFileStr()', function () {
        it('Replace umlauts, 1+ ascii special characters and spaces (\s) with "_" and prepend the date', function () {
            testData.forEach((testCase) => {
                equal(renameFileStr(testCase.filename, testCase.date), testCase.result);
            });
        });
    });
});
