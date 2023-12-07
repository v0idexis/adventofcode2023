const fs = require('fs');

const r = /\d+/g;

let data = fs.readFileSync('d1.txt', 'utf8').split('\n').map(item => {
    if (item.length === 0) { console.log('found empty'); return }
    let newStr = item.match(r).join('');
    return newStr[0] + newStr[newStr.length - 1]
});

console.log(data)
let sum = data.reduce((total, num) => { total += Number(num); return total }, 0)
console.log(sum);