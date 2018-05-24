const fs = require('fs');
const readline = require('readline');

module.exports = class AgeReducer {
  constructor(filepath) {
    this.filepath = filepath;
    this.ages = {};
    this.users = {};
  }

  getAgeCounts() {
    this.parseFile().then(this.printOutput);
  }

  parseFile() {
    return new Promise(resolve => {
      readline
        .createInterface({ input: fs.createReadStream(this.filepath) })
        .on('line', line => this.lineHandler(line))
        .on('close', () => resolve(this.ages));
    });
  }

  lineHandler(line) {
    const [id, age] = line.split(',');
    if (!this.users[id]) {
      this.users[id] = true;
      this.ages[age] = (this.ages[age] || 0) + 1;
    }
  }

  printOutput(ages) {
    Object.keys(ages).forEach(age => {
      console.log(`${age},${ages[age]}`);
    });
  }
};
