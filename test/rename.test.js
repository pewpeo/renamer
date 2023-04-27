import { equal } from 'assert';
import { renameFileStr } from '../rename.mjs';


const testData = [
    { filename: 'test 123 ü `',    date: new Date('2023-04-25'), result: '2023-04-25_test_123_ue_' },
    { filename: 'test    123 ü `', date: new Date('2023-04-25'), result: '2023-04-25_test_123_ue_' },
    { filename: '1  è',            date: new Date('2023-04-25'), result: '2023-04-25_1_' },
    { filename: 'this-is-a-test',  date: new Date('2023-04-25'), result: '2023-04-25_this-is-a-test' },
    { filename: 'this-is-a--test', date: new Date('2023-04-25'), result: '2023-04-25_this-is-a_test' },
];

describe('Rename', function () {
    describe('#renameFileStr()', function () {
        it('Replace umlauts, non-ascii, ascii special characters and spaces (\s) with "_" and prepend the date', function () {
            testData.forEach((testCase) => {
                equal(renameFileStr(testCase.filename, testCase.date), testCase.result);
            });
        });
    });
});
