const fs = require('fs');

const r = /\d+/g;
const numStrings = { 'one': '1', 'two': '2', 'three': '3', 'four': '4', 'five': '5', 'six': '6', 'seven': '7', 'eight': '8', 'nine': '9' };
const numKeys = Object.keys(numStrings);
const numValues = Object.values(numStrings);

function numbered(string) {
    let retString = [];
    for (let i = 0; i < string.length; i++) {
        if (numValues.includes(string[i])) {
            // console.log(string[i]);
            retString.push(string[i]);
        }
        else {
            let tempStr = string.slice(i, string.length);
            for (let key of numKeys) {
                if (tempStr.startsWith(key)) {
                    retString.push(numStrings[key]);
                    // i += (key.length - 1);
                    break;
                }
            }
        }
    }
    // console.log('Log:', string, retString.join(''))
    return retString.length > 0 ? retString.join('') : '0'
}

let data = fs.readFileSync('d1.txt', 'utf8').split('\n').map(item => {
    if (item.length === 0) { return 0 }
    // let newStr = numbered(item).match(r).join('');
    let newStr = numbered(item);
    // console.log('newStr', item, newStr, newStr[0] + newStr[newStr.length - 1])
    return newStr[0] + newStr[newStr.length - 1]
});
// data.forEach(item => console.log(item));
let sum = data.reduce((total, num) => { total += Number(num); return total }, 0)
console.log(sum);

// console.log('Numbered: znlgdncm99twofmhmftsnlk', numbered('znlgdncm99twofmhmftsnlk'));