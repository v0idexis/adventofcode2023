const fs = require('fs');

let lowest = null;
let data = fs.readFileSync('input', 'utf8').split(/\r?\n\r?\n/).map((item, idx) => {
    if (item.length === 0) { console.log('found empty'); return '' }

    let final = '';
    if (0 != idx) {
        let item2 = item.split(/\r?\n/);
        final = item2.filter((item, idx) => {
            if (0 != idx) {
                // return item.split(/\r?\n/);
                // let numstring = item.split(/\r?\n/);
                return item;

            }
        })
        // console.log(final)
        final = final.map(item => item.split(' ').map(num => parseInt(num)));
    }
    else {
        final = item.split(': ')[1].split(' ').map(num => parseInt(num));
    }

    // let newStr = item.match(r).join('');
    // return newStr[0] + newStr[newStr.length - 1]
    return final;
});

let seeds = data[0];
let seedMap = data.slice(1, data.length);
let locationArr = [];

console.log(seedMap)
// Loop through seeds
seeds.forEach(seed => {
    console.log('Active seed: ' + seed);
    let activeItem = seed;
    let lastActiveIdx = 0;
    // Loop through maps
    seedMap.forEach((item, idx) => {
        let found = false;
        //Loop through map items
        item.forEach((subItem, subIdx) => {
            if (false == found) {
                // console.log(subItem);
                let startBeg = subItem[1];
                let startEnd = subItem[1] + subItem[2];
                if (startBeg <= activeItem && activeItem <= startEnd) {
                    //calculating the next start item position
                    activeItem = parseInt(subItem[0] + (activeItem - subItem[1]));
                    // console.log(activeItem)
                    lastActiveIdx = idx;
                    found = true;
                }
            }
        })
    })
    console.log(`LastActiveIdx = ${lastActiveIdx}, Active Item = ${activeItem}`)
    // if (lastActiveIdx == 6) { locationArr.push(activeItem); }
    // else if (lastActiveIdx == 0) { locationArr.push(activeItem); }
    locationArr.push(activeItem);
})

console.log(locationArr);
console.log(Math.min(...locationArr));